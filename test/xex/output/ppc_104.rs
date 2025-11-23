pub fn sub_826A66B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A66B0 size=112
    let mut pc: u32 = 0x826A66B0;
    'dispatch: loop {
        match pc {
            0x826A66B0 => {
    //   block [0x826A66B0..0x826A6720)
	// 826A66B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A66B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A66B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A66BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A66C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A66C4: 392AC800  addi r9, r10, -0x3800
	ctx.r[9].s64 = ctx.r[10].s64 + -14336;
	// 826A66C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A66CC: 390B3748  addi r8, r11, 0x3748
	ctx.r[8].s64 = ctx.r[11].s64 + 14152;
	// 826A66D0: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826A66D4: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826A66D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A66DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A66E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A66E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A66E8: 386A8618  addi r3, r10, -0x79e8
	ctx.r[3].s64 = ctx.r[10].s64 + -31208;
	// 826A66EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A66F0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A66F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A66F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A66FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6704: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A670C: 4BDC0715  bl 0x82466e20
	ctx.lr = 0x826A6710;
	sub_82466E20(ctx, base);
	// 826A6710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A671C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6720 size=108
    let mut pc: u32 = 0x826A6720;
    'dispatch: loop {
        match pc {
            0x826A6720 => {
    //   block [0x826A6720..0x826A678C)
	// 826A6720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A672C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6730: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6734: 38EBBF08  addi r7, r11, -0x40f8
	ctx.r[7].s64 = ctx.r[11].s64 + -16632;
	// 826A6738: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A673C: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826A6740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6744: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6748: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A674C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6750: 386A8648  addi r3, r10, -0x79b8
	ctx.r[3].s64 = ctx.r[10].s64 + -31160;
	// 826A6754: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A675C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A676C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6774: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6778: 4BDC06A9  bl 0x82466e20
	ctx.lr = 0x826A677C;
	sub_82466E20(ctx, base);
	// 826A677C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6790 size=24
    let mut pc: u32 = 0x826A6790;
    'dispatch: loop {
        match pc {
            0x826A6790 => {
    //   block [0x826A6790..0x826A67A8)
	// 826A6790: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6794: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6798: 394A3838  addi r10, r10, 0x3838
	ctx.r[10].s64 = ctx.r[10].s64 + 14392;
	// 826A679C: 816BBF04  lwz r11, -0x40fc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16636 as u32) ) } as u64;
	// 826A67A0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826A67A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A67A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A67A8 size=112
    let mut pc: u32 = 0x826A67A8;
    'dispatch: loop {
        match pc {
            0x826A67A8 => {
    //   block [0x826A67A8..0x826A6818)
	// 826A67A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A67AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A67B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A67B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A67B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A67BC: 392AC830  addi r9, r10, -0x37d0
	ctx.r[9].s64 = ctx.r[10].s64 + -14288;
	// 826A67C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A67C4: 390B3838  addi r8, r11, 0x3838
	ctx.r[8].s64 = ctx.r[11].s64 + 14392;
	// 826A67C8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826A67CC: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826A67D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A67D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A67D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A67DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A67E0: 386A8678  addi r3, r10, -0x7988
	ctx.r[3].s64 = ctx.r[10].s64 + -31112;
	// 826A67E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A67E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A67EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A67F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A67F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A67F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A67FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6804: 4BDC061D  bl 0x82466e20
	ctx.lr = 0x826A6808;
	sub_82466E20(ctx, base);
	// 826A6808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A680C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6818 size=40
    let mut pc: u32 = 0x826A6818;
    'dispatch: loop {
        match pc {
            0x826A6818 => {
    //   block [0x826A6818..0x826A6840)
	// 826A6818: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A681C: 814BBF38  lwz r10, -0x40c8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16584 as u32) ) } as u64;
	// 826A6820: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6824: 396B3898  addi r11, r11, 0x3898
	ctx.r[11].s64 = ctx.r[11].s64 + 14488;
	// 826A6828: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826A682C: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 826A6830: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6834: 814ABF3C  lwz r10, -0x40c4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-16580 as u32) ) } as u64;
	// 826A6838: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 826A683C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6840 size=112
    let mut pc: u32 = 0x826A6840;
    'dispatch: loop {
        match pc {
            0x826A6840 => {
    //   block [0x826A6840..0x826A68B0)
	// 826A6840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A684C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A6850: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6854: 392AC9A8  addi r9, r10, -0x3658
	ctx.r[9].s64 = ctx.r[10].s64 + -13912;
	// 826A6858: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A685C: 390B3898  addi r8, r11, 0x3898
	ctx.r[8].s64 = ctx.r[11].s64 + 14488;
	// 826A6860: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826A6864: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826A6868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A686C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A6874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6878: 386A86A8  addi r3, r10, -0x7958
	ctx.r[3].s64 = ctx.r[10].s64 + -31064;
	// 826A687C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A6880: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826A6884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A688C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6894: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A689C: 4BDC0585  bl 0x82466e20
	ctx.lr = 0x826A68A0;
	sub_82466E20(ctx, base);
	// 826A68A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A68A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A68A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A68AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A68B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A68B0 size=108
    let mut pc: u32 = 0x826A68B0;
    'dispatch: loop {
        match pc {
            0x826A68B0 => {
    //   block [0x826A68B0..0x826A691C)
	// 826A68B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A68B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A68B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A68BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A68C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A68C4: 38EBBF44  addi r7, r11, -0x40bc
	ctx.r[7].s64 = ctx.r[11].s64 + -16572;
	// 826A68C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A68CC: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 826A68D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A68D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A68D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A68DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A68E0: 386A86D8  addi r3, r10, -0x7928
	ctx.r[3].s64 = ctx.r[10].s64 + -31016;
	// 826A68E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A68E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A68EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A68F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A68F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A68F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A68FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6908: 4BDC0519  bl 0x82466e20
	ctx.lr = 0x826A690C;
	sub_82466E20(ctx, base);
	// 826A690C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6920 size=108
    let mut pc: u32 = 0x826A6920;
    'dispatch: loop {
        match pc {
            0x826A6920 => {
    //   block [0x826A6920..0x826A698C)
	// 826A6920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A692C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6930: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6934: 38EBBF74  addi r7, r11, -0x408c
	ctx.r[7].s64 = ctx.r[11].s64 + -16524;
	// 826A6938: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A693C: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 826A6940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6948: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A694C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6950: 386A8708  addi r3, r10, -0x78f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30968;
	// 826A6954: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A695C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A696C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6978: 4BDC04A9  bl 0x82466e20
	ctx.lr = 0x826A697C;
	sub_82466E20(ctx, base);
	// 826A697C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6990 size=108
    let mut pc: u32 = 0x826A6990;
    'dispatch: loop {
        match pc {
            0x826A6990 => {
    //   block [0x826A6990..0x826A69FC)
	// 826A6990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A699C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A69A0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A69A4: 38EBBF8C  addi r7, r11, -0x4074
	ctx.r[7].s64 = ctx.r[11].s64 + -16500;
	// 826A69A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A69AC: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826A69B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A69B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A69B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A69BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A69C0: 386A8738  addi r3, r10, -0x78c8
	ctx.r[3].s64 = ctx.r[10].s64 + -30920;
	// 826A69C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A69C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A69CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A69D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A69D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A69D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A69DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A69E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A69E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A69E8: 4BDC0439  bl 0x82466e20
	ctx.lr = 0x826A69EC;
	sub_82466E20(ctx, base);
	// 826A69EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A69F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A69F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A69F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6A00 size=108
    let mut pc: u32 = 0x826A6A00;
    'dispatch: loop {
        match pc {
            0x826A6A00 => {
    //   block [0x826A6A00..0x826A6A6C)
	// 826A6A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6A0C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6A10: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6A14: 38EBBFC0  addi r7, r11, -0x4040
	ctx.r[7].s64 = ctx.r[11].s64 + -16448;
	// 826A6A18: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A6A1C: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 826A6A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6A24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6A28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6A30: 386A8768  addi r3, r10, -0x7898
	ctx.r[3].s64 = ctx.r[10].s64 + -30872;
	// 826A6A34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6A3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6A44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6A54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6A58: 4BDC03C9  bl 0x82466e20
	ctx.lr = 0x826A6A5C;
	sub_82466E20(ctx, base);
	// 826A6A5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6A70 size=108
    let mut pc: u32 = 0x826A6A70;
    'dispatch: loop {
        match pc {
            0x826A6A70 => {
    //   block [0x826A6A70..0x826A6ADC)
	// 826A6A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6A7C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6A80: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6A84: 38EBC050  addi r7, r11, -0x3fb0
	ctx.r[7].s64 = ctx.r[11].s64 + -16304;
	// 826A6A88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A6A8C: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826A6A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6A94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6AA0: 386A8798  addi r3, r10, -0x7868
	ctx.r[3].s64 = ctx.r[10].s64 + -30824;
	// 826A6AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6AC8: 4BDC0359  bl 0x82466e20
	ctx.lr = 0x826A6ACC;
	sub_82466E20(ctx, base);
	// 826A6ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6AE0 size=108
    let mut pc: u32 = 0x826A6AE0;
    'dispatch: loop {
        match pc {
            0x826A6AE0 => {
    //   block [0x826A6AE0..0x826A6B4C)
	// 826A6AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6AEC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6AF0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6AF4: 38EBC068  addi r7, r11, -0x3f98
	ctx.r[7].s64 = ctx.r[11].s64 + -16280;
	// 826A6AF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6AFC: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826A6B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6B04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6B10: 386A87C8  addi r3, r10, -0x7838
	ctx.r[3].s64 = ctx.r[10].s64 + -30776;
	// 826A6B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6B38: 4BDC02E9  bl 0x82466e20
	ctx.lr = 0x826A6B3C;
	sub_82466E20(ctx, base);
	// 826A6B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6B50 size=112
    let mut pc: u32 = 0x826A6B50;
    'dispatch: loop {
        match pc {
            0x826A6B50 => {
    //   block [0x826A6B50..0x826A6BC0)
	// 826A6B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6B5C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A6B60: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6B64: 392AC9FC  addi r9, r10, -0x3604
	ctx.r[9].s64 = ctx.r[10].s64 + -13828;
	// 826A6B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6B6C: 390BC098  addi r8, r11, -0x3f68
	ctx.r[8].s64 = ctx.r[11].s64 + -16232;
	// 826A6B70: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A6B74: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826A6B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6B7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A6B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6B88: 386A87F8  addi r3, r10, -0x7808
	ctx.r[3].s64 = ctx.r[10].s64 + -30728;
	// 826A6B8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A6B90: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A6B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6BA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6BAC: 4BDC0275  bl 0x82466e20
	ctx.lr = 0x826A6BB0;
	sub_82466E20(ctx, base);
	// 826A6BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6BC0 size=108
    let mut pc: u32 = 0x826A6BC0;
    'dispatch: loop {
        match pc {
            0x826A6BC0 => {
    //   block [0x826A6BC0..0x826A6C2C)
	// 826A6BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6BCC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6BD0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826A6BD4: 38EBC110  addi r7, r11, -0x3ef0
	ctx.r[7].s64 = ctx.r[11].s64 + -16112;
	// 826A6BD8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A6BDC: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826A6BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6BE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6BF0: 386A8828  addi r3, r10, -0x77d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30680;
	// 826A6BF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6BF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6C0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6C14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6C18: 4BDC0209  bl 0x82466e20
	ctx.lr = 0x826A6C1C;
	sub_82466E20(ctx, base);
	// 826A6C1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6C30 size=24
    let mut pc: u32 = 0x826A6C30;
    'dispatch: loop {
        match pc {
            0x826A6C30 => {
    //   block [0x826A6C30..0x826A6C48)
	// 826A6C30: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6C34: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6C38: 394A3970  addi r10, r10, 0x3970
	ctx.r[10].s64 = ctx.r[10].s64 + 14704;
	// 826A6C3C: 816BC200  lwz r11, -0x3e00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15872 as u32) ) } as u64;
	// 826A6C40: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A6C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6C48 size=108
    let mut pc: u32 = 0x826A6C48;
    'dispatch: loop {
        match pc {
            0x826A6C48 => {
    //   block [0x826A6C48..0x826A6CB4)
	// 826A6C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6C54: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6C5C: 38EB3970  addi r7, r11, 0x3970
	ctx.r[7].s64 = ctx.r[11].s64 + 14704;
	// 826A6C60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6C64: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826A6C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6C6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6C78: 386A8858  addi r3, r10, -0x77a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30632;
	// 826A6C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6CA0: 4BDC0181  bl 0x82466e20
	ctx.lr = 0x826A6CA4;
	sub_82466E20(ctx, base);
	// 826A6CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6CB8 size=24
    let mut pc: u32 = 0x826A6CB8;
    'dispatch: loop {
        match pc {
            0x826A6CB8 => {
    //   block [0x826A6CB8..0x826A6CD0)
	// 826A6CB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6CBC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6CC0: 394A39A0  addi r10, r10, 0x39a0
	ctx.r[10].s64 = ctx.r[10].s64 + 14752;
	// 826A6CC4: 816BC200  lwz r11, -0x3e00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15872 as u32) ) } as u64;
	// 826A6CC8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A6CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6CD0 size=108
    let mut pc: u32 = 0x826A6CD0;
    'dispatch: loop {
        match pc {
            0x826A6CD0 => {
    //   block [0x826A6CD0..0x826A6D3C)
	// 826A6CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6CDC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6CE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6CE4: 38EB39A0  addi r7, r11, 0x39a0
	ctx.r[7].s64 = ctx.r[11].s64 + 14752;
	// 826A6CE8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6CEC: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826A6CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6CF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6CF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6D00: 386A8888  addi r3, r10, -0x7778
	ctx.r[3].s64 = ctx.r[10].s64 + -30584;
	// 826A6D04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6D24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6D28: 4BDC00F9  bl 0x82466e20
	ctx.lr = 0x826A6D2C;
	sub_82466E20(ctx, base);
	// 826A6D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6D40 size=108
    let mut pc: u32 = 0x826A6D40;
    'dispatch: loop {
        match pc {
            0x826A6D40 => {
    //   block [0x826A6D40..0x826A6DAC)
	// 826A6D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6D4C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6D50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6D54: 38EBC1E8  addi r7, r11, -0x3e18
	ctx.r[7].s64 = ctx.r[11].s64 + -15896;
	// 826A6D58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A6D5C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826A6D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6D64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6D68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6D70: 386A88B8  addi r3, r10, -0x7748
	ctx.r[3].s64 = ctx.r[10].s64 + -30536;
	// 826A6D74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6D7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6D98: 4BDC0089  bl 0x82466e20
	ctx.lr = 0x826A6D9C;
	sub_82466E20(ctx, base);
	// 826A6D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A6DB0 size=24
    let mut pc: u32 = 0x826A6DB0;
    'dispatch: loop {
        match pc {
            0x826A6DB0 => {
    //   block [0x826A6DB0..0x826A6DC8)
	// 826A6DB0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6DB4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A6DB8: 394A39D0  addi r10, r10, 0x39d0
	ctx.r[10].s64 = ctx.r[10].s64 + 14800;
	// 826A6DBC: 816BC200  lwz r11, -0x3e00(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15872 as u32) ) } as u64;
	// 826A6DC0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A6DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6DC8 size=108
    let mut pc: u32 = 0x826A6DC8;
    'dispatch: loop {
        match pc {
            0x826A6DC8 => {
    //   block [0x826A6DC8..0x826A6E34)
	// 826A6DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6DD4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6DDC: 38EB39D0  addi r7, r11, 0x39d0
	ctx.r[7].s64 = ctx.r[11].s64 + 14800;
	// 826A6DE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6DE4: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826A6DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6DEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6DF8: 386A88E8  addi r3, r10, -0x7718
	ctx.r[3].s64 = ctx.r[10].s64 + -30488;
	// 826A6DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6E20: 4BDC0001  bl 0x82466e20
	ctx.lr = 0x826A6E24;
	sub_82466E20(ctx, base);
	// 826A6E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6E38 size=112
    let mut pc: u32 = 0x826A6E38;
    'dispatch: loop {
        match pc {
            0x826A6E38 => {
    //   block [0x826A6E38..0x826A6EA8)
	// 826A6E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6E44: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A6E48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6E4C: 392ACA40  addi r9, r10, -0x35c0
	ctx.r[9].s64 = ctx.r[10].s64 + -13760;
	// 826A6E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6E54: 390BC204  addi r8, r11, -0x3dfc
	ctx.r[8].s64 = ctx.r[11].s64 + -15868;
	// 826A6E58: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826A6E5C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826A6E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6E64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A6E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6E70: 386A8918  addi r3, r10, -0x76e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30440;
	// 826A6E74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A6E78: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A6E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6E84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6E8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6E94: 4BDBFF8D  bl 0x82466e20
	ctx.lr = 0x826A6E98;
	sub_82466E20(ctx, base);
	// 826A6E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6EA8 size=108
    let mut pc: u32 = 0x826A6EA8;
    'dispatch: loop {
        match pc {
            0x826A6EA8 => {
    //   block [0x826A6EA8..0x826A6F14)
	// 826A6EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6EB4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6EBC: 38EBC234  addi r7, r11, -0x3dcc
	ctx.r[7].s64 = ctx.r[11].s64 + -15820;
	// 826A6EC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6EC4: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826A6EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6ECC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6ED8: 386A8948  addi r3, r10, -0x76b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30392;
	// 826A6EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6F00: 4BDBFF21  bl 0x82466e20
	ctx.lr = 0x826A6F04;
	sub_82466E20(ctx, base);
	// 826A6F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6F18 size=108
    let mut pc: u32 = 0x826A6F18;
    'dispatch: loop {
        match pc {
            0x826A6F18 => {
    //   block [0x826A6F18..0x826A6F84)
	// 826A6F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6F24: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6F2C: 38EBC264  addi r7, r11, -0x3d9c
	ctx.r[7].s64 = ctx.r[11].s64 + -15772;
	// 826A6F30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A6F34: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826A6F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6F3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6F40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6F44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6F48: 386A8978  addi r3, r10, -0x7688
	ctx.r[3].s64 = ctx.r[10].s64 + -30344;
	// 826A6F4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6F54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6F6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6F70: 4BDBFEB1  bl 0x82466e20
	ctx.lr = 0x826A6F74;
	sub_82466E20(ctx, base);
	// 826A6F74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6F80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6F88 size=108
    let mut pc: u32 = 0x826A6F88;
    'dispatch: loop {
        match pc {
            0x826A6F88 => {
    //   block [0x826A6F88..0x826A6FF4)
	// 826A6F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A6F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A6F94: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A6F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A6F9C: 38EBC27C  addi r7, r11, -0x3d84
	ctx.r[7].s64 = ctx.r[11].s64 + -15748;
	// 826A6FA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A6FA4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826A6FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A6FAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A6FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A6FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A6FB8: 386A89A8  addi r3, r10, -0x7658
	ctx.r[3].s64 = ctx.r[10].s64 + -30296;
	// 826A6FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A6FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A6FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A6FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A6FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A6FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A6FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A6FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A6FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A6FE0: 4BDBFE41  bl 0x82466e20
	ctx.lr = 0x826A6FE4;
	sub_82466E20(ctx, base);
	// 826A6FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A6FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A6FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A6FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A6FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A6FF8 size=112
    let mut pc: u32 = 0x826A6FF8;
    'dispatch: loop {
        match pc {
            0x826A6FF8 => {
    //   block [0x826A6FF8..0x826A7068)
	// 826A6FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A6FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7004: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7008: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A700C: 38AA8A08  addi r5, r10, -0x75f8
	ctx.r[5].s64 = ctx.r[10].s64 + -30200;
	// 826A7010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7014: 390BC2AC  addi r8, r11, -0x3d54
	ctx.r[8].s64 = ctx.r[11].s64 + -15700;
	// 826A7018: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A701C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826A7020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7024: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A702C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7030: 386A89D8  addi r3, r10, -0x7628
	ctx.r[3].s64 = ctx.r[10].s64 + -30248;
	// 826A7034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A7038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A703C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A704C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7054: 4BDBFDCD  bl 0x82466e20
	ctx.lr = 0x826A7058;
	sub_82466E20(ctx, base);
	// 826A7058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A705C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7068 size=108
    let mut pc: u32 = 0x826A7068;
    'dispatch: loop {
        match pc {
            0x826A7068 => {
    //   block [0x826A7068..0x826A70D4)
	// 826A7068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A706C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7074: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A707C: 38EBC2C4  addi r7, r11, -0x3d3c
	ctx.r[7].s64 = ctx.r[11].s64 + -15676;
	// 826A7080: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A7084: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826A7088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A708C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7098: 386A8A08  addi r3, r10, -0x75f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30200;
	// 826A709C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A70A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A70A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A70A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A70AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A70B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A70B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A70B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A70BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A70C0: 4BDBFD61  bl 0x82466e20
	ctx.lr = 0x826A70C4;
	sub_82466E20(ctx, base);
	// 826A70C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A70C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A70CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A70D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A70D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A70D8 size=108
    let mut pc: u32 = 0x826A70D8;
    'dispatch: loop {
        match pc {
            0x826A70D8 => {
    //   block [0x826A70D8..0x826A7144)
	// 826A70D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A70DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A70E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A70E4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A70E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A70EC: 38EBC2F4  addi r7, r11, -0x3d0c
	ctx.r[7].s64 = ctx.r[11].s64 + -15628;
	// 826A70F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A70F4: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826A70F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A70FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7100: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7104: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7108: 386A8A38  addi r3, r10, -0x75c8
	ctx.r[3].s64 = ctx.r[10].s64 + -30152;
	// 826A710C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A711C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A712C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7130: 4BDBFCF1  bl 0x82466e20
	ctx.lr = 0x826A7134;
	sub_82466E20(ctx, base);
	// 826A7134: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A713C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7148 size=108
    let mut pc: u32 = 0x826A7148;
    'dispatch: loop {
        match pc {
            0x826A7148 => {
    //   block [0x826A7148..0x826A71B4)
	// 826A7148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A714C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7154: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A715C: 38EBC30C  addi r7, r11, -0x3cf4
	ctx.r[7].s64 = ctx.r[11].s64 + -15604;
	// 826A7160: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A7164: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826A7168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A716C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7178: 386A8A68  addi r3, r10, -0x7598
	ctx.r[3].s64 = ctx.r[10].s64 + -30104;
	// 826A717C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A718C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A719C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A71A0: 4BDBFC81  bl 0x82466e20
	ctx.lr = 0x826A71A4;
	sub_82466E20(ctx, base);
	// 826A71A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A71A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A71AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A71B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A71B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A71B8 size=108
    let mut pc: u32 = 0x826A71B8;
    'dispatch: loop {
        match pc {
            0x826A71B8 => {
    //   block [0x826A71B8..0x826A7224)
	// 826A71B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A71BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A71C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A71C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A71C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A71CC: 38EBC340  addi r7, r11, -0x3cc0
	ctx.r[7].s64 = ctx.r[11].s64 + -15552;
	// 826A71D0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A71D4: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826A71D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A71DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A71E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A71E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A71E8: 386A8A98  addi r3, r10, -0x7568
	ctx.r[3].s64 = ctx.r[10].s64 + -30056;
	// 826A71EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A71F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A71F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A71F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A71FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A720C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7210: 4BDBFC11  bl 0x82466e20
	ctx.lr = 0x826A7214;
	sub_82466E20(ctx, base);
	// 826A7214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A721C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7228 size=108
    let mut pc: u32 = 0x826A7228;
    'dispatch: loop {
        match pc {
            0x826A7228 => {
    //   block [0x826A7228..0x826A7294)
	// 826A7228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A722C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7234: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A723C: 38EBC3E8  addi r7, r11, -0x3c18
	ctx.r[7].s64 = ctx.r[11].s64 + -15384;
	// 826A7240: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A7244: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826A7248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A724C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7250: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7254: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7258: 386A8AC8  addi r3, r10, -0x7538
	ctx.r[3].s64 = ctx.r[10].s64 + -30008;
	// 826A725C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7260: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A726C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A727C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7280: 4BDBFBA1  bl 0x82466e20
	ctx.lr = 0x826A7284;
	sub_82466E20(ctx, base);
	// 826A7284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A728C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7298 size=108
    let mut pc: u32 = 0x826A7298;
    'dispatch: loop {
        match pc {
            0x826A7298 => {
    //   block [0x826A7298..0x826A7304)
	// 826A7298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A729C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A72A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A72A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A72A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A72AC: 38EBC418  addi r7, r11, -0x3be8
	ctx.r[7].s64 = ctx.r[11].s64 + -15336;
	// 826A72B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A72B4: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826A72B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A72BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A72C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A72C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A72C8: 386A8AF8  addi r3, r10, -0x7508
	ctx.r[3].s64 = ctx.r[10].s64 + -29960;
	// 826A72CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A72D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A72D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A72D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A72DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A72E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A72E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A72E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A72EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A72F0: 4BDBFB31  bl 0x82466e20
	ctx.lr = 0x826A72F4;
	sub_82466E20(ctx, base);
	// 826A72F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A72F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A72FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7308 size=108
    let mut pc: u32 = 0x826A7308;
    'dispatch: loop {
        match pc {
            0x826A7308 => {
    //   block [0x826A7308..0x826A7374)
	// 826A7308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A730C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7314: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7318: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A731C: 38EBC430  addi r7, r11, -0x3bd0
	ctx.r[7].s64 = ctx.r[11].s64 + -15312;
	// 826A7320: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A7324: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826A7328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A732C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7330: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7334: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7338: 386A8B28  addi r3, r10, -0x74d8
	ctx.r[3].s64 = ctx.r[10].s64 + -29912;
	// 826A733C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A734C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7354: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A735C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7360: 4BDBFAC1  bl 0x82466e20
	ctx.lr = 0x826A7364;
	sub_82466E20(ctx, base);
	// 826A7364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A736C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7378 size=112
    let mut pc: u32 = 0x826A7378;
    'dispatch: loop {
        match pc {
            0x826A7378 => {
    //   block [0x826A7378..0x826A73E8)
	// 826A7378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A737C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7384: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7388: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A738C: 38AA8978  addi r5, r10, -0x7688
	ctx.r[5].s64 = ctx.r[10].s64 + -30344;
	// 826A7390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7394: 390BC460  addi r8, r11, -0x3ba0
	ctx.r[8].s64 = ctx.r[11].s64 + -15264;
	// 826A7398: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826A739C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826A73A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A73A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A73A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A73AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A73B0: 386A8B58  addi r3, r10, -0x74a8
	ctx.r[3].s64 = ctx.r[10].s64 + -29864;
	// 826A73B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A73B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A73BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A73C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A73C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A73C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A73CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A73D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A73D4: 4BDBFA4D  bl 0x82466e20
	ctx.lr = 0x826A73D8;
	sub_82466E20(ctx, base);
	// 826A73D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A73DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A73E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A73E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A73E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A73E8 size=24
    let mut pc: u32 = 0x826A73E8;
    'dispatch: loop {
        match pc {
            0x826A73E8 => {
    //   block [0x826A73E8..0x826A7400)
	// 826A73E8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A73EC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A73F0: 394A3A00  addi r10, r10, 0x3a00
	ctx.r[10].s64 = ctx.r[10].s64 + 14848;
	// 826A73F4: 816BC33C  lwz r11, -0x3cc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15556 as u32) ) } as u64;
	// 826A73F8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A73FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7400 size=112
    let mut pc: u32 = 0x826A7400;
    'dispatch: loop {
        match pc {
            0x826A7400 => {
    //   block [0x826A7400..0x826A7470)
	// 826A7400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A740C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A7410: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7414: 392ACA6C  addi r9, r10, -0x3594
	ctx.r[9].s64 = ctx.r[10].s64 + -13716;
	// 826A7418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A741C: 390B3A00  addi r8, r11, 0x3a00
	ctx.r[8].s64 = ctx.r[11].s64 + 14848;
	// 826A7420: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A7424: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826A7428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A742C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A7434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7438: 386A8B88  addi r3, r10, -0x7478
	ctx.r[3].s64 = ctx.r[10].s64 + -29816;
	// 826A743C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A7440: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A7444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A744C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A745C: 4BDBF9C5  bl 0x82466e20
	ctx.lr = 0x826A7460;
	sub_82466E20(ctx, base);
	// 826A7460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A746C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7470 size=108
    let mut pc: u32 = 0x826A7470;
    'dispatch: loop {
        match pc {
            0x826A7470 => {
    //   block [0x826A7470..0x826A74DC)
	// 826A7470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A747C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7484: 38EBC50C  addi r7, r11, -0x3af4
	ctx.r[7].s64 = ctx.r[11].s64 + -15092;
	// 826A7488: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A748C: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826A7490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A749C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A74A0: 386A8BB8  addi r3, r10, -0x7448
	ctx.r[3].s64 = ctx.r[10].s64 + -29768;
	// 826A74A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A74A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A74AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A74B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A74B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A74B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A74BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A74C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A74C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A74C8: 4BDBF959  bl 0x82466e20
	ctx.lr = 0x826A74CC;
	sub_82466E20(ctx, base);
	// 826A74CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A74D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A74D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A74D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A74E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A74E0 size=116
    let mut pc: u32 = 0x826A74E0;
    'dispatch: loop {
        match pc {
            0x826A74E0 => {
    //   block [0x826A74E0..0x826A7554)
	// 826A74E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A74E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A74E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A74EC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A74F0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A74F4: 390BC540  addi r8, r11, -0x3ac0
	ctx.r[8].s64 = ctx.r[11].s64 + -15040;
	// 826A74F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A74FC: 392ACAB0  addi r9, r10, -0x3550
	ctx.r[9].s64 = ctx.r[10].s64 + -13648;
	// 826A7500: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7504: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826A7508: 38AA8978  addi r5, r10, -0x7688
	ctx.r[5].s64 = ctx.r[10].s64 + -30344;
	// 826A750C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A7510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7514: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A751C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7524: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A7528: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826A752C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A7530: 386B8BE8  addi r3, r11, -0x7418
	ctx.r[3].s64 = ctx.r[11].s64 + -29720;
	// 826A7534: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A7538: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A753C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7540: 4BDBF8E1  bl 0x82466e20
	ctx.lr = 0x826A7544;
	sub_82466E20(ctx, base);
	// 826A7544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A754C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A7558 size=24
    let mut pc: u32 = 0x826A7558;
    'dispatch: loop {
        match pc {
            0x826A7558 => {
    //   block [0x826A7558..0x826A7570)
	// 826A7558: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A755C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A7560: 394A3A78  addi r10, r10, 0x3a78
	ctx.r[10].s64 = ctx.r[10].s64 + 14968;
	// 826A7564: 816BC53C  lwz r11, -0x3ac4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15044 as u32) ) } as u64;
	// 826A7568: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A756C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7570 size=112
    let mut pc: u32 = 0x826A7570;
    'dispatch: loop {
        match pc {
            0x826A7570 => {
    //   block [0x826A7570..0x826A75E0)
	// 826A7570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A757C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A7580: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7584: 392ACAEC  addi r9, r10, -0x3514
	ctx.r[9].s64 = ctx.r[10].s64 + -13588;
	// 826A7588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A758C: 390B3A78  addi r8, r11, 0x3a78
	ctx.r[8].s64 = ctx.r[11].s64 + 14968;
	// 826A7590: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A7594: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826A7598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A759C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A75A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A75A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A75A8: 386A8C18  addi r3, r10, -0x73e8
	ctx.r[3].s64 = ctx.r[10].s64 + -29672;
	// 826A75AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A75B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A75B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A75B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A75BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A75C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A75C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A75C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A75CC: 4BDBF855  bl 0x82466e20
	ctx.lr = 0x826A75D0;
	sub_82466E20(ctx, base);
	// 826A75D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A75D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A75D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A75DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A75E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A75E0 size=108
    let mut pc: u32 = 0x826A75E0;
    'dispatch: loop {
        match pc {
            0x826A75E0 => {
    //   block [0x826A75E0..0x826A764C)
	// 826A75E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A75E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A75E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A75EC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A75F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A75F4: 38EBC600  addi r7, r11, -0x3a00
	ctx.r[7].s64 = ctx.r[11].s64 + -14848;
	// 826A75F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A75FC: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826A7600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A760C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7610: 386A8C48  addi r3, r10, -0x73b8
	ctx.r[3].s64 = ctx.r[10].s64 + -29624;
	// 826A7614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A761C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A762C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7638: 4BDBF7E9  bl 0x82466e20
	ctx.lr = 0x826A763C;
	sub_82466E20(ctx, base);
	// 826A763C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7650 size=108
    let mut pc: u32 = 0x826A7650;
    'dispatch: loop {
        match pc {
            0x826A7650 => {
    //   block [0x826A7650..0x826A76BC)
	// 826A7650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A765C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7664: 38EBC618  addi r7, r11, -0x39e8
	ctx.r[7].s64 = ctx.r[11].s64 + -14824;
	// 826A7668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A766C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826A7670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A767C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7680: 386A8C78  addi r3, r10, -0x7388
	ctx.r[3].s64 = ctx.r[10].s64 + -29576;
	// 826A7684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A768C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A769C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A76A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A76A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A76A8: 4BDBF779  bl 0x82466e20
	ctx.lr = 0x826A76AC;
	sub_82466E20(ctx, base);
	// 826A76AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A76B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A76B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A76B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A76C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A76C0 size=24
    let mut pc: u32 = 0x826A76C0;
    'dispatch: loop {
        match pc {
            0x826A76C0 => {
    //   block [0x826A76C0..0x826A76D8)
	// 826A76C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A76C4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A76C8: 394A3AC0  addi r10, r10, 0x3ac0
	ctx.r[10].s64 = ctx.r[10].s64 + 15040;
	// 826A76CC: 816BC648  lwz r11, -0x39b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14776 as u32) ) } as u64;
	// 826A76D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A76D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A76D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A76D8 size=112
    let mut pc: u32 = 0x826A76D8;
    'dispatch: loop {
        match pc {
            0x826A76D8 => {
    //   block [0x826A76D8..0x826A7748)
	// 826A76D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A76DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A76E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A76E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A76E8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A76EC: 392ACB28  addi r9, r10, -0x34d8
	ctx.r[9].s64 = ctx.r[10].s64 + -13528;
	// 826A76F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A76F4: 390B3AC0  addi r8, r11, 0x3ac0
	ctx.r[8].s64 = ctx.r[11].s64 + 15040;
	// 826A76F8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A76FC: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826A7700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7704: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7708: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A770C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7710: 386A8CA8  addi r3, r10, -0x7358
	ctx.r[3].s64 = ctx.r[10].s64 + -29528;
	// 826A7714: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A7718: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A771C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A772C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7734: 4BDBF6ED  bl 0x82466e20
	ctx.lr = 0x826A7738;
	sub_82466E20(ctx, base);
	// 826A7738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A773C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7740: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7748 size=112
    let mut pc: u32 = 0x826A7748;
    'dispatch: loop {
        match pc {
            0x826A7748 => {
    //   block [0x826A7748..0x826A77B8)
	// 826A7748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A774C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7758: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A775C: 38AA8978  addi r5, r10, -0x7688
	ctx.r[5].s64 = ctx.r[10].s64 + -30344;
	// 826A7760: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7764: 390BC64C  addi r8, r11, -0x39b4
	ctx.r[8].s64 = ctx.r[11].s64 + -14772;
	// 826A7768: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A776C: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 826A7770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7778: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A777C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7780: 386A8CD8  addi r3, r10, -0x7328
	ctx.r[3].s64 = ctx.r[10].s64 + -29480;
	// 826A7784: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A7788: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A778C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A779C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A77A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A77A4: 4BDBF67D  bl 0x82466e20
	ctx.lr = 0x826A77A8;
	sub_82466E20(ctx, base);
	// 826A77A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A77AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A77B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A77B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A77B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A77B8 size=108
    let mut pc: u32 = 0x826A77B8;
    'dispatch: loop {
        match pc {
            0x826A77B8 => {
    //   block [0x826A77B8..0x826A7824)
	// 826A77B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A77BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A77C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A77C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A77C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A77CC: 38EBC67C  addi r7, r11, -0x3984
	ctx.r[7].s64 = ctx.r[11].s64 + -14724;
	// 826A77D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A77D4: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826A77D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A77DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A77E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A77E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A77E8: 386A8D08  addi r3, r10, -0x72f8
	ctx.r[3].s64 = ctx.r[10].s64 + -29432;
	// 826A77EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A77F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A77F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A77F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A77FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A780C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7810: 4BDBF611  bl 0x82466e20
	ctx.lr = 0x826A7814;
	sub_82466E20(ctx, base);
	// 826A7814: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7818: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A781C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7820: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7828 size=108
    let mut pc: u32 = 0x826A7828;
    'dispatch: loop {
        match pc {
            0x826A7828 => {
    //   block [0x826A7828..0x826A7894)
	// 826A7828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A782C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7834: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A783C: 38EBC6B0  addi r7, r11, -0x3950
	ctx.r[7].s64 = ctx.r[11].s64 + -14672;
	// 826A7840: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A7844: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826A7848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A784C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7850: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7858: 386A8D38  addi r3, r10, -0x72c8
	ctx.r[3].s64 = ctx.r[10].s64 + -29384;
	// 826A785C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A786C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A787C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7880: 4BDBF5A1  bl 0x82466e20
	ctx.lr = 0x826A7884;
	sub_82466E20(ctx, base);
	// 826A7884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A788C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7898 size=108
    let mut pc: u32 = 0x826A7898;
    'dispatch: loop {
        match pc {
            0x826A7898 => {
    //   block [0x826A7898..0x826A7904)
	// 826A7898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A789C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A78A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A78A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A78A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A78AC: 38EBC710  addi r7, r11, -0x38f0
	ctx.r[7].s64 = ctx.r[11].s64 + -14576;
	// 826A78B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A78B4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826A78B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A78BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A78C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A78C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A78C8: 386A8D68  addi r3, r10, -0x7298
	ctx.r[3].s64 = ctx.r[10].s64 + -29336;
	// 826A78CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A78D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A78D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A78D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A78DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A78E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A78E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A78E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A78EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A78F0: 4BDBF531  bl 0x82466e20
	ctx.lr = 0x826A78F4;
	sub_82466E20(ctx, base);
	// 826A78F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A78F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A78FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7908 size=108
    let mut pc: u32 = 0x826A7908;
    'dispatch: loop {
        match pc {
            0x826A7908 => {
    //   block [0x826A7908..0x826A7974)
	// 826A7908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7914: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A791C: 38EBC740  addi r7, r11, -0x38c0
	ctx.r[7].s64 = ctx.r[11].s64 + -14528;
	// 826A7920: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826A7924: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826A7928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A792C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7930: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7938: 386A8D98  addi r3, r10, -0x7268
	ctx.r[3].s64 = ctx.r[10].s64 + -29288;
	// 826A793C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A794C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A795C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7960: 4BDBF4C1  bl 0x82466e20
	ctx.lr = 0x826A7964;
	sub_82466E20(ctx, base);
	// 826A7964: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A796C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7978 size=108
    let mut pc: u32 = 0x826A7978;
    'dispatch: loop {
        match pc {
            0x826A7978 => {
    //   block [0x826A7978..0x826A79E4)
	// 826A7978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A797C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7984: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A798C: 38EBC860  addi r7, r11, -0x37a0
	ctx.r[7].s64 = ctx.r[11].s64 + -14240;
	// 826A7990: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7994: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 826A7998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A799C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A79A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A79A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A79A8: 386A8DC8  addi r3, r10, -0x7238
	ctx.r[3].s64 = ctx.r[10].s64 + -29240;
	// 826A79AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A79B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A79B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A79B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A79BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A79C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A79C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A79C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A79CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A79D0: 4BDBF451  bl 0x82466e20
	ctx.lr = 0x826A79D4;
	sub_82466E20(ctx, base);
	// 826A79D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A79D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A79DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A79E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A79E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A79E8 size=108
    let mut pc: u32 = 0x826A79E8;
    'dispatch: loop {
        match pc {
            0x826A79E8 => {
    //   block [0x826A79E8..0x826A7A54)
	// 826A79E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A79EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A79F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A79F4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A79F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A79FC: 38EBC878  addi r7, r11, -0x3788
	ctx.r[7].s64 = ctx.r[11].s64 + -14216;
	// 826A7A00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7A04: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 826A7A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7A0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7A18: 386A8DF8  addi r3, r10, -0x7208
	ctx.r[3].s64 = ctx.r[10].s64 + -29192;
	// 826A7A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7A40: 4BDBF3E1  bl 0x82466e20
	ctx.lr = 0x826A7A44;
	sub_82466E20(ctx, base);
	// 826A7A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7A58 size=108
    let mut pc: u32 = 0x826A7A58;
    'dispatch: loop {
        match pc {
            0x826A7A58 => {
    //   block [0x826A7A58..0x826A7AC4)
	// 826A7A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7A64: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7A68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7A6C: 38EBC890  addi r7, r11, -0x3770
	ctx.r[7].s64 = ctx.r[11].s64 + -14192;
	// 826A7A70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7A74: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 826A7A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7A7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7A80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7A88: 386A8E28  addi r3, r10, -0x71d8
	ctx.r[3].s64 = ctx.r[10].s64 + -29144;
	// 826A7A8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7A90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7A98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7A9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7AA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7AA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7AA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7AAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7AB0: 4BDBF371  bl 0x82466e20
	ctx.lr = 0x826A7AB4;
	sub_82466E20(ctx, base);
	// 826A7AB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7AC8 size=108
    let mut pc: u32 = 0x826A7AC8;
    'dispatch: loop {
        match pc {
            0x826A7AC8 => {
    //   block [0x826A7AC8..0x826A7B34)
	// 826A7AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7AD4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7ADC: 38EBC8A8  addi r7, r11, -0x3758
	ctx.r[7].s64 = ctx.r[11].s64 + -14168;
	// 826A7AE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7AE4: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 826A7AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7AEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7AF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7AF8: 386A8E58  addi r3, r10, -0x71a8
	ctx.r[3].s64 = ctx.r[10].s64 + -29096;
	// 826A7AFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7B0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7B1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7B20: 4BDBF301  bl 0x82466e20
	ctx.lr = 0x826A7B24;
	sub_82466E20(ctx, base);
	// 826A7B24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7B28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7B2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7B30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7B38 size=108
    let mut pc: u32 = 0x826A7B38;
    'dispatch: loop {
        match pc {
            0x826A7B38 => {
    //   block [0x826A7B38..0x826A7BA4)
	// 826A7B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7B44: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7B4C: 38EBC8C0  addi r7, r11, -0x3740
	ctx.r[7].s64 = ctx.r[11].s64 + -14144;
	// 826A7B50: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7B54: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826A7B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7B5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7B60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7B68: 386A8E88  addi r3, r10, -0x7178
	ctx.r[3].s64 = ctx.r[10].s64 + -29048;
	// 826A7B6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7B90: 4BDBF291  bl 0x82466e20
	ctx.lr = 0x826A7B94;
	sub_82466E20(ctx, base);
	// 826A7B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7BA8 size=108
    let mut pc: u32 = 0x826A7BA8;
    'dispatch: loop {
        match pc {
            0x826A7BA8 => {
    //   block [0x826A7BA8..0x826A7C14)
	// 826A7BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7BB4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7BBC: 38EBC8D8  addi r7, r11, -0x3728
	ctx.r[7].s64 = ctx.r[11].s64 + -14120;
	// 826A7BC0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A7BC4: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 826A7BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7BD8: 386A8EB8  addi r3, r10, -0x7148
	ctx.r[3].s64 = ctx.r[10].s64 + -29000;
	// 826A7BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7C00: 4BDBF221  bl 0x82466e20
	ctx.lr = 0x826A7C04;
	sub_82466E20(ctx, base);
	// 826A7C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7C18 size=108
    let mut pc: u32 = 0x826A7C18;
    'dispatch: loop {
        match pc {
            0x826A7C18 => {
    //   block [0x826A7C18..0x826A7C84)
	// 826A7C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7C24: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7C2C: 38EBC8F0  addi r7, r11, -0x3710
	ctx.r[7].s64 = ctx.r[11].s64 + -14096;
	// 826A7C30: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A7C34: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826A7C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7C40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7C48: 386A8EE8  addi r3, r10, -0x7118
	ctx.r[3].s64 = ctx.r[10].s64 + -28952;
	// 826A7C4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7C6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7C70: 4BDBF1B1  bl 0x82466e20
	ctx.lr = 0x826A7C74;
	sub_82466E20(ctx, base);
	// 826A7C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7C88 size=108
    let mut pc: u32 = 0x826A7C88;
    'dispatch: loop {
        match pc {
            0x826A7C88 => {
    //   block [0x826A7C88..0x826A7CF4)
	// 826A7C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7C94: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7C9C: 38EBC980  addi r7, r11, -0x3680
	ctx.r[7].s64 = ctx.r[11].s64 + -13952;
	// 826A7CA0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826A7CA4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826A7CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7CAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7CB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7CB8: 386A8F18  addi r3, r10, -0x70e8
	ctx.r[3].s64 = ctx.r[10].s64 + -28904;
	// 826A7CBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7CDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7CE0: 4BDBF141  bl 0x82466e20
	ctx.lr = 0x826A7CE4;
	sub_82466E20(ctx, base);
	// 826A7CE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7CF8 size=108
    let mut pc: u32 = 0x826A7CF8;
    'dispatch: loop {
        match pc {
            0x826A7CF8 => {
    //   block [0x826A7CF8..0x826A7D64)
	// 826A7CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7D04: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7D08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7D0C: 38EBCA40  addi r7, r11, -0x35c0
	ctx.r[7].s64 = ctx.r[11].s64 + -13760;
	// 826A7D10: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A7D14: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826A7D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7D20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7D24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7D28: 386A8F48  addi r3, r10, -0x70b8
	ctx.r[3].s64 = ctx.r[10].s64 + -28856;
	// 826A7D2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7D30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7D38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7D40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7D44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7D48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7D50: 4BDBF0D1  bl 0x82466e20
	ctx.lr = 0x826A7D54;
	sub_82466E20(ctx, base);
	// 826A7D54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7D58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7D5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7D68 size=108
    let mut pc: u32 = 0x826A7D68;
    'dispatch: loop {
        match pc {
            0x826A7D68 => {
    //   block [0x826A7D68..0x826A7DD4)
	// 826A7D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7D6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7D70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7D74: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7D78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7D7C: 38EBCB18  addi r7, r11, -0x34e8
	ctx.r[7].s64 = ctx.r[11].s64 + -13544;
	// 826A7D80: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826A7D84: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826A7D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7D8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7D90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7D98: 386A8F78  addi r3, r10, -0x7088
	ctx.r[3].s64 = ctx.r[10].s64 + -28808;
	// 826A7D9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7DA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7DA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7DB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7DB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7DB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7DBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7DC0: 4BDBF061  bl 0x82466e20
	ctx.lr = 0x826A7DC4;
	sub_82466E20(ctx, base);
	// 826A7DC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7DD8 size=108
    let mut pc: u32 = 0x826A7DD8;
    'dispatch: loop {
        match pc {
            0x826A7DD8 => {
    //   block [0x826A7DD8..0x826A7E44)
	// 826A7DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7DDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7DE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7DE4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7DE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7DEC: 38EBCBD8  addi r7, r11, -0x3428
	ctx.r[7].s64 = ctx.r[11].s64 + -13352;
	// 826A7DF0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A7DF4: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826A7DF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7E00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7E08: 386A8FA8  addi r3, r10, -0x7058
	ctx.r[3].s64 = ctx.r[10].s64 + -28760;
	// 826A7E0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7E10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7E14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7E18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7E1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7E20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7E24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7E28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7E2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7E30: 4BDBEFF1  bl 0x82466e20
	ctx.lr = 0x826A7E34;
	sub_82466E20(ctx, base);
	// 826A7E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7E48 size=112
    let mut pc: u32 = 0x826A7E48;
    'dispatch: loop {
        match pc {
            0x826A7E48 => {
    //   block [0x826A7E48..0x826A7EB8)
	// 826A7E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7E54: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A7E58: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826A7E5C: 38EACC80  addi r7, r10, -0x3380
	ctx.r[7].s64 = ctx.r[10].s64 + -13184;
	// 826A7E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7E64: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A7E68: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826A7E6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7E70: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7E74: 396BCB40  addi r11, r11, -0x34c0
	ctx.r[11].s64 = ctx.r[11].s64 + -13504;
	// 826A7E78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7E7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7E80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7E84: 386A8FD8  addi r3, r10, -0x7028
	ctx.r[3].s64 = ctx.r[10].s64 + -28712;
	// 826A7E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7E8C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A7E90: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7E94: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A7E98: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7E9C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7EA0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7EA4: 4BDBEF7D  bl 0x82466e20
	ctx.lr = 0x826A7EA8;
	sub_82466E20(ctx, base);
	// 826A7EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7EB8 size=112
    let mut pc: u32 = 0x826A7EB8;
    'dispatch: loop {
        match pc {
            0x826A7EB8 => {
    //   block [0x826A7EB8..0x826A7F28)
	// 826A7EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7EC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7EC8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7ECC: 38AA8978  addi r5, r10, -0x7688
	ctx.r[5].s64 = ctx.r[10].s64 + -30344;
	// 826A7ED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7ED4: 390BCDB8  addi r8, r11, -0x3248
	ctx.r[8].s64 = ctx.r[11].s64 + -12872;
	// 826A7ED8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A7EDC: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 826A7EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7EE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7EE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A7EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7EF0: 386A9008  addi r3, r10, -0x6ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -28664;
	// 826A7EF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A7EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7F14: 4BDBEF0D  bl 0x82466e20
	ctx.lr = 0x826A7F18;
	sub_82466E20(ctx, base);
	// 826A7F18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7F28 size=108
    let mut pc: u32 = 0x826A7F28;
    'dispatch: loop {
        match pc {
            0x826A7F28 => {
    //   block [0x826A7F28..0x826A7F94)
	// 826A7F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7F34: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7F38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7F3C: 38EBCDE8  addi r7, r11, -0x3218
	ctx.r[7].s64 = ctx.r[11].s64 + -12824;
	// 826A7F40: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A7F44: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826A7F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7F4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7F50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7F58: 386A9038  addi r3, r10, -0x6fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -28616;
	// 826A7F5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7F60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7F68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7F70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7F74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7F78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7F7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7F80: 4BDBEEA1  bl 0x82466e20
	ctx.lr = 0x826A7F84;
	sub_82466E20(ctx, base);
	// 826A7F84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7F88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7F8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A7F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A7F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A7F98 size=108
    let mut pc: u32 = 0x826A7F98;
    'dispatch: loop {
        match pc {
            0x826A7F98 => {
    //   block [0x826A7F98..0x826A8004)
	// 826A7F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A7F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A7FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A7FA4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A7FA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A7FAC: 38EBCE48  addi r7, r11, -0x31b8
	ctx.r[7].s64 = ctx.r[11].s64 + -12728;
	// 826A7FB0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826A7FB4: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826A7FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A7FBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A7FC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A7FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A7FC8: 386A9068  addi r3, r10, -0x6f98
	ctx.r[3].s64 = ctx.r[10].s64 + -28568;
	// 826A7FCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A7FD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A7FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A7FD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A7FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A7FE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A7FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A7FE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A7FEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A7FF0: 4BDBEE31  bl 0x82466e20
	ctx.lr = 0x826A7FF4;
	sub_82466E20(ctx, base);
	// 826A7FF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A7FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A7FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8000: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8008 size=108
    let mut pc: u32 = 0x826A8008;
    'dispatch: loop {
        match pc {
            0x826A8008 => {
    //   block [0x826A8008..0x826A8074)
	// 826A8008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A800C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8014: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A801C: 38EBCF50  addi r7, r11, -0x30b0
	ctx.r[7].s64 = ctx.r[11].s64 + -12464;
	// 826A8020: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A8024: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826A8028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A802C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8030: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8038: 386A9098  addi r3, r10, -0x6f68
	ctx.r[3].s64 = ctx.r[10].s64 + -28520;
	// 826A803C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A804C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A805C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8060: 4BDBEDC1  bl 0x82466e20
	ctx.lr = 0x826A8064;
	sub_82466E20(ctx, base);
	// 826A8064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A806C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8078 size=108
    let mut pc: u32 = 0x826A8078;
    'dispatch: loop {
        match pc {
            0x826A8078 => {
    //   block [0x826A8078..0x826A80E4)
	// 826A8078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A807C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8084: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A808C: 38EBD028  addi r7, r11, -0x2fd8
	ctx.r[7].s64 = ctx.r[11].s64 + -12248;
	// 826A8090: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A8094: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826A8098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A809C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A80A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A80A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A80A8: 386A90C8  addi r3, r10, -0x6f38
	ctx.r[3].s64 = ctx.r[10].s64 + -28472;
	// 826A80AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A80B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A80B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A80B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A80BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A80C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A80C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A80C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A80CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A80D0: 4BDBED51  bl 0x82466e20
	ctx.lr = 0x826A80D4;
	sub_82466E20(ctx, base);
	// 826A80D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A80D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A80DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A80E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A80E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A80E8 size=108
    let mut pc: u32 = 0x826A80E8;
    'dispatch: loop {
        match pc {
            0x826A80E8 => {
    //   block [0x826A80E8..0x826A8154)
	// 826A80E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A80EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A80F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A80F4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A80F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A80FC: 38EBD070  addi r7, r11, -0x2f90
	ctx.r[7].s64 = ctx.r[11].s64 + -12176;
	// 826A8100: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8104: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826A8108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A810C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8110: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8118: 386A90F8  addi r3, r10, -0x6f08
	ctx.r[3].s64 = ctx.r[10].s64 + -28424;
	// 826A811C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A812C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A813C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8140: 4BDBECE1  bl 0x82466e20
	ctx.lr = 0x826A8144;
	sub_82466E20(ctx, base);
	// 826A8144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A814C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8158 size=108
    let mut pc: u32 = 0x826A8158;
    'dispatch: loop {
        match pc {
            0x826A8158 => {
    //   block [0x826A8158..0x826A81C4)
	// 826A8158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A815C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8164: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8168: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A816C: 38EBD088  addi r7, r11, -0x2f78
	ctx.r[7].s64 = ctx.r[11].s64 + -12152;
	// 826A8170: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A8174: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 826A8178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A817C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8180: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8184: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8188: 386A9128  addi r3, r10, -0x6ed8
	ctx.r[3].s64 = ctx.r[10].s64 + -28376;
	// 826A818C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8194: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A819C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A81A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A81A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A81A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A81AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A81B0: 4BDBEC71  bl 0x82466e20
	ctx.lr = 0x826A81B4;
	sub_82466E20(ctx, base);
	// 826A81B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A81B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A81BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A81C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A81C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A81C8 size=108
    let mut pc: u32 = 0x826A81C8;
    'dispatch: loop {
        match pc {
            0x826A81C8 => {
    //   block [0x826A81C8..0x826A8234)
	// 826A81C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A81CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A81D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A81D4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A81D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A81DC: 38EBD0E8  addi r7, r11, -0x2f18
	ctx.r[7].s64 = ctx.r[11].s64 + -12056;
	// 826A81E0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826A81E4: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 826A81E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A81EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A81F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A81F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A81F8: 386A9158  addi r3, r10, -0x6ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -28328;
	// 826A81FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A820C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A821C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8220: 4BDBEC01  bl 0x82466e20
	ctx.lr = 0x826A8224;
	sub_82466E20(ctx, base);
	// 826A8224: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8228: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A822C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8238 size=116
    let mut pc: u32 = 0x826A8238;
    'dispatch: loop {
        match pc {
            0x826A8238 => {
    //   block [0x826A8238..0x826A82AC)
	// 826A8238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A823C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8244: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8248: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A824C: 390BD1A8  addi r8, r11, -0x2e58
	ctx.r[8].s64 = ctx.r[11].s64 + -11864;
	// 826A8250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8254: 392ACBBC  addi r9, r10, -0x3444
	ctx.r[9].s64 = ctx.r[10].s64 + -13380;
	// 826A8258: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A825C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826A8260: 38AA9128  addi r5, r10, -0x6ed8
	ctx.r[5].s64 = ctx.r[10].s64 + -28376;
	// 826A8264: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A826C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8274: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A827C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A8280: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 826A8284: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A8288: 386B9188  addi r3, r11, -0x6e78
	ctx.r[3].s64 = ctx.r[11].s64 + -28280;
	// 826A828C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A8290: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8298: 4BDBEB89  bl 0x82466e20
	ctx.lr = 0x826A829C;
	sub_82466E20(ctx, base);
	// 826A829C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A82A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A82A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A82A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A82B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A82B0 size=112
    let mut pc: u32 = 0x826A82B0;
    'dispatch: loop {
        match pc {
            0x826A82B0 => {
    //   block [0x826A82B0..0x826A8320)
	// 826A82B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A82B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A82B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A82BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A82C0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A82C4: 38AAB468  addi r5, r10, -0x4b98
	ctx.r[5].s64 = ctx.r[10].s64 + -19352;
	// 826A82C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A82CC: 390BD238  addi r8, r11, -0x2dc8
	ctx.r[8].s64 = ctx.r[11].s64 + -11720;
	// 826A82D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A82D4: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 826A82D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A82DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A82E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A82E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A82E8: 386A91B8  addi r3, r10, -0x6e48
	ctx.r[3].s64 = ctx.r[10].s64 + -28232;
	// 826A82EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A82F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A82F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A82F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A82FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A830C: 4BDBEB15  bl 0x82466e20
	ctx.lr = 0x826A8310;
	sub_82466E20(ctx, base);
	// 826A8310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A831C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8320 size=96
    let mut pc: u32 = 0x826A8320;
    'dispatch: loop {
        match pc {
            0x826A8320 => {
    //   block [0x826A8320..0x826A8380)
	// 826A8320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A832C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8334: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 826A8338: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A833C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8340: 386A91E8  addi r3, r10, -0x6e18
	ctx.r[3].s64 = ctx.r[10].s64 + -28184;
	// 826A8344: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8348: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A834C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8350: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8358: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A835C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8360: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A8364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8368: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A836C: 4BDBEAB5  bl 0x82466e20
	ctx.lr = 0x826A8370;
	sub_82466E20(ctx, base);
	// 826A8370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A837C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8380 size=24
    let mut pc: u32 = 0x826A8380;
    'dispatch: loop {
        match pc {
            0x826A8380 => {
    //   block [0x826A8380..0x826A8398)
	// 826A8380: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8384: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8388: 394A3B38  addi r10, r10, 0x3b38
	ctx.r[10].s64 = ctx.r[10].s64 + 15160;
	// 826A838C: 816BD298  lwz r11, -0x2d68(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11624 as u32) ) } as u64;
	// 826A8390: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A8394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8398 size=116
    let mut pc: u32 = 0x826A8398;
    'dispatch: loop {
        match pc {
            0x826A8398 => {
    //   block [0x826A8398..0x826A840C)
	// 826A8398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A839C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A83A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A83A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A83A8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A83AC: 390B3B38  addi r8, r11, 0x3b38
	ctx.r[8].s64 = ctx.r[11].s64 + 15160;
	// 826A83B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A83B4: 392ACC08  addi r9, r10, -0x33f8
	ctx.r[9].s64 = ctx.r[10].s64 + -13304;
	// 826A83B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A83BC: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A83C0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A83C4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A83C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A83CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A83D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A83D4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A83D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A83DC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A83E0: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 826A83E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A83E8: 386B9218  addi r3, r11, -0x6de8
	ctx.r[3].s64 = ctx.r[11].s64 + -28136;
	// 826A83EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A83F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A83F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A83F8: 4BDBEA29  bl 0x82466e20
	ctx.lr = 0x826A83FC;
	sub_82466E20(ctx, base);
	// 826A83FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8410 size=104
    let mut pc: u32 = 0x826A8410;
    'dispatch: loop {
        match pc {
            0x826A8410 => {
    //   block [0x826A8410..0x826A8478)
	// 826A8410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A841C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A8420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8424: 392ACC34  addi r9, r10, -0x33cc
	ctx.r[9].s64 = ctx.r[10].s64 + -13260;
	// 826A8428: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A842C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8430: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8434: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A843C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8444: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 826A8448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A844C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8450: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A8454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8458: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A845C: 386A9248  addi r3, r10, -0x6db8
	ctx.r[3].s64 = ctx.r[10].s64 + -28088;
	// 826A8460: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A8464: 4BDBE9BD  bl 0x82466e20
	ctx.lr = 0x826A8468;
	sub_82466E20(ctx, base);
	// 826A8468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A846C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8478 size=96
    let mut pc: u32 = 0x826A8478;
    'dispatch: loop {
        match pc {
            0x826A8478 => {
    //   block [0x826A8478..0x826A84D8)
	// 826A8478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A847C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8484: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A848C: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 826A8490: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8498: 386A9278  addi r3, r10, -0x6d88
	ctx.r[3].s64 = ctx.r[10].s64 + -28040;
	// 826A849C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A84A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A84A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A84A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A84AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A84B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A84B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A84B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A84BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A84C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A84C4: 4BDBE95D  bl 0x82466e20
	ctx.lr = 0x826A84C8;
	sub_82466E20(ctx, base);
	// 826A84C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A84CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A84D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A84D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A84D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A84D8 size=96
    let mut pc: u32 = 0x826A84D8;
    'dispatch: loop {
        match pc {
            0x826A84D8 => {
    //   block [0x826A84D8..0x826A8538)
	// 826A84D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A84DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A84E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A84E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A84E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A84EC: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 826A84F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A84F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A84F8: 386A92A8  addi r3, r10, -0x6d58
	ctx.r[3].s64 = ctx.r[10].s64 + -27992;
	// 826A84FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8504: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A850C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8518: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A851C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8520: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A8524: 4BDBE8FD  bl 0x82466e20
	ctx.lr = 0x826A8528;
	sub_82466E20(ctx, base);
	// 826A8528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A852C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8538 size=100
    let mut pc: u32 = 0x826A8538;
    'dispatch: loop {
        match pc {
            0x826A8538 => {
    //   block [0x826A8538..0x826A859C)
	// 826A8538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A853C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8544: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A854C: 38AA9248  addi r5, r10, -0x6db8
	ctx.r[5].s64 = ctx.r[10].s64 + -28088;
	// 826A8550: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8558: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 826A855C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A856C: 386A92D8  addi r3, r10, -0x6d28
	ctx.r[3].s64 = ctx.r[10].s64 + -27944;
	// 826A8570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8578: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A857C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8580: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A8584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8588: 4BDBE899  bl 0x82466e20
	ctx.lr = 0x826A858C;
	sub_82466E20(ctx, base);
	// 826A858C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A85A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A85A0 size=112
    let mut pc: u32 = 0x826A85A0;
    'dispatch: loop {
        match pc {
            0x826A85A0 => {
    //   block [0x826A85A0..0x826A8610)
	// 826A85A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A85A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A85A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A85AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A85B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A85B4: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826A85B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A85BC: 390BD2A0  addi r8, r11, -0x2d60
	ctx.r[8].s64 = ctx.r[11].s64 + -11616;
	// 826A85C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A85C4: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 826A85C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A85CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A85D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A85D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A85D8: 386A9308  addi r3, r10, -0x6cf8
	ctx.r[3].s64 = ctx.r[10].s64 + -27896;
	// 826A85DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A85E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A85E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A85E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A85EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A85F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A85F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A85F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A85FC: 4BDBE825  bl 0x82466e20
	ctx.lr = 0x826A8600;
	sub_82466E20(ctx, base);
	// 826A8600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A860C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8610 size=112
    let mut pc: u32 = 0x826A8610;
    'dispatch: loop {
        match pc {
            0x826A8610 => {
    //   block [0x826A8610..0x826A8680)
	// 826A8610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A861C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8620: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8624: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826A8628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A862C: 390BD2E8  addi r8, r11, -0x2d18
	ctx.r[8].s64 = ctx.r[11].s64 + -11544;
	// 826A8630: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A8634: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 826A8638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A863C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8648: 386A9338  addi r3, r10, -0x6cc8
	ctx.r[3].s64 = ctx.r[10].s64 + -27848;
	// 826A864C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A865C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A866C: 4BDBE7B5  bl 0x82466e20
	ctx.lr = 0x826A8670;
	sub_82466E20(ctx, base);
	// 826A8670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A867C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8680 size=100
    let mut pc: u32 = 0x826A8680;
    'dispatch: loop {
        match pc {
            0x826A8680 => {
    //   block [0x826A8680..0x826A86E4)
	// 826A8680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A868C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8690: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8694: 38AA9218  addi r5, r10, -0x6de8
	ctx.r[5].s64 = ctx.r[10].s64 + -28136;
	// 826A8698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A869C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A86A0: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 826A86A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A86A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A86AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A86B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A86B4: 386A9368  addi r3, r10, -0x6c98
	ctx.r[3].s64 = ctx.r[10].s64 + -27800;
	// 826A86B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A86BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A86C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A86C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A86C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A86CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A86D0: 4BDBE751  bl 0x82466e20
	ctx.lr = 0x826A86D4;
	sub_82466E20(ctx, base);
	// 826A86D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A86D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A86DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A86E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A86E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A86E8 size=112
    let mut pc: u32 = 0x826A86E8;
    'dispatch: loop {
        match pc {
            0x826A86E8 => {
    //   block [0x826A86E8..0x826A8758)
	// 826A86E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A86EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A86F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A86F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A86F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A86FC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8704: 390BD300  addi r8, r11, -0x2d00
	ctx.r[8].s64 = ctx.r[11].s64 + -11520;
	// 826A8708: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A870C: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 826A8710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8714: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A871C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8720: 386A9398  addi r3, r10, -0x6c68
	ctx.r[3].s64 = ctx.r[10].s64 + -27752;
	// 826A8724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A872C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A873C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8744: 4BDBE6DD  bl 0x82466e20
	ctx.lr = 0x826A8748;
	sub_82466E20(ctx, base);
	// 826A8748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A874C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8758 size=96
    let mut pc: u32 = 0x826A8758;
    'dispatch: loop {
        match pc {
            0x826A8758 => {
    //   block [0x826A8758..0x826A87B8)
	// 826A8758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A875C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8764: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8768: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A876C: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 826A8770: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8778: 386A93C8  addi r3, r10, -0x6c38
	ctx.r[3].s64 = ctx.r[10].s64 + -27704;
	// 826A877C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8784: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A878C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8798: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A879C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A87A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A87A4: 4BDBE67D  bl 0x82466e20
	ctx.lr = 0x826A87A8;
	sub_82466E20(ctx, base);
	// 826A87A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A87AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A87B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A87B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A87B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A87B8 size=112
    let mut pc: u32 = 0x826A87B8;
    'dispatch: loop {
        match pc {
            0x826A87B8 => {
    //   block [0x826A87B8..0x826A8828)
	// 826A87B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A87BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A87C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A87C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A87C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A87CC: 38AA93C8  addi r5, r10, -0x6c38
	ctx.r[5].s64 = ctx.r[10].s64 + -27704;
	// 826A87D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A87D4: 390BD330  addi r8, r11, -0x2cd0
	ctx.r[8].s64 = ctx.r[11].s64 + -11472;
	// 826A87D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A87DC: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 826A87E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A87E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A87E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A87EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A87F0: 386A93F8  addi r3, r10, -0x6c08
	ctx.r[3].s64 = ctx.r[10].s64 + -27656;
	// 826A87F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A87F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A87FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A880C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8814: 4BDBE60D  bl 0x82466e20
	ctx.lr = 0x826A8818;
	sub_82466E20(ctx, base);
	// 826A8818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A881C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8828 size=112
    let mut pc: u32 = 0x826A8828;
    'dispatch: loop {
        match pc {
            0x826A8828 => {
    //   block [0x826A8828..0x826A8898)
	// 826A8828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A882C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8834: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8838: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A883C: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A8840: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8844: 390BD348  addi r8, r11, -0x2cb8
	ctx.r[8].s64 = ctx.r[11].s64 + -11448;
	// 826A8848: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A884C: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 826A8850: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8858: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A885C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8860: 386A9428  addi r3, r10, -0x6bd8
	ctx.r[3].s64 = ctx.r[10].s64 + -27608;
	// 826A8864: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8868: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A886C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8874: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A887C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8884: 4BDBE59D  bl 0x82466e20
	ctx.lr = 0x826A8888;
	sub_82466E20(ctx, base);
	// 826A8888: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A888C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8898 size=36
    let mut pc: u32 = 0x826A8898;
    'dispatch: loop {
        match pc {
            0x826A8898 => {
    //   block [0x826A8898..0x826A88BC)
	// 826A8898: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A889C: 814BD368  lwz r10, -0x2c98(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11416 as u32) ) } as u64;
	// 826A88A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A88A4: 396B3B80  addi r11, r11, 0x3b80
	ctx.r[11].s64 = ctx.r[11].s64 + 15232;
	// 826A88A8: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 826A88AC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A88B0: 814AD360  lwz r10, -0x2ca0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-11424 as u32) ) } as u64;
	// 826A88B4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 826A88B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A88C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A88C0 size=108
    let mut pc: u32 = 0x826A88C0;
    'dispatch: loop {
        match pc {
            0x826A88C0 => {
    //   block [0x826A88C0..0x826A892C)
	// 826A88C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A88C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A88C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A88CC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A88D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A88D4: 38EB3B80  addi r7, r11, 0x3b80
	ctx.r[7].s64 = ctx.r[11].s64 + 15232;
	// 826A88D8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A88DC: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 826A88E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A88E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A88E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A88EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A88F0: 386A9458  addi r3, r10, -0x6ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -27560;
	// 826A88F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A88F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A88FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A890C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8918: 4BDBE509  bl 0x82466e20
	ctx.lr = 0x826A891C;
	sub_82466E20(ctx, base);
	// 826A891C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8930 size=24
    let mut pc: u32 = 0x826A8930;
    'dispatch: loop {
        match pc {
            0x826A8930 => {
    //   block [0x826A8930..0x826A8948)
	// 826A8930: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8934: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8938: 394A3C28  addi r10, r10, 0x3c28
	ctx.r[10].s64 = ctx.r[10].s64 + 15400;
	// 826A893C: 816BD360  lwz r11, -0x2ca0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11424 as u32) ) } as u64;
	// 826A8940: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826A8944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8948 size=116
    let mut pc: u32 = 0x826A8948;
    'dispatch: loop {
        match pc {
            0x826A8948 => {
    //   block [0x826A8948..0x826A89BC)
	// 826A8948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A894C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8954: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8958: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826A895C: 390A3C28  addi r8, r10, 0x3c28
	ctx.r[8].s64 = ctx.r[10].s64 + 15400;
	// 826A8960: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8964: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A8968: 38AA9458  addi r5, r10, -0x6ba8
	ctx.r[5].s64 = ctx.r[10].s64 + -27560;
	// 826A896C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8970: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A8974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8978: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A897C: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 826A8980: 396BCCD4  addi r11, r11, -0x332c
	ctx.r[11].s64 = ctx.r[11].s64 + -13100;
	// 826A8984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8988: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A898C: 386A9488  addi r3, r10, -0x6b78
	ctx.r[3].s64 = ctx.r[10].s64 + -27512;
	// 826A8990: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A8994: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8998: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A899C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A89A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A89A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A89A8: 4BDBE479  bl 0x82466e20
	ctx.lr = 0x826A89AC;
	sub_82466E20(ctx, base);
	// 826A89AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A89B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A89B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A89B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A89C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A89C0 size=112
    let mut pc: u32 = 0x826A89C0;
    'dispatch: loop {
        match pc {
            0x826A89C0 => {
    //   block [0x826A89C0..0x826A8A30)
	// 826A89C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A89C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A89C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A89CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A89D0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A89D4: 38AA9458  addi r5, r10, -0x6ba8
	ctx.r[5].s64 = ctx.r[10].s64 + -27560;
	// 826A89D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A89DC: 390BD370  addi r8, r11, -0x2c90
	ctx.r[8].s64 = ctx.r[11].s64 + -11408;
	// 826A89E0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826A89E4: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 826A89E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A89EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A89F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A89F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A89F8: 386A94B8  addi r3, r10, -0x6b48
	ctx.r[3].s64 = ctx.r[10].s64 + -27464;
	// 826A89FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8A1C: 4BDBE405  bl 0x82466e20
	ctx.lr = 0x826A8A20;
	sub_82466E20(ctx, base);
	// 826A8A20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8A24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8A28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8A30 size=24
    let mut pc: u32 = 0x826A8A30;
    'dispatch: loop {
        match pc {
            0x826A8A30 => {
    //   block [0x826A8A30..0x826A8A48)
	// 826A8A30: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8A34: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8A38: 394A3D18  addi r10, r10, 0x3d18
	ctx.r[10].s64 = ctx.r[10].s64 + 15640;
	// 826A8A3C: 816BDE18  lwz r11, -0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8680 as u32) ) } as u64;
	// 826A8A40: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 826A8A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8A48 size=116
    let mut pc: u32 = 0x826A8A48;
    'dispatch: loop {
        match pc {
            0x826A8A48 => {
    //   block [0x826A8A48..0x826A8ABC)
	// 826A8A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8A54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A8A58: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8A5C: 392BCC98  addi r9, r11, -0x3368
	ctx.r[9].s64 = ctx.r[11].s64 + -13160;
	// 826A8A60: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A8A64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8A68: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826A8A6C: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 826A8A70: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8A74: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 826A8A78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8A7C: 396B3D18  addi r11, r11, 0x3d18
	ctx.r[11].s64 = ctx.r[11].s64 + 15640;
	// 826A8A80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A8A84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8A88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A8A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8A90: 386A94E8  addi r3, r10, -0x6b18
	ctx.r[3].s64 = ctx.r[10].s64 + -27416;
	// 826A8A94: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826A8A98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A8A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8AA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A8AA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A8AA8: 4BDBE379  bl 0x82466e20
	ctx.lr = 0x826A8AAC;
	sub_82466E20(ctx, base);
	// 826A8AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8AC0 size=108
    let mut pc: u32 = 0x826A8AC0;
    'dispatch: loop {
        match pc {
            0x826A8AC0 => {
    //   block [0x826A8AC0..0x826A8B2C)
	// 826A8AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8ACC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8AD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8AD4: 38EBD3E8  addi r7, r11, -0x2c18
	ctx.r[7].s64 = ctx.r[11].s64 + -11288;
	// 826A8AD8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A8ADC: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 826A8AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8AE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8AF0: 386A9518  addi r3, r10, -0x6ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -27368;
	// 826A8AF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8B18: 4BDBE309  bl 0x82466e20
	ctx.lr = 0x826A8B1C;
	sub_82466E20(ctx, base);
	// 826A8B1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8B30 size=112
    let mut pc: u32 = 0x826A8B30;
    'dispatch: loop {
        match pc {
            0x826A8B30 => {
    //   block [0x826A8B30..0x826A8BA0)
	// 826A8B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8B3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8B40: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8B44: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A8B48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8B4C: 390BD448  addi r8, r11, -0x2bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -11192;
	// 826A8B50: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A8B54: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 826A8B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8B5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8B68: 386A9548  addi r3, r10, -0x6ab8
	ctx.r[3].s64 = ctx.r[10].s64 + -27320;
	// 826A8B6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8B8C: 4BDBE295  bl 0x82466e20
	ctx.lr = 0x826A8B90;
	sub_82466E20(ctx, base);
	// 826A8B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8BA0 size=108
    let mut pc: u32 = 0x826A8BA0;
    'dispatch: loop {
        match pc {
            0x826A8BA0 => {
    //   block [0x826A8BA0..0x826A8C0C)
	// 826A8BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8BAC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8BB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8BB4: 38EBD4A8  addi r7, r11, -0x2b58
	ctx.r[7].s64 = ctx.r[11].s64 + -11096;
	// 826A8BB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8BBC: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 826A8BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8BC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8BD0: 386A9578  addi r3, r10, -0x6a88
	ctx.r[3].s64 = ctx.r[10].s64 + -27272;
	// 826A8BD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8BF8: 4BDBE229  bl 0x82466e20
	ctx.lr = 0x826A8BFC;
	sub_82466E20(ctx, base);
	// 826A8BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8C10 size=108
    let mut pc: u32 = 0x826A8C10;
    'dispatch: loop {
        match pc {
            0x826A8C10 => {
    //   block [0x826A8C10..0x826A8C7C)
	// 826A8C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8C1C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8C20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8C24: 38EBD4C0  addi r7, r11, -0x2b40
	ctx.r[7].s64 = ctx.r[11].s64 + -11072;
	// 826A8C28: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A8C2C: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 826A8C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8C38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A8C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8C40: 386A95A8  addi r3, r10, -0x6a58
	ctx.r[3].s64 = ctx.r[10].s64 + -27224;
	// 826A8C44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A8C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8C54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8C64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A8C68: 4BDBE1B9  bl 0x82466e20
	ctx.lr = 0x826A8C6C;
	sub_82466E20(ctx, base);
	// 826A8C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A8C80 size=24
    let mut pc: u32 = 0x826A8C80;
    'dispatch: loop {
        match pc {
            0x826A8C80 => {
    //   block [0x826A8C80..0x826A8C98)
	// 826A8C80: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8C84: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8C88: 394A3E20  addi r10, r10, 0x3e20
	ctx.r[10].s64 = ctx.r[10].s64 + 15904;
	// 826A8C8C: 816BDE18  lwz r11, -0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8680 as u32) ) } as u64;
	// 826A8C90: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826A8C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8C98 size=116
    let mut pc: u32 = 0x826A8C98;
    'dispatch: loop {
        match pc {
            0x826A8C98 => {
    //   block [0x826A8C98..0x826A8D0C)
	// 826A8C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8CA4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A8CA8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826A8CAC: 390A3E20  addi r8, r10, 0x3e20
	ctx.r[8].s64 = ctx.r[10].s64 + 15904;
	// 826A8CB0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8CB4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A8CB8: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A8CBC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8CC0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A8CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8CCC: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 826A8CD0: 396BCD30  addi r11, r11, -0x32d0
	ctx.r[11].s64 = ctx.r[11].s64 + -13008;
	// 826A8CD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8CD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8CDC: 386A95D8  addi r3, r10, -0x6a28
	ctx.r[3].s64 = ctx.r[10].s64 + -27176;
	// 826A8CE0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A8CE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8CE8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A8CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8CF8: 4BDBE129  bl 0x82466e20
	ctx.lr = 0x826A8CFC;
	sub_82466E20(ctx, base);
	// 826A8CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8D10 size=112
    let mut pc: u32 = 0x826A8D10;
    'dispatch: loop {
        match pc {
            0x826A8D10 => {
    //   block [0x826A8D10..0x826A8D80)
	// 826A8D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8D1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8D20: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8D24: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8D28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8D2C: 390BD520  addi r8, r11, -0x2ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -10976;
	// 826A8D30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826A8D34: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 826A8D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8D3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8D40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8D44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8D48: 386A9608  addi r3, r10, -0x69f8
	ctx.r[3].s64 = ctx.r[10].s64 + -27128;
	// 826A8D4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8D54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8D5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8D6C: 4BDBE0B5  bl 0x82466e20
	ctx.lr = 0x826A8D70;
	sub_82466E20(ctx, base);
	// 826A8D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8D80 size=112
    let mut pc: u32 = 0x826A8D80;
    'dispatch: loop {
        match pc {
            0x826A8D80 => {
    //   block [0x826A8D80..0x826A8DF0)
	// 826A8D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8D8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8D90: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8D94: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8D98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8D9C: 390BD5B0  addi r8, r11, -0x2a50
	ctx.r[8].s64 = ctx.r[11].s64 + -10832;
	// 826A8DA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A8DA4: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 826A8DA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8DAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8DB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8DB8: 386A9638  addi r3, r10, -0x69c8
	ctx.r[3].s64 = ctx.r[10].s64 + -27080;
	// 826A8DBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8DC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8DC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8DCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8DD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8DD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8DDC: 4BDBE045  bl 0x82466e20
	ctx.lr = 0x826A8DE0;
	sub_82466E20(ctx, base);
	// 826A8DE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8DE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8DE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8DF0 size=112
    let mut pc: u32 = 0x826A8DF0;
    'dispatch: loop {
        match pc {
            0x826A8DF0 => {
    //   block [0x826A8DF0..0x826A8E60)
	// 826A8DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8DF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8DFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8E00: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8E04: 38AA94E8  addi r5, r10, -0x6b18
	ctx.r[5].s64 = ctx.r[10].s64 + -27416;
	// 826A8E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8E0C: 390BD610  addi r8, r11, -0x29f0
	ctx.r[8].s64 = ctx.r[11].s64 + -10736;
	// 826A8E10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A8E14: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 826A8E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8E1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8E20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8E28: 386A9668  addi r3, r10, -0x6998
	ctx.r[3].s64 = ctx.r[10].s64 + -27032;
	// 826A8E2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8E4C: 4BDBDFD5  bl 0x82466e20
	ctx.lr = 0x826A8E50;
	sub_82466E20(ctx, base);
	// 826A8E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8E60 size=100
    let mut pc: u32 = 0x826A8E60;
    'dispatch: loop {
        match pc {
            0x826A8E60 => {
    //   block [0x826A8E60..0x826A8EC4)
	// 826A8E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8E6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8E74: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A8E78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8E80: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 826A8E84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8E94: 386A9698  addi r3, r10, -0x6968
	ctx.r[3].s64 = ctx.r[10].s64 + -26984;
	// 826A8E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8EA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A8EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8EA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A8EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8EB0: 4BDBDF71  bl 0x82466e20
	ctx.lr = 0x826A8EB4;
	sub_82466E20(ctx, base);
	// 826A8EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8EC8 size=112
    let mut pc: u32 = 0x826A8EC8;
    'dispatch: loop {
        match pc {
            0x826A8EC8 => {
    //   block [0x826A8EC8..0x826A8F38)
	// 826A8EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8ED4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8ED8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8EDC: 38AA9698  addi r5, r10, -0x6968
	ctx.r[5].s64 = ctx.r[10].s64 + -26984;
	// 826A8EE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8EE4: 390BD640  addi r8, r11, -0x29c0
	ctx.r[8].s64 = ctx.r[11].s64 + -10688;
	// 826A8EE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A8EEC: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 826A8EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8EF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8F00: 386A96C8  addi r3, r10, -0x6938
	ctx.r[3].s64 = ctx.r[10].s64 + -26936;
	// 826A8F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8F24: 4BDBDEFD  bl 0x82466e20
	ctx.lr = 0x826A8F28;
	sub_82466E20(ctx, base);
	// 826A8F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8F38 size=112
    let mut pc: u32 = 0x826A8F38;
    'dispatch: loop {
        match pc {
            0x826A8F38 => {
    //   block [0x826A8F38..0x826A8FA8)
	// 826A8F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8F44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8F48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8F4C: 38AA96C8  addi r5, r10, -0x6938
	ctx.r[5].s64 = ctx.r[10].s64 + -26936;
	// 826A8F50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8F54: 390BD6A0  addi r8, r11, -0x2960
	ctx.r[8].s64 = ctx.r[11].s64 + -10592;
	// 826A8F58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A8F5C: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 826A8F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8F70: 386A96F8  addi r3, r10, -0x6908
	ctx.r[3].s64 = ctx.r[10].s64 + -26888;
	// 826A8F74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A8F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A8F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A8F94: 4BDBDE8D  bl 0x82466e20
	ctx.lr = 0x826A8F98;
	sub_82466E20(ctx, base);
	// 826A8F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A8F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A8FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A8FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A8FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A8FA8 size=112
    let mut pc: u32 = 0x826A8FA8;
    'dispatch: loop {
        match pc {
            0x826A8FA8 => {
    //   block [0x826A8FA8..0x826A9018)
	// 826A8FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A8FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A8FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A8FB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8FB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A8FBC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A8FC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A8FC4: 390BD6D0  addi r8, r11, -0x2930
	ctx.r[8].s64 = ctx.r[11].s64 + -10544;
	// 826A8FC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A8FCC: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 826A8FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A8FD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A8FD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A8FDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A8FE0: 386A9728  addi r3, r10, -0x68d8
	ctx.r[3].s64 = ctx.r[10].s64 + -26840;
	// 826A8FE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A8FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A8FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A8FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A8FF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A8FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A8FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9004: 4BDBDE1D  bl 0x82466e20
	ctx.lr = 0x826A9008;
	sub_82466E20(ctx, base);
	// 826A9008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A900C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9018 size=116
    let mut pc: u32 = 0x826A9018;
    'dispatch: loop {
        match pc {
            0x826A9018 => {
    //   block [0x826A9018..0x826A908C)
	// 826A9018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A901C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9024: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9028: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A902C: 390BD700  addi r8, r11, -0x2900
	ctx.r[8].s64 = ctx.r[11].s64 + -10496;
	// 826A9030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9034: 392ACD60  addi r9, r10, -0x32a0
	ctx.r[9].s64 = ctx.r[10].s64 + -12960;
	// 826A9038: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A903C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826A9040: 38AA9AE8  addi r5, r10, -0x6518
	ctx.r[5].s64 = ctx.r[10].s64 + -25880;
	// 826A9044: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A904C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A905C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A9060: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 826A9064: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A9068: 386B9758  addi r3, r11, -0x68a8
	ctx.r[3].s64 = ctx.r[11].s64 + -26792;
	// 826A906C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A9070: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9078: 4BDBDDA9  bl 0x82466e20
	ctx.lr = 0x826A907C;
	sub_82466E20(ctx, base);
	// 826A907C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9090 size=112
    let mut pc: u32 = 0x826A9090;
    'dispatch: loop {
        match pc {
            0x826A9090 => {
    //   block [0x826A9090..0x826A9100)
	// 826A9090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A909C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A90A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A90A4: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A90A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A90AC: 390BD718  addi r8, r11, -0x28e8
	ctx.r[8].s64 = ctx.r[11].s64 + -10472;
	// 826A90B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A90B4: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 826A90B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A90BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A90C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A90C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A90C8: 386A9788  addi r3, r10, -0x6878
	ctx.r[3].s64 = ctx.r[10].s64 + -26744;
	// 826A90CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A90D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A90D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A90D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A90DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A90E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A90E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A90E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A90EC: 4BDBDD35  bl 0x82466e20
	ctx.lr = 0x826A90F0;
	sub_82466E20(ctx, base);
	// 826A90F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A90F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A90F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A90FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9100 size=116
    let mut pc: u32 = 0x826A9100;
    'dispatch: loop {
        match pc {
            0x826A9100 => {
    //   block [0x826A9100..0x826A9174)
	// 826A9100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A910C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9110: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A9114: 390BD734  addi r8, r11, -0x28cc
	ctx.r[8].s64 = ctx.r[11].s64 + -10444;
	// 826A9118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A911C: 392ACD8C  addi r9, r10, -0x3274
	ctx.r[9].s64 = ctx.r[10].s64 + -12916;
	// 826A9120: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9124: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826A9128: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A912C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9134: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A913C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9144: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826A9148: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 826A914C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A9150: 386B97B8  addi r3, r11, -0x6848
	ctx.r[3].s64 = ctx.r[11].s64 + -26696;
	// 826A9154: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A9158: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A915C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9160: 4BDBDCC1  bl 0x82466e20
	ctx.lr = 0x826A9164;
	sub_82466E20(ctx, base);
	// 826A9164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A916C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9178 size=112
    let mut pc: u32 = 0x826A9178;
    'dispatch: loop {
        match pc {
            0x826A9178 => {
    //   block [0x826A9178..0x826A91E8)
	// 826A9178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A917C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9188: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A918C: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9194: 390BD768  addi r8, r11, -0x2898
	ctx.r[8].s64 = ctx.r[11].s64 + -10392;
	// 826A9198: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A919C: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 826A91A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A91A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A91A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A91AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A91B0: 386A97E8  addi r3, r10, -0x6818
	ctx.r[3].s64 = ctx.r[10].s64 + -26648;
	// 826A91B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A91B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A91BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A91C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A91C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A91C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A91CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A91D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A91D4: 4BDBDC4D  bl 0x82466e20
	ctx.lr = 0x826A91D8;
	sub_82466E20(ctx, base);
	// 826A91D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A91DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A91E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A91E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A91E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A91E8 size=112
    let mut pc: u32 = 0x826A91E8;
    'dispatch: loop {
        match pc {
            0x826A91E8 => {
    //   block [0x826A91E8..0x826A9258)
	// 826A91E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A91EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A91F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A91F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A91F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A91FC: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9200: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9204: 390BD7B0  addi r8, r11, -0x2850
	ctx.r[8].s64 = ctx.r[11].s64 + -10320;
	// 826A9208: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A920C: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 826A9210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A921C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9220: 386A9818  addi r3, r10, -0x67e8
	ctx.r[3].s64 = ctx.r[10].s64 + -26600;
	// 826A9224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A922C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A923C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9244: 4BDBDBDD  bl 0x82466e20
	ctx.lr = 0x826A9248;
	sub_82466E20(ctx, base);
	// 826A9248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A924C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9258 size=112
    let mut pc: u32 = 0x826A9258;
    'dispatch: loop {
        match pc {
            0x826A9258 => {
    //   block [0x826A9258..0x826A92C8)
	// 826A9258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A925C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9268: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A926C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A9270: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A9274: 390BD7F8  addi r8, r11, -0x2808
	ctx.r[8].s64 = ctx.r[11].s64 + -10248;
	// 826A9278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A927C: 388AB39C  addi r4, r10, -0x4c64
	ctx.r[4].s64 = ctx.r[10].s64 + -19556;
	// 826A9280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A928C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9290: 386A9848  addi r3, r10, -0x67b8
	ctx.r[3].s64 = ctx.r[10].s64 + -26552;
	// 826A9294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A929C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A92A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A92A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A92A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A92AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A92B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A92B4: 4BDBDB6D  bl 0x82466e20
	ctx.lr = 0x826A92B8;
	sub_82466E20(ctx, base);
	// 826A92B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A92BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A92C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A92C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A92C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A92C8 size=108
    let mut pc: u32 = 0x826A92C8;
    'dispatch: loop {
        match pc {
            0x826A92C8 => {
    //   block [0x826A92C8..0x826A9334)
	// 826A92C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A92CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A92D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A92D4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A92D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A92DC: 38EBD828  addi r7, r11, -0x27d8
	ctx.r[7].s64 = ctx.r[11].s64 + -10200;
	// 826A92E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A92E4: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 826A92E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A92EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A92F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A92F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A92F8: 386A9878  addi r3, r10, -0x6788
	ctx.r[3].s64 = ctx.r[10].s64 + -26504;
	// 826A92FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A9300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A930C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A931C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9320: 4BDBDB01  bl 0x82466e20
	ctx.lr = 0x826A9324;
	sub_82466E20(ctx, base);
	// 826A9324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A932C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9338 size=112
    let mut pc: u32 = 0x826A9338;
    'dispatch: loop {
        match pc {
            0x826A9338 => {
    //   block [0x826A9338..0x826A93A8)
	// 826A9338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A933C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9348: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A934C: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9350: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9354: 390BD870  addi r8, r11, -0x2790
	ctx.r[8].s64 = ctx.r[11].s64 + -10128;
	// 826A9358: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826A935C: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 826A9360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A936C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9370: 386A98A8  addi r3, r10, -0x6758
	ctx.r[3].s64 = ctx.r[10].s64 + -26456;
	// 826A9374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A937C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A938C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9394: 4BDBDA8D  bl 0x82466e20
	ctx.lr = 0x826A9398;
	sub_82466E20(ctx, base);
	// 826A9398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A939C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A93A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A93A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A93A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A93A8 size=116
    let mut pc: u32 = 0x826A93A8;
    'dispatch: loop {
        match pc {
            0x826A93A8 => {
    //   block [0x826A93A8..0x826A941C)
	// 826A93A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A93AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A93B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A93B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A93B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A93BC: 392BCDC8  addi r9, r11, -0x3238
	ctx.r[9].s64 = ctx.r[11].s64 + -12856;
	// 826A93C0: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A93C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A93C8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A93CC: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826A93D0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A93D4: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 826A93D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A93DC: 396BD900  addi r11, r11, -0x2700
	ctx.r[11].s64 = ctx.r[11].s64 + -9984;
	// 826A93E0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A93E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A93E8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A93EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A93F0: 386A98D8  addi r3, r10, -0x6728
	ctx.r[3].s64 = ctx.r[10].s64 + -26408;
	// 826A93F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A93F8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A93FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9400: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A9404: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9408: 4BDBDA19  bl 0x82466e20
	ctx.lr = 0x826A940C;
	sub_82466E20(ctx, base);
	// 826A940C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9420 size=112
    let mut pc: u32 = 0x826A9420;
    'dispatch: loop {
        match pc {
            0x826A9420 => {
    //   block [0x826A9420..0x826A9490)
	// 826A9420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A942C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9430: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9434: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A943C: 390BD990  addi r8, r11, -0x2670
	ctx.r[8].s64 = ctx.r[11].s64 + -9840;
	// 826A9440: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A9444: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 826A9448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A944C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9458: 386A9908  addi r3, r10, -0x66f8
	ctx.r[3].s64 = ctx.r[10].s64 + -26360;
	// 826A945C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A946C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A947C: 4BDBD9A5  bl 0x82466e20
	ctx.lr = 0x826A9480;
	sub_82466E20(ctx, base);
	// 826A9480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A948C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A9490 size=24
    let mut pc: u32 = 0x826A9490;
    'dispatch: loop {
        match pc {
            0x826A9490 => {
    //   block [0x826A9490..0x826A94A8)
	// 826A9490: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9494: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9498: 394A3E98  addi r10, r10, 0x3e98
	ctx.r[10].s64 = ctx.r[10].s64 + 16024;
	// 826A949C: 816BDE18  lwz r11, -0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8680 as u32) ) } as u64;
	// 826A94A0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826A94A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A94A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A94A8 size=116
    let mut pc: u32 = 0x826A94A8;
    'dispatch: loop {
        match pc {
            0x826A94A8 => {
    //   block [0x826A94A8..0x826A951C)
	// 826A94A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A94AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A94B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A94B4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A94B8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826A94BC: 390A3E98  addi r8, r10, 0x3e98
	ctx.r[8].s64 = ctx.r[10].s64 + 16024;
	// 826A94C0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A94C4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A94C8: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A94CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A94D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A94D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A94D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A94DC: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 826A94E0: 396BCDF8  addi r11, r11, -0x3208
	ctx.r[11].s64 = ctx.r[11].s64 + -12808;
	// 826A94E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A94E8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A94EC: 386A9938  addi r3, r10, -0x66c8
	ctx.r[3].s64 = ctx.r[10].s64 + -26312;
	// 826A94F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A94F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A94F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A94FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9508: 4BDBD919  bl 0x82466e20
	ctx.lr = 0x826A950C;
	sub_82466E20(ctx, base);
	// 826A950C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9520 size=112
    let mut pc: u32 = 0x826A9520;
    'dispatch: loop {
        match pc {
            0x826A9520 => {
    //   block [0x826A9520..0x826A9590)
	// 826A9520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A952C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9530: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9534: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A9538: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A953C: 390BD9A8  addi r8, r11, -0x2658
	ctx.r[8].s64 = ctx.r[11].s64 + -9816;
	// 826A9540: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A9544: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 826A9548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A954C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9558: 386A9968  addi r3, r10, -0x6698
	ctx.r[3].s64 = ctx.r[10].s64 + -26264;
	// 826A955C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A956C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A957C: 4BDBD8A5  bl 0x82466e20
	ctx.lr = 0x826A9580;
	sub_82466E20(ctx, base);
	// 826A9580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A958C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9590 size=100
    let mut pc: u32 = 0x826A9590;
    'dispatch: loop {
        match pc {
            0x826A9590 => {
    //   block [0x826A9590..0x826A95F4)
	// 826A9590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A959C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A95A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A95A4: 38AA99F8  addi r5, r10, -0x6608
	ctx.r[5].s64 = ctx.r[10].s64 + -26120;
	// 826A95A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A95AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A95B0: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 826A95B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A95B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A95BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A95C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A95C4: 386A9998  addi r3, r10, -0x6668
	ctx.r[3].s64 = ctx.r[10].s64 + -26216;
	// 826A95C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A95CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A95D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A95D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A95D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A95DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A95E0: 4BDBD841  bl 0x82466e20
	ctx.lr = 0x826A95E4;
	sub_82466E20(ctx, base);
	// 826A95E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A95E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A95EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A95F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A95F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A95F8 size=28
    let mut pc: u32 = 0x826A95F8;
    'dispatch: loop {
        match pc {
            0x826A95F8 => {
    //   block [0x826A95F8..0x826A9614)
	// 826A95F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A95FC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9600: 394A3F40  addi r10, r10, 0x3f40
	ctx.r[10].s64 = ctx.r[10].s64 + 16192;
	// 826A9604: 816BD9F0  lwz r11, -0x2610(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9744 as u32) ) } as u64;
	// 826A9608: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826A960C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826A9610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9618 size=112
    let mut pc: u32 = 0x826A9618;
    'dispatch: loop {
        match pc {
            0x826A9618 => {
    //   block [0x826A9618..0x826A9688)
	// 826A9618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9624: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9628: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 826A962C: 38EA3F40  addi r7, r10, 0x3f40
	ctx.r[7].s64 = ctx.r[10].s64 + 16192;
	// 826A9630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9634: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A9638: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 826A963C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9640: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A9644: 396BCE80  addi r11, r11, -0x3180
	ctx.r[11].s64 = ctx.r[11].s64 + -12672;
	// 826A9648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A964C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9654: 386A99C8  addi r3, r10, -0x6638
	ctx.r[3].s64 = ctx.r[10].s64 + -26168;
	// 826A9658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A965C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A9660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9664: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A9668: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A966C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9670: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9674: 4BDBD7AD  bl 0x82466e20
	ctx.lr = 0x826A9678;
	sub_82466E20(ctx, base);
	// 826A9678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A967C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A9688 size=24
    let mut pc: u32 = 0x826A9688;
    'dispatch: loop {
        match pc {
            0x826A9688 => {
    //   block [0x826A9688..0x826A96A0)
	// 826A9688: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A968C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9690: 394A40A8  addi r10, r10, 0x40a8
	ctx.r[10].s64 = ctx.r[10].s64 + 16552;
	// 826A9694: 816BDE18  lwz r11, -0x21e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8680 as u32) ) } as u64;
	// 826A9698: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826A969C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A96A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A96A0 size=116
    let mut pc: u32 = 0x826A96A0;
    'dispatch: loop {
        match pc {
            0x826A96A0 => {
    //   block [0x826A96A0..0x826A9714)
	// 826A96A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A96A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A96A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A96AC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A96B0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A96B4: 392BCE58  addi r9, r11, -0x31a8
	ctx.r[9].s64 = ctx.r[11].s64 + -12712;
	// 826A96B8: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A96BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A96C0: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826A96C4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826A96C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A96CC: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 826A96D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A96D4: 396B40A8  addi r11, r11, 0x40a8
	ctx.r[11].s64 = ctx.r[11].s64 + 16552;
	// 826A96D8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A96DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A96E0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A96E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A96E8: 386A99F8  addi r3, r10, -0x6608
	ctx.r[3].s64 = ctx.r[10].s64 + -26120;
	// 826A96EC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A96F0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A96F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A96F8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A96FC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9700: 4BDBD721  bl 0x82466e20
	ctx.lr = 0x826A9704;
	sub_82466E20(ctx, base);
	// 826A9704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9718 size=112
    let mut pc: u32 = 0x826A9718;
    'dispatch: loop {
        match pc {
            0x826A9718 => {
    //   block [0x826A9718..0x826A9788)
	// 826A9718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A971C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9728: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A972C: 38AA9AE8  addi r5, r10, -0x6518
	ctx.r[5].s64 = ctx.r[10].s64 + -25880;
	// 826A9730: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9734: 390BD9F8  addi r8, r11, -0x2608
	ctx.r[8].s64 = ctx.r[11].s64 + -9736;
	// 826A9738: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A973C: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 826A9740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9744: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A974C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9750: 386A9A28  addi r3, r10, -0x65d8
	ctx.r[3].s64 = ctx.r[10].s64 + -26072;
	// 826A9754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A975C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A976C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9774: 4BDBD6AD  bl 0x82466e20
	ctx.lr = 0x826A9778;
	sub_82466E20(ctx, base);
	// 826A9778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A977C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9788 size=112
    let mut pc: u32 = 0x826A9788;
    'dispatch: loop {
        match pc {
            0x826A9788 => {
    //   block [0x826A9788..0x826A97F8)
	// 826A9788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A978C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9798: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A979C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A97A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A97A4: 390BDA28  addi r8, r11, -0x25d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9688;
	// 826A97A8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826A97AC: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 826A97B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A97B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A97B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A97BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A97C0: 386A9A58  addi r3, r10, -0x65a8
	ctx.r[3].s64 = ctx.r[10].s64 + -26024;
	// 826A97C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A97C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A97CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A97D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A97D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A97D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A97DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A97E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A97E4: 4BDBD63D  bl 0x82466e20
	ctx.lr = 0x826A97E8;
	sub_82466E20(ctx, base);
	// 826A97E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A97EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A97F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A97F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A97F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A97F8 size=112
    let mut pc: u32 = 0x826A97F8;
    'dispatch: loop {
        match pc {
            0x826A97F8 => {
    //   block [0x826A97F8..0x826A9868)
	// 826A97F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A97FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9808: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A980C: 38AA99F8  addi r5, r10, -0x6608
	ctx.r[5].s64 = ctx.r[10].s64 + -26120;
	// 826A9810: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9814: 390BDAB8  addi r8, r11, -0x2548
	ctx.r[8].s64 = ctx.r[11].s64 + -9544;
	// 826A9818: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A981C: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 826A9820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9824: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9828: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A982C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9830: 386A9A88  addi r3, r10, -0x6578
	ctx.r[3].s64 = ctx.r[10].s64 + -25976;
	// 826A9834: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A983C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A984C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9854: 4BDBD5CD  bl 0x82466e20
	ctx.lr = 0x826A9858;
	sub_82466E20(ctx, base);
	// 826A9858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A985C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9868 size=100
    let mut pc: u32 = 0x826A9868;
    'dispatch: loop {
        match pc {
            0x826A9868 => {
    //   block [0x826A9868..0x826A98CC)
	// 826A9868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A986C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A987C: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9888: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 826A988C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A989C: 386A9AB8  addi r3, r10, -0x6548
	ctx.r[3].s64 = ctx.r[10].s64 + -25928;
	// 826A98A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A98A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A98A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A98AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A98B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A98B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A98B8: 4BDBD569  bl 0x82466e20
	ctx.lr = 0x826A98BC;
	sub_82466E20(ctx, base);
	// 826A98BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A98C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A98C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A98C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A98D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A98D0 size=100
    let mut pc: u32 = 0x826A98D0;
    'dispatch: loop {
        match pc {
            0x826A98D0 => {
    //   block [0x826A98D0..0x826A9934)
	// 826A98D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A98D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A98D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A98DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A98E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A98E4: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A98E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A98EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A98F0: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 826A98F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A98F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A98FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9904: 386A9AE8  addi r3, r10, -0x6518
	ctx.r[3].s64 = ctx.r[10].s64 + -25880;
	// 826A9908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A990C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9910: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A9914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9918: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A991C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9920: 4BDBD501  bl 0x82466e20
	ctx.lr = 0x826A9924;
	sub_82466E20(ctx, base);
	// 826A9924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A992C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9938 size=112
    let mut pc: u32 = 0x826A9938;
    'dispatch: loop {
        match pc {
            0x826A9938 => {
    //   block [0x826A9938..0x826A99A8)
	// 826A9938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A993C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9948: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A994C: 38AA9AB8  addi r5, r10, -0x6548
	ctx.r[5].s64 = ctx.r[10].s64 + -25928;
	// 826A9950: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9954: 390BDAD0  addi r8, r11, -0x2530
	ctx.r[8].s64 = ctx.r[11].s64 + -9520;
	// 826A9958: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A995C: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 826A9960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9964: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A996C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9970: 386A9B18  addi r3, r10, -0x64e8
	ctx.r[3].s64 = ctx.r[10].s64 + -25832;
	// 826A9974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A997C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A998C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9994: 4BDBD48D  bl 0x82466e20
	ctx.lr = 0x826A9998;
	sub_82466E20(ctx, base);
	// 826A9998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A999C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A99A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A99A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A99A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A99A8 size=112
    let mut pc: u32 = 0x826A99A8;
    'dispatch: loop {
        match pc {
            0x826A99A8 => {
    //   block [0x826A99A8..0x826A9A18)
	// 826A99A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A99AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A99B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A99B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A99B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A99BC: 38AA9AB8  addi r5, r10, -0x6548
	ctx.r[5].s64 = ctx.r[10].s64 + -25928;
	// 826A99C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A99C4: 390BDB18  addi r8, r11, -0x24e8
	ctx.r[8].s64 = ctx.r[11].s64 + -9448;
	// 826A99C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826A99CC: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 826A99D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A99D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A99D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A99DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A99E0: 386A9B48  addi r3, r10, -0x64b8
	ctx.r[3].s64 = ctx.r[10].s64 + -25784;
	// 826A99E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A99E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A99EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A99F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A99F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A99F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A99FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9A04: 4BDBD41D  bl 0x82466e20
	ctx.lr = 0x826A9A08;
	sub_82466E20(ctx, base);
	// 826A9A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9A18 size=112
    let mut pc: u32 = 0x826A9A18;
    'dispatch: loop {
        match pc {
            0x826A9A18 => {
    //   block [0x826A9A18..0x826A9A88)
	// 826A9A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9A24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9A28: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9A2C: 38AA9B48  addi r5, r10, -0x64b8
	ctx.r[5].s64 = ctx.r[10].s64 + -25784;
	// 826A9A30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9A34: 390BDBD8  addi r8, r11, -0x2428
	ctx.r[8].s64 = ctx.r[11].s64 + -9256;
	// 826A9A38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9A3C: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 826A9A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9A44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9A48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9A4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9A50: 386A9B78  addi r3, r10, -0x6488
	ctx.r[3].s64 = ctx.r[10].s64 + -25736;
	// 826A9A54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9A58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9A5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9A60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9A68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9A6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9A70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9A74: 4BDBD3AD  bl 0x82466e20
	ctx.lr = 0x826A9A78;
	sub_82466E20(ctx, base);
	// 826A9A78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9A7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9A80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9A88 size=112
    let mut pc: u32 = 0x826A9A88;
    'dispatch: loop {
        match pc {
            0x826A9A88 => {
    //   block [0x826A9A88..0x826A9AF8)
	// 826A9A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9A94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9A98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9A9C: 38AA9698  addi r5, r10, -0x6968
	ctx.r[5].s64 = ctx.r[10].s64 + -26984;
	// 826A9AA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9AA4: 390BDC08  addi r8, r11, -0x23f8
	ctx.r[8].s64 = ctx.r[11].s64 + -9208;
	// 826A9AA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9AAC: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 826A9AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9AB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9AC0: 386A9BA8  addi r3, r10, -0x6458
	ctx.r[3].s64 = ctx.r[10].s64 + -25688;
	// 826A9AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9AE4: 4BDBD33D  bl 0x82466e20
	ctx.lr = 0x826A9AE8;
	sub_82466E20(ctx, base);
	// 826A9AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9AF8 size=112
    let mut pc: u32 = 0x826A9AF8;
    'dispatch: loop {
        match pc {
            0x826A9AF8 => {
    //   block [0x826A9AF8..0x826A9B68)
	// 826A9AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9B04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9B08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9B0C: 38AA9428  addi r5, r10, -0x6bd8
	ctx.r[5].s64 = ctx.r[10].s64 + -27608;
	// 826A9B10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9B14: 390BDC38  addi r8, r11, -0x23c8
	ctx.r[8].s64 = ctx.r[11].s64 + -9160;
	// 826A9B18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9B1C: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 826A9B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9B24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9B28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9B30: 386A9BD8  addi r3, r10, -0x6428
	ctx.r[3].s64 = ctx.r[10].s64 + -25640;
	// 826A9B34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9B38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9B40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9B44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9B48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9B4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9B50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9B54: 4BDBD2CD  bl 0x82466e20
	ctx.lr = 0x826A9B58;
	sub_82466E20(ctx, base);
	// 826A9B58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9B68 size=112
    let mut pc: u32 = 0x826A9B68;
    'dispatch: loop {
        match pc {
            0x826A9B68 => {
    //   block [0x826A9B68..0x826A9BD8)
	// 826A9B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9B74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9B78: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9B7C: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9B80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9B84: 390BDC68  addi r8, r11, -0x2398
	ctx.r[8].s64 = ctx.r[11].s64 + -9112;
	// 826A9B88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9B8C: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 826A9B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9B94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9BA0: 386A9C08  addi r3, r10, -0x63f8
	ctx.r[3].s64 = ctx.r[10].s64 + -25592;
	// 826A9BA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9BC4: 4BDBD25D  bl 0x82466e20
	ctx.lr = 0x826A9BC8;
	sub_82466E20(ctx, base);
	// 826A9BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9BD8 size=112
    let mut pc: u32 = 0x826A9BD8;
    'dispatch: loop {
        match pc {
            0x826A9BD8 => {
    //   block [0x826A9BD8..0x826A9C48)
	// 826A9BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9BE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9BEC: 38AA9758  addi r5, r10, -0x68a8
	ctx.r[5].s64 = ctx.r[10].s64 + -26792;
	// 826A9BF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9BF4: 390BDC98  addi r8, r11, -0x2368
	ctx.r[8].s64 = ctx.r[11].s64 + -9064;
	// 826A9BF8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826A9BFC: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 826A9C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9C04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9C10: 386A9C38  addi r3, r10, -0x63c8
	ctx.r[3].s64 = ctx.r[10].s64 + -25544;
	// 826A9C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9C24: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A9C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9C34: 4BDBD1ED  bl 0x82466e20
	ctx.lr = 0x826A9C38;
	sub_82466E20(ctx, base);
	// 826A9C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9C48 size=108
    let mut pc: u32 = 0x826A9C48;
    'dispatch: loop {
        match pc {
            0x826A9C48 => {
    //   block [0x826A9C48..0x826A9CB4)
	// 826A9C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9C54: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9C58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9C5C: 38EBDD10  addi r7, r11, -0x22f0
	ctx.r[7].s64 = ctx.r[11].s64 + -8944;
	// 826A9C60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A9C64: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 826A9C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9C6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9C70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A9C74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9C78: 386A9C68  addi r3, r10, -0x6398
	ctx.r[3].s64 = ctx.r[10].s64 + -25496;
	// 826A9C7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A9C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9C8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9C9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9CA0: 4BDBD181  bl 0x82466e20
	ctx.lr = 0x826A9CA4;
	sub_82466E20(ctx, base);
	// 826A9CA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9CA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9CAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9CB8 size=112
    let mut pc: u32 = 0x826A9CB8;
    'dispatch: loop {
        match pc {
            0x826A9CB8 => {
    //   block [0x826A9CB8..0x826A9D28)
	// 826A9CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9CC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9CC8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9CCC: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9CD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9CD4: 390BDD40  addi r8, r11, -0x22c0
	ctx.r[8].s64 = ctx.r[11].s64 + -8896;
	// 826A9CD8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9CDC: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 826A9CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9CE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9CE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9CF0: 386A9C98  addi r3, r10, -0x6368
	ctx.r[3].s64 = ctx.r[10].s64 + -25448;
	// 826A9CF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9CFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9D14: 4BDBD10D  bl 0x82466e20
	ctx.lr = 0x826A9D18;
	sub_82466E20(ctx, base);
	// 826A9D18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9D1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9D20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9D28 size=100
    let mut pc: u32 = 0x826A9D28;
    'dispatch: loop {
        match pc {
            0x826A9D28 => {
    //   block [0x826A9D28..0x826A9D8C)
	// 826A9D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9D34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9D3C: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9D40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9D48: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 826A9D4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9D5C: 386A9CC8  addi r3, r10, -0x6338
	ctx.r[3].s64 = ctx.r[10].s64 + -25400;
	// 826A9D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9D64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9D68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A9D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9D70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9D78: 4BDBD0A9  bl 0x82466e20
	ctx.lr = 0x826A9D7C;
	sub_82466E20(ctx, base);
	// 826A9D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9D90 size=112
    let mut pc: u32 = 0x826A9D90;
    'dispatch: loop {
        match pc {
            0x826A9D90 => {
    //   block [0x826A9D90..0x826A9E00)
	// 826A9D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9D9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9DA0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9DA4: 38AA9398  addi r5, r10, -0x6c68
	ctx.r[5].s64 = ctx.r[10].s64 + -27752;
	// 826A9DA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9DAC: 390BDD70  addi r8, r11, -0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + -8848;
	// 826A9DB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A9DB4: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 826A9DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9DBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9DC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9DC8: 386A9CF8  addi r3, r10, -0x6308
	ctx.r[3].s64 = ctx.r[10].s64 + -25352;
	// 826A9DCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A9DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9DEC: 4BDBD035  bl 0x82466e20
	ctx.lr = 0x826A9DF0;
	sub_82466E20(ctx, base);
	// 826A9DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9E00 size=96
    let mut pc: u32 = 0x826A9E00;
    'dispatch: loop {
        match pc {
            0x826A9E00 => {
    //   block [0x826A9E00..0x826A9E60)
	// 826A9E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9E0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9E14: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 826A9E18: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9E1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9E20: 386A9D28  addi r3, r10, -0x62d8
	ctx.r[3].s64 = ctx.r[10].s64 + -25304;
	// 826A9E24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9E2C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A9E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9E40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A9E44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9E48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9E4C: 4BDBCFD5  bl 0x82466e20
	ctx.lr = 0x826A9E50;
	sub_82466E20(ctx, base);
	// 826A9E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9E54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9E58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9E5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9E60 size=108
    let mut pc: u32 = 0x826A9E60;
    'dispatch: loop {
        match pc {
            0x826A9E60 => {
    //   block [0x826A9E60..0x826A9ECC)
	// 826A9E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9E6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9E70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9E74: 38EBDDB8  addi r7, r11, -0x2248
	ctx.r[7].s64 = ctx.r[11].s64 + -8776;
	// 826A9E78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A9E7C: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 826A9E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9E84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9E88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A9E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A9E90: 386A9D58  addi r3, r10, -0x62a8
	ctx.r[3].s64 = ctx.r[10].s64 + -25256;
	// 826A9E94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A9E98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A9E9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9EA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9EA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9EB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9EB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9EB8: 4BDBCF69  bl 0x82466e20
	ctx.lr = 0x826A9EBC;
	sub_82466E20(ctx, base);
	// 826A9EBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9EC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9ED0 size=100
    let mut pc: u32 = 0x826A9ED0;
    'dispatch: loop {
        match pc {
            0x826A9ED0 => {
    //   block [0x826A9ED0..0x826A9F34)
	// 826A9ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9ED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9ED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9EDC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A9EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9EE4: 392ACF30  addi r9, r10, -0x30d0
	ctx.r[9].s64 = ctx.r[10].s64 + -12496;
	// 826A9EE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9EF0: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 826A9EF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9F04: 386A9D88  addi r3, r10, -0x6278
	ctx.r[3].s64 = ctx.r[10].s64 + -25208;
	// 826A9F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9F0C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826A9F10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A9F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9F18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A9F1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9F20: 4BDBCF01  bl 0x82466e20
	ctx.lr = 0x826A9F24;
	sub_82466E20(ctx, base);
	// 826A9F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9F28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9F2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9F30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A9F38 size=24
    let mut pc: u32 = 0x826A9F38;
    'dispatch: loop {
        match pc {
            0x826A9F38 => {
    //   block [0x826A9F38..0x826A9F50)
	// 826A9F38: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9F3C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A9F40: 394A4150  addi r10, r10, 0x4150
	ctx.r[10].s64 = ctx.r[10].s64 + 16720;
	// 826A9F44: 816BDE24  lwz r11, -0x21dc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8668 as u32) ) } as u64;
	// 826A9F48: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A9F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9F50 size=112
    let mut pc: u32 = 0x826A9F50;
    'dispatch: loop {
        match pc {
            0x826A9F50 => {
    //   block [0x826A9F50..0x826A9FC0)
	// 826A9F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9F5C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A9F60: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9F64: 392AD078  addi r9, r10, -0x2f88
	ctx.r[9].s64 = ctx.r[10].s64 + -12168;
	// 826A9F68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9F6C: 390B4150  addi r8, r11, 0x4150
	ctx.r[8].s64 = ctx.r[11].s64 + 16720;
	// 826A9F70: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826A9F74: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 826A9F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9F7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9F80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9F84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A9F88: 386A9DB8  addi r3, r10, -0x6248
	ctx.r[3].s64 = ctx.r[10].s64 + -25160;
	// 826A9F8C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A9F90: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A9F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A9F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A9F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A9FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A9FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A9FAC: 4BDBCE75  bl 0x82466e20
	ctx.lr = 0x826A9FB0;
	sub_82466E20(ctx, base);
	// 826A9FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A9FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A9FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A9FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A9FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A9FC0 size=112
    let mut pc: u32 = 0x826A9FC0;
    'dispatch: loop {
        match pc {
            0x826A9FC0 => {
    //   block [0x826A9FC0..0x826AA030)
	// 826A9FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A9FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A9FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A9FCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9FD0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A9FD4: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826A9FD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A9FDC: 390BDE2C  addi r8, r11, -0x21d4
	ctx.r[8].s64 = ctx.r[11].s64 + -8660;
	// 826A9FE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A9FE4: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 826A9FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A9FEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A9FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A9FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A9FF8: 386A9DE8  addi r3, r10, -0x6218
	ctx.r[3].s64 = ctx.r[10].s64 + -25112;
	// 826A9FFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA00C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA01C: 4BDBCE05  bl 0x82466e20
	ctx.lr = 0x826AA020;
	sub_82466E20(ctx, base);
	// 826AA020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA02C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA030 size=108
    let mut pc: u32 = 0x826AA030;
    'dispatch: loop {
        match pc {
            0x826AA030 => {
    //   block [0x826AA030..0x826AA09C)
	// 826AA030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA03C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA044: 38EBDE5C  addi r7, r11, -0x21a4
	ctx.r[7].s64 = ctx.r[11].s64 + -8612;
	// 826AA048: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AA04C: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 826AA050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AA05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA060: 386A9E18  addi r3, r10, -0x61e8
	ctx.r[3].s64 = ctx.r[10].s64 + -25064;
	// 826AA064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AA068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA06C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AA088: 4BDBCD99  bl 0x82466e20
	ctx.lr = 0x826AA08C;
	sub_82466E20(ctx, base);
	// 826AA08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA0A0 size=112
    let mut pc: u32 = 0x826AA0A0;
    'dispatch: loop {
        match pc {
            0x826AA0A0 => {
    //   block [0x826AA0A0..0x826AA110)
	// 826AA0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA0AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA0B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA0B4: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA0B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA0BC: 390BDE78  addi r8, r11, -0x2188
	ctx.r[8].s64 = ctx.r[11].s64 + -8584;
	// 826AA0C0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826AA0C4: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 826AA0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA0CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA0D8: 386A9E48  addi r3, r10, -0x61b8
	ctx.r[3].s64 = ctx.r[10].s64 + -25016;
	// 826AA0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA0FC: 4BDBCD25  bl 0x82466e20
	ctx.lr = 0x826AA100;
	sub_82466E20(ctx, base);
	// 826AA100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA110 size=100
    let mut pc: u32 = 0x826AA110;
    'dispatch: loop {
        match pc {
            0x826AA110 => {
    //   block [0x826AA110..0x826AA174)
	// 826AA110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA11C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA124: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA128: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA130: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 826AA134: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA144: 386A9E78  addi r3, r10, -0x6188
	ctx.r[3].s64 = ctx.r[10].s64 + -24968;
	// 826AA148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA14C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA150: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AA154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA158: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AA15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA160: 4BDBCCC1  bl 0x82466e20
	ctx.lr = 0x826AA164;
	sub_82466E20(ctx, base);
	// 826AA164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA178 size=112
    let mut pc: u32 = 0x826AA178;
    'dispatch: loop {
        match pc {
            0x826AA178 => {
    //   block [0x826AA178..0x826AA1E8)
	// 826AA178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA188: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA18C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA190: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA194: 390BDF38  addi r8, r11, -0x20c8
	ctx.r[8].s64 = ctx.r[11].s64 + -8392;
	// 826AA198: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AA19C: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 826AA1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA1A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA1A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA1B0: 386A9EA8  addi r3, r10, -0x6158
	ctx.r[3].s64 = ctx.r[10].s64 + -24920;
	// 826AA1B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA1D4: 4BDBCC4D  bl 0x82466e20
	ctx.lr = 0x826AA1D8;
	sub_82466E20(ctx, base);
	// 826AA1D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA1E8 size=112
    let mut pc: u32 = 0x826AA1E8;
    'dispatch: loop {
        match pc {
            0x826AA1E8 => {
    //   block [0x826AA1E8..0x826AA258)
	// 826AA1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA1F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA1F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA1FC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA200: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA204: 390BDF50  addi r8, r11, -0x20b0
	ctx.r[8].s64 = ctx.r[11].s64 + -8368;
	// 826AA208: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AA20C: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 826AA210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA220: 386A9ED8  addi r3, r10, -0x6128
	ctx.r[3].s64 = ctx.r[10].s64 + -24872;
	// 826AA224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA244: 4BDBCBDD  bl 0x82466e20
	ctx.lr = 0x826AA248;
	sub_82466E20(ctx, base);
	// 826AA248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA258 size=112
    let mut pc: u32 = 0x826AA258;
    'dispatch: loop {
        match pc {
            0x826AA258 => {
    //   block [0x826AA258..0x826AA2C8)
	// 826AA258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA268: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA26C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA270: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA274: 390BDF80  addi r8, r11, -0x2080
	ctx.r[8].s64 = ctx.r[11].s64 + -8320;
	// 826AA278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AA27C: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 826AA280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA290: 386A9F08  addi r3, r10, -0x60f8
	ctx.r[3].s64 = ctx.r[10].s64 + -24824;
	// 826AA294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA2B4: 4BDBCB6D  bl 0x82466e20
	ctx.lr = 0x826AA2B8;
	sub_82466E20(ctx, base);
	// 826AA2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA2C8 size=112
    let mut pc: u32 = 0x826AA2C8;
    'dispatch: loop {
        match pc {
            0x826AA2C8 => {
    //   block [0x826AA2C8..0x826AA338)
	// 826AA2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA2D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA2D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA2DC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA2E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA2E4: 390BDFB0  addi r8, r11, -0x2050
	ctx.r[8].s64 = ctx.r[11].s64 + -8272;
	// 826AA2E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AA2EC: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 826AA2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA2F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA2F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA300: 386A9F38  addi r3, r10, -0x60c8
	ctx.r[3].s64 = ctx.r[10].s64 + -24776;
	// 826AA304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA324: 4BDBCAFD  bl 0x82466e20
	ctx.lr = 0x826AA328;
	sub_82466E20(ctx, base);
	// 826AA328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA338 size=112
    let mut pc: u32 = 0x826AA338;
    'dispatch: loop {
        match pc {
            0x826AA338 => {
    //   block [0x826AA338..0x826AA3A8)
	// 826AA338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA348: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA34C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA350: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA354: 390BDFE0  addi r8, r11, -0x2020
	ctx.r[8].s64 = ctx.r[11].s64 + -8224;
	// 826AA358: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AA35C: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 826AA360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA370: 386A9F68  addi r3, r10, -0x6098
	ctx.r[3].s64 = ctx.r[10].s64 + -24728;
	// 826AA374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA37C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA394: 4BDBCA8D  bl 0x82466e20
	ctx.lr = 0x826AA398;
	sub_82466E20(ctx, base);
	// 826AA398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA3A8 size=112
    let mut pc: u32 = 0x826AA3A8;
    'dispatch: loop {
        match pc {
            0x826AA3A8 => {
    //   block [0x826AA3A8..0x826AA418)
	// 826AA3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA3B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA3B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA3BC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA3C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA3C4: 390BDFF8  addi r8, r11, -0x2008
	ctx.r[8].s64 = ctx.r[11].s64 + -8200;
	// 826AA3C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AA3CC: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 826AA3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA3D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA3E0: 386A9F98  addi r3, r10, -0x6068
	ctx.r[3].s64 = ctx.r[10].s64 + -24680;
	// 826AA3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA404: 4BDBCA1D  bl 0x82466e20
	ctx.lr = 0x826AA408;
	sub_82466E20(ctx, base);
	// 826AA408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA418 size=112
    let mut pc: u32 = 0x826AA418;
    'dispatch: loop {
        match pc {
            0x826AA418 => {
    //   block [0x826AA418..0x826AA488)
	// 826AA418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA424: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA428: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA42C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA430: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA434: 390BE010  addi r8, r11, -0x1ff0
	ctx.r[8].s64 = ctx.r[11].s64 + -8176;
	// 826AA438: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AA43C: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 826AA440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA450: 386A9FC8  addi r3, r10, -0x6038
	ctx.r[3].s64 = ctx.r[10].s64 + -24632;
	// 826AA454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA474: 4BDBC9AD  bl 0x82466e20
	ctx.lr = 0x826AA478;
	sub_82466E20(ctx, base);
	// 826AA478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA47C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA488 size=112
    let mut pc: u32 = 0x826AA488;
    'dispatch: loop {
        match pc {
            0x826AA488 => {
    //   block [0x826AA488..0x826AA4F8)
	// 826AA488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA48C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA498: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA49C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA4A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA4A4: 390BE058  addi r8, r11, -0x1fa8
	ctx.r[8].s64 = ctx.r[11].s64 + -8104;
	// 826AA4A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AA4AC: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 826AA4B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA4B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA4B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA4C0: 386A9FF8  addi r3, r10, -0x6008
	ctx.r[3].s64 = ctx.r[10].s64 + -24584;
	// 826AA4C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA4D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA4D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA4D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA4E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA4E4: 4BDBC93D  bl 0x82466e20
	ctx.lr = 0x826AA4E8;
	sub_82466E20(ctx, base);
	// 826AA4E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA4F8 size=112
    let mut pc: u32 = 0x826AA4F8;
    'dispatch: loop {
        match pc {
            0x826AA4F8 => {
    //   block [0x826AA4F8..0x826AA568)
	// 826AA4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA508: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA50C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA514: 390BE0A0  addi r8, r11, -0x1f60
	ctx.r[8].s64 = ctx.r[11].s64 + -8032;
	// 826AA518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AA51C: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 826AA520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA530: 386AA028  addi r3, r10, -0x5fd8
	ctx.r[3].s64 = ctx.r[10].s64 + -24536;
	// 826AA534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA554: 4BDBC8CD  bl 0x82466e20
	ctx.lr = 0x826AA558;
	sub_82466E20(ctx, base);
	// 826AA558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA568 size=112
    let mut pc: u32 = 0x826AA568;
    'dispatch: loop {
        match pc {
            0x826AA568 => {
    //   block [0x826AA568..0x826AA5D8)
	// 826AA568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA574: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA578: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA57C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA584: 390BE0B8  addi r8, r11, -0x1f48
	ctx.r[8].s64 = ctx.r[11].s64 + -8008;
	// 826AA588: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AA58C: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 826AA590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA5A0: 386AA058  addi r3, r10, -0x5fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -24488;
	// 826AA5A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA5B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA5C4: 4BDBC85D  bl 0x82466e20
	ctx.lr = 0x826AA5C8;
	sub_82466E20(ctx, base);
	// 826AA5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA5D8 size=116
    let mut pc: u32 = 0x826AA5D8;
    'dispatch: loop {
        match pc {
            0x826AA5D8 => {
    //   block [0x826AA5D8..0x826AA64C)
	// 826AA5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA5E4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AA5E8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826AA5EC: 390AE0E8  addi r8, r10, -0x1f18
	ctx.r[8].s64 = ctx.r[10].s64 + -7960;
	// 826AA5F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA5F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AA5F8: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA5FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA600: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AA604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA60C: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 826AA610: 396BD0A0  addi r11, r11, -0x2f60
	ctx.r[11].s64 = ctx.r[11].s64 + -12128;
	// 826AA614: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA618: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA61C: 386AA088  addi r3, r10, -0x5f78
	ctx.r[3].s64 = ctx.r[10].s64 + -24440;
	// 826AA620: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AA624: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA628: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AA62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA638: 4BDBC7E9  bl 0x82466e20
	ctx.lr = 0x826AA63C;
	sub_82466E20(ctx, base);
	// 826AA63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA650 size=116
    let mut pc: u32 = 0x826AA650;
    'dispatch: loop {
        match pc {
            0x826AA650 => {
    //   block [0x826AA650..0x826AA6C4)
	// 826AA650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA65C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AA660: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AA664: 390AE160  addi r8, r10, -0x1ea0
	ctx.r[8].s64 = ctx.r[10].s64 + -7840;
	// 826AA668: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA66C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AA670: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA674: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA678: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AA67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA684: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 826AA688: 396BD0B8  addi r11, r11, -0x2f48
	ctx.r[11].s64 = ctx.r[11].s64 + -12104;
	// 826AA68C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA690: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA694: 386AA0B8  addi r3, r10, -0x5f48
	ctx.r[3].s64 = ctx.r[10].s64 + -24392;
	// 826AA698: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826AA69C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA6A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826AA6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA6B0: 4BDBC771  bl 0x82466e20
	ctx.lr = 0x826AA6B4;
	sub_82466E20(ctx, base);
	// 826AA6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AA6C8 size=24
    let mut pc: u32 = 0x826AA6C8;
    'dispatch: loop {
        match pc {
            0x826AA6C8 => {
    //   block [0x826AA6C8..0x826AA6E0)
	// 826AA6C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA6CC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AA6D0: 394A4168  addi r10, r10, 0x4168
	ctx.r[10].s64 = ctx.r[10].s64 + 16744;
	// 826AA6D4: 816BDE74  lwz r11, -0x218c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8588 as u32) ) } as u64;
	// 826AA6D8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826AA6DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA6E0 size=116
    let mut pc: u32 = 0x826AA6E0;
    'dispatch: loop {
        match pc {
            0x826AA6E0 => {
    //   block [0x826AA6E0..0x826AA754)
	// 826AA6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA6EC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AA6F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA6F4: 392BD0E4  addi r9, r11, -0x2f1c
	ctx.r[9].s64 = ctx.r[11].s64 + -12060;
	// 826AA6F8: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA6FC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA700: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826AA704: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826AA708: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA70C: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 826AA710: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA714: 396B4168  addi r11, r11, 0x4168
	ctx.r[11].s64 = ctx.r[11].s64 + 16744;
	// 826AA718: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826AA71C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA720: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AA724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA728: 386AA0E8  addi r3, r10, -0x5f18
	ctx.r[3].s64 = ctx.r[10].s64 + -24344;
	// 826AA72C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AA730: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826AA734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA738: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AA73C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AA740: 4BDBC6E1  bl 0x82466e20
	ctx.lr = 0x826AA744;
	sub_82466E20(ctx, base);
	// 826AA744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA758 size=112
    let mut pc: u32 = 0x826AA758;
    'dispatch: loop {
        match pc {
            0x826AA758 => {
    //   block [0x826AA758..0x826AA7C8)
	// 826AA758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA768: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA76C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA770: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA774: 390BE1F0  addi r8, r11, -0x1e10
	ctx.r[8].s64 = ctx.r[11].s64 + -7696;
	// 826AA778: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AA77C: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 826AA780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA784: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA790: 386AA118  addi r3, r10, -0x5ee8
	ctx.r[3].s64 = ctx.r[10].s64 + -24296;
	// 826AA794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA7B4: 4BDBC66D  bl 0x82466e20
	ctx.lr = 0x826AA7B8;
	sub_82466E20(ctx, base);
	// 826AA7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA7C8 size=112
    let mut pc: u32 = 0x826AA7C8;
    'dispatch: loop {
        match pc {
            0x826AA7C8 => {
    //   block [0x826AA7C8..0x826AA838)
	// 826AA7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA7D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA7D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA7DC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA7E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA7E4: 390BE250  addi r8, r11, -0x1db0
	ctx.r[8].s64 = ctx.r[11].s64 + -7600;
	// 826AA7E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826AA7EC: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 826AA7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA7F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA7F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA800: 386AA148  addi r3, r10, -0x5eb8
	ctx.r[3].s64 = ctx.r[10].s64 + -24248;
	// 826AA804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA824: 4BDBC5FD  bl 0x82466e20
	ctx.lr = 0x826AA828;
	sub_82466E20(ctx, base);
	// 826AA828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA838 size=112
    let mut pc: u32 = 0x826AA838;
    'dispatch: loop {
        match pc {
            0x826AA838 => {
    //   block [0x826AA838..0x826AA8A8)
	// 826AA838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA848: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA84C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA850: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA854: 390BE2F8  addi r8, r11, -0x1d08
	ctx.r[8].s64 = ctx.r[11].s64 + -7432;
	// 826AA858: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826AA85C: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 826AA860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA870: 386AA178  addi r3, r10, -0x5e88
	ctx.r[3].s64 = ctx.r[10].s64 + -24200;
	// 826AA874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA894: 4BDBC58D  bl 0x82466e20
	ctx.lr = 0x826AA898;
	sub_82466E20(ctx, base);
	// 826AA898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA8A8 size=112
    let mut pc: u32 = 0x826AA8A8;
    'dispatch: loop {
        match pc {
            0x826AA8A8 => {
    //   block [0x826AA8A8..0x826AA918)
	// 826AA8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA8B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA8B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA8BC: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA8C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA8C4: 390BE370  addi r8, r11, -0x1c90
	ctx.r[8].s64 = ctx.r[11].s64 + -7312;
	// 826AA8C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AA8CC: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 826AA8D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA8D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA8D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA8DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA8E0: 386AA1A8  addi r3, r10, -0x5e58
	ctx.r[3].s64 = ctx.r[10].s64 + -24152;
	// 826AA8E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA8E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA8FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA904: 4BDBC51D  bl 0x82466e20
	ctx.lr = 0x826AA908;
	sub_82466E20(ctx, base);
	// 826AA908: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA90C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA910: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA914: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA918 size=112
    let mut pc: u32 = 0x826AA918;
    'dispatch: loop {
        match pc {
            0x826AA918 => {
    //   block [0x826AA918..0x826AA988)
	// 826AA918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA928: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA92C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA930: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA934: 390BE3B8  addi r8, r11, -0x1c48
	ctx.r[8].s64 = ctx.r[11].s64 + -7240;
	// 826AA938: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826AA93C: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 826AA940: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA944: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA948: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA94C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA950: 386AA1D8  addi r3, r10, -0x5e28
	ctx.r[3].s64 = ctx.r[10].s64 + -24104;
	// 826AA954: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA958: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA960: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA968: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA96C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA970: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA974: 4BDBC4AD  bl 0x82466e20
	ctx.lr = 0x826AA978;
	sub_82466E20(ctx, base);
	// 826AA978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA97C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA988 size=112
    let mut pc: u32 = 0x826AA988;
    'dispatch: loop {
        match pc {
            0x826AA988 => {
    //   block [0x826AA988..0x826AA9F8)
	// 826AA988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA98C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AA990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AA994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA998: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AA99C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AA9A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AA9A4: 390BE448  addi r8, r11, -0x1bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -7096;
	// 826AA9A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AA9AC: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 826AA9B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AA9B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AA9B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AA9BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AA9C0: 386AA208  addi r3, r10, -0x5df8
	ctx.r[3].s64 = ctx.r[10].s64 + -24056;
	// 826AA9C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AA9C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AA9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AA9D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AA9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AA9D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AA9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AA9E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AA9E4: 4BDBC43D  bl 0x82466e20
	ctx.lr = 0x826AA9E8;
	sub_82466E20(ctx, base);
	// 826AA9E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AA9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AA9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AA9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AA9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AA9F8 size=112
    let mut pc: u32 = 0x826AA9F8;
    'dispatch: loop {
        match pc {
            0x826AA9F8 => {
    //   block [0x826AA9F8..0x826AAA68)
	// 826AA9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AA9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAA04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAA08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAA0C: 38AA9DB8  addi r5, r10, -0x6248
	ctx.r[5].s64 = ctx.r[10].s64 + -25160;
	// 826AAA10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAA14: 390BE4A8  addi r8, r11, -0x1b58
	ctx.r[8].s64 = ctx.r[11].s64 + -7000;
	// 826AAA18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AAA1C: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 826AAA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAA24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAA28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAA2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAA30: 386AA238  addi r3, r10, -0x5dc8
	ctx.r[3].s64 = ctx.r[10].s64 + -24008;
	// 826AAA34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAA38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAA3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAA40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAA44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAA48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAA4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAA50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAA54: 4BDBC3CD  bl 0x82466e20
	ctx.lr = 0x826AAA58;
	sub_82466E20(ctx, base);
	// 826AAA58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAA5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAA60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAA64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAA68 size=112
    let mut pc: u32 = 0x826AAA68;
    'dispatch: loop {
        match pc {
            0x826AAA68 => {
    //   block [0x826AAA68..0x826AAAD8)
	// 826AAA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAA74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAA78: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAA7C: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AAA80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAA84: 390BE508  addi r8, r11, -0x1af8
	ctx.r[8].s64 = ctx.r[11].s64 + -6904;
	// 826AAA88: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AAA8C: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 826AAA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAA94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAAA0: 386AA268  addi r3, r10, -0x5d98
	ctx.r[3].s64 = ctx.r[10].s64 + -23960;
	// 826AAAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAAC4: 4BDBC35D  bl 0x82466e20
	ctx.lr = 0x826AAAC8;
	sub_82466E20(ctx, base);
	// 826AAAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAAD8 size=112
    let mut pc: u32 = 0x826AAAD8;
    'dispatch: loop {
        match pc {
            0x826AAAD8 => {
    //   block [0x826AAAD8..0x826AAB48)
	// 826AAAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAAE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAAE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAAEC: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AAAF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAAF4: 390BE538  addi r8, r11, -0x1ac8
	ctx.r[8].s64 = ctx.r[11].s64 + -6856;
	// 826AAAF8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AAAFC: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 826AAB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAB04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAB08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAB0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAB10: 386AA298  addi r3, r10, -0x5d68
	ctx.r[3].s64 = ctx.r[10].s64 + -23912;
	// 826AAB14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAB18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAB1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAB24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAB34: 4BDBC2ED  bl 0x82466e20
	ctx.lr = 0x826AAB38;
	sub_82466E20(ctx, base);
	// 826AAB38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAB48 size=100
    let mut pc: u32 = 0x826AAB48;
    'dispatch: loop {
        match pc {
            0x826AAB48 => {
    //   block [0x826AAB48..0x826AABAC)
	// 826AAB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAB54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAB58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAB5C: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AAB60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAB64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAB68: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 826AAB6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAB74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAB7C: 386AA2C8  addi r3, r10, -0x5d38
	ctx.r[3].s64 = ctx.r[10].s64 + -23864;
	// 826AAB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAB84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAB88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AAB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAB90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AAB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAB98: 4BDBC289  bl 0x82466e20
	ctx.lr = 0x826AAB9C;
	sub_82466E20(ctx, base);
	// 826AAB9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AABA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AABA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AABA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AABB0 size=112
    let mut pc: u32 = 0x826AABB0;
    'dispatch: loop {
        match pc {
            0x826AABB0 => {
    //   block [0x826AABB0..0x826AAC20)
	// 826AABB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AABB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AABB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AABBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AABC0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AABC4: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AABC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AABCC: 390BE580  addi r8, r11, -0x1a80
	ctx.r[8].s64 = ctx.r[11].s64 + -6784;
	// 826AABD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AABD4: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 826AABD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AABDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AABE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AABE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AABE8: 386AA2F8  addi r3, r10, -0x5d08
	ctx.r[3].s64 = ctx.r[10].s64 + -23816;
	// 826AABEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AABF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AABF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AABF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AABFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAC04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAC0C: 4BDBC215  bl 0x82466e20
	ctx.lr = 0x826AAC10;
	sub_82466E20(ctx, base);
	// 826AAC10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAC14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAC18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAC1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAC20 size=100
    let mut pc: u32 = 0x826AAC20;
    'dispatch: loop {
        match pc {
            0x826AAC20 => {
    //   block [0x826AAC20..0x826AAC84)
	// 826AAC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAC2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAC34: 38AAA238  addi r5, r10, -0x5dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -24008;
	// 826AAC38: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AAC3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAC40: 388AB3BC  addi r4, r10, -0x4c44
	ctx.r[4].s64 = ctx.r[10].s64 + -19524;
	// 826AAC44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAC4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAC54: 386AA328  addi r3, r10, -0x5cd8
	ctx.r[3].s64 = ctx.r[10].s64 + -23768;
	// 826AAC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAC5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAC60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AAC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAC68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AAC6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAC70: 4BDBC1B1  bl 0x82466e20
	ctx.lr = 0x826AAC74;
	sub_82466E20(ctx, base);
	// 826AAC74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAC78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAC7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAC80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAC88 size=108
    let mut pc: u32 = 0x826AAC88;
    'dispatch: loop {
        match pc {
            0x826AAC88 => {
    //   block [0x826AAC88..0x826AACF4)
	// 826AAC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAC8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAC90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAC94: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAC98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAC9C: 38EBE598  addi r7, r11, -0x1a68
	ctx.r[7].s64 = ctx.r[11].s64 + -6760;
	// 826AACA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AACA4: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 826AACA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AACAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AACB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AACB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AACB8: 386AA358  addi r3, r10, -0x5ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -23720;
	// 826AACBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AACC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AACC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AACC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AACCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AACD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AACD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AACD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AACDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AACE0: 4BDBC141  bl 0x82466e20
	ctx.lr = 0x826AACE4;
	sub_82466E20(ctx, base);
	// 826AACE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AACE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AACEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AACF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AACF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AACF8 size=112
    let mut pc: u32 = 0x826AACF8;
    'dispatch: loop {
        match pc {
            0x826AACF8 => {
    //   block [0x826AACF8..0x826AAD68)
	// 826AACF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AACFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAD00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAD04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAD08: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAD0C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AAD10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAD14: 390BE5E0  addi r8, r11, -0x1a20
	ctx.r[8].s64 = ctx.r[11].s64 + -6688;
	// 826AAD18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AAD1C: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 826AAD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAD24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAD28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAD30: 386AA388  addi r3, r10, -0x5c78
	ctx.r[3].s64 = ctx.r[10].s64 + -23672;
	// 826AAD34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAD38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAD3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAD40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAD44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAD48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAD4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAD50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAD54: 4BDBC0CD  bl 0x82466e20
	ctx.lr = 0x826AAD58;
	sub_82466E20(ctx, base);
	// 826AAD58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAD68 size=112
    let mut pc: u32 = 0x826AAD68;
    'dispatch: loop {
        match pc {
            0x826AAD68 => {
    //   block [0x826AAD68..0x826AADD8)
	// 826AAD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAD70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAD74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAD78: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAD7C: 38AAA388  addi r5, r10, -0x5c78
	ctx.r[5].s64 = ctx.r[10].s64 + -23672;
	// 826AAD80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAD84: 390BE640  addi r8, r11, -0x19c0
	ctx.r[8].s64 = ctx.r[11].s64 + -6592;
	// 826AAD88: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AAD8C: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 826AAD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAD94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAD98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AADA0: 386AA3B8  addi r3, r10, -0x5c48
	ctx.r[3].s64 = ctx.r[10].s64 + -23624;
	// 826AADA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AADA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AADAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AADB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AADB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AADB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AADBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AADC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AADC4: 4BDBC05D  bl 0x82466e20
	ctx.lr = 0x826AADC8;
	sub_82466E20(ctx, base);
	// 826AADC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AADCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AADD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AADD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AADD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AADD8 size=112
    let mut pc: u32 = 0x826AADD8;
    'dispatch: loop {
        match pc {
            0x826AADD8 => {
    //   block [0x826AADD8..0x826AAE48)
	// 826AADD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AADDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AADE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AADE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AADE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AADEC: 38AAA388  addi r5, r10, -0x5c78
	ctx.r[5].s64 = ctx.r[10].s64 + -23672;
	// 826AADF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AADF4: 390BE658  addi r8, r11, -0x19a8
	ctx.r[8].s64 = ctx.r[11].s64 + -6568;
	// 826AADF8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AADFC: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 826AAE00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAE04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAE08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAE10: 386AA3E8  addi r3, r10, -0x5c18
	ctx.r[3].s64 = ctx.r[10].s64 + -23576;
	// 826AAE14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAE18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAE1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAE20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAE24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAE28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAE2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAE30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAE34: 4BDBBFED  bl 0x82466e20
	ctx.lr = 0x826AAE38;
	sub_82466E20(ctx, base);
	// 826AAE38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAE3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAE40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAE44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAE48 size=112
    let mut pc: u32 = 0x826AAE48;
    'dispatch: loop {
        match pc {
            0x826AAE48 => {
    //   block [0x826AAE48..0x826AAEB8)
	// 826AAE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAE54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAE58: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAE5C: 38AAA388  addi r5, r10, -0x5c78
	ctx.r[5].s64 = ctx.r[10].s64 + -23672;
	// 826AAE60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAE64: 390BE688  addi r8, r11, -0x1978
	ctx.r[8].s64 = ctx.r[11].s64 + -6520;
	// 826AAE68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AAE6C: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 826AAE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAE74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAE78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAE80: 386AA418  addi r3, r10, -0x5be8
	ctx.r[3].s64 = ctx.r[10].s64 + -23528;
	// 826AAE84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AAE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAE94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAEA4: 4BDBBF7D  bl 0x82466e20
	ctx.lr = 0x826AAEA8;
	sub_82466E20(ctx, base);
	// 826AAEA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAEAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAEB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAEB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAEB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AAEB8 size=24
    let mut pc: u32 = 0x826AAEB8;
    'dispatch: loop {
        match pc {
            0x826AAEB8 => {
    //   block [0x826AAEB8..0x826AAED0)
	// 826AAEB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAEBC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AAEC0: 394A4228  addi r10, r10, 0x4228
	ctx.r[10].s64 = ctx.r[10].s64 + 16936;
	// 826AAEC4: 816BE6A0  lwz r11, -0x1960(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6496 as u32) ) } as u64;
	// 826AAEC8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826AAECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAED0 size=112
    let mut pc: u32 = 0x826AAED0;
    'dispatch: loop {
        match pc {
            0x826AAED0 => {
    //   block [0x826AAED0..0x826AAF40)
	// 826AAED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAEDC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AAEE0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAEE4: 392AD148  addi r9, r10, -0x2eb8
	ctx.r[9].s64 = ctx.r[10].s64 + -11960;
	// 826AAEE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAEEC: 390B4228  addi r8, r11, 0x4228
	ctx.r[8].s64 = ctx.r[11].s64 + 16936;
	// 826AAEF0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826AAEF4: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 826AAEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAEFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAF00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AAF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAF08: 386AA448  addi r3, r10, -0x5bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -23480;
	// 826AAF0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AAF10: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AAF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAF1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAF24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AAF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAF2C: 4BDBBEF5  bl 0x82466e20
	ctx.lr = 0x826AAF30;
	sub_82466E20(ctx, base);
	// 826AAF30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAF40 size=108
    let mut pc: u32 = 0x826AAF40;
    'dispatch: loop {
        match pc {
            0x826AAF40 => {
    //   block [0x826AAF40..0x826AAFAC)
	// 826AAF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAF4C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAF50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAF54: 38EBE6A4  addi r7, r11, -0x195c
	ctx.r[7].s64 = ctx.r[11].s64 + -6492;
	// 826AAF58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826AAF5C: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 826AAF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAF64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAF68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AAF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAF70: 386AA478  addi r3, r10, -0x5b88
	ctx.r[3].s64 = ctx.r[10].s64 + -23432;
	// 826AAF74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AAF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAF80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAF88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AAF90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AAF94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AAF98: 4BDBBE89  bl 0x82466e20
	ctx.lr = 0x826AAF9C;
	sub_82466E20(ctx, base);
	// 826AAF9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AAFA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AAFA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AAFA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AAFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AAFB0 size=108
    let mut pc: u32 = 0x826AAFB0;
    'dispatch: loop {
        match pc {
            0x826AAFB0 => {
    //   block [0x826AAFB0..0x826AB01C)
	// 826AAFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AAFB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AAFB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AAFBC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AAFC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AAFC4: 38EBE6C0  addi r7, r11, -0x1940
	ctx.r[7].s64 = ctx.r[11].s64 + -6464;
	// 826AAFC8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AAFCC: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 826AAFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AAFD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AAFD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AAFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AAFE0: 386AA4A8  addi r3, r10, -0x5b58
	ctx.r[3].s64 = ctx.r[10].s64 + -23384;
	// 826AAFE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AAFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AAFEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AAFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AAFF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AAFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AAFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB008: 4BDBBE19  bl 0x82466e20
	ctx.lr = 0x826AB00C;
	sub_82466E20(ctx, base);
	// 826AB00C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB020 size=116
    let mut pc: u32 = 0x826AB020;
    'dispatch: loop {
        match pc {
            0x826AB020 => {
    //   block [0x826AB020..0x826AB094)
	// 826AB020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB02C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB030: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB034: 390BE708  addi r8, r11, -0x18f8
	ctx.r[8].s64 = ctx.r[11].s64 + -6392;
	// 826AB038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB03C: 392AD210  addi r9, r10, -0x2df0
	ctx.r[9].s64 = ctx.r[10].s64 + -11760;
	// 826AB040: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB044: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AB048: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AB04C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB054: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB05C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB064: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AB068: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 826AB06C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB070: 386BA4D8  addi r3, r11, -0x5b28
	ctx.r[3].s64 = ctx.r[11].s64 + -23336;
	// 826AB074: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB078: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB080: 4BDBBDA1  bl 0x82466e20
	ctx.lr = 0x826AB084;
	sub_82466E20(ctx, base);
	// 826AB084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AB098 size=24
    let mut pc: u32 = 0x826AB098;
    'dispatch: loop {
        match pc {
            0x826AB098 => {
    //   block [0x826AB098..0x826AB0B0)
	// 826AB098: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB09C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AB0A0: 394A4270  addi r10, r10, 0x4270
	ctx.r[10].s64 = ctx.r[10].s64 + 17008;
	// 826AB0A4: 816BE720  lwz r11, -0x18e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6368 as u32) ) } as u64;
	// 826AB0A8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826AB0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB0B0 size=116
    let mut pc: u32 = 0x826AB0B0;
    'dispatch: loop {
        match pc {
            0x826AB0B0 => {
    //   block [0x826AB0B0..0x826AB124)
	// 826AB0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB0BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB0C0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB0C4: 390B4270  addi r8, r11, 0x4270
	ctx.r[8].s64 = ctx.r[11].s64 + 17008;
	// 826AB0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB0CC: 392AD280  addi r9, r10, -0x2d80
	ctx.r[9].s64 = ctx.r[10].s64 + -11648;
	// 826AB0D0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB0D4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826AB0D8: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AB0DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB0E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB0F4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AB0F8: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 826AB0FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB100: 386BA508  addi r3, r11, -0x5af8
	ctx.r[3].s64 = ctx.r[11].s64 + -23288;
	// 826AB104: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826AB108: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB110: 4BDBBD11  bl 0x82466e20
	ctx.lr = 0x826AB114;
	sub_82466E20(ctx, base);
	// 826AB114: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB11C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB128 size=108
    let mut pc: u32 = 0x826AB128;
    'dispatch: loop {
        match pc {
            0x826AB128 => {
    //   block [0x826AB128..0x826AB194)
	// 826AB128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB134: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB138: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB13C: 38EBE730  addi r7, r11, -0x18d0
	ctx.r[7].s64 = ctx.r[11].s64 + -6352;
	// 826AB140: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AB144: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 826AB148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB14C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AB154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB158: 386AA538  addi r3, r10, -0x5ac8
	ctx.r[3].s64 = ctx.r[10].s64 + -23240;
	// 826AB15C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AB160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB180: 4BDBBCA1  bl 0x82466e20
	ctx.lr = 0x826AB184;
	sub_82466E20(ctx, base);
	// 826AB184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB198 size=112
    let mut pc: u32 = 0x826AB198;
    'dispatch: loop {
        match pc {
            0x826AB198 => {
    //   block [0x826AB198..0x826AB208)
	// 826AB198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB1A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB1A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB1AC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB1B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB1B4: 390BE760  addi r8, r11, -0x18a0
	ctx.r[8].s64 = ctx.r[11].s64 + -6304;
	// 826AB1B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB1BC: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 826AB1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB1C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB1C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB1D0: 386AA568  addi r3, r10, -0x5a98
	ctx.r[3].s64 = ctx.r[10].s64 + -23192;
	// 826AB1D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB1F4: 4BDBBC2D  bl 0x82466e20
	ctx.lr = 0x826AB1F8;
	sub_82466E20(ctx, base);
	// 826AB1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB208 size=112
    let mut pc: u32 = 0x826AB208;
    'dispatch: loop {
        match pc {
            0x826AB208 => {
    //   block [0x826AB208..0x826AB278)
	// 826AB208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB214: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB218: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB21C: 392AD2D8  addi r9, r10, -0x2d28
	ctx.r[9].s64 = ctx.r[10].s64 + -11560;
	// 826AB220: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB224: 390BE780  addi r8, r11, -0x1880
	ctx.r[8].s64 = ctx.r[11].s64 + -6272;
	// 826AB228: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826AB22C: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 826AB230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB234: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB238: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB240: 386AA598  addi r3, r10, -0x5a68
	ctx.r[3].s64 = ctx.r[10].s64 + -23144;
	// 826AB244: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB248: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB25C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB264: 4BDBBBBD  bl 0x82466e20
	ctx.lr = 0x826AB268;
	sub_82466E20(ctx, base);
	// 826AB268: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB278 size=112
    let mut pc: u32 = 0x826AB278;
    'dispatch: loop {
        match pc {
            0x826AB278 => {
    //   block [0x826AB278..0x826AB2E8)
	// 826AB278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB284: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB288: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB28C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB290: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB294: 390BE7C8  addi r8, r11, -0x1838
	ctx.r[8].s64 = ctx.r[11].s64 + -6200;
	// 826AB298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB29C: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 826AB2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB2A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB2A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB2B0: 386AA5C8  addi r3, r10, -0x5a38
	ctx.r[3].s64 = ctx.r[10].s64 + -23096;
	// 826AB2B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB2B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB2C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB2C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB2D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB2D4: 4BDBBB4D  bl 0x82466e20
	ctx.lr = 0x826AB2D8;
	sub_82466E20(ctx, base);
	// 826AB2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB2E8 size=112
    let mut pc: u32 = 0x826AB2E8;
    'dispatch: loop {
        match pc {
            0x826AB2E8 => {
    //   block [0x826AB2E8..0x826AB358)
	// 826AB2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB2F4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB2F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB2FC: 392AD304  addi r9, r10, -0x2cfc
	ctx.r[9].s64 = ctx.r[10].s64 + -11516;
	// 826AB300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB304: 390BE7E0  addi r8, r11, -0x1820
	ctx.r[8].s64 = ctx.r[11].s64 + -6176;
	// 826AB308: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826AB30C: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 826AB310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB314: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB320: 386AA5F8  addi r3, r10, -0x5a08
	ctx.r[3].s64 = ctx.r[10].s64 + -23048;
	// 826AB324: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB328: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB32C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB33C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB344: 4BDBBADD  bl 0x82466e20
	ctx.lr = 0x826AB348;
	sub_82466E20(ctx, base);
	// 826AB348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB358 size=112
    let mut pc: u32 = 0x826AB358;
    'dispatch: loop {
        match pc {
            0x826AB358 => {
    //   block [0x826AB358..0x826AB3C8)
	// 826AB358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB364: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB368: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB36C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB370: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB374: 390BE870  addi r8, r11, -0x1790
	ctx.r[8].s64 = ctx.r[11].s64 + -6032;
	// 826AB378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB37C: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 826AB380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB384: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB390: 386AA628  addi r3, r10, -0x59d8
	ctx.r[3].s64 = ctx.r[10].s64 + -23000;
	// 826AB394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB3B4: 4BDBBA6D  bl 0x82466e20
	ctx.lr = 0x826AB3B8;
	sub_82466E20(ctx, base);
	// 826AB3B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB3C8 size=112
    let mut pc: u32 = 0x826AB3C8;
    'dispatch: loop {
        match pc {
            0x826AB3C8 => {
    //   block [0x826AB3C8..0x826AB438)
	// 826AB3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB3D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB3D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB3DC: 38AAA688  addi r5, r10, -0x5978
	ctx.r[5].s64 = ctx.r[10].s64 + -22904;
	// 826AB3E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB3E4: 390BE888  addi r8, r11, -0x1778
	ctx.r[8].s64 = ctx.r[11].s64 + -6008;
	// 826AB3E8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826AB3EC: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 826AB3F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB3F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB3F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB3FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB400: 386AA658  addi r3, r10, -0x59a8
	ctx.r[3].s64 = ctx.r[10].s64 + -22952;
	// 826AB404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB41C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB424: 4BDBB9FD  bl 0x82466e20
	ctx.lr = 0x826AB428;
	sub_82466E20(ctx, base);
	// 826AB428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB42C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB438 size=100
    let mut pc: u32 = 0x826AB438;
    'dispatch: loop {
        match pc {
            0x826AB438 => {
    //   block [0x826AB438..0x826AB49C)
	// 826AB438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB444: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB44C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AB450: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB458: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 826AB45C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB46C: 386AA688  addi r3, r10, -0x5978
	ctx.r[3].s64 = ctx.r[10].s64 + -22904;
	// 826AB470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB474: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB478: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AB47C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB480: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AB484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB488: 4BDBB999  bl 0x82466e20
	ctx.lr = 0x826AB48C;
	sub_82466E20(ctx, base);
	// 826AB48C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AB4A0 size=24
    let mut pc: u32 = 0x826AB4A0;
    'dispatch: loop {
        match pc {
            0x826AB4A0 => {
    //   block [0x826AB4A0..0x826AB4B8)
	// 826AB4A0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB4A4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AB4A8: 394A4348  addi r10, r10, 0x4348
	ctx.r[10].s64 = ctx.r[10].s64 + 17224;
	// 826AB4AC: 816BE900  lwz r11, -0x1700(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5888 as u32) ) } as u64;
	// 826AB4B0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826AB4B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB4B8 size=116
    let mut pc: u32 = 0x826AB4B8;
    'dispatch: loop {
        match pc {
            0x826AB4B8 => {
    //   block [0x826AB4B8..0x826AB52C)
	// 826AB4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB4C4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB4C8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB4CC: 390B4348  addi r8, r11, 0x4348
	ctx.r[8].s64 = ctx.r[11].s64 + 17224;
	// 826AB4D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB4D4: 392AD340  addi r9, r10, -0x2cc0
	ctx.r[9].s64 = ctx.r[10].s64 + -11456;
	// 826AB4D8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB4DC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826AB4E0: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB4E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB4EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB4FC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AB500: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 826AB504: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB508: 386BA6B8  addi r3, r11, -0x5948
	ctx.r[3].s64 = ctx.r[11].s64 + -22856;
	// 826AB50C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB510: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB518: 4BDBB909  bl 0x82466e20
	ctx.lr = 0x826AB51C;
	sub_82466E20(ctx, base);
	// 826AB51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB530 size=108
    let mut pc: u32 = 0x826AB530;
    'dispatch: loop {
        match pc {
            0x826AB530 => {
    //   block [0x826AB530..0x826AB59C)
	// 826AB530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB53C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB540: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB544: 38EBE904  addi r7, r11, -0x16fc
	ctx.r[7].s64 = ctx.r[11].s64 + -5884;
	// 826AB548: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AB54C: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 826AB550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB554: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB558: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AB55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB560: 386AA6E8  addi r3, r10, -0x5918
	ctx.r[3].s64 = ctx.r[10].s64 + -22808;
	// 826AB564: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AB568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB56C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB584: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB588: 4BDBB899  bl 0x82466e20
	ctx.lr = 0x826AB58C;
	sub_82466E20(ctx, base);
	// 826AB58C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB5A0 size=112
    let mut pc: u32 = 0x826AB5A0;
    'dispatch: loop {
        match pc {
            0x826AB5A0 => {
    //   block [0x826AB5A0..0x826AB610)
	// 826AB5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB5AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB5B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB5B4: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB5B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB5BC: 390BE934  addi r8, r11, -0x16cc
	ctx.r[8].s64 = ctx.r[11].s64 + -5836;
	// 826AB5C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB5C4: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 826AB5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB5CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB5D8: 386AA718  addi r3, r10, -0x58e8
	ctx.r[3].s64 = ctx.r[10].s64 + -22760;
	// 826AB5DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB5E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB5F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB5FC: 4BDBB825  bl 0x82466e20
	ctx.lr = 0x826AB600;
	sub_82466E20(ctx, base);
	// 826AB600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB610 size=112
    let mut pc: u32 = 0x826AB610;
    'dispatch: loop {
        match pc {
            0x826AB610 => {
    //   block [0x826AB610..0x826AB680)
	// 826AB610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB61C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB620: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB624: 392AD364  addi r9, r10, -0x2c9c
	ctx.r[9].s64 = ctx.r[10].s64 + -11420;
	// 826AB628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB62C: 390BE950  addi r8, r11, -0x16b0
	ctx.r[8].s64 = ctx.r[11].s64 + -5808;
	// 826AB630: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826AB634: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 826AB638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB63C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB640: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB648: 386AA748  addi r3, r10, -0x58b8
	ctx.r[3].s64 = ctx.r[10].s64 + -22712;
	// 826AB64C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB650: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB66C: 4BDBB7B5  bl 0x82466e20
	ctx.lr = 0x826AB670;
	sub_82466E20(ctx, base);
	// 826AB670: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB674: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB678: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB67C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB680 size=112
    let mut pc: u32 = 0x826AB680;
    'dispatch: loop {
        match pc {
            0x826AB680 => {
    //   block [0x826AB680..0x826AB6F0)
	// 826AB680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB68C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB690: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB694: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB69C: 390BE9F8  addi r8, r11, -0x1608
	ctx.r[8].s64 = ctx.r[11].s64 + -5640;
	// 826AB6A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB6A4: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 826AB6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB6AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB6B8: 386AA778  addi r3, r10, -0x5888
	ctx.r[3].s64 = ctx.r[10].s64 + -22664;
	// 826AB6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB6DC: 4BDBB745  bl 0x82466e20
	ctx.lr = 0x826AB6E0;
	sub_82466E20(ctx, base);
	// 826AB6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB6F0 size=112
    let mut pc: u32 = 0x826AB6F0;
    'dispatch: loop {
        match pc {
            0x826AB6F0 => {
    //   block [0x826AB6F0..0x826AB760)
	// 826AB6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB6FC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB700: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB704: 392AD3BC  addi r9, r10, -0x2c44
	ctx.r[9].s64 = ctx.r[10].s64 + -11332;
	// 826AB708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB70C: 390BEA18  addi r8, r11, -0x15e8
	ctx.r[8].s64 = ctx.r[11].s64 + -5608;
	// 826AB710: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826AB714: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 826AB718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB71C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB728: 386AA7A8  addi r3, r10, -0x5858
	ctx.r[3].s64 = ctx.r[10].s64 + -22616;
	// 826AB72C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB730: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB74C: 4BDBB6D5  bl 0x82466e20
	ctx.lr = 0x826AB750;
	sub_82466E20(ctx, base);
	// 826AB750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB760 size=116
    let mut pc: u32 = 0x826AB760;
    'dispatch: loop {
        match pc {
            0x826AB760 => {
    //   block [0x826AB760..0x826AB7D4)
	// 826AB760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB76C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB770: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB774: 390BEAC0  addi r8, r11, -0x1540
	ctx.r[8].s64 = ctx.r[11].s64 + -5440;
	// 826AB778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB77C: 392AD390  addi r9, r10, -0x2c70
	ctx.r[9].s64 = ctx.r[10].s64 + -11376;
	// 826AB780: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB784: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AB788: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB78C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB794: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB7A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB7A4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AB7A8: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 826AB7AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB7B0: 386BA7D8  addi r3, r11, -0x5828
	ctx.r[3].s64 = ctx.r[11].s64 + -22568;
	// 826AB7B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB7B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB7C0: 4BDBB661  bl 0x82466e20
	ctx.lr = 0x826AB7C4;
	sub_82466E20(ctx, base);
	// 826AB7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB7D8 size=108
    let mut pc: u32 = 0x826AB7D8;
    'dispatch: loop {
        match pc {
            0x826AB7D8 => {
    //   block [0x826AB7D8..0x826AB844)
	// 826AB7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB7E4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB7E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB7EC: 38EBEAD8  addi r7, r11, -0x1528
	ctx.r[7].s64 = ctx.r[11].s64 + -5416;
	// 826AB7F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AB7F4: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 826AB7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB7FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB800: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AB804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB808: 386AA808  addi r3, r10, -0x57f8
	ctx.r[3].s64 = ctx.r[10].s64 + -22520;
	// 826AB80C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AB810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB82C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB830: 4BDBB5F1  bl 0x82466e20
	ctx.lr = 0x826AB834;
	sub_82466E20(ctx, base);
	// 826AB834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB848 size=112
    let mut pc: u32 = 0x826AB848;
    'dispatch: loop {
        match pc {
            0x826AB848 => {
    //   block [0x826AB848..0x826AB8B8)
	// 826AB848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB858: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB85C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB860: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB864: 390BEB08  addi r8, r11, -0x14f8
	ctx.r[8].s64 = ctx.r[11].s64 + -5368;
	// 826AB868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AB86C: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 826AB870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB87C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB880: 386AA838  addi r3, r10, -0x57c8
	ctx.r[3].s64 = ctx.r[10].s64 + -22472;
	// 826AB884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB8A4: 4BDBB57D  bl 0x82466e20
	ctx.lr = 0x826AB8A8;
	sub_82466E20(ctx, base);
	// 826AB8A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB8AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB8B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB8B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB8B8 size=112
    let mut pc: u32 = 0x826AB8B8;
    'dispatch: loop {
        match pc {
            0x826AB8B8 => {
    //   block [0x826AB8B8..0x826AB928)
	// 826AB8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB8BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB8C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB8C4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AB8C8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB8CC: 392AD3F0  addi r9, r10, -0x2c10
	ctx.r[9].s64 = ctx.r[10].s64 + -11280;
	// 826AB8D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB8D4: 390BEB28  addi r8, r11, -0x14d8
	ctx.r[8].s64 = ctx.r[11].s64 + -5336;
	// 826AB8D8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826AB8DC: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 826AB8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB8E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB8E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB8EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB8F0: 386AA868  addi r3, r10, -0x5798
	ctx.r[3].s64 = ctx.r[10].s64 + -22424;
	// 826AB8F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AB8F8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AB8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB90C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB914: 4BDBB50D  bl 0x82466e20
	ctx.lr = 0x826AB918;
	sub_82466E20(ctx, base);
	// 826AB918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB91C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB928 size=112
    let mut pc: u32 = 0x826AB928;
    'dispatch: loop {
        match pc {
            0x826AB928 => {
    //   block [0x826AB928..0x826AB998)
	// 826AB928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB934: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB938: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB93C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AB940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB944: 390BEBD0  addi r8, r11, -0x1430
	ctx.r[8].s64 = ctx.r[11].s64 + -5168;
	// 826AB948: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AB94C: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 826AB950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AB95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB960: 386AA898  addi r3, r10, -0x5768
	ctx.r[3].s64 = ctx.r[10].s64 + -22376;
	// 826AB964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AB968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB984: 4BDBB49D  bl 0x82466e20
	ctx.lr = 0x826AB988;
	sub_82466E20(ctx, base);
	// 826AB988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AB994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AB998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AB998 size=108
    let mut pc: u32 = 0x826AB998;
    'dispatch: loop {
        match pc {
            0x826AB998 => {
    //   block [0x826AB998..0x826ABA04)
	// 826AB998: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AB99C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AB9A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AB9A4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AB9A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AB9AC: 38EBEC18  addi r7, r11, -0x13e8
	ctx.r[7].s64 = ctx.r[11].s64 + -5096;
	// 826AB9B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AB9B4: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 826AB9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AB9BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AB9C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AB9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AB9C8: 386AA8C8  addi r3, r10, -0x5738
	ctx.r[3].s64 = ctx.r[10].s64 + -22328;
	// 826AB9CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AB9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AB9D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AB9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AB9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AB9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AB9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AB9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AB9EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AB9F0: 4BDBB431  bl 0x82466e20
	ctx.lr = 0x826AB9F4;
	sub_82466E20(ctx, base);
	// 826AB9F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AB9F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AB9FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABA00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABA08 size=112
    let mut pc: u32 = 0x826ABA08;
    'dispatch: loop {
        match pc {
            0x826ABA08 => {
    //   block [0x826ABA08..0x826ABA78)
	// 826ABA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABA0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABA10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABA14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABA18: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABA1C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABA20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABA24: 390BEC48  addi r8, r11, -0x13b8
	ctx.r[8].s64 = ctx.r[11].s64 + -5048;
	// 826ABA28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ABA2C: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 826ABA30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABA34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABA38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABA40: 386AA8F8  addi r3, r10, -0x5708
	ctx.r[3].s64 = ctx.r[10].s64 + -22280;
	// 826ABA44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABA48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABA50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABA54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABA58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABA5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABA60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABA64: 4BDBB3BD  bl 0x82466e20
	ctx.lr = 0x826ABA68;
	sub_82466E20(ctx, base);
	// 826ABA68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABA78 size=112
    let mut pc: u32 = 0x826ABA78;
    'dispatch: loop {
        match pc {
            0x826ABA78 => {
    //   block [0x826ABA78..0x826ABAE8)
	// 826ABA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABA84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABA88: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABA8C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABA90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABA94: 390BEC60  addi r8, r11, -0x13a0
	ctx.r[8].s64 = ctx.r[11].s64 + -5024;
	// 826ABA98: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826ABA9C: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 826ABAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABAA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABAA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABAAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABAB0: 386AA928  addi r3, r10, -0x56d8
	ctx.r[3].s64 = ctx.r[10].s64 + -22232;
	// 826ABAB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABAD4: 4BDBB34D  bl 0x82466e20
	ctx.lr = 0x826ABAD8;
	sub_82466E20(ctx, base);
	// 826ABAD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABAE8 size=100
    let mut pc: u32 = 0x826ABAE8;
    'dispatch: loop {
        match pc {
            0x826ABAE8 => {
    //   block [0x826ABAE8..0x826ABB4C)
	// 826ABAE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABAEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABAF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABAF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABAFC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABB00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABB04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABB08: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 826ABB0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABB10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABB14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABB18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABB1C: 386AA958  addi r3, r10, -0x56a8
	ctx.r[3].s64 = ctx.r[10].s64 + -22184;
	// 826ABB20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABB24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABB28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ABB2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABB30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ABB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABB38: 4BDBB2E9  bl 0x82466e20
	ctx.lr = 0x826ABB3C;
	sub_82466E20(ctx, base);
	// 826ABB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABB50 size=112
    let mut pc: u32 = 0x826ABB50;
    'dispatch: loop {
        match pc {
            0x826ABB50 => {
    //   block [0x826ABB50..0x826ABBC0)
	// 826ABB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABB5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABB60: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABB64: 38AAA508  addi r5, r10, -0x5af8
	ctx.r[5].s64 = ctx.r[10].s64 + -23288;
	// 826ABB68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABB6C: 390BED20  addi r8, r11, -0x12e0
	ctx.r[8].s64 = ctx.r[11].s64 + -4832;
	// 826ABB70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826ABB74: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 826ABB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABB7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABB80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABB88: 386AA988  addi r3, r10, -0x5678
	ctx.r[3].s64 = ctx.r[10].s64 + -22136;
	// 826ABB8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABBAC: 4BDBB275  bl 0x82466e20
	ctx.lr = 0x826ABBB0;
	sub_82466E20(ctx, base);
	// 826ABBB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABBC0 size=112
    let mut pc: u32 = 0x826ABBC0;
    'dispatch: loop {
        match pc {
            0x826ABBC0 => {
    //   block [0x826ABBC0..0x826ABC30)
	// 826ABBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABBCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABBD0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABBD4: 38AAA388  addi r5, r10, -0x5c78
	ctx.r[5].s64 = ctx.r[10].s64 + -23672;
	// 826ABBD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABBDC: 390BED50  addi r8, r11, -0x12b0
	ctx.r[8].s64 = ctx.r[11].s64 + -4784;
	// 826ABBE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ABBE4: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 826ABBE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABBEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABBF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABBF8: 386AA9B8  addi r3, r10, -0x5648
	ctx.r[3].s64 = ctx.r[10].s64 + -22088;
	// 826ABBFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABC1C: 4BDBB205  bl 0x82466e20
	ctx.lr = 0x826ABC20;
	sub_82466E20(ctx, base);
	// 826ABC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABC30 size=108
    let mut pc: u32 = 0x826ABC30;
    'dispatch: loop {
        match pc {
            0x826ABC30 => {
    //   block [0x826ABC30..0x826ABC9C)
	// 826ABC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABC3C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABC40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABC44: 38EBED68  addi r7, r11, -0x1298
	ctx.r[7].s64 = ctx.r[11].s64 + -4760;
	// 826ABC48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826ABC4C: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 826ABC50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABC54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABC58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ABC5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABC60: 386AA9E8  addi r3, r10, -0x5618
	ctx.r[3].s64 = ctx.r[10].s64 + -22040;
	// 826ABC64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ABC68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABC6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABC70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABC78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABC7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABC80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABC84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ABC88: 4BDBB199  bl 0x82466e20
	ctx.lr = 0x826ABC8C;
	sub_82466E20(ctx, base);
	// 826ABC8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABC90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABC94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABC98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABCA0 size=112
    let mut pc: u32 = 0x826ABCA0;
    'dispatch: loop {
        match pc {
            0x826ABCA0 => {
    //   block [0x826ABCA0..0x826ABD10)
	// 826ABCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABCAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABCB0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABCB4: 38AAA958  addi r5, r10, -0x56a8
	ctx.r[5].s64 = ctx.r[10].s64 + -22184;
	// 826ABCB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABCBC: 390BED98  addi r8, r11, -0x1268
	ctx.r[8].s64 = ctx.r[11].s64 + -4712;
	// 826ABCC0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826ABCC4: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 826ABCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABCCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABCD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABCD8: 386AAA18  addi r3, r10, -0x55e8
	ctx.r[3].s64 = ctx.r[10].s64 + -21992;
	// 826ABCDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABCE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABCE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABCE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABCEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABCF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABCF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABCFC: 4BDBB125  bl 0x82466e20
	ctx.lr = 0x826ABD00;
	sub_82466E20(ctx, base);
	// 826ABD00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABD04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABD08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABD10 size=112
    let mut pc: u32 = 0x826ABD10;
    'dispatch: loop {
        match pc {
            0x826ABD10 => {
    //   block [0x826ABD10..0x826ABD80)
	// 826ABD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABD1C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826ABD20: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABD24: 392AD41C  addi r9, r10, -0x2be4
	ctx.r[9].s64 = ctx.r[10].s64 + -11236;
	// 826ABD28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABD2C: 390BEE28  addi r8, r11, -0x11d8
	ctx.r[8].s64 = ctx.r[11].s64 + -4568;
	// 826ABD30: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826ABD34: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 826ABD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABD3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABD40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABD44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABD48: 386AAA48  addi r3, r10, -0x55b8
	ctx.r[3].s64 = ctx.r[10].s64 + -21944;
	// 826ABD4C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ABD50: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ABD54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABD64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ABD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABD6C: 4BDBB0B5  bl 0x82466e20
	ctx.lr = 0x826ABD70;
	sub_82466E20(ctx, base);
	// 826ABD70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABD80 size=112
    let mut pc: u32 = 0x826ABD80;
    'dispatch: loop {
        match pc {
            0x826ABD80 => {
    //   block [0x826ABD80..0x826ABDF0)
	// 826ABD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABD8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABD90: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABD94: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABD98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABD9C: 390BEE70  addi r8, r11, -0x1190
	ctx.r[8].s64 = ctx.r[11].s64 + -4496;
	// 826ABDA0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ABDA4: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 826ABDA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABDAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABDB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABDB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABDB8: 386AAA78  addi r3, r10, -0x5588
	ctx.r[3].s64 = ctx.r[10].s64 + -21896;
	// 826ABDBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABDC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABDC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABDC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABDCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABDD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABDD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABDD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABDDC: 4BDBB045  bl 0x82466e20
	ctx.lr = 0x826ABDE0;
	sub_82466E20(ctx, base);
	// 826ABDE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABDF0 size=108
    let mut pc: u32 = 0x826ABDF0;
    'dispatch: loop {
        match pc {
            0x826ABDF0 => {
    //   block [0x826ABDF0..0x826ABE5C)
	// 826ABDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABDF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABDFC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABE00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABE04: 38EBEE88  addi r7, r11, -0x1178
	ctx.r[7].s64 = ctx.r[11].s64 + -4472;
	// 826ABE08: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826ABE0C: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 826ABE10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABE14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABE18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ABE1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABE20: 386AAAA8  addi r3, r10, -0x5558
	ctx.r[3].s64 = ctx.r[10].s64 + -21848;
	// 826ABE24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ABE28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABE30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABE34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABE38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABE40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABE44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ABE48: 4BDBAFD9  bl 0x82466e20
	ctx.lr = 0x826ABE4C;
	sub_82466E20(ctx, base);
	// 826ABE4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABE50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABE54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABE58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABE60 size=116
    let mut pc: u32 = 0x826ABE60;
    'dispatch: loop {
        match pc {
            0x826ABE60 => {
    //   block [0x826ABE60..0x826ABED4)
	// 826ABE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABE6C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826ABE70: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826ABE74: 390AEF18  addi r8, r10, -0x10e8
	ctx.r[8].s64 = ctx.r[10].s64 + -4328;
	// 826ABE78: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABE7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826ABE80: 38AAA958  addi r5, r10, -0x56a8
	ctx.r[5].s64 = ctx.r[10].s64 + -22184;
	// 826ABE84: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABE88: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ABE8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABE90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABE94: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 826ABE98: 396BD430  addi r11, r11, -0x2bd0
	ctx.r[11].s64 = ctx.r[11].s64 + -11216;
	// 826ABE9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABEA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABEA4: 386AAAD8  addi r3, r10, -0x5528
	ctx.r[3].s64 = ctx.r[10].s64 + -21800;
	// 826ABEA8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826ABEAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABEB0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826ABEB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABEB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABEC0: 4BDBAF61  bl 0x82466e20
	ctx.lr = 0x826ABEC4;
	sub_82466E20(ctx, base);
	// 826ABEC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABEC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABED8 size=112
    let mut pc: u32 = 0x826ABED8;
    'dispatch: loop {
        match pc {
            0x826ABED8 => {
    //   block [0x826ABED8..0x826ABF48)
	// 826ABED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABEE4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826ABEE8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABEEC: 392AD47C  addi r9, r10, -0x2b84
	ctx.r[9].s64 = ctx.r[10].s64 + -11140;
	// 826ABEF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABEF4: 390BEFF8  addi r8, r11, -0x1008
	ctx.r[8].s64 = ctx.r[11].s64 + -4104;
	// 826ABEF8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826ABEFC: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 826ABF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABF04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABF10: 386AAB08  addi r3, r10, -0x54f8
	ctx.r[3].s64 = ctx.r[10].s64 + -21752;
	// 826ABF14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ABF18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ABF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABF2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826ABF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABF34: 4BDBAEED  bl 0x82466e20
	ctx.lr = 0x826ABF38;
	sub_82466E20(ctx, base);
	// 826ABF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABF48 size=112
    let mut pc: u32 = 0x826ABF48;
    'dispatch: loop {
        match pc {
            0x826ABF48 => {
    //   block [0x826ABF48..0x826ABFB8)
	// 826ABF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABF54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABF58: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABF5C: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826ABF60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABF64: 390BF058  addi r8, r11, -0xfa8
	ctx.r[8].s64 = ctx.r[11].s64 + -4008;
	// 826ABF68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826ABF6C: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 826ABF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABF74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABF78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ABF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ABF80: 386AAB38  addi r3, r10, -0x54c8
	ctx.r[3].s64 = ctx.r[10].s64 + -21704;
	// 826ABF84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ABF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ABF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ABFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ABFA4: 4BDBAE7D  bl 0x82466e20
	ctx.lr = 0x826ABFA8;
	sub_82466E20(ctx, base);
	// 826ABFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ABFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ABFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ABFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ABFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ABFB8 size=108
    let mut pc: u32 = 0x826ABFB8;
    'dispatch: loop {
        match pc {
            0x826ABFB8 => {
    //   block [0x826ABFB8..0x826AC024)
	// 826ABFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ABFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ABFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ABFC4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ABFC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ABFCC: 38EBF070  addi r7, r11, -0xf90
	ctx.r[7].s64 = ctx.r[11].s64 + -3984;
	// 826ABFD0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826ABFD4: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 826ABFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ABFDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ABFE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826ABFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ABFE8: 386AAB68  addi r3, r10, -0x5498
	ctx.r[3].s64 = ctx.r[10].s64 + -21656;
	// 826ABFEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826ABFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ABFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ABFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ABFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC010: 4BDBAE11  bl 0x82466e20
	ctx.lr = 0x826AC014;
	sub_82466E20(ctx, base);
	// 826AC014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC028 size=112
    let mut pc: u32 = 0x826AC028;
    'dispatch: loop {
        match pc {
            0x826AC028 => {
    //   block [0x826AC028..0x826AC098)
	// 826AC028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC034: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC038: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC03C: 38AAA958  addi r5, r10, -0x56a8
	ctx.r[5].s64 = ctx.r[10].s64 + -22184;
	// 826AC040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC044: 390BF0B8  addi r8, r11, -0xf48
	ctx.r[8].s64 = ctx.r[11].s64 + -3912;
	// 826AC048: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826AC04C: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 826AC050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC054: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC060: 386AAB98  addi r3, r10, -0x5468
	ctx.r[3].s64 = ctx.r[10].s64 + -21608;
	// 826AC064: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC084: 4BDBAD9D  bl 0x82466e20
	ctx.lr = 0x826AC088;
	sub_82466E20(ctx, base);
	// 826AC088: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC08C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC090: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC098 size=112
    let mut pc: u32 = 0x826AC098;
    'dispatch: loop {
        match pc {
            0x826AC098 => {
    //   block [0x826AC098..0x826AC108)
	// 826AC098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC0A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC0A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC0AC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AC0B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC0B4: 390BF130  addi r8, r11, -0xed0
	ctx.r[8].s64 = ctx.r[11].s64 + -3792;
	// 826AC0B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AC0BC: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 826AC0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC0C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC0D0: 386AABC8  addi r3, r10, -0x5438
	ctx.r[3].s64 = ctx.r[10].s64 + -21560;
	// 826AC0D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC0F4: 4BDBAD2D  bl 0x82466e20
	ctx.lr = 0x826AC0F8;
	sub_82466E20(ctx, base);
	// 826AC0F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC108 size=108
    let mut pc: u32 = 0x826AC108;
    'dispatch: loop {
        match pc {
            0x826AC108 => {
    //   block [0x826AC108..0x826AC174)
	// 826AC108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC114: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC118: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC11C: 38EBF160  addi r7, r11, -0xea0
	ctx.r[7].s64 = ctx.r[11].s64 + -3744;
	// 826AC120: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AC124: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 826AC128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC12C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC130: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC138: 386AABF8  addi r3, r10, -0x5408
	ctx.r[3].s64 = ctx.r[10].s64 + -21512;
	// 826AC13C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC140: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC148: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC14C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC150: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC15C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC160: 4BDBACC1  bl 0x82466e20
	ctx.lr = 0x826AC164;
	sub_82466E20(ctx, base);
	// 826AC164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC178 size=108
    let mut pc: u32 = 0x826AC178;
    'dispatch: loop {
        match pc {
            0x826AC178 => {
    //   block [0x826AC178..0x826AC1E4)
	// 826AC178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC184: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC18C: 38EBF1C0  addi r7, r11, -0xe40
	ctx.r[7].s64 = ctx.r[11].s64 + -3648;
	// 826AC190: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826AC194: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 826AC198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC19C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC1A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC1A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC1A8: 386AAC28  addi r3, r10, -0x53d8
	ctx.r[3].s64 = ctx.r[10].s64 + -21464;
	// 826AC1AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC1B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC1B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC1C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC1C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC1C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC1CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC1D0: 4BDBAC51  bl 0x82466e20
	ctx.lr = 0x826AC1D4;
	sub_82466E20(ctx, base);
	// 826AC1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC1E8 size=112
    let mut pc: u32 = 0x826AC1E8;
    'dispatch: loop {
        match pc {
            0x826AC1E8 => {
    //   block [0x826AC1E8..0x826AC258)
	// 826AC1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC1F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC1F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC1FC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AC200: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC204: 390BF238  addi r8, r11, -0xdc8
	ctx.r[8].s64 = ctx.r[11].s64 + -3528;
	// 826AC208: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AC20C: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 826AC210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC214: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC220: 386AAC58  addi r3, r10, -0x53a8
	ctx.r[3].s64 = ctx.r[10].s64 + -21416;
	// 826AC224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC244: 4BDBABDD  bl 0x82466e20
	ctx.lr = 0x826AC248;
	sub_82466E20(ctx, base);
	// 826AC248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AC258 size=24
    let mut pc: u32 = 0x826AC258;
    'dispatch: loop {
        match pc {
            0x826AC258 => {
    //   block [0x826AC258..0x826AC270)
	// 826AC258: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC25C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AC260: 394A43C0  addi r10, r10, 0x43c0
	ctx.r[10].s64 = ctx.r[10].s64 + 17344;
	// 826AC264: 816BEFF4  lwz r11, -0x100c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4108 as u32) ) } as u64;
	// 826AC268: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826AC26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC270 size=116
    let mut pc: u32 = 0x826AC270;
    'dispatch: loop {
        match pc {
            0x826AC270 => {
    //   block [0x826AC270..0x826AC2E4)
	// 826AC270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC27C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC280: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AC284: 390B43C0  addi r8, r11, 0x43c0
	ctx.r[8].s64 = ctx.r[11].s64 + 17344;
	// 826AC288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC28C: 392AD4C0  addi r9, r10, -0x2b40
	ctx.r[9].s64 = ctx.r[10].s64 + -11072;
	// 826AC290: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC294: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826AC298: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AC29C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC2A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC2B4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AC2B8: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 826AC2BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AC2C0: 386BAC88  addi r3, r11, -0x5378
	ctx.r[3].s64 = ctx.r[11].s64 + -21368;
	// 826AC2C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AC2C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC2D0: 4BDBAB51  bl 0x82466e20
	ctx.lr = 0x826AC2D4;
	sub_82466E20(ctx, base);
	// 826AC2D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC2D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC2DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC2E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC2E8 size=112
    let mut pc: u32 = 0x826AC2E8;
    'dispatch: loop {
        match pc {
            0x826AC2E8 => {
    //   block [0x826AC2E8..0x826AC358)
	// 826AC2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC2F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC2F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC2FC: 38AAAC88  addi r5, r10, -0x5378
	ctx.r[5].s64 = ctx.r[10].s64 + -21368;
	// 826AC300: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC304: 390BF280  addi r8, r11, -0xd80
	ctx.r[8].s64 = ctx.r[11].s64 + -3456;
	// 826AC308: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AC30C: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 826AC310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC314: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC31C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC320: 386AACB8  addi r3, r10, -0x5348
	ctx.r[3].s64 = ctx.r[10].s64 + -21320;
	// 826AC324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC33C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC344: 4BDBAADD  bl 0x82466e20
	ctx.lr = 0x826AC348;
	sub_82466E20(ctx, base);
	// 826AC348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826AC358 size=24
    let mut pc: u32 = 0x826AC358;
    'dispatch: loop {
        match pc {
            0x826AC358 => {
    //   block [0x826AC358..0x826AC370)
	// 826AC358: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC35C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826AC360: 394A43D8  addi r10, r10, 0x43d8
	ctx.r[10].s64 = ctx.r[10].s64 + 17368;
	// 826AC364: 816BF2B0  lwz r11, -0xd50(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3408 as u32) ) } as u64;
	// 826AC368: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826AC36C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC370 size=116
    let mut pc: u32 = 0x826AC370;
    'dispatch: loop {
        match pc {
            0x826AC370 => {
    //   block [0x826AC370..0x826AC3E4)
	// 826AC370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC37C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC380: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AC384: 390B43D8  addi r8, r11, 0x43d8
	ctx.r[8].s64 = ctx.r[11].s64 + 17368;
	// 826AC388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC38C: 392AD4FC  addi r9, r10, -0x2b04
	ctx.r[9].s64 = ctx.r[10].s64 + -11012;
	// 826AC390: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC394: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826AC398: 38AAACB8  addi r5, r10, -0x5348
	ctx.r[5].s64 = ctx.r[10].s64 + -21320;
	// 826AC39C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC3A4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC3AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC3B4: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826AC3B8: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 826AC3BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826AC3C0: 386BACE8  addi r3, r11, -0x5318
	ctx.r[3].s64 = ctx.r[11].s64 + -21272;
	// 826AC3C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AC3C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC3CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC3D0: 4BDBAA51  bl 0x82466e20
	ctx.lr = 0x826AC3D4;
	sub_82466E20(ctx, base);
	// 826AC3D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC3D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC3DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC3E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC3E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC3E8 size=112
    let mut pc: u32 = 0x826AC3E8;
    'dispatch: loop {
        match pc {
            0x826AC3E8 => {
    //   block [0x826AC3E8..0x826AC458)
	// 826AC3E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC3EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC3F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC3F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC3F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC3FC: 38AAACB8  addi r5, r10, -0x5348
	ctx.r[5].s64 = ctx.r[10].s64 + -21320;
	// 826AC400: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC404: 390BF2B8  addi r8, r11, -0xd48
	ctx.r[8].s64 = ctx.r[11].s64 + -3400;
	// 826AC408: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826AC40C: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 826AC410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC414: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC418: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC41C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC420: 386AAD18  addi r3, r10, -0x52e8
	ctx.r[3].s64 = ctx.r[10].s64 + -21224;
	// 826AC424: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC42C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC444: 4BDBA9DD  bl 0x82466e20
	ctx.lr = 0x826AC448;
	sub_82466E20(ctx, base);
	// 826AC448: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC458 size=112
    let mut pc: u32 = 0x826AC458;
    'dispatch: loop {
        match pc {
            0x826AC458 => {
    //   block [0x826AC458..0x826AC4C8)
	// 826AC458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC464: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC468: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC46C: 38AAACB8  addi r5, r10, -0x5348
	ctx.r[5].s64 = ctx.r[10].s64 + -21320;
	// 826AC470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC474: 390BF318  addi r8, r11, -0xce8
	ctx.r[8].s64 = ctx.r[11].s64 + -3304;
	// 826AC478: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826AC47C: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 826AC480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC484: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC490: 386AAD48  addi r3, r10, -0x52b8
	ctx.r[3].s64 = ctx.r[10].s64 + -21176;
	// 826AC494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC4A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC4AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC4B4: 4BDBA96D  bl 0x82466e20
	ctx.lr = 0x826AC4B8;
	sub_82466E20(ctx, base);
	// 826AC4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC4C8 size=112
    let mut pc: u32 = 0x826AC4C8;
    'dispatch: loop {
        match pc {
            0x826AC4C8 => {
    //   block [0x826AC4C8..0x826AC538)
	// 826AC4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC4D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC4D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC4DC: 38AAACB8  addi r5, r10, -0x5348
	ctx.r[5].s64 = ctx.r[10].s64 + -21320;
	// 826AC4E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC4E4: 390BF348  addi r8, r11, -0xcb8
	ctx.r[8].s64 = ctx.r[11].s64 + -3256;
	// 826AC4E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826AC4EC: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 826AC4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC4F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC500: 386AAD78  addi r3, r10, -0x5288
	ctx.r[3].s64 = ctx.r[10].s64 + -21128;
	// 826AC504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC524: 4BDBA8FD  bl 0x82466e20
	ctx.lr = 0x826AC528;
	sub_82466E20(ctx, base);
	// 826AC528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC538 size=108
    let mut pc: u32 = 0x826AC538;
    'dispatch: loop {
        match pc {
            0x826AC538 => {
    //   block [0x826AC538..0x826AC5A4)
	// 826AC538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC544: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC54C: 38EBF390  addi r7, r11, -0xc70
	ctx.r[7].s64 = ctx.r[11].s64 + -3184;
	// 826AC550: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AC554: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 826AC558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC55C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC568: 386AADA8  addi r3, r10, -0x5258
	ctx.r[3].s64 = ctx.r[10].s64 + -21080;
	// 826AC56C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC590: 4BDBA891  bl 0x82466e20
	ctx.lr = 0x826AC594;
	sub_82466E20(ctx, base);
	// 826AC594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC5A8 size=112
    let mut pc: u32 = 0x826AC5A8;
    'dispatch: loop {
        match pc {
            0x826AC5A8 => {
    //   block [0x826AC5A8..0x826AC618)
	// 826AC5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC5B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC5B8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC5BC: 38AAA4D8  addi r5, r10, -0x5b28
	ctx.r[5].s64 = ctx.r[10].s64 + -23336;
	// 826AC5C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC5C4: 390BF3C0  addi r8, r11, -0xc40
	ctx.r[8].s64 = ctx.r[11].s64 + -3136;
	// 826AC5C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826AC5CC: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 826AC5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC5D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826AC5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC5E0: 386AADD8  addi r3, r10, -0x5228
	ctx.r[3].s64 = ctx.r[10].s64 + -21032;
	// 826AC5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826AC5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC604: 4BDBA81D  bl 0x82466e20
	ctx.lr = 0x826AC608;
	sub_82466E20(ctx, base);
	// 826AC608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC618 size=108
    let mut pc: u32 = 0x826AC618;
    'dispatch: loop {
        match pc {
            0x826AC618 => {
    //   block [0x826AC618..0x826AC684)
	// 826AC618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC624: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC628: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC62C: 38EBF3D8  addi r7, r11, -0xc28
	ctx.r[7].s64 = ctx.r[11].s64 + -3112;
	// 826AC630: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826AC634: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 826AC638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC63C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC640: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC648: 386AAE08  addi r3, r10, -0x51f8
	ctx.r[3].s64 = ctx.r[10].s64 + -20984;
	// 826AC64C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC658: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC660: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC668: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC66C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC670: 4BDBA7B1  bl 0x82466e20
	ctx.lr = 0x826AC674;
	sub_82466E20(ctx, base);
	// 826AC674: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC688 size=108
    let mut pc: u32 = 0x826AC688;
    'dispatch: loop {
        match pc {
            0x826AC688 => {
    //   block [0x826AC688..0x826AC6F4)
	// 826AC688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC694: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC69C: 38EBF420  addi r7, r11, -0xbe0
	ctx.r[7].s64 = ctx.r[11].s64 + -3040;
	// 826AC6A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826AC6A4: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 826AC6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC6AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC6B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC6B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC6B8: 386AAE38  addi r3, r10, -0x51c8
	ctx.r[3].s64 = ctx.r[10].s64 + -20936;
	// 826AC6BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC6CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC6DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC6E0: 4BDBA741  bl 0x82466e20
	ctx.lr = 0x826AC6E4;
	sub_82466E20(ctx, base);
	// 826AC6E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC6E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC6EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC6F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC6F8 size=108
    let mut pc: u32 = 0x826AC6F8;
    'dispatch: loop {
        match pc {
            0x826AC6F8 => {
    //   block [0x826AC6F8..0x826AC764)
	// 826AC6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC704: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC70C: 38EBF480  addi r7, r11, -0xb80
	ctx.r[7].s64 = ctx.r[11].s64 + -2944;
	// 826AC710: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AC714: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 826AC718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC71C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC720: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC724: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC728: 386AAE68  addi r3, r10, -0x5198
	ctx.r[3].s64 = ctx.r[10].s64 + -20888;
	// 826AC72C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC73C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC74C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC750: 4BDBA6D1  bl 0x82466e20
	ctx.lr = 0x826AC754;
	sub_82466E20(ctx, base);
	// 826AC754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC768 size=116
    let mut pc: u32 = 0x826AC768;
    'dispatch: loop {
        match pc {
            0x826AC768 => {
    //   block [0x826AC768..0x826AC7DC)
	// 826AC768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC774: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826AC778: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC77C: 392BD538  addi r9, r11, -0x2ac8
	ctx.r[9].s64 = ctx.r[11].s64 + -10952;
	// 826AC780: 38AAB378  addi r5, r10, -0x4c88
	ctx.r[5].s64 = ctx.r[10].s64 + -19592;
	// 826AC784: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC788: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826AC78C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826AC790: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC794: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 826AC798: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC79C: 396BF4B0  addi r11, r11, -0xb50
	ctx.r[11].s64 = ctx.r[11].s64 + -2896;
	// 826AC7A0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826AC7A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC7A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826AC7AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC7B0: 386AAE98  addi r3, r10, -0x5168
	ctx.r[3].s64 = ctx.r[10].s64 + -20840;
	// 826AC7B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AC7B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826AC7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC7C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826AC7C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC7C8: 4BDBA659  bl 0x82466e20
	ctx.lr = 0x826AC7CC;
	sub_82466E20(ctx, base);
	// 826AC7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC7E0 size=100
    let mut pc: u32 = 0x826AC7E0;
    'dispatch: loop {
        match pc {
            0x826AC7E0 => {
    //   block [0x826AC7E0..0x826AC844)
	// 826AC7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC7EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC7F4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826AC7F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC800: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 826AC804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC814: 386AAEC8  addi r3, r10, -0x5138
	ctx.r[3].s64 = ctx.r[10].s64 + -20792;
	// 826AC818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC81C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC820: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AC824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC828: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC830: 4BDBA5F1  bl 0x82466e20
	ctx.lr = 0x826AC834;
	sub_82466E20(ctx, base);
	// 826AC834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC83C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC848 size=100
    let mut pc: u32 = 0x826AC848;
    'dispatch: loop {
        match pc {
            0x826AC848 => {
    //   block [0x826AC848..0x826AC8AC)
	// 826AC848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC85C: 38AAAF58  addi r5, r10, -0x50a8
	ctx.r[5].s64 = ctx.r[10].s64 + -20648;
	// 826AC860: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC868: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 826AC86C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC870: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC874: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC878: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC87C: 386AAEF8  addi r3, r10, -0x5108
	ctx.r[3].s64 = ctx.r[10].s64 + -20744;
	// 826AC880: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC884: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC888: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AC88C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC890: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC898: 4BDBA589  bl 0x82466e20
	ctx.lr = 0x826AC89C;
	sub_82466E20(ctx, base);
	// 826AC89C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC8B0 size=100
    let mut pc: u32 = 0x826AC8B0;
    'dispatch: loop {
        match pc {
            0x826AC8B0 => {
    //   block [0x826AC8B0..0x826AC914)
	// 826AC8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC8BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC8C4: 38AAAE98  addi r5, r10, -0x5168
	ctx.r[5].s64 = ctx.r[10].s64 + -20840;
	// 826AC8C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC8CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC8D0: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 826AC8D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC8E4: 386AAF28  addi r3, r10, -0x50d8
	ctx.r[3].s64 = ctx.r[10].s64 + -20696;
	// 826AC8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC8EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC8F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AC8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC8F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC8FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC900: 4BDBA521  bl 0x82466e20
	ctx.lr = 0x826AC904;
	sub_82466E20(ctx, base);
	// 826AC904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC918 size=104
    let mut pc: u32 = 0x826AC918;
    'dispatch: loop {
        match pc {
            0x826AC918 => {
    //   block [0x826AC918..0x826AC980)
	// 826AC918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC924: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826AC928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC92C: 392AD5B8  addi r9, r10, -0x2a48
	ctx.r[9].s64 = ctx.r[10].s64 + -10824;
	// 826AC930: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC938: 38AAAEC8  addi r5, r10, -0x5138
	ctx.r[5].s64 = ctx.r[10].s64 + -20792;
	// 826AC93C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC94C: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 826AC950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC958: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826AC95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC960: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826AC964: 386AAF58  addi r3, r10, -0x50a8
	ctx.r[3].s64 = ctx.r[10].s64 + -20648;
	// 826AC968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826AC96C: 4BDBA4B5  bl 0x82466e20
	ctx.lr = 0x826AC970;
	sub_82466E20(ctx, base);
	// 826AC970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC980 size=108
    let mut pc: u32 = 0x826AC980;
    'dispatch: loop {
        match pc {
            0x826AC980 => {
    //   block [0x826AC980..0x826AC9EC)
	// 826AC980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC98C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826AC990: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826AC994: 38EBF64C  addi r7, r11, -0x9b4
	ctx.r[7].s64 = ctx.r[11].s64 + -2484;
	// 826AC998: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826AC99C: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 826AC9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826AC9A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826AC9A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826AC9AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826AC9B0: 386AAF88  addi r3, r10, -0x5078
	ctx.r[3].s64 = ctx.r[10].s64 + -20600;
	// 826AC9B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826AC9B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826AC9BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826AC9C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826AC9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826AC9C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826AC9CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826AC9D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826AC9D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826AC9D8: 4BDBA449  bl 0x82466e20
	ctx.lr = 0x826AC9DC;
	sub_82466E20(ctx, base);
	// 826AC9DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826AC9E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826AC9E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826AC9E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826AC9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826AC9F0 size=112
    let mut pc: u32 = 0x826AC9F0;
    'dispatch: loop {
        match pc {
            0x826AC9F0 => {
    //   block [0x826AC9F0..0x826ACA60)
	// 826AC9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826AC9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826AC9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826AC9FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACA00: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACA04: 38AAAF58  addi r5, r10, -0x50a8
	ctx.r[5].s64 = ctx.r[10].s64 + -20648;
	// 826ACA08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACA0C: 390BF680  addi r8, r11, -0x980
	ctx.r[8].s64 = ctx.r[11].s64 + -2432;
	// 826ACA10: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826ACA14: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 826ACA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACA1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACA20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACA28: 386AAFB8  addi r3, r10, -0x5048
	ctx.r[3].s64 = ctx.r[10].s64 + -20552;
	// 826ACA2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826ACA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACA34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACA3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACA4C: 4BDBA3D5  bl 0x82466e20
	ctx.lr = 0x826ACA50;
	sub_82466E20(ctx, base);
	// 826ACA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826ACA60 size=24
    let mut pc: u32 = 0x826ACA60;
    'dispatch: loop {
        match pc {
            0x826ACA60 => {
    //   block [0x826ACA60..0x826ACA78)
	// 826ACA60: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACA64: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826ACA68: 394A4450  addi r10, r10, 0x4450
	ctx.r[10].s64 = ctx.r[10].s64 + 17488;
	// 826ACA6C: 816BF67C  lwz r11, -0x984(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2436 as u32) ) } as u64;
	// 826ACA70: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826ACA74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACA78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACA78 size=116
    let mut pc: u32 = 0x826ACA78;
    'dispatch: loop {
        match pc {
            0x826ACA78 => {
    //   block [0x826ACA78..0x826ACAEC)
	// 826ACA78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACA7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACA80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACA84: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826ACA88: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826ACA8C: 390B4450  addi r8, r11, 0x4450
	ctx.r[8].s64 = ctx.r[11].s64 + 17488;
	// 826ACA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACA94: 392AD628  addi r9, r10, -0x29d8
	ctx.r[9].s64 = ctx.r[10].s64 + -10712;
	// 826ACA98: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACA9C: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826ACAA0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826ACAA4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826ACAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACAAC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACABC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826ACAC0: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 826ACAC4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826ACAC8: 386BAFE8  addi r3, r11, -0x5018
	ctx.r[3].s64 = ctx.r[11].s64 + -20504;
	// 826ACACC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826ACAD0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACAD8: 4BDBA349  bl 0x82466e20
	ctx.lr = 0x826ACADC;
	sub_82466E20(ctx, base);
	// 826ACADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACAF0 size=100
    let mut pc: u32 = 0x826ACAF0;
    'dispatch: loop {
        match pc {
            0x826ACAF0 => {
    //   block [0x826ACAF0..0x826ACB54)
	// 826ACAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACAFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACB00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACB04: 38AAAFE8  addi r5, r10, -0x5018
	ctx.r[5].s64 = ctx.r[10].s64 + -20504;
	// 826ACB08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACB10: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 826ACB14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACB18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACB1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACB20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACB24: 386AB018  addi r3, r10, -0x4fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -20456;
	// 826ACB28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACB2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACB30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACB38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACB40: 4BDBA2E1  bl 0x82466e20
	ctx.lr = 0x826ACB44;
	sub_82466E20(ctx, base);
	// 826ACB44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826ACB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826ACB58 size=100
    let mut pc: u32 = 0x826ACB58;
    'dispatch: loop {
        match pc {
            0x826ACB58 => {
    //   block [0x826ACB58..0x826ACBBC)
	// 826ACB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826ACB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826ACB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826ACB64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826ACB6C: 38AAAFE8  addi r5, r10, -0x5018
	ctx.r[5].s64 = ctx.r[10].s64 + -20504;
	// 826ACB70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826ACB74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826ACB78: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 826ACB7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826ACB80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826ACB84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826ACB88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826ACB8C: 386AB048  addi r3, r10, -0x4fb8
	ctx.r[3].s64 = ctx.r[10].s64 + -20408;
	// 826ACB90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826ACB94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826ACB98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826ACB9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826ACBA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826ACBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826ACBA8: 4BDBA279  bl 0x82466e20
	ctx.lr = 0x826ACBAC;
	sub_82466E20(ctx, base);
	// 826ACBAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826ACBB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826ACBB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826ACBB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


