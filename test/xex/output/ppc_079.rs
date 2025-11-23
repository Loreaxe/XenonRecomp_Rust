pub fn sub_82605620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605620 size=116
    let mut pc: u32 = 0x82605620;
    'dispatch: loop {
        match pc {
            0x82605620 => {
    //   block [0x82605620..0x82605694)
	// 82605620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260562C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82605634: 390BBF4C  addi r8, r11, -0x40b4
	ctx.r[8].s64 = ctx.r[11].s64 + -16564;
	// 82605638: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260563C: 392ACFFC  addi r9, r10, -0x3004
	ctx.r[9].s64 = ctx.r[10].s64 + -12292;
	// 82605640: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605644: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82605648: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260564C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260565C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605664: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82605668: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8260566C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605670: 386B4784  addi r3, r11, 0x4784
	ctx.r[3].s64 = ctx.r[11].s64 + 18308;
	// 82605674: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605678: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260567C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605680: 4BE617A1  bl 0x82466e20
	ctx.lr = 0x82605684;
	sub_82466E20(ctx, base);
	// 82605684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260568C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605698 size=108
    let mut pc: u32 = 0x82605698;
    'dispatch: loop {
        match pc {
            0x82605698 => {
    //   block [0x82605698..0x82605704)
	// 82605698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260569C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826056A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826056A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826056A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826056AC: 38EBBF68  addi r7, r11, -0x4098
	ctx.r[7].s64 = ctx.r[11].s64 + -16536;
	// 826056B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826056B4: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826056B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826056BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826056C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826056C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826056C8: 386A47B4  addi r3, r10, 0x47b4
	ctx.r[3].s64 = ctx.r[10].s64 + 18356;
	// 826056CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826056D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826056D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826056D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826056DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826056E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826056E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826056E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826056EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826056F0: 4BE61731  bl 0x82466e20
	ctx.lr = 0x826056F4;
	sub_82466E20(ctx, base);
	// 826056F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826056F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826056FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605708 size=108
    let mut pc: u32 = 0x82605708;
    'dispatch: loop {
        match pc {
            0x82605708 => {
    //   block [0x82605708..0x82605774)
	// 82605708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260570C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605714: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260571C: 38EBBFB0  addi r7, r11, -0x4050
	ctx.r[7].s64 = ctx.r[11].s64 + -16464;
	// 82605720: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82605724: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 82605728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260572C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82605734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605738: 386A47E4  addi r3, r10, 0x47e4
	ctx.r[3].s64 = ctx.r[10].s64 + 18404;
	// 8260573C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260574C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260575C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605760: 4BE616C1  bl 0x82466e20
	ctx.lr = 0x82605764;
	sub_82466E20(ctx, base);
	// 82605764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260576C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605778 size=108
    let mut pc: u32 = 0x82605778;
    'dispatch: loop {
        match pc {
            0x82605778 => {
    //   block [0x82605778..0x826057E4)
	// 82605778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260577C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605784: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260578C: 38EBC040  addi r7, r11, -0x3fc0
	ctx.r[7].s64 = ctx.r[11].s64 + -16320;
	// 82605790: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82605794: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 82605798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260579C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826057A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826057A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826057A8: 386A4814  addi r3, r10, 0x4814
	ctx.r[3].s64 = ctx.r[10].s64 + 18452;
	// 826057AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826057B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826057B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826057B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826057BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826057C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826057C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826057C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826057CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826057D0: 4BE61651  bl 0x82466e20
	ctx.lr = 0x826057D4;
	sub_82466E20(ctx, base);
	// 826057D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826057D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826057DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826057E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826057E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826057E8 size=100
    let mut pc: u32 = 0x826057E8;
    'dispatch: loop {
        match pc {
            0x826057E8 => {
    //   block [0x826057E8..0x8260584C)
	// 826057E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826057EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826057F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826057F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826057F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826057FC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605808: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 8260580C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260581C: 386A4844  addi r3, r10, 0x4844
	ctx.r[3].s64 = ctx.r[10].s64 + 18500;
	// 82605820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605824: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605828: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260582C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605830: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605838: 4BE615E9  bl 0x82466e20
	ctx.lr = 0x8260583C;
	sub_82466E20(ctx, base);
	// 8260583C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605850 size=112
    let mut pc: u32 = 0x82605850;
    'dispatch: loop {
        match pc {
            0x82605850 => {
    //   block [0x82605850..0x826058C0)
	// 82605850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260585C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605860: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605864: 38AA4844  addi r5, r10, 0x4844
	ctx.r[5].s64 = ctx.r[10].s64 + 18500;
	// 82605868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260586C: 390BC0D0  addi r8, r11, -0x3f30
	ctx.r[8].s64 = ctx.r[11].s64 + -16176;
	// 82605870: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82605874: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 82605878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260587C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605888: 386A4874  addi r3, r10, 0x4874
	ctx.r[3].s64 = ctx.r[10].s64 + 18548;
	// 8260588C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260589C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826058A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826058A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826058A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826058AC: 4BE61575  bl 0x82466e20
	ctx.lr = 0x826058B0;
	sub_82466E20(ctx, base);
	// 826058B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826058B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826058B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826058BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826058C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826058C0 size=108
    let mut pc: u32 = 0x826058C0;
    'dispatch: loop {
        match pc {
            0x826058C0 => {
    //   block [0x826058C0..0x8260592C)
	// 826058C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826058C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826058C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826058CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826058D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826058D4: 38EBC130  addi r7, r11, -0x3ed0
	ctx.r[7].s64 = ctx.r[11].s64 + -16080;
	// 826058D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826058DC: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826058E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826058E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826058E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826058EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826058F0: 386A48A4  addi r3, r10, 0x48a4
	ctx.r[3].s64 = ctx.r[10].s64 + 18596;
	// 826058F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826058F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826058FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260590C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605918: 4BE61509  bl 0x82466e20
	ctx.lr = 0x8260591C;
	sub_82466E20(ctx, base);
	// 8260591C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605930 size=108
    let mut pc: u32 = 0x82605930;
    'dispatch: loop {
        match pc {
            0x82605930 => {
    //   block [0x82605930..0x8260599C)
	// 82605930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260593C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605944: 38EBC160  addi r7, r11, -0x3ea0
	ctx.r[7].s64 = ctx.r[11].s64 + -16032;
	// 82605948: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260594C: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 82605950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605954: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260595C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605960: 386A48D4  addi r3, r10, 0x48d4
	ctx.r[3].s64 = ctx.r[10].s64 + 18644;
	// 82605964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82605968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260596C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260597C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605988: 4BE61499  bl 0x82466e20
	ctx.lr = 0x8260598C;
	sub_82466E20(ctx, base);
	// 8260598C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826059A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826059A0 size=108
    let mut pc: u32 = 0x826059A0;
    'dispatch: loop {
        match pc {
            0x826059A0 => {
    //   block [0x826059A0..0x82605A0C)
	// 826059A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826059A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826059A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826059AC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826059B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826059B4: 38EBC1C0  addi r7, r11, -0x3e40
	ctx.r[7].s64 = ctx.r[11].s64 + -15936;
	// 826059B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826059BC: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826059C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826059C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826059C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826059CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826059D0: 386A4904  addi r3, r10, 0x4904
	ctx.r[3].s64 = ctx.r[10].s64 + 18692;
	// 826059D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826059D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826059DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826059E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826059E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826059E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826059EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826059F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826059F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826059F8: 4BE61429  bl 0x82466e20
	ctx.lr = 0x826059FC;
	sub_82466E20(ctx, base);
	// 826059FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605A00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605A04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82605A10 size=24
    let mut pc: u32 = 0x82605A10;
    'dispatch: loop {
        match pc {
            0x82605A10 => {
    //   block [0x82605A10..0x82605A28)
	// 82605A10: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605A14: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605A18: 394A2D38  addi r10, r10, 0x2d38
	ctx.r[10].s64 = ctx.r[10].s64 + 11576;
	// 82605A1C: 816BBF64  lwz r11, -0x409c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16540 as u32) ) } as u64;
	// 82605A20: 916A0128  stw r11, 0x128(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(296 as u32), ctx.r[11].u32 ) };
	// 82605A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605A28 size=116
    let mut pc: u32 = 0x82605A28;
    'dispatch: loop {
        match pc {
            0x82605A28 => {
    //   block [0x82605A28..0x82605A9C)
	// 82605A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605A30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605A34: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605A38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82605A3C: 390B2D38  addi r8, r11, 0x2d38
	ctx.r[8].s64 = ctx.r[11].s64 + 11576;
	// 82605A40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605A44: 392AD030  addi r9, r10, -0x2fd0
	ctx.r[9].s64 = ctx.r[10].s64 + -12240;
	// 82605A48: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605A4C: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 82605A50: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605A54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605A5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605A64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605A6C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82605A70: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 82605A74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605A78: 386B4934  addi r3, r11, 0x4934
	ctx.r[3].s64 = ctx.r[11].s64 + 18740;
	// 82605A7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605A80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605A88: 4BE61399  bl 0x82466e20
	ctx.lr = 0x82605A8C;
	sub_82466E20(ctx, base);
	// 82605A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605AA0 size=112
    let mut pc: u32 = 0x82605AA0;
    'dispatch: loop {
        match pc {
            0x82605AA0 => {
    //   block [0x82605AA0..0x82605B10)
	// 82605AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605AA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605AAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605AB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605AB4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605AB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605ABC: 390BC220  addi r8, r11, -0x3de0
	ctx.r[8].s64 = ctx.r[11].s64 + -15840;
	// 82605AC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82605AC4: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 82605AC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605ACC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605AD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605AD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605AD8: 386A4964  addi r3, r10, 0x4964
	ctx.r[3].s64 = ctx.r[10].s64 + 18788;
	// 82605ADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605AE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605AE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605AEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605AF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605AF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605AF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605AFC: 4BE61325  bl 0x82466e20
	ctx.lr = 0x82605B00;
	sub_82466E20(ctx, base);
	// 82605B00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605B04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605B08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605B10 size=112
    let mut pc: u32 = 0x82605B10;
    'dispatch: loop {
        match pc {
            0x82605B10 => {
    //   block [0x82605B10..0x82605B80)
	// 82605B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605B1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605B20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605B24: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605B28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605B2C: 390BC268  addi r8, r11, -0x3d98
	ctx.r[8].s64 = ctx.r[11].s64 + -15768;
	// 82605B30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82605B34: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 82605B38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605B3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605B40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605B44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605B48: 386A4994  addi r3, r10, 0x4994
	ctx.r[3].s64 = ctx.r[10].s64 + 18836;
	// 82605B4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605B50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605B6C: 4BE612B5  bl 0x82466e20
	ctx.lr = 0x82605B70;
	sub_82466E20(ctx, base);
	// 82605B70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605B80 size=112
    let mut pc: u32 = 0x82605B80;
    'dispatch: loop {
        match pc {
            0x82605B80 => {
    //   block [0x82605B80..0x82605BF0)
	// 82605B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605B88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605B8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605B90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605B94: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82605B98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605B9C: 390BC2B0  addi r8, r11, -0x3d50
	ctx.r[8].s64 = ctx.r[11].s64 + -15696;
	// 82605BA0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82605BA4: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 82605BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605BAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605BB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605BB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605BB8: 386A49C4  addi r3, r10, 0x49c4
	ctx.r[3].s64 = ctx.r[10].s64 + 18884;
	// 82605BBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605BCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605BDC: 4BE61245  bl 0x82466e20
	ctx.lr = 0x82605BE0;
	sub_82466E20(ctx, base);
	// 82605BE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605BF0 size=112
    let mut pc: u32 = 0x82605BF0;
    'dispatch: loop {
        match pc {
            0x82605BF0 => {
    //   block [0x82605BF0..0x82605C60)
	// 82605BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605BF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605BFC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82605C00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605C04: 392BD064  addi r9, r11, -0x2f9c
	ctx.r[9].s64 = ctx.r[11].s64 + -12188;
	// 82605C08: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 82605C0C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82605C10: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605C14: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 82605C18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605C1C: 396BC390  addi r11, r11, -0x3c70
	ctx.r[11].s64 = ctx.r[11].s64 + -15472;
	// 82605C20: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82605C24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605C28: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82605C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605C30: 386A49F4  addi r3, r10, 0x49f4
	ctx.r[3].s64 = ctx.r[10].s64 + 18932;
	// 82605C34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605C38: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82605C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605C40: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82605C44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605C48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605C4C: 4BE611D5  bl 0x82466e20
	ctx.lr = 0x82605C50;
	sub_82466E20(ctx, base);
	// 82605C50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605C54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605C58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605C5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605C60 size=112
    let mut pc: u32 = 0x82605C60;
    'dispatch: loop {
        match pc {
            0x82605C60 => {
    //   block [0x82605C60..0x82605CD0)
	// 82605C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605C68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605C6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605C70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605C74: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605C78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605C7C: 390BC4E0  addi r8, r11, -0x3b20
	ctx.r[8].s64 = ctx.r[11].s64 + -15136;
	// 82605C80: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82605C84: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 82605C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605C8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605C90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605C94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605C98: 386A4A24  addi r3, r10, 0x4a24
	ctx.r[3].s64 = ctx.r[10].s64 + 18980;
	// 82605C9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605CA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605CBC: 4BE61165  bl 0x82466e20
	ctx.lr = 0x82605CC0;
	sub_82466E20(ctx, base);
	// 82605CC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605CC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605CC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605CCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605CD0 size=112
    let mut pc: u32 = 0x82605CD0;
    'dispatch: loop {
        match pc {
            0x82605CD0 => {
    //   block [0x82605CD0..0x82605D40)
	// 82605CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605CDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605CE0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605CE4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605CE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605CEC: 390BC588  addi r8, r11, -0x3a78
	ctx.r[8].s64 = ctx.r[11].s64 + -14968;
	// 82605CF0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82605CF4: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 82605CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605CFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605D00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605D08: 386A4A54  addi r3, r10, 0x4a54
	ctx.r[3].s64 = ctx.r[10].s64 + 19028;
	// 82605D0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605D10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605D14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605D18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605D1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605D20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605D28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605D2C: 4BE610F5  bl 0x82466e20
	ctx.lr = 0x82605D30;
	sub_82466E20(ctx, base);
	// 82605D30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605D34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605D38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605D40 size=112
    let mut pc: u32 = 0x82605D40;
    'dispatch: loop {
        match pc {
            0x82605D40 => {
    //   block [0x82605D40..0x82605DB0)
	// 82605D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605D4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82605D50: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605D54: 392AD0C0  addi r9, r10, -0x2f40
	ctx.r[9].s64 = ctx.r[10].s64 + -12096;
	// 82605D58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605D5C: 390BC618  addi r8, r11, -0x39e8
	ctx.r[8].s64 = ctx.r[11].s64 + -14824;
	// 82605D60: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82605D64: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 82605D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605D6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605D78: 386A4A84  addi r3, r10, 0x4a84
	ctx.r[3].s64 = ctx.r[10].s64 + 19076;
	// 82605D7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605D80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605D94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82605D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605D9C: 4BE61085  bl 0x82466e20
	ctx.lr = 0x82605DA0;
	sub_82466E20(ctx, base);
	// 82605DA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605DB0 size=100
    let mut pc: u32 = 0x82605DB0;
    'dispatch: loop {
        match pc {
            0x82605DB0 => {
    //   block [0x82605DB0..0x82605E14)
	// 82605DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605DBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605DC4: 38AA51D4  addi r5, r10, 0x51d4
	ctx.r[5].s64 = ctx.r[10].s64 + 20948;
	// 82605DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605DD0: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 82605DD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605DE4: 386A4AB4  addi r3, r10, 0x4ab4
	ctx.r[3].s64 = ctx.r[10].s64 + 19124;
	// 82605DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605DEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605DF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82605DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605DF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605E00: 4BE61021  bl 0x82466e20
	ctx.lr = 0x82605E04;
	sub_82466E20(ctx, base);
	// 82605E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605E18 size=112
    let mut pc: u32 = 0x82605E18;
    'dispatch: loop {
        match pc {
            0x82605E18 => {
    //   block [0x82605E18..0x82605E88)
	// 82605E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605E24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605E28: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605E2C: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82605E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605E34: 390BC648  addi r8, r11, -0x39b8
	ctx.r[8].s64 = ctx.r[11].s64 + -14776;
	// 82605E38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82605E3C: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 82605E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605E50: 386A4AE4  addi r3, r10, 0x4ae4
	ctx.r[3].s64 = ctx.r[10].s64 + 19172;
	// 82605E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82605E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605E74: 4BE60FAD  bl 0x82466e20
	ctx.lr = 0x82605E78;
	sub_82466E20(ctx, base);
	// 82605E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605E88 size=116
    let mut pc: u32 = 0x82605E88;
    'dispatch: loop {
        match pc {
            0x82605E88 => {
    //   block [0x82605E88..0x82605EFC)
	// 82605E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605E94: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605E98: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82605E9C: 390AC678  addi r8, r10, -0x3988
	ctx.r[8].s64 = ctx.r[10].s64 + -14728;
	// 82605EA0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605EA4: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82605EA8: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82605EAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605EB0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82605EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82605EBC: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 82605EC0: 396BD0D4  addi r11, r11, -0x2f2c
	ctx.r[11].s64 = ctx.r[11].s64 + -12076;
	// 82605EC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605EC8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605ECC: 386A4B14  addi r3, r10, 0x4b14
	ctx.r[3].s64 = ctx.r[10].s64 + 19220;
	// 82605ED0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82605ED4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605ED8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82605EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605EE8: 4BE60F39  bl 0x82466e20
	ctx.lr = 0x82605EEC;
	sub_82466E20(ctx, base);
	// 82605EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605F00 size=100
    let mut pc: u32 = 0x82605F00;
    'dispatch: loop {
        match pc {
            0x82605F00 => {
    //   block [0x82605F00..0x82605F64)
	// 82605F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605F0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82605F14: 38AA4B14  addi r5, r10, 0x4b14
	ctx.r[5].s64 = ctx.r[10].s64 + 19220;
	// 82605F18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82605F20: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 82605F24: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82605F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82605F34: 386A4B44  addi r3, r10, 0x4b44
	ctx.r[3].s64 = ctx.r[10].s64 + 19268;
	// 82605F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82605F3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82605F40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82605F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605F48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605F50: 4BE60ED1  bl 0x82466e20
	ctx.lr = 0x82605F54;
	sub_82466E20(ctx, base);
	// 82605F54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605F58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605F5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82605F68 size=24
    let mut pc: u32 = 0x82605F68;
    'dispatch: loop {
        match pc {
            0x82605F68 => {
    //   block [0x82605F68..0x82605F80)
	// 82605F68: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605F6C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82605F70: 394A2E70  addi r10, r10, 0x2e70
	ctx.r[10].s64 = ctx.r[10].s64 + 11888;
	// 82605F74: 816BC720  lwz r11, -0x38e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14560 as u32) ) } as u64;
	// 82605F78: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82605F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605F80 size=116
    let mut pc: u32 = 0x82605F80;
    'dispatch: loop {
        match pc {
            0x82605F80 => {
    //   block [0x82605F80..0x82605FF4)
	// 82605F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82605F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82605F8C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82605F90: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605F94: 392BD110  addi r9, r11, -0x2ef0
	ctx.r[9].s64 = ctx.r[11].s64 + -12016;
	// 82605F98: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82605F9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82605FA0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82605FA4: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82605FA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82605FAC: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 82605FB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82605FB4: 396B2E70  addi r11, r11, 0x2e70
	ctx.r[11].s64 = ctx.r[11].s64 + 11888;
	// 82605FB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82605FBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82605FC0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82605FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82605FC8: 386A4B74  addi r3, r10, 0x4b74
	ctx.r[3].s64 = ctx.r[10].s64 + 19316;
	// 82605FCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82605FD0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82605FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82605FD8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82605FDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82605FE0: 4BE60E41  bl 0x82466e20
	ctx.lr = 0x82605FE4;
	sub_82466E20(ctx, base);
	// 82605FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82605FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82605FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82605FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82605FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82605FF8 size=116
    let mut pc: u32 = 0x82605FF8;
    'dispatch: loop {
        match pc {
            0x82605FF8 => {
    //   block [0x82605FF8..0x8260606C)
	// 82605FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82605FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606004: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606008: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260600C: 392BD154  addi r9, r11, -0x2eac
	ctx.r[9].s64 = ctx.r[11].s64 + -11948;
	// 82606010: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82606014: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606018: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260601C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 82606020: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606024: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 82606028: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260602C: 396BC728  addi r11, r11, -0x38d8
	ctx.r[11].s64 = ctx.r[11].s64 + -14552;
	// 82606030: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82606034: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606038: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260603C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606040: 386A4BA4  addi r3, r10, 0x4ba4
	ctx.r[3].s64 = ctx.r[10].s64 + 19364;
	// 82606044: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82606048: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260604C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606050: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82606054: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82606058: 4BE60DC9  bl 0x82466e20
	ctx.lr = 0x8260605C;
	sub_82466E20(ctx, base);
	// 8260605C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606070 size=108
    let mut pc: u32 = 0x82606070;
    'dispatch: loop {
        match pc {
            0x82606070 => {
    //   block [0x82606070..0x826060DC)
	// 82606070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260607C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606084: 38EBC7D0  addi r7, r11, -0x3830
	ctx.r[7].s64 = ctx.r[11].s64 + -14384;
	// 82606088: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260608C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 82606090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606094: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260609C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826060A0: 386A4BD4  addi r3, r10, 0x4bd4
	ctx.r[3].s64 = ctx.r[10].s64 + 19412;
	// 826060A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826060A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826060AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826060B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826060B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826060B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826060BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826060C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826060C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826060C8: 4BE60D59  bl 0x82466e20
	ctx.lr = 0x826060CC;
	sub_82466E20(ctx, base);
	// 826060CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826060D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826060D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826060D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826060E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826060E0 size=24
    let mut pc: u32 = 0x826060E0;
    'dispatch: loop {
        match pc {
            0x826060E0 => {
    //   block [0x826060E0..0x826060F8)
	// 826060E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826060E4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826060E8: 394A2EE8  addi r10, r10, 0x2ee8
	ctx.r[10].s64 = ctx.r[10].s64 + 12008;
	// 826060EC: 816BC818  lwz r11, -0x37e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14312 as u32) ) } as u64;
	// 826060F0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826060F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826060F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826060F8 size=116
    let mut pc: u32 = 0x826060F8;
    'dispatch: loop {
        match pc {
            0x826060F8 => {
    //   block [0x826060F8..0x8260616C)
	// 826060F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826060FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606104: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606108: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260610C: 390B2EE8  addi r8, r11, 0x2ee8
	ctx.r[8].s64 = ctx.r[11].s64 + 12008;
	// 82606110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606114: 392AD1C0  addi r9, r10, -0x2e40
	ctx.r[9].s64 = ctx.r[10].s64 + -11840;
	// 82606118: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260611C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82606120: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82606124: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260612C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260613C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82606140: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 82606144: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606148: 386B4C04  addi r3, r11, 0x4c04
	ctx.r[3].s64 = ctx.r[11].s64 + 19460;
	// 8260614C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82606150: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606158: 4BE60CC9  bl 0x82466e20
	ctx.lr = 0x8260615C;
	sub_82466E20(ctx, base);
	// 8260615C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606170 size=112
    let mut pc: u32 = 0x82606170;
    'dispatch: loop {
        match pc {
            0x82606170 => {
    //   block [0x82606170..0x826061E0)
	// 82606170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260617C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606180: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606184: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82606188: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260618C: 390BC820  addi r8, r11, -0x37e0
	ctx.r[8].s64 = ctx.r[11].s64 + -14304;
	// 82606190: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82606194: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 82606198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260619C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826061A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826061A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826061A8: 386A4C34  addi r3, r10, 0x4c34
	ctx.r[3].s64 = ctx.r[10].s64 + 19508;
	// 826061AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826061B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826061B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826061B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826061BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826061C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826061C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826061C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826061CC: 4BE60C55  bl 0x82466e20
	ctx.lr = 0x826061D0;
	sub_82466E20(ctx, base);
	// 826061D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826061D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826061D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826061DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826061E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826061E0 size=24
    let mut pc: u32 = 0x826061E0;
    'dispatch: loop {
        match pc {
            0x826061E0 => {
    //   block [0x826061E0..0x826061F8)
	// 826061E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826061E4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826061E8: 394A3068  addi r10, r10, 0x3068
	ctx.r[10].s64 = ctx.r[10].s64 + 12392;
	// 826061EC: 816BC850  lwz r11, -0x37b0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14256 as u32) ) } as u64;
	// 826061F0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826061F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826061F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826061F8 size=116
    let mut pc: u32 = 0x826061F8;
    'dispatch: loop {
        match pc {
            0x826061F8 => {
    //   block [0x826061F8..0x8260626C)
	// 826061F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826061FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606204: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606208: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260620C: 390B3068  addi r8, r11, 0x3068
	ctx.r[8].s64 = ctx.r[11].s64 + 12392;
	// 82606210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606214: 392AD1F8  addi r9, r10, -0x2e08
	ctx.r[9].s64 = ctx.r[10].s64 + -11784;
	// 82606218: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260621C: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 82606220: 38AA4BA4  addi r5, r10, 0x4ba4
	ctx.r[5].s64 = ctx.r[10].s64 + 19364;
	// 82606224: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260622C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260623C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82606240: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 82606244: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606248: 386B4C64  addi r3, r11, 0x4c64
	ctx.r[3].s64 = ctx.r[11].s64 + 19556;
	// 8260624C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82606250: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606258: 4BE60BC9  bl 0x82466e20
	ctx.lr = 0x8260625C;
	sub_82466E20(ctx, base);
	// 8260625C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606270 size=112
    let mut pc: u32 = 0x82606270;
    'dispatch: loop {
        match pc {
            0x82606270 => {
    //   block [0x82606270..0x826062E0)
	// 82606270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260627C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606280: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606284: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 82606288: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260628C: 390BC854  addi r8, r11, -0x37ac
	ctx.r[8].s64 = ctx.r[11].s64 + -14252;
	// 82606290: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606294: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 82606298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260629C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826062A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826062A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826062A8: 386A4C94  addi r3, r10, 0x4c94
	ctx.r[3].s64 = ctx.r[10].s64 + 19604;
	// 826062AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826062B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826062B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826062B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826062BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826062C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826062C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826062C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826062CC: 4BE60B55  bl 0x82466e20
	ctx.lr = 0x826062D0;
	sub_82466E20(ctx, base);
	// 826062D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826062D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826062D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826062DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826062E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826062E0 size=100
    let mut pc: u32 = 0x826062E0;
    'dispatch: loop {
        match pc {
            0x826062E0 => {
    //   block [0x826062E0..0x82606344)
	// 826062E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826062E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826062E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826062EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826062F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826062F4: 38AA51D4  addi r5, r10, 0x51d4
	ctx.r[5].s64 = ctx.r[10].s64 + 20948;
	// 826062F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826062FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606300: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 82606304: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606308: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260630C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606310: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606314: 386A4CC4  addi r3, r10, 0x4cc4
	ctx.r[3].s64 = ctx.r[10].s64 + 19652;
	// 82606318: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260631C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606320: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82606324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606328: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260632C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606330: 4BE60AF1  bl 0x82466e20
	ctx.lr = 0x82606334;
	sub_82466E20(ctx, base);
	// 82606334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260633C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606348 size=108
    let mut pc: u32 = 0x82606348;
    'dispatch: loop {
        match pc {
            0x82606348 => {
    //   block [0x82606348..0x826063B4)
	// 82606348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260634C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606354: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260635C: 38EBC870  addi r7, r11, -0x3790
	ctx.r[7].s64 = ctx.r[11].s64 + -14224;
	// 82606360: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82606364: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 82606368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260636C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606378: 386A4CF4  addi r3, r10, 0x4cf4
	ctx.r[3].s64 = ctx.r[10].s64 + 19700;
	// 8260637C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260638C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260639C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826063A0: 4BE60A81  bl 0x82466e20
	ctx.lr = 0x826063A4;
	sub_82466E20(ctx, base);
	// 826063A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826063A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826063AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826063B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826063B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826063B8 size=112
    let mut pc: u32 = 0x826063B8;
    'dispatch: loop {
        match pc {
            0x826063B8 => {
    //   block [0x826063B8..0x82606428)
	// 826063B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826063BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826063C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826063C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826063C8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826063CC: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 826063D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826063D4: 390BC948  addi r8, r11, -0x36b8
	ctx.r[8].s64 = ctx.r[11].s64 + -14008;
	// 826063D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826063DC: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826063E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826063E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826063E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826063EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826063F0: 386A4D24  addi r3, r10, 0x4d24
	ctx.r[3].s64 = ctx.r[10].s64 + 19748;
	// 826063F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826063F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826063FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260640C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606414: 4BE60A0D  bl 0x82466e20
	ctx.lr = 0x82606418;
	sub_82466E20(ctx, base);
	// 82606418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260641C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606428 size=108
    let mut pc: u32 = 0x82606428;
    'dispatch: loop {
        match pc {
            0x82606428 => {
    //   block [0x82606428..0x82606494)
	// 82606428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260642C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606434: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260643C: 38EBC978  addi r7, r11, -0x3688
	ctx.r[7].s64 = ctx.r[11].s64 + -13960;
	// 82606440: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82606444: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 82606448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260644C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606450: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606458: 386A4D54  addi r3, r10, 0x4d54
	ctx.r[3].s64 = ctx.r[10].s64 + 19796;
	// 8260645C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260646C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260647C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606480: 4BE609A1  bl 0x82466e20
	ctx.lr = 0x82606484;
	sub_82466E20(ctx, base);
	// 82606484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260648C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606498 size=112
    let mut pc: u32 = 0x82606498;
    'dispatch: loop {
        match pc {
            0x82606498 => {
    //   block [0x82606498..0x82606508)
	// 82606498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260649C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826064A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826064A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826064A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826064AC: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 826064B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826064B4: 390BC9A8  addi r8, r11, -0x3658
	ctx.r[8].s64 = ctx.r[11].s64 + -13912;
	// 826064B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826064BC: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826064C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826064C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826064C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826064CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826064D0: 386A4D84  addi r3, r10, 0x4d84
	ctx.r[3].s64 = ctx.r[10].s64 + 19844;
	// 826064D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826064D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826064DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826064E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826064E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826064E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826064EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826064F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826064F4: 4BE6092D  bl 0x82466e20
	ctx.lr = 0x826064F8;
	sub_82466E20(ctx, base);
	// 826064F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826064FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606508 size=112
    let mut pc: u32 = 0x82606508;
    'dispatch: loop {
        match pc {
            0x82606508 => {
    //   block [0x82606508..0x82606578)
	// 82606508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606514: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606518: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8260651C: 38EAC9C0  addi r7, r10, -0x3640
	ctx.r[7].s64 = ctx.r[10].s64 + -13888;
	// 82606520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606524: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606528: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 8260652C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606530: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606534: 396BD20C  addi r11, r11, -0x2df4
	ctx.r[11].s64 = ctx.r[11].s64 + -11764;
	// 82606538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260653C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606540: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606544: 386A4DB4  addi r3, r10, 0x4db4
	ctx.r[3].s64 = ctx.r[10].s64 + 19892;
	// 82606548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260654C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82606550: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606554: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82606558: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260655C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606560: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606564: 4BE608BD  bl 0x82466e20
	ctx.lr = 0x82606568;
	sub_82466E20(ctx, base);
	// 82606568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260656C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606578 size=108
    let mut pc: u32 = 0x82606578;
    'dispatch: loop {
        match pc {
            0x82606578 => {
    //   block [0x82606578..0x826065E4)
	// 82606578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260657C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606584: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260658C: 38EBCA98  addi r7, r11, -0x3568
	ctx.r[7].s64 = ctx.r[11].s64 + -13672;
	// 82606590: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82606594: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 82606598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260659C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826065A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826065A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826065A8: 386A4DE4  addi r3, r10, 0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + 19940;
	// 826065AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826065B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826065B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826065B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826065BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826065C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826065C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826065C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826065CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826065D0: 4BE60851  bl 0x82466e20
	ctx.lr = 0x826065D4;
	sub_82466E20(ctx, base);
	// 826065D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826065D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826065DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826065E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826065E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826065E8 size=108
    let mut pc: u32 = 0x826065E8;
    'dispatch: loop {
        match pc {
            0x826065E8 => {
    //   block [0x826065E8..0x82606654)
	// 826065E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826065EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826065F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826065F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826065F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826065FC: 38EBCAB0  addi r7, r11, -0x3550
	ctx.r[7].s64 = ctx.r[11].s64 + -13648;
	// 82606600: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82606604: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 82606608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260660C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606618: 386A4E14  addi r3, r10, 0x4e14
	ctx.r[3].s64 = ctx.r[10].s64 + 19988;
	// 8260661C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260662C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260663C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606640: 4BE607E1  bl 0x82466e20
	ctx.lr = 0x82606644;
	sub_82466E20(ctx, base);
	// 82606644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260664C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606658 size=112
    let mut pc: u32 = 0x82606658;
    'dispatch: loop {
        match pc {
            0x82606658 => {
    //   block [0x82606658..0x826066C8)
	// 82606658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260665C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606664: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606668: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260666C: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606674: 390BCBB8  addi r8, r11, -0x3448
	ctx.r[8].s64 = ctx.r[11].s64 + -13384;
	// 82606678: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 8260667C: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 82606680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606684: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260668C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606690: 386A4E44  addi r3, r10, 0x4e44
	ctx.r[3].s64 = ctx.r[10].s64 + 20036;
	// 82606694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260669C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826066A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826066A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826066A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826066AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826066B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826066B4: 4BE6076D  bl 0x82466e20
	ctx.lr = 0x826066B8;
	sub_82466E20(ctx, base);
	// 826066B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826066BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826066C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826066C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826066C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826066C8 size=112
    let mut pc: u32 = 0x826066C8;
    'dispatch: loop {
        match pc {
            0x826066C8 => {
    //   block [0x826066C8..0x82606738)
	// 826066C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826066CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826066D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826066D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826066D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826066DC: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 826066E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826066E4: 390BCCC0  addi r8, r11, -0x3340
	ctx.r[8].s64 = ctx.r[11].s64 + -13120;
	// 826066E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826066EC: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826066F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826066F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826066F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826066FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606700: 386A4E74  addi r3, r10, 0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + 20084;
	// 82606704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260670C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260671C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606724: 4BE606FD  bl 0x82466e20
	ctx.lr = 0x82606728;
	sub_82466E20(ctx, base);
	// 82606728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260672C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606738 size=116
    let mut pc: u32 = 0x82606738;
    'dispatch: loop {
        match pc {
            0x82606738 => {
    //   block [0x82606738..0x826067AC)
	// 82606738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260673C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606744: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606748: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8260674C: 390ACCD8  addi r8, r10, -0x3328
	ctx.r[8].s64 = ctx.r[10].s64 + -13096;
	// 82606750: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606754: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606758: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 8260675C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606760: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260676C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 82606770: 396BD23C  addi r11, r11, -0x2dc4
	ctx.r[11].s64 = ctx.r[11].s64 + -11716;
	// 82606774: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606778: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260677C: 386A4EA4  addi r3, r10, 0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + 20132;
	// 82606780: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82606784: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606788: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260678C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606798: 4BE60689  bl 0x82466e20
	ctx.lr = 0x8260679C;
	sub_82466E20(ctx, base);
	// 8260679C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826067A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826067A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826067A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826067B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826067B0 size=108
    let mut pc: u32 = 0x826067B0;
    'dispatch: loop {
        match pc {
            0x826067B0 => {
    //   block [0x826067B0..0x8260681C)
	// 826067B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826067B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826067B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826067BC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826067C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826067C4: 38EBCD38  addi r7, r11, -0x32c8
	ctx.r[7].s64 = ctx.r[11].s64 + -13000;
	// 826067C8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826067CC: 388A3224  addi r4, r10, 0x3224
	ctx.r[4].s64 = ctx.r[10].s64 + 12836;
	// 826067D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826067D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826067D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826067DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826067E0: 386A4ED4  addi r3, r10, 0x4ed4
	ctx.r[3].s64 = ctx.r[10].s64 + 20180;
	// 826067E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826067E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826067EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826067F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826067F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826067F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826067FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606804: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606808: 4BE60619  bl 0x82466e20
	ctx.lr = 0x8260680C;
	sub_82466E20(ctx, base);
	// 8260680C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606820 size=108
    let mut pc: u32 = 0x82606820;
    'dispatch: loop {
        match pc {
            0x82606820 => {
    //   block [0x82606820..0x8260688C)
	// 82606820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260682C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606834: 38EBCD80  addi r7, r11, -0x3280
	ctx.r[7].s64 = ctx.r[11].s64 + -12928;
	// 82606838: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260683C: 388A3244  addi r4, r10, 0x3244
	ctx.r[4].s64 = ctx.r[10].s64 + 12868;
	// 82606840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606844: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260684C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606850: 386A4F04  addi r3, r10, 0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + 20228;
	// 82606854: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260685C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260686C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606878: 4BE605A9  bl 0x82466e20
	ctx.lr = 0x8260687C;
	sub_82466E20(ctx, base);
	// 8260687C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606890 size=112
    let mut pc: u32 = 0x82606890;
    'dispatch: loop {
        match pc {
            0x82606890 => {
    //   block [0x82606890..0x82606900)
	// 82606890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260689C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826068A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826068A4: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 826068A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826068AC: 390BCDC8  addi r8, r11, -0x3238
	ctx.r[8].s64 = ctx.r[11].s64 + -12856;
	// 826068B0: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826068B4: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826068B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826068BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826068C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826068C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826068C8: 386A4F34  addi r3, r10, 0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + 20276;
	// 826068CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826068D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826068D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826068D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826068DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826068E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826068E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826068E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826068EC: 4BE60535  bl 0x82466e20
	ctx.lr = 0x826068F0;
	sub_82466E20(ctx, base);
	// 826068F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826068F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826068F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826068FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606900 size=112
    let mut pc: u32 = 0x82606900;
    'dispatch: loop {
        match pc {
            0x82606900 => {
    //   block [0x82606900..0x82606970)
	// 82606900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260690C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606910: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606914: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260691C: 390BCED0  addi r8, r11, -0x3130
	ctx.r[8].s64 = ctx.r[11].s64 + -12592;
	// 82606920: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82606924: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 82606928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260692C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606938: 386A4F64  addi r3, r10, 0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + 20324;
	// 8260693C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260694C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260695C: 4BE604C5  bl 0x82466e20
	ctx.lr = 0x82606960;
	sub_82466E20(ctx, base);
	// 82606960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260696C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606970 size=112
    let mut pc: u32 = 0x82606970;
    'dispatch: loop {
        match pc {
            0x82606970 => {
    //   block [0x82606970..0x826069E0)
	// 82606970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260697C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606980: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606984: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260698C: 390BCFD8  addi r8, r11, -0x3028
	ctx.r[8].s64 = ctx.r[11].s64 + -12328;
	// 82606990: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606994: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 82606998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260699C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826069A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826069A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826069A8: 386A4F94  addi r3, r10, 0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + 20372;
	// 826069AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826069B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826069B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826069B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826069BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826069C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826069C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826069C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826069CC: 4BE60455  bl 0x82466e20
	ctx.lr = 0x826069D0;
	sub_82466E20(ctx, base);
	// 826069D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826069D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826069D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826069DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826069E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826069E0 size=112
    let mut pc: u32 = 0x826069E0;
    'dispatch: loop {
        match pc {
            0x826069E0 => {
    //   block [0x826069E0..0x82606A50)
	// 826069E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826069E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826069E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826069EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826069F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826069F4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 826069F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826069FC: 390BCFF0  addi r8, r11, -0x3010
	ctx.r[8].s64 = ctx.r[11].s64 + -12304;
	// 82606A00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82606A04: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 82606A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606A10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606A18: 386A4FC4  addi r3, r10, 0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + 20420;
	// 82606A1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606A24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606A2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606A3C: 4BE603E5  bl 0x82466e20
	ctx.lr = 0x82606A40;
	sub_82466E20(ctx, base);
	// 82606A40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606A44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606A48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606A4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606A50 size=108
    let mut pc: u32 = 0x82606A50;
    'dispatch: loop {
        match pc {
            0x82606A50 => {
    //   block [0x82606A50..0x82606ABC)
	// 82606A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606A5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606A64: 38EBD020  addi r7, r11, -0x2fe0
	ctx.r[7].s64 = ctx.r[11].s64 + -12256;
	// 82606A68: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82606A6C: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 82606A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606A74: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606A80: 386A4FF4  addi r3, r10, 0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + 20468;
	// 82606A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606AA8: 4BE60379  bl 0x82466e20
	ctx.lr = 0x82606AAC;
	sub_82466E20(ctx, base);
	// 82606AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82606AC0 size=24
    let mut pc: u32 = 0x82606AC0;
    'dispatch: loop {
        match pc {
            0x82606AC0 => {
    //   block [0x82606AC0..0x82606AD8)
	// 82606AC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606AC4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606AC8: 394A3158  addi r10, r10, 0x3158
	ctx.r[10].s64 = ctx.r[10].s64 + 12632;
	// 82606ACC: 816BC86C  lwz r11, -0x3794(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14228 as u32) ) } as u64;
	// 82606AD0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82606AD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606AD8 size=116
    let mut pc: u32 = 0x82606AD8;
    'dispatch: loop {
        match pc {
            0x82606AD8 => {
    //   block [0x82606AD8..0x82606B4C)
	// 82606AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606AE4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606AE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82606AEC: 390B3158  addi r8, r11, 0x3158
	ctx.r[8].s64 = ctx.r[11].s64 + 12632;
	// 82606AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606AF4: 392AD268  addi r9, r10, -0x2d98
	ctx.r[9].s64 = ctx.r[10].s64 + -11672;
	// 82606AF8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606AFC: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82606B00: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606B04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606B0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606B1C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82606B20: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 82606B24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606B28: 386B5024  addi r3, r11, 0x5024
	ctx.r[3].s64 = ctx.r[11].s64 + 20516;
	// 82606B2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82606B30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606B38: 4BE602E9  bl 0x82466e20
	ctx.lr = 0x82606B3C;
	sub_82466E20(ctx, base);
	// 82606B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606B50 size=112
    let mut pc: u32 = 0x82606B50;
    'dispatch: loop {
        match pc {
            0x82606B50 => {
    //   block [0x82606B50..0x82606BC0)
	// 82606B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606B5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606B60: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606B64: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606B6C: 390BD098  addi r8, r11, -0x2f68
	ctx.r[8].s64 = ctx.r[11].s64 + -12136;
	// 82606B70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82606B74: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 82606B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606B7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606B80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606B84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606B88: 386A5054  addi r3, r10, 0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + 20564;
	// 82606B8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606B94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606BAC: 4BE60275  bl 0x82466e20
	ctx.lr = 0x82606BB0;
	sub_82466E20(ctx, base);
	// 82606BB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606BC0 size=116
    let mut pc: u32 = 0x82606BC0;
    'dispatch: loop {
        match pc {
            0x82606BC0 => {
    //   block [0x82606BC0..0x82606C34)
	// 82606BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606BCC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606BD0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82606BD4: 390AD0C8  addi r8, r10, -0x2f38
	ctx.r[8].s64 = ctx.r[10].s64 + -12088;
	// 82606BD8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606BDC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606BE0: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606BE4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606BE8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606BF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606BF4: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 82606BF8: 396BD27C  addi r11, r11, -0x2d84
	ctx.r[11].s64 = ctx.r[11].s64 + -11652;
	// 82606BFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606C00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606C04: 386A5084  addi r3, r10, 0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + 20612;
	// 82606C08: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82606C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606C10: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82606C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606C18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606C20: 4BE60201  bl 0x82466e20
	ctx.lr = 0x82606C24;
	sub_82466E20(ctx, base);
	// 82606C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606C38 size=112
    let mut pc: u32 = 0x82606C38;
    'dispatch: loop {
        match pc {
            0x82606C38 => {
    //   block [0x82606C38..0x82606CA8)
	// 82606C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606C44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606C48: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606C4C: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606C54: 390BD188  addi r8, r11, -0x2e78
	ctx.r[8].s64 = ctx.r[11].s64 + -11896;
	// 82606C58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606C5C: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 82606C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606C64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606C70: 386A50B4  addi r3, r10, 0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + 20660;
	// 82606C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606C94: 4BE6018D  bl 0x82466e20
	ctx.lr = 0x82606C98;
	sub_82466E20(ctx, base);
	// 82606C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606CA8 size=108
    let mut pc: u32 = 0x82606CA8;
    'dispatch: loop {
        match pc {
            0x82606CA8 => {
    //   block [0x82606CA8..0x82606D14)
	// 82606CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606CB4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606CBC: 38EBD1A0  addi r7, r11, -0x2e60
	ctx.r[7].s64 = ctx.r[11].s64 + -11872;
	// 82606CC0: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 82606CC4: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 82606CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606CCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82606CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606CD8: 386A50E4  addi r3, r10, 0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + 20708;
	// 82606CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82606CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82606D00: 4BE60121  bl 0x82466e20
	ctx.lr = 0x82606D04;
	sub_82466E20(ctx, base);
	// 82606D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606D18 size=116
    let mut pc: u32 = 0x82606D18;
    'dispatch: loop {
        match pc {
            0x82606D18 => {
    //   block [0x82606D18..0x82606D8C)
	// 82606D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606D24: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82606D28: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82606D2C: 390AD2D8  addi r8, r10, -0x2d28
	ctx.r[8].s64 = ctx.r[10].s64 + -11560;
	// 82606D30: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606D34: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82606D38: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606D3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606D40: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606D4C: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 82606D50: 396BD2A0  addi r11, r11, -0x2d60
	ctx.r[11].s64 = ctx.r[11].s64 + -11616;
	// 82606D54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606D58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606D5C: 386A5114  addi r3, r10, 0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + 20756;
	// 82606D60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82606D64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606D68: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82606D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606D78: 4BE600A9  bl 0x82466e20
	ctx.lr = 0x82606D7C;
	sub_82466E20(ctx, base);
	// 82606D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606D90 size=112
    let mut pc: u32 = 0x82606D90;
    'dispatch: loop {
        match pc {
            0x82606D90 => {
    //   block [0x82606D90..0x82606E00)
	// 82606D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606D9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606DA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606DA4: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606DAC: 390BD380  addi r8, r11, -0x2c80
	ctx.r[8].s64 = ctx.r[11].s64 + -11392;
	// 82606DB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606DB4: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 82606DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606DBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606DC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606DC8: 386A5144  addi r3, r10, 0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + 20804;
	// 82606DCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606DEC: 4BE60035  bl 0x82466e20
	ctx.lr = 0x82606DF0;
	sub_82466E20(ctx, base);
	// 82606DF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606E00 size=112
    let mut pc: u32 = 0x82606E00;
    'dispatch: loop {
        match pc {
            0x82606E00 => {
    //   block [0x82606E00..0x82606E70)
	// 82606E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606E0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606E10: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606E14: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606E1C: 390BD398  addi r8, r11, -0x2c68
	ctx.r[8].s64 = ctx.r[11].s64 + -11368;
	// 82606E20: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82606E24: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 82606E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606E2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606E30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606E34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606E38: 386A5174  addi r3, r10, 0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + 20852;
	// 82606E3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606E40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606E5C: 4BE5FFC5  bl 0x82466e20
	ctx.lr = 0x82606E60;
	sub_82466E20(ctx, base);
	// 82606E60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606E6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606E70 size=112
    let mut pc: u32 = 0x82606E70;
    'dispatch: loop {
        match pc {
            0x82606E70 => {
    //   block [0x82606E70..0x82606EE0)
	// 82606E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606E7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606E80: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606E84: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82606E88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606E8C: 390BD4B8  addi r8, r11, -0x2b48
	ctx.r[8].s64 = ctx.r[11].s64 + -11080;
	// 82606E90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606E94: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 82606E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606E9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606EA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606EA8: 386A51A4  addi r3, r10, 0x51a4
	ctx.r[3].s64 = ctx.r[10].s64 + 20900;
	// 82606EAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82606EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606ECC: 4BE5FF55  bl 0x82466e20
	ctx.lr = 0x82606ED0;
	sub_82466E20(ctx, base);
	// 82606ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606EE0 size=116
    let mut pc: u32 = 0x82606EE0;
    'dispatch: loop {
        match pc {
            0x82606EE0 => {
    //   block [0x82606EE0..0x82606F54)
	// 82606EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606EEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606EF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82606EF4: 390BD4D4  addi r8, r11, -0x2b2c
	ctx.r[8].s64 = ctx.r[11].s64 + -11052;
	// 82606EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606EFC: 392AD2D8  addi r9, r10, -0x2d28
	ctx.r[9].s64 = ctx.r[10].s64 + -11560;
	// 82606F00: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606F04: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82606F08: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82606F0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606F14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606F1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606F24: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82606F28: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 82606F2C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82606F30: 386B51D4  addi r3, r11, 0x51d4
	ctx.r[3].s64 = ctx.r[11].s64 + 20948;
	// 82606F34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82606F38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606F40: 4BE5FEE1  bl 0x82466e20
	ctx.lr = 0x82606F44;
	sub_82466E20(ctx, base);
	// 82606F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606F58 size=100
    let mut pc: u32 = 0x82606F58;
    'dispatch: loop {
        match pc {
            0x82606F58 => {
    //   block [0x82606F58..0x82606FBC)
	// 82606F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606F64: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606F6C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82606F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82606F78: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 82606F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82606F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82606F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82606F8C: 386A5204  addi r3, r10, 0x5204
	ctx.r[3].s64 = ctx.r[10].s64 + 20996;
	// 82606F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82606F94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82606F98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82606F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606FA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82606FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82606FA8: 4BE5FE79  bl 0x82466e20
	ctx.lr = 0x82606FAC;
	sub_82466E20(ctx, base);
	// 82606FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82606FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82606FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82606FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82606FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82606FC0 size=112
    let mut pc: u32 = 0x82606FC0;
    'dispatch: loop {
        match pc {
            0x82606FC0 => {
    //   block [0x82606FC0..0x82607030)
	// 82606FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82606FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82606FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82606FCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606FD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82606FD4: 38AA5204  addi r5, r10, 0x5204
	ctx.r[5].s64 = ctx.r[10].s64 + 20996;
	// 82606FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82606FDC: 390BD504  addi r8, r11, -0x2afc
	ctx.r[8].s64 = ctx.r[11].s64 + -11004;
	// 82606FE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82606FE4: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 82606FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82606FEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82606FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82606FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82606FF8: 386A5234  addi r3, r10, 0x5234
	ctx.r[3].s64 = ctx.r[10].s64 + 21044;
	// 82606FFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260700C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260701C: 4BE5FE05  bl 0x82466e20
	ctx.lr = 0x82607020;
	sub_82466E20(ctx, base);
	// 82607020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260702C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607030 size=112
    let mut pc: u32 = 0x82607030;
    'dispatch: loop {
        match pc {
            0x82607030 => {
    //   block [0x82607030..0x826070A0)
	// 82607030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260703C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607040: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607044: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260704C: 390BD520  addi r8, r11, -0x2ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -10976;
	// 82607050: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82607054: 388A3428  addi r4, r10, 0x3428
	ctx.r[4].s64 = ctx.r[10].s64 + 13352;
	// 82607058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260705C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607068: 386A5264  addi r3, r10, 0x5264
	ctx.r[3].s64 = ctx.r[10].s64 + 21092;
	// 8260706C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260707C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260708C: 4BE5FD95  bl 0x82466e20
	ctx.lr = 0x82607090;
	sub_82466E20(ctx, base);
	// 82607090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260709C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826070A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826070A0 size=112
    let mut pc: u32 = 0x826070A0;
    'dispatch: loop {
        match pc {
            0x826070A0 => {
    //   block [0x826070A0..0x82607110)
	// 826070A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826070A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826070A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826070AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826070B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826070B4: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 826070B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826070BC: 390BD580  addi r8, r11, -0x2a80
	ctx.r[8].s64 = ctx.r[11].s64 + -10880;
	// 826070C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826070C4: 388A3440  addi r4, r10, 0x3440
	ctx.r[4].s64 = ctx.r[10].s64 + 13376;
	// 826070C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826070CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826070D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826070D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826070D8: 386A5294  addi r3, r10, 0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + 21140;
	// 826070DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826070E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826070E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826070E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826070EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826070F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826070F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826070F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826070FC: 4BE5FD25  bl 0x82466e20
	ctx.lr = 0x82607100;
	sub_82466E20(ctx, base);
	// 82607100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260710C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607110 size=112
    let mut pc: u32 = 0x82607110;
    'dispatch: loop {
        match pc {
            0x82607110 => {
    //   block [0x82607110..0x82607180)
	// 82607110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260711C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607120: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607124: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260712C: 390BD5C8  addi r8, r11, -0x2a38
	ctx.r[8].s64 = ctx.r[11].s64 + -10808;
	// 82607130: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82607134: 388A3450  addi r4, r10, 0x3450
	ctx.r[4].s64 = ctx.r[10].s64 + 13392;
	// 82607138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260713C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607148: 386A52C4  addi r3, r10, 0x52c4
	ctx.r[3].s64 = ctx.r[10].s64 + 21188;
	// 8260714C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260715C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260716C: 4BE5FCB5  bl 0x82466e20
	ctx.lr = 0x82607170;
	sub_82466E20(ctx, base);
	// 82607170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260717C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607180 size=112
    let mut pc: u32 = 0x82607180;
    'dispatch: loop {
        match pc {
            0x82607180 => {
    //   block [0x82607180..0x826071F0)
	// 82607180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260718C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607190: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607194: 38AA4CC4  addi r5, r10, 0x4cc4
	ctx.r[5].s64 = ctx.r[10].s64 + 19652;
	// 82607198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260719C: 390BD5F8  addi r8, r11, -0x2a08
	ctx.r[8].s64 = ctx.r[11].s64 + -10760;
	// 826071A0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826071A4: 388A3468  addi r4, r10, 0x3468
	ctx.r[4].s64 = ctx.r[10].s64 + 13416;
	// 826071A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826071AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826071B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826071B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826071B8: 386A52F4  addi r3, r10, 0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + 21236;
	// 826071BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826071C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826071C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826071C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826071CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826071D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826071D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826071D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826071DC: 4BE5FC45  bl 0x82466e20
	ctx.lr = 0x826071E0;
	sub_82466E20(ctx, base);
	// 826071E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826071E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826071E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826071EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826071F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826071F0 size=100
    let mut pc: u32 = 0x826071F0;
    'dispatch: loop {
        match pc {
            0x826071F0 => {
    //   block [0x826071F0..0x82607254)
	// 826071F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826071F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826071F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826071FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607204: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260720C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607210: 388A3474  addi r4, r10, 0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + 13428;
	// 82607214: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260721C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607224: 386A5324  addi r3, r10, 0x5324
	ctx.r[3].s64 = ctx.r[10].s64 + 21284;
	// 82607228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260722C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607230: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82607234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607238: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260723C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607240: 4BE5FBE1  bl 0x82466e20
	ctx.lr = 0x82607244;
	sub_82466E20(ctx, base);
	// 82607244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260724C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607258 size=108
    let mut pc: u32 = 0x82607258;
    'dispatch: loop {
        match pc {
            0x82607258 => {
    //   block [0x82607258..0x826072C4)
	// 82607258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260725C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607264: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607268: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260726C: 38EBD670  addi r7, r11, -0x2990
	ctx.r[7].s64 = ctx.r[11].s64 + -10640;
	// 82607270: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607274: 388A3488  addi r4, r10, 0x3488
	ctx.r[4].s64 = ctx.r[10].s64 + 13448;
	// 82607278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260727C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607288: 386A5354  addi r3, r10, 0x5354
	ctx.r[3].s64 = ctx.r[10].s64 + 21332;
	// 8260728C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260729C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826072A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826072A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826072A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826072AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826072B0: 4BE5FB71  bl 0x82466e20
	ctx.lr = 0x826072B4;
	sub_82466E20(ctx, base);
	// 826072B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826072B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826072BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826072C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826072C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826072C8 size=112
    let mut pc: u32 = 0x826072C8;
    'dispatch: loop {
        match pc {
            0x826072C8 => {
    //   block [0x826072C8..0x82607338)
	// 826072C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826072CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826072D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826072D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826072D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826072DC: 38AA5324  addi r5, r10, 0x5324
	ctx.r[5].s64 = ctx.r[10].s64 + 21284;
	// 826072E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826072E4: 390BD6A0  addi r8, r11, -0x2960
	ctx.r[8].s64 = ctx.r[11].s64 + -10592;
	// 826072E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826072EC: 388A34AC  addi r4, r10, 0x34ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13484;
	// 826072F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826072F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826072F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826072FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607300: 386A5384  addi r3, r10, 0x5384
	ctx.r[3].s64 = ctx.r[10].s64 + 21380;
	// 82607304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260730C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260731C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607324: 4BE5FAFD  bl 0x82466e20
	ctx.lr = 0x82607328;
	sub_82466E20(ctx, base);
	// 82607328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260732C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607338 size=108
    let mut pc: u32 = 0x82607338;
    'dispatch: loop {
        match pc {
            0x82607338 => {
    //   block [0x82607338..0x826073A4)
	// 82607338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260733C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607344: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607348: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260734C: 38EBD6D0  addi r7, r11, -0x2930
	ctx.r[7].s64 = ctx.r[11].s64 + -10544;
	// 82607350: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607354: 388A34C4  addi r4, r10, 0x34c4
	ctx.r[4].s64 = ctx.r[10].s64 + 13508;
	// 82607358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260735C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607360: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607368: 386A53B4  addi r3, r10, 0x53b4
	ctx.r[3].s64 = ctx.r[10].s64 + 21428;
	// 8260736C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607370: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607378: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260737C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607380: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607388: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260738C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607390: 4BE5FA91  bl 0x82466e20
	ctx.lr = 0x82607394;
	sub_82466E20(ctx, base);
	// 82607394: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260739C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826073A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826073A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826073A8 size=112
    let mut pc: u32 = 0x826073A8;
    'dispatch: loop {
        match pc {
            0x826073A8 => {
    //   block [0x826073A8..0x82607418)
	// 826073A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826073AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826073B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826073B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826073B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826073BC: 38AA5324  addi r5, r10, 0x5324
	ctx.r[5].s64 = ctx.r[10].s64 + 21284;
	// 826073C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826073C4: 390BD700  addi r8, r11, -0x2900
	ctx.r[8].s64 = ctx.r[11].s64 + -10496;
	// 826073C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826073CC: 388A34F4  addi r4, r10, 0x34f4
	ctx.r[4].s64 = ctx.r[10].s64 + 13556;
	// 826073D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826073D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826073D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826073DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826073E0: 386A53E4  addi r3, r10, 0x53e4
	ctx.r[3].s64 = ctx.r[10].s64 + 21476;
	// 826073E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826073E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826073EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826073F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826073F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826073F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826073FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607404: 4BE5FA1D  bl 0x82466e20
	ctx.lr = 0x82607408;
	sub_82466E20(ctx, base);
	// 82607408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260740C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607418 size=108
    let mut pc: u32 = 0x82607418;
    'dispatch: loop {
        match pc {
            0x82607418 => {
    //   block [0x82607418..0x82607484)
	// 82607418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260741C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607424: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260742C: 38EBD748  addi r7, r11, -0x28b8
	ctx.r[7].s64 = ctx.r[11].s64 + -10424;
	// 82607430: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607434: 388A3514  addi r4, r10, 0x3514
	ctx.r[4].s64 = ctx.r[10].s64 + 13588;
	// 82607438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260743C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607440: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607448: 386A5414  addi r3, r10, 0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + 21524;
	// 8260744C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260745C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260746C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607470: 4BE5F9B1  bl 0x82466e20
	ctx.lr = 0x82607474;
	sub_82466E20(ctx, base);
	// 82607474: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260747C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607480: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607488 size=112
    let mut pc: u32 = 0x82607488;
    'dispatch: loop {
        match pc {
            0x82607488 => {
    //   block [0x82607488..0x826074F8)
	// 82607488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260748C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607498: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260749C: 38AA5324  addi r5, r10, 0x5324
	ctx.r[5].s64 = ctx.r[10].s64 + 21284;
	// 826074A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826074A4: 390BD778  addi r8, r11, -0x2888
	ctx.r[8].s64 = ctx.r[11].s64 + -10376;
	// 826074A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826074AC: 388A3544  addi r4, r10, 0x3544
	ctx.r[4].s64 = ctx.r[10].s64 + 13636;
	// 826074B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826074B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826074B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826074BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826074C0: 386A5444  addi r3, r10, 0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + 21572;
	// 826074C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826074C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826074CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826074D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826074D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826074D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826074DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826074E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826074E4: 4BE5F93D  bl 0x82466e20
	ctx.lr = 0x826074E8;
	sub_82466E20(ctx, base);
	// 826074E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826074EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826074F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826074F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826074F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826074F8 size=108
    let mut pc: u32 = 0x826074F8;
    'dispatch: loop {
        match pc {
            0x826074F8 => {
    //   block [0x826074F8..0x82607564)
	// 826074F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826074FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607504: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260750C: 38EBD7C0  addi r7, r11, -0x2840
	ctx.r[7].s64 = ctx.r[11].s64 + -10304;
	// 82607510: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607514: 388A3564  addi r4, r10, 0x3564
	ctx.r[4].s64 = ctx.r[10].s64 + 13668;
	// 82607518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260751C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607528: 386A5474  addi r3, r10, 0x5474
	ctx.r[3].s64 = ctx.r[10].s64 + 21620;
	// 8260752C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260753C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260754C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607550: 4BE5F8D1  bl 0x82466e20
	ctx.lr = 0x82607554;
	sub_82466E20(ctx, base);
	// 82607554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260755C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607568 size=112
    let mut pc: u32 = 0x82607568;
    'dispatch: loop {
        match pc {
            0x82607568 => {
    //   block [0x82607568..0x826075D8)
	// 82607568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260756C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607574: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607578: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260757C: 38AA5324  addi r5, r10, 0x5324
	ctx.r[5].s64 = ctx.r[10].s64 + 21284;
	// 82607580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607584: 390BD7F0  addi r8, r11, -0x2810
	ctx.r[8].s64 = ctx.r[11].s64 + -10256;
	// 82607588: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260758C: 388A3594  addi r4, r10, 0x3594
	ctx.r[4].s64 = ctx.r[10].s64 + 13716;
	// 82607590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607594: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260759C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826075A0: 386A54A4  addi r3, r10, 0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + 21668;
	// 826075A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826075A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826075AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826075B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826075B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826075B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826075BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826075C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826075C4: 4BE5F85D  bl 0x82466e20
	ctx.lr = 0x826075C8;
	sub_82466E20(ctx, base);
	// 826075C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826075CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826075D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826075D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826075D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826075D8 size=108
    let mut pc: u32 = 0x826075D8;
    'dispatch: loop {
        match pc {
            0x826075D8 => {
    //   block [0x826075D8..0x82607644)
	// 826075D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826075DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826075E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826075E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826075E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826075EC: 38EBD838  addi r7, r11, -0x27c8
	ctx.r[7].s64 = ctx.r[11].s64 + -10184;
	// 826075F0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826075F4: 388A35B0  addi r4, r10, 0x35b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13744;
	// 826075F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826075FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607608: 386A54D4  addi r3, r10, 0x54d4
	ctx.r[3].s64 = ctx.r[10].s64 + 21716;
	// 8260760C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260761C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260762C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607630: 4BE5F7F1  bl 0x82466e20
	ctx.lr = 0x82607634;
	sub_82466E20(ctx, base);
	// 82607634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260763C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607648 size=112
    let mut pc: u32 = 0x82607648;
    'dispatch: loop {
        match pc {
            0x82607648 => {
    //   block [0x82607648..0x826076B8)
	// 82607648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260764C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607654: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607658: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260765C: 392AD348  addi r9, r10, -0x2cb8
	ctx.r[9].s64 = ctx.r[10].s64 + -11448;
	// 82607660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607664: 390BD898  addi r8, r11, -0x2768
	ctx.r[8].s64 = ctx.r[11].s64 + -10088;
	// 82607668: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8260766C: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 82607670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607674: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260767C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607680: 386A5504  addi r3, r10, 0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + 21764;
	// 82607684: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607688: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260768C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260769C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826076A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826076A4: 4BE5F77D  bl 0x82466e20
	ctx.lr = 0x826076A8;
	sub_82466E20(ctx, base);
	// 826076A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826076AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826076B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826076B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826076B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826076B8 size=108
    let mut pc: u32 = 0x826076B8;
    'dispatch: loop {
        match pc {
            0x826076B8 => {
    //   block [0x826076B8..0x82607724)
	// 826076B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826076BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826076C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826076C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826076C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826076CC: 38EBD958  addi r7, r11, -0x26a8
	ctx.r[7].s64 = ctx.r[11].s64 + -9896;
	// 826076D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826076D4: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826076D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826076DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826076E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826076E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826076E8: 386A5534  addi r3, r10, 0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + 21812;
	// 826076EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826076F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826076F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826076F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826076FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260770C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607710: 4BE5F711  bl 0x82466e20
	ctx.lr = 0x82607714;
	sub_82466E20(ctx, base);
	// 82607714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260771C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607728 size=116
    let mut pc: u32 = 0x82607728;
    'dispatch: loop {
        match pc {
            0x82607728 => {
    //   block [0x82607728..0x8260779C)
	// 82607728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260772C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607734: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607738: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 8260773C: 390AD9D0  addi r8, r10, -0x2630
	ctx.r[8].s64 = ctx.r[10].s64 + -9776;
	// 82607740: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607744: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82607748: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 8260774C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607750: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260775C: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 82607760: 396BD360  addi r11, r11, -0x2ca0
	ctx.r[11].s64 = ctx.r[11].s64 + -11424;
	// 82607764: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607768: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260776C: 386A5564  addi r3, r10, 0x5564
	ctx.r[3].s64 = ctx.r[10].s64 + 21860;
	// 82607770: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82607774: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607778: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260777C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607788: 4BE5F699  bl 0x82466e20
	ctx.lr = 0x8260778C;
	sub_82466E20(ctx, base);
	// 8260778C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826077A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826077A0 size=100
    let mut pc: u32 = 0x826077A0;
    'dispatch: loop {
        match pc {
            0x826077A0 => {
    //   block [0x826077A0..0x82607804)
	// 826077A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826077A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826077A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826077AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826077B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826077B4: 38AA4AB4  addi r5, r10, 0x4ab4
	ctx.r[5].s64 = ctx.r[10].s64 + 19124;
	// 826077B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826077BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826077C0: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826077C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826077C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826077CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826077D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826077D4: 386A5594  addi r3, r10, 0x5594
	ctx.r[3].s64 = ctx.r[10].s64 + 21908;
	// 826077D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826077DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826077E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826077E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826077E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826077EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826077F0: 4BE5F631  bl 0x82466e20
	ctx.lr = 0x826077F4;
	sub_82466E20(ctx, base);
	// 826077F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826077F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826077FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82607808 size=24
    let mut pc: u32 = 0x82607808;
    'dispatch: loop {
        match pc {
            0x82607808 => {
    //   block [0x82607808..0x82607820)
	// 82607808: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260780C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607810: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 82607814: 816BDB6C  lwz r11, -0x2494(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9364 as u32) ) } as u64;
	// 82607818: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8260781C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607820 size=116
    let mut pc: u32 = 0x82607820;
    'dispatch: loop {
        match pc {
            0x82607820 => {
    //   block [0x82607820..0x82607894)
	// 82607820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260782C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607830: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607834: 390B3260  addi r8, r11, 0x3260
	ctx.r[8].s64 = ctx.r[11].s64 + 12896;
	// 82607838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260783C: 392AD3D8  addi r9, r10, -0x2c28
	ctx.r[9].s64 = ctx.r[10].s64 + -11304;
	// 82607840: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607844: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 82607848: 38AA5594  addi r5, r10, 0x5594
	ctx.r[5].s64 = ctx.r[10].s64 + 21908;
	// 8260784C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607854: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260785C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607864: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82607868: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 8260786C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607870: 386B55C4  addi r3, r11, 0x55c4
	ctx.r[3].s64 = ctx.r[11].s64 + 21956;
	// 82607874: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82607878: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260787C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607880: 4BE5F5A1  bl 0x82466e20
	ctx.lr = 0x82607884;
	sub_82466E20(ctx, base);
	// 82607884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607888: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260788C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607898 size=108
    let mut pc: u32 = 0x82607898;
    'dispatch: loop {
        match pc {
            0x82607898 => {
    //   block [0x82607898..0x82607904)
	// 82607898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260789C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826078A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826078A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826078A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826078AC: 38EBDB70  addi r7, r11, -0x2490
	ctx.r[7].s64 = ctx.r[11].s64 + -9360;
	// 826078B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826078B4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826078B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826078BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826078C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826078C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826078C8: 386A55F4  addi r3, r10, 0x55f4
	ctx.r[3].s64 = ctx.r[10].s64 + 22004;
	// 826078CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826078D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826078D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826078D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826078DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826078E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826078E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826078E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826078EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826078F0: 4BE5F531  bl 0x82466e20
	ctx.lr = 0x826078F4;
	sub_82466E20(ctx, base);
	// 826078F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826078F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826078FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82607908 size=24
    let mut pc: u32 = 0x82607908;
    'dispatch: loop {
        match pc {
            0x82607908 => {
    //   block [0x82607908..0x82607920)
	// 82607908: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260790C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607910: 394A3380  addi r10, r10, 0x3380
	ctx.r[10].s64 = ctx.r[10].s64 + 13184;
	// 82607914: 816BDB88  lwz r11, -0x2478(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9336 as u32) ) } as u64;
	// 82607918: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8260791C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607920 size=112
    let mut pc: u32 = 0x82607920;
    'dispatch: loop {
        match pc {
            0x82607920 => {
    //   block [0x82607920..0x82607990)
	// 82607920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260792C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607930: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607934: 392AD430  addi r9, r10, -0x2bd0
	ctx.r[9].s64 = ctx.r[10].s64 + -11216;
	// 82607938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260793C: 390B3380  addi r8, r11, 0x3380
	ctx.r[8].s64 = ctx.r[11].s64 + 13184;
	// 82607940: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82607944: 388A3728  addi r4, r10, 0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + 14120;
	// 82607948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260794C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607958: 386A5624  addi r3, r10, 0x5624
	ctx.r[3].s64 = ctx.r[10].s64 + 22052;
	// 8260795C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607960: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82607964: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260796C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260797C: 4BE5F4A5  bl 0x82466e20
	ctx.lr = 0x82607980;
	sub_82466E20(ctx, base);
	// 82607980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260798C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607990 size=108
    let mut pc: u32 = 0x82607990;
    'dispatch: loop {
        match pc {
            0x82607990 => {
    //   block [0x82607990..0x826079FC)
	// 82607990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260799C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826079A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826079A4: 38EBDB90  addi r7, r11, -0x2470
	ctx.r[7].s64 = ctx.r[11].s64 + -9328;
	// 826079A8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826079AC: 388A3738  addi r4, r10, 0x3738
	ctx.r[4].s64 = ctx.r[10].s64 + 14136;
	// 826079B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826079B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826079B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826079BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826079C0: 386A5654  addi r3, r10, 0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + 22100;
	// 826079C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826079C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826079CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826079D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826079D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826079D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826079DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826079E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826079E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826079E8: 4BE5F439  bl 0x82466e20
	ctx.lr = 0x826079EC;
	sub_82466E20(ctx, base);
	// 826079EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826079F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826079F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826079F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607A00 size=112
    let mut pc: u32 = 0x82607A00;
    'dispatch: loop {
        match pc {
            0x82607A00 => {
    //   block [0x82607A00..0x82607A70)
	// 82607A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607A0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607A10: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607A14: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607A18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607A1C: 390BDBF0  addi r8, r11, -0x2410
	ctx.r[8].s64 = ctx.r[11].s64 + -9232;
	// 82607A20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82607A24: 388A3758  addi r4, r10, 0x3758
	ctx.r[4].s64 = ctx.r[10].s64 + 14168;
	// 82607A28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607A2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607A38: 386A5684  addi r3, r10, 0x5684
	ctx.r[3].s64 = ctx.r[10].s64 + 22148;
	// 82607A3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607A40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607A48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607A50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607A54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607A5C: 4BE5F3C5  bl 0x82466e20
	ctx.lr = 0x82607A60;
	sub_82466E20(ctx, base);
	// 82607A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607A64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607A68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607A70 size=108
    let mut pc: u32 = 0x82607A70;
    'dispatch: loop {
        match pc {
            0x82607A70 => {
    //   block [0x82607A70..0x82607ADC)
	// 82607A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607A7C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607A84: 38EBDC08  addi r7, r11, -0x23f8
	ctx.r[7].s64 = ctx.r[11].s64 + -9208;
	// 82607A88: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82607A8C: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 82607A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607A94: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607A98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607AA0: 386A56B4  addi r3, r10, 0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22196;
	// 82607AA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607AA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607AAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607AB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607AB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607AB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607ABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607AC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607AC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607AC8: 4BE5F359  bl 0x82466e20
	ctx.lr = 0x82607ACC;
	sub_82466E20(ctx, base);
	// 82607ACC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607AD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607AD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607AE0 size=108
    let mut pc: u32 = 0x82607AE0;
    'dispatch: loop {
        match pc {
            0x82607AE0 => {
    //   block [0x82607AE0..0x82607B4C)
	// 82607AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607AE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607AEC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607AF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607AF4: 38EBDC68  addi r7, r11, -0x2398
	ctx.r[7].s64 = ctx.r[11].s64 + -9112;
	// 82607AF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607AFC: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 82607B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607B04: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607B08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607B10: 386A56E4  addi r3, r10, 0x56e4
	ctx.r[3].s64 = ctx.r[10].s64 + 22244;
	// 82607B14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607B18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607B20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607B28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607B30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607B34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607B38: 4BE5F2E9  bl 0x82466e20
	ctx.lr = 0x82607B3C;
	sub_82466E20(ctx, base);
	// 82607B3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607B40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607B44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607B48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607B50 size=116
    let mut pc: u32 = 0x82607B50;
    'dispatch: loop {
        match pc {
            0x82607B50 => {
    //   block [0x82607B50..0x82607BC4)
	// 82607B50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607B54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607B58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607B5C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607B60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607B64: 390BDC98  addi r8, r11, -0x2368
	ctx.r[8].s64 = ctx.r[11].s64 + -9064;
	// 82607B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607B6C: 392AD45C  addi r9, r10, -0x2ba4
	ctx.r[9].s64 = ctx.r[10].s64 + -11172;
	// 82607B70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607B74: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82607B78: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82607B7C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607B80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607B84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82607B88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607B90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607B94: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82607B98: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 82607B9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607BA0: 386B5714  addi r3, r11, 0x5714
	ctx.r[3].s64 = ctx.r[11].s64 + 22292;
	// 82607BA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82607BA8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607BB0: 4BE5F271  bl 0x82466e20
	ctx.lr = 0x82607BB4;
	sub_82466E20(ctx, base);
	// 82607BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607BC8 size=96
    let mut pc: u32 = 0x82607BC8;
    'dispatch: loop {
        match pc {
            0x82607BC8 => {
    //   block [0x82607BC8..0x82607C28)
	// 82607BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607BD4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607BDC: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 82607BE0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607BE8: 386A5744  addi r3, r10, 0x5744
	ctx.r[3].s64 = ctx.r[10].s64 + 22340;
	// 82607BEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607BF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82607BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607C08: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82607C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607C10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82607C14: 4BE5F20D  bl 0x82466e20
	ctx.lr = 0x82607C18;
	sub_82466E20(ctx, base);
	// 82607C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607C28 size=112
    let mut pc: u32 = 0x82607C28;
    'dispatch: loop {
        match pc {
            0x82607C28 => {
    //   block [0x82607C28..0x82607C98)
	// 82607C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607C34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607C38: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607C3C: 38AA5744  addi r5, r10, 0x5744
	ctx.r[5].s64 = ctx.r[10].s64 + 22340;
	// 82607C40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607C44: 390BDCB0  addi r8, r11, -0x2350
	ctx.r[8].s64 = ctx.r[11].s64 + -9040;
	// 82607C48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82607C4C: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82607C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607C54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607C60: 386A5774  addi r3, r10, 0x5774
	ctx.r[3].s64 = ctx.r[10].s64 + 22388;
	// 82607C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82607C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607C84: 4BE5F19D  bl 0x82466e20
	ctx.lr = 0x82607C88;
	sub_82466E20(ctx, base);
	// 82607C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607C98 size=112
    let mut pc: u32 = 0x82607C98;
    'dispatch: loop {
        match pc {
            0x82607C98 => {
    //   block [0x82607C98..0x82607D08)
	// 82607C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607CA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607CA8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607CAC: 392AD478  addi r9, r10, -0x2b88
	ctx.r[9].s64 = ctx.r[10].s64 + -11144;
	// 82607CB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607CB4: 390BDCE8  addi r8, r11, -0x2318
	ctx.r[8].s64 = ctx.r[11].s64 + -8984;
	// 82607CB8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82607CBC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82607CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607CC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607CD0: 386A57A4  addi r3, r10, 0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + 22436;
	// 82607CD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607CD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82607CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607CE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607CEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607CF4: 4BE5F12D  bl 0x82466e20
	ctx.lr = 0x82607CF8;
	sub_82466E20(ctx, base);
	// 82607CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607D08 size=108
    let mut pc: u32 = 0x82607D08;
    'dispatch: loop {
        match pc {
            0x82607D08 => {
    //   block [0x82607D08..0x82607D74)
	// 82607D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607D14: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607D18: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607D1C: 38EBDD90  addi r7, r11, -0x2270
	ctx.r[7].s64 = ctx.r[11].s64 + -8816;
	// 82607D20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607D24: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82607D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607D2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607D38: 386A57D4  addi r3, r10, 0x57d4
	ctx.r[3].s64 = ctx.r[10].s64 + 22484;
	// 82607D3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607D5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607D60: 4BE5F0C1  bl 0x82466e20
	ctx.lr = 0x82607D64;
	sub_82466E20(ctx, base);
	// 82607D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607D78 size=108
    let mut pc: u32 = 0x82607D78;
    'dispatch: loop {
        match pc {
            0x82607D78 => {
    //   block [0x82607D78..0x82607DE4)
	// 82607D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607D84: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607D88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607D8C: 38EBDDC0  addi r7, r11, -0x2240
	ctx.r[7].s64 = ctx.r[11].s64 + -8768;
	// 82607D90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607D94: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82607D98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607D9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607DA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607DA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607DA8: 386A5804  addi r3, r10, 0x5804
	ctx.r[3].s64 = ctx.r[10].s64 + 22532;
	// 82607DAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607DB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607DB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607DCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607DD0: 4BE5F051  bl 0x82466e20
	ctx.lr = 0x82607DD4;
	sub_82466E20(ctx, base);
	// 82607DD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82607DE8 size=28
    let mut pc: u32 = 0x82607DE8;
    'dispatch: loop {
        match pc {
            0x82607DE8 => {
    //   block [0x82607DE8..0x82607E04)
	// 82607DE8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607DEC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607DF0: 394A33B0  addi r10, r10, 0x33b0
	ctx.r[10].s64 = ctx.r[10].s64 + 13232;
	// 82607DF4: 816BDCE4  lwz r11, -0x231c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8988 as u32) ) } as u64;
	// 82607DF8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82607DFC: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82607E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607E08 size=112
    let mut pc: u32 = 0x82607E08;
    'dispatch: loop {
        match pc {
            0x82607E08 => {
    //   block [0x82607E08..0x82607E78)
	// 82607E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607E14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607E18: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607E1C: 392AD608  addi r9, r10, -0x29f8
	ctx.r[9].s64 = ctx.r[10].s64 + -10744;
	// 82607E20: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607E24: 390B33B0  addi r8, r11, 0x33b0
	ctx.r[8].s64 = ctx.r[11].s64 + 13232;
	// 82607E28: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82607E2C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82607E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607E34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82607E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607E40: 386A5834  addi r3, r10, 0x5834
	ctx.r[3].s64 = ctx.r[10].s64 + 22580;
	// 82607E44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82607E48: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82607E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607E5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607E64: 4BE5EFBD  bl 0x82466e20
	ctx.lr = 0x82607E68;
	sub_82466E20(ctx, base);
	// 82607E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607E78 size=108
    let mut pc: u32 = 0x82607E78;
    'dispatch: loop {
        match pc {
            0x82607E78 => {
    //   block [0x82607E78..0x82607EE4)
	// 82607E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607E84: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607E88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607E8C: 38EBDDF8  addi r7, r11, -0x2208
	ctx.r[7].s64 = ctx.r[11].s64 + -8712;
	// 82607E90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607E94: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82607E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607E9C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607EA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607EA8: 386A5864  addi r3, r10, 0x5864
	ctx.r[3].s64 = ctx.r[10].s64 + 22628;
	// 82607EAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607EB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607ECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607ED0: 4BE5EF51  bl 0x82466e20
	ctx.lr = 0x82607ED4;
	sub_82466E20(ctx, base);
	// 82607ED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607ED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607EDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607EE8 size=108
    let mut pc: u32 = 0x82607EE8;
    'dispatch: loop {
        match pc {
            0x82607EE8 => {
    //   block [0x82607EE8..0x82607F54)
	// 82607EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607EF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607EF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607EFC: 38EBDE28  addi r7, r11, -0x21d8
	ctx.r[7].s64 = ctx.r[11].s64 + -8664;
	// 82607F00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82607F04: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 82607F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607F0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607F10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607F18: 386A5894  addi r3, r10, 0x5894
	ctx.r[3].s64 = ctx.r[10].s64 + 22676;
	// 82607F1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607F3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607F40: 4BE5EEE1  bl 0x82466e20
	ctx.lr = 0x82607F44;
	sub_82466E20(ctx, base);
	// 82607F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607F58 size=108
    let mut pc: u32 = 0x82607F58;
    'dispatch: loop {
        match pc {
            0x82607F58 => {
    //   block [0x82607F58..0x82607FC4)
	// 82607F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607F64: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607F68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607F6C: 38EBDE58  addi r7, r11, -0x21a8
	ctx.r[7].s64 = ctx.r[11].s64 + -8616;
	// 82607F70: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82607F74: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 82607F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82607F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82607F80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82607F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82607F88: 386A58C4  addi r3, r10, 0x58c4
	ctx.r[3].s64 = ctx.r[10].s64 + 22724;
	// 82607F8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82607F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82607F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82607F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82607F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82607FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82607FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82607FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82607FAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82607FB0: 4BE5EE71  bl 0x82466e20
	ctx.lr = 0x82607FB4;
	sub_82466E20(ctx, base);
	// 82607FB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82607FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82607FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82607FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82607FC8 size=24
    let mut pc: u32 = 0x82607FC8;
    'dispatch: loop {
        match pc {
            0x82607FC8 => {
    //   block [0x82607FC8..0x82607FE0)
	// 82607FC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607FCC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82607FD0: 394A3470  addi r10, r10, 0x3470
	ctx.r[10].s64 = ctx.r[10].s64 + 13424;
	// 82607FD4: 816BDE70  lwz r11, -0x2190(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8592 as u32) ) } as u64;
	// 82607FD8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82607FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82607FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82607FE0 size=112
    let mut pc: u32 = 0x82607FE0;
    'dispatch: loop {
        match pc {
            0x82607FE0 => {
    //   block [0x82607FE0..0x82608050)
	// 82607FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82607FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82607FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82607FEC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82607FF0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82607FF4: 392AD65C  addi r9, r10, -0x29a4
	ctx.r[9].s64 = ctx.r[10].s64 + -10660;
	// 82607FF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82607FFC: 390B3470  addi r8, r11, 0x3470
	ctx.r[8].s64 = ctx.r[11].s64 + 13424;
	// 82608000: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82608004: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 82608008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260800C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608010: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608018: 386A58F4  addi r3, r10, 0x58f4
	ctx.r[3].s64 = ctx.r[10].s64 + 22772;
	// 8260801C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608020: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260802C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608034: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260803C: 4BE5EDE5  bl 0x82466e20
	ctx.lr = 0x82608040;
	sub_82466E20(ctx, base);
	// 82608040: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260804C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608050 size=112
    let mut pc: u32 = 0x82608050;
    'dispatch: loop {
        match pc {
            0x82608050 => {
    //   block [0x82608050..0x826080C0)
	// 82608050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260805C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608060: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608064: 392AD698  addi r9, r10, -0x2968
	ctx.r[9].s64 = ctx.r[10].s64 + -10600;
	// 82608068: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260806C: 390BDE80  addi r8, r11, -0x2180
	ctx.r[8].s64 = ctx.r[11].s64 + -8576;
	// 82608070: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82608074: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 82608078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260807C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608088: 386A5924  addi r3, r10, 0x5924
	ctx.r[3].s64 = ctx.r[10].s64 + 22820;
	// 8260808C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608090: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82608094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608098: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260809C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826080A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826080A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826080A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826080AC: 4BE5ED75  bl 0x82466e20
	ctx.lr = 0x826080B0;
	sub_82466E20(ctx, base);
	// 826080B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826080B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826080B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826080BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826080C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826080C0 size=108
    let mut pc: u32 = 0x826080C0;
    'dispatch: loop {
        match pc {
            0x826080C0 => {
    //   block [0x826080C0..0x8260812C)
	// 826080C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826080C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826080C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826080CC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826080D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826080D4: 38EBDEC8  addi r7, r11, -0x2138
	ctx.r[7].s64 = ctx.r[11].s64 + -8504;
	// 826080D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826080DC: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826080E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826080E4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826080E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826080EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826080F0: 386A5954  addi r3, r10, 0x5954
	ctx.r[3].s64 = ctx.r[10].s64 + 22868;
	// 826080F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826080F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826080FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260810C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608114: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608118: 4BE5ED09  bl 0x82466e20
	ctx.lr = 0x8260811C;
	sub_82466E20(ctx, base);
	// 8260811C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608130 size=108
    let mut pc: u32 = 0x82608130;
    'dispatch: loop {
        match pc {
            0x82608130 => {
    //   block [0x82608130..0x8260819C)
	// 82608130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260813C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608140: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82608144: 38EBDEF8  addi r7, r11, -0x2108
	ctx.r[7].s64 = ctx.r[11].s64 + -8456;
	// 82608148: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260814C: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82608150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608154: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608158: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260815C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608160: 386A5984  addi r3, r10, 0x5984
	ctx.r[3].s64 = ctx.r[10].s64 + 22916;
	// 82608164: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260816C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608170: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608178: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260817C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608180: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608184: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608188: 4BE5EC99  bl 0x82466e20
	ctx.lr = 0x8260818C;
	sub_82466E20(ctx, base);
	// 8260818C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826081A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826081A0 size=112
    let mut pc: u32 = 0x826081A0;
    'dispatch: loop {
        match pc {
            0x826081A0 => {
    //   block [0x826081A0..0x82608210)
	// 826081A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826081A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826081A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826081AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826081B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826081B4: 392AD6D0  addi r9, r10, -0x2930
	ctx.r[9].s64 = ctx.r[10].s64 + -10544;
	// 826081B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826081BC: 390BDF28  addi r8, r11, -0x20d8
	ctx.r[8].s64 = ctx.r[11].s64 + -8408;
	// 826081C0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826081C4: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826081C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826081CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826081D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826081D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826081D8: 386A59B4  addi r3, r10, 0x59b4
	ctx.r[3].s64 = ctx.r[10].s64 + 22964;
	// 826081DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826081E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826081E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826081E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826081EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826081F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826081F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826081F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826081FC: 4BE5EC25  bl 0x82466e20
	ctx.lr = 0x82608200;
	sub_82466E20(ctx, base);
	// 82608200: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608204: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608208: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260820C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608210 size=108
    let mut pc: u32 = 0x82608210;
    'dispatch: loop {
        match pc {
            0x82608210 => {
    //   block [0x82608210..0x8260827C)
	// 82608210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260821C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608220: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82608224: 38EBDF88  addi r7, r11, -0x2078
	ctx.r[7].s64 = ctx.r[11].s64 + -8312;
	// 82608228: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8260822C: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 82608230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608238: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260823C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608240: 386A59E4  addi r3, r10, 0x59e4
	ctx.r[3].s64 = ctx.r[10].s64 + 23012;
	// 82608244: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608248: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260824C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608250: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608258: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260825C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608260: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608268: 4BE5EBB9  bl 0x82466e20
	ctx.lr = 0x8260826C;
	sub_82466E20(ctx, base);
	// 8260826C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608270: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608274: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608280 size=108
    let mut pc: u32 = 0x82608280;
    'dispatch: loop {
        match pc {
            0x82608280 => {
    //   block [0x82608280..0x826082EC)
	// 82608280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608288: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260828C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608290: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82608294: 38EBE090  addi r7, r11, -0x1f70
	ctx.r[7].s64 = ctx.r[11].s64 + -8048;
	// 82608298: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260829C: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826082A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826082A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826082A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826082AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826082B0: 386A5A14  addi r3, r10, 0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + 23060;
	// 826082B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826082B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826082BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826082C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826082C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826082C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826082CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826082D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826082D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826082D8: 4BE5EB49  bl 0x82466e20
	ctx.lr = 0x826082DC;
	sub_82466E20(ctx, base);
	// 826082DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826082E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826082E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826082E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826082F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826082F0 size=108
    let mut pc: u32 = 0x826082F0;
    'dispatch: loop {
        match pc {
            0x826082F0 => {
    //   block [0x826082F0..0x8260835C)
	// 826082F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826082F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826082F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826082FC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608304: 38EBE0A8  addi r7, r11, -0x1f58
	ctx.r[7].s64 = ctx.r[11].s64 + -8024;
	// 82608308: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8260830C: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82608310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608314: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608318: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260831C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608320: 386A5A44  addi r3, r10, 0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + 23108;
	// 82608324: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260832C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260833C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608344: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608348: 4BE5EAD9  bl 0x82466e20
	ctx.lr = 0x8260834C;
	sub_82466E20(ctx, base);
	// 8260834C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82608360 size=24
    let mut pc: u32 = 0x82608360;
    'dispatch: loop {
        match pc {
            0x82608360 => {
    //   block [0x82608360..0x82608378)
	// 82608360: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608364: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82608368: 394A3548  addi r10, r10, 0x3548
	ctx.r[10].s64 = ctx.r[10].s64 + 13640;
	// 8260836C: 816BE138  lwz r11, -0x1ec8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7880 as u32) ) } as u64;
	// 82608370: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82608374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608378 size=108
    let mut pc: u32 = 0x82608378;
    'dispatch: loop {
        match pc {
            0x82608378 => {
    //   block [0x82608378..0x826083E4)
	// 82608378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260837C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608384: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608388: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260838C: 38EB3548  addi r7, r11, 0x3548
	ctx.r[7].s64 = ctx.r[11].s64 + 13640;
	// 82608390: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608394: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82608398: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260839C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826083A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826083A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826083A8: 386A5A74  addi r3, r10, 0x5a74
	ctx.r[3].s64 = ctx.r[10].s64 + 23156;
	// 826083AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826083B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826083B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826083B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826083BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826083C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826083C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826083C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826083CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826083D0: 4BE5EA51  bl 0x82466e20
	ctx.lr = 0x826083D4;
	sub_82466E20(ctx, base);
	// 826083D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826083D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826083DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826083E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826083E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826083E8 size=24
    let mut pc: u32 = 0x826083E8;
    'dispatch: loop {
        match pc {
            0x826083E8 => {
    //   block [0x826083E8..0x82608400)
	// 826083E8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826083EC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826083F0: 394A3578  addi r10, r10, 0x3578
	ctx.r[10].s64 = ctx.r[10].s64 + 13688;
	// 826083F4: 816BE138  lwz r11, -0x1ec8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7880 as u32) ) } as u64;
	// 826083F8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826083FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608400 size=108
    let mut pc: u32 = 0x82608400;
    'dispatch: loop {
        match pc {
            0x82608400 => {
    //   block [0x82608400..0x8260846C)
	// 82608400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260840C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608410: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608414: 38EB3578  addi r7, r11, 0x3578
	ctx.r[7].s64 = ctx.r[11].s64 + 13688;
	// 82608418: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260841C: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 82608420: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608424: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608428: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260842C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608430: 386A5AA4  addi r3, r10, 0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + 23204;
	// 82608434: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608438: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260843C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260844C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608454: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608458: 4BE5E9C9  bl 0x82466e20
	ctx.lr = 0x8260845C;
	sub_82466E20(ctx, base);
	// 8260845C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608470 size=108
    let mut pc: u32 = 0x82608470;
    'dispatch: loop {
        match pc {
            0x82608470 => {
    //   block [0x82608470..0x826084DC)
	// 82608470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260847C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608484: 38EBE120  addi r7, r11, -0x1ee0
	ctx.r[7].s64 = ctx.r[11].s64 + -7904;
	// 82608488: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260848C: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82608490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608494: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608498: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260849C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826084A0: 386A5AD4  addi r3, r10, 0x5ad4
	ctx.r[3].s64 = ctx.r[10].s64 + 23252;
	// 826084A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826084A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826084AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826084B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826084B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826084B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826084BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826084C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826084C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826084C8: 4BE5E959  bl 0x82466e20
	ctx.lr = 0x826084CC;
	sub_82466E20(ctx, base);
	// 826084CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826084D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826084D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826084D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826084E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826084E0 size=24
    let mut pc: u32 = 0x826084E0;
    'dispatch: loop {
        match pc {
            0x826084E0 => {
    //   block [0x826084E0..0x826084F8)
	// 826084E0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826084E4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 826084E8: 394A35A8  addi r10, r10, 0x35a8
	ctx.r[10].s64 = ctx.r[10].s64 + 13736;
	// 826084EC: 816BE138  lwz r11, -0x1ec8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7880 as u32) ) } as u64;
	// 826084F0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826084F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826084F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826084F8 size=108
    let mut pc: u32 = 0x826084F8;
    'dispatch: loop {
        match pc {
            0x826084F8 => {
    //   block [0x826084F8..0x82608564)
	// 826084F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826084FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608504: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608508: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260850C: 38EB35A8  addi r7, r11, 0x35a8
	ctx.r[7].s64 = ctx.r[11].s64 + 13736;
	// 82608510: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608514: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 82608518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260851C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608520: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608528: 386A5B04  addi r3, r10, 0x5b04
	ctx.r[3].s64 = ctx.r[10].s64 + 23300;
	// 8260852C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260853C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260854C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608550: 4BE5E8D1  bl 0x82466e20
	ctx.lr = 0x82608554;
	sub_82466E20(ctx, base);
	// 82608554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260855C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608568 size=112
    let mut pc: u32 = 0x82608568;
    'dispatch: loop {
        match pc {
            0x82608568 => {
    //   block [0x82608568..0x826085D8)
	// 82608568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260856C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608574: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608578: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260857C: 392AD714  addi r9, r10, -0x28ec
	ctx.r[9].s64 = ctx.r[10].s64 + -10476;
	// 82608580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608584: 390BE13C  addi r8, r11, -0x1ec4
	ctx.r[8].s64 = ctx.r[11].s64 + -7876;
	// 82608588: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8260858C: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82608590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608594: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260859C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826085A0: 386A5B34  addi r3, r10, 0x5b34
	ctx.r[3].s64 = ctx.r[10].s64 + 23348;
	// 826085A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826085A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826085AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826085B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826085B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826085B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826085BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826085C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826085C4: 4BE5E85D  bl 0x82466e20
	ctx.lr = 0x826085C8;
	sub_82466E20(ctx, base);
	// 826085C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826085CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826085D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826085D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826085D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826085D8 size=108
    let mut pc: u32 = 0x826085D8;
    'dispatch: loop {
        match pc {
            0x826085D8 => {
    //   block [0x826085D8..0x82608644)
	// 826085D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826085DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826085E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826085E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826085E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826085EC: 38EBE16C  addi r7, r11, -0x1e94
	ctx.r[7].s64 = ctx.r[11].s64 + -7828;
	// 826085F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826085F4: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826085F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826085FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608600: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608608: 386A5B64  addi r3, r10, 0x5b64
	ctx.r[3].s64 = ctx.r[10].s64 + 23396;
	// 8260860C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260861C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260862C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608630: 4BE5E7F1  bl 0x82466e20
	ctx.lr = 0x82608634;
	sub_82466E20(ctx, base);
	// 82608634: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260863C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608648 size=108
    let mut pc: u32 = 0x82608648;
    'dispatch: loop {
        match pc {
            0x82608648 => {
    //   block [0x82608648..0x826086B4)
	// 82608648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260864C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608654: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260865C: 38EBE19C  addi r7, r11, -0x1e64
	ctx.r[7].s64 = ctx.r[11].s64 + -7780;
	// 82608660: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82608664: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 82608668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260866C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608678: 386A5B94  addi r3, r10, 0x5b94
	ctx.r[3].s64 = ctx.r[10].s64 + 23444;
	// 8260867C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260868C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260869C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826086A0: 4BE5E781  bl 0x82466e20
	ctx.lr = 0x826086A4;
	sub_82466E20(ctx, base);
	// 826086A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826086A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826086AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826086B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826086B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826086B8 size=108
    let mut pc: u32 = 0x826086B8;
    'dispatch: loop {
        match pc {
            0x826086B8 => {
    //   block [0x826086B8..0x82608724)
	// 826086B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826086BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826086C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826086C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826086C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826086CC: 38EBE1B4  addi r7, r11, -0x1e4c
	ctx.r[7].s64 = ctx.r[11].s64 + -7756;
	// 826086D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826086D4: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826086D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826086DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826086E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826086E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826086E8: 386A5BC4  addi r3, r10, 0x5bc4
	ctx.r[3].s64 = ctx.r[10].s64 + 23492;
	// 826086EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826086F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826086F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826086F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826086FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608700: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608708: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260870C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608710: 4BE5E711  bl 0x82466e20
	ctx.lr = 0x82608714;
	sub_82466E20(ctx, base);
	// 82608714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260871C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608728 size=112
    let mut pc: u32 = 0x82608728;
    'dispatch: loop {
        match pc {
            0x82608728 => {
    //   block [0x82608728..0x82608798)
	// 82608728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260872C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608734: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608738: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260873C: 38AA5C24  addi r5, r10, 0x5c24
	ctx.r[5].s64 = ctx.r[10].s64 + 23588;
	// 82608740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608744: 390BE1E4  addi r8, r11, -0x1e1c
	ctx.r[8].s64 = ctx.r[11].s64 + -7708;
	// 82608748: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260874C: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82608750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608754: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260875C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608760: 386A5BF4  addi r3, r10, 0x5bf4
	ctx.r[3].s64 = ctx.r[10].s64 + 23540;
	// 82608764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82608768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260876C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260877C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608784: 4BE5E69D  bl 0x82466e20
	ctx.lr = 0x82608788;
	sub_82466E20(ctx, base);
	// 82608788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260878C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608798 size=108
    let mut pc: u32 = 0x82608798;
    'dispatch: loop {
        match pc {
            0x82608798 => {
    //   block [0x82608798..0x82608804)
	// 82608798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260879C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826087A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826087A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826087A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826087AC: 38EBE1FC  addi r7, r11, -0x1e04
	ctx.r[7].s64 = ctx.r[11].s64 + -7684;
	// 826087B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826087B4: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826087B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826087BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826087C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826087C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826087C8: 386A5C24  addi r3, r10, 0x5c24
	ctx.r[3].s64 = ctx.r[10].s64 + 23588;
	// 826087CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826087D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826087D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826087D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826087DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826087E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826087E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826087E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826087EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826087F0: 4BE5E631  bl 0x82466e20
	ctx.lr = 0x826087F4;
	sub_82466E20(ctx, base);
	// 826087F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826087F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826087FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608808 size=108
    let mut pc: u32 = 0x82608808;
    'dispatch: loop {
        match pc {
            0x82608808 => {
    //   block [0x82608808..0x82608874)
	// 82608808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260880C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608814: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608818: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260881C: 38EBE22C  addi r7, r11, -0x1dd4
	ctx.r[7].s64 = ctx.r[11].s64 + -7636;
	// 82608820: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82608824: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82608828: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260882C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608830: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608838: 386A5C54  addi r3, r10, 0x5c54
	ctx.r[3].s64 = ctx.r[10].s64 + 23636;
	// 8260883C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608840: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608848: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260884C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608850: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608858: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260885C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608860: 4BE5E5C1  bl 0x82466e20
	ctx.lr = 0x82608864;
	sub_82466E20(ctx, base);
	// 82608864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260886C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608878 size=108
    let mut pc: u32 = 0x82608878;
    'dispatch: loop {
        match pc {
            0x82608878 => {
    //   block [0x82608878..0x826088E4)
	// 82608878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260887C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608884: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260888C: 38EBE244  addi r7, r11, -0x1dbc
	ctx.r[7].s64 = ctx.r[11].s64 + -7612;
	// 82608890: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608894: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82608898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260889C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826088A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826088A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826088A8: 386A5C84  addi r3, r10, 0x5c84
	ctx.r[3].s64 = ctx.r[10].s64 + 23684;
	// 826088AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826088B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826088B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826088B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826088BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826088C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826088C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826088C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826088CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826088D0: 4BE5E551  bl 0x82466e20
	ctx.lr = 0x826088D4;
	sub_82466E20(ctx, base);
	// 826088D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826088D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826088DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826088E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826088E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826088E8 size=108
    let mut pc: u32 = 0x826088E8;
    'dispatch: loop {
        match pc {
            0x826088E8 => {
    //   block [0x826088E8..0x82608954)
	// 826088E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826088EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826088F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826088F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826088F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826088FC: 38EBE278  addi r7, r11, -0x1d88
	ctx.r[7].s64 = ctx.r[11].s64 + -7560;
	// 82608900: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82608904: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82608908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260890C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608910: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608918: 386A5CB4  addi r3, r10, 0x5cb4
	ctx.r[3].s64 = ctx.r[10].s64 + 23732;
	// 8260891C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260892C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260893C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608940: 4BE5E4E1  bl 0x82466e20
	ctx.lr = 0x82608944;
	sub_82466E20(ctx, base);
	// 82608944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260894C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608958 size=108
    let mut pc: u32 = 0x82608958;
    'dispatch: loop {
        match pc {
            0x82608958 => {
    //   block [0x82608958..0x826089C4)
	// 82608958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260895C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608964: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260896C: 38EBE320  addi r7, r11, -0x1ce0
	ctx.r[7].s64 = ctx.r[11].s64 + -7392;
	// 82608970: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608974: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82608978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260897C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608988: 386A5CE4  addi r3, r10, 0x5ce4
	ctx.r[3].s64 = ctx.r[10].s64 + 23780;
	// 8260898C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260899C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826089A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826089A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826089A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826089AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826089B0: 4BE5E471  bl 0x82466e20
	ctx.lr = 0x826089B4;
	sub_82466E20(ctx, base);
	// 826089B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826089B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826089BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826089C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826089C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826089C8 size=108
    let mut pc: u32 = 0x826089C8;
    'dispatch: loop {
        match pc {
            0x826089C8 => {
    //   block [0x826089C8..0x82608A34)
	// 826089C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826089CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826089D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826089D4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826089D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826089DC: 38EBE350  addi r7, r11, -0x1cb0
	ctx.r[7].s64 = ctx.r[11].s64 + -7344;
	// 826089E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826089E4: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826089E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826089EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826089F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826089F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826089F8: 386A5D14  addi r3, r10, 0x5d14
	ctx.r[3].s64 = ctx.r[10].s64 + 23828;
	// 826089FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608A1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608A20: 4BE5E401  bl 0x82466e20
	ctx.lr = 0x82608A24;
	sub_82466E20(ctx, base);
	// 82608A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608A38 size=108
    let mut pc: u32 = 0x82608A38;
    'dispatch: loop {
        match pc {
            0x82608A38 => {
    //   block [0x82608A38..0x82608AA4)
	// 82608A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608A44: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608A4C: 38EBE368  addi r7, r11, -0x1c98
	ctx.r[7].s64 = ctx.r[11].s64 + -7320;
	// 82608A50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608A54: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82608A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608A5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608A60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608A68: 386A5D44  addi r3, r10, 0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + 23876;
	// 82608A6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608A70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608A90: 4BE5E391  bl 0x82466e20
	ctx.lr = 0x82608A94;
	sub_82466E20(ctx, base);
	// 82608A94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608A98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608A9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608AA8 size=112
    let mut pc: u32 = 0x82608AA8;
    'dispatch: loop {
        match pc {
            0x82608AA8 => {
    //   block [0x82608AA8..0x82608B18)
	// 82608AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608AB4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608AB8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608ABC: 38AA5B94  addi r5, r10, 0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + 23444;
	// 82608AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608AC4: 390BE398  addi r8, r11, -0x1c68
	ctx.r[8].s64 = ctx.r[11].s64 + -7272;
	// 82608AC8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82608ACC: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 82608AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608AD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608AE0: 386A5D74  addi r3, r10, 0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + 23924;
	// 82608AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82608AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608B04: 4BE5E31D  bl 0x82466e20
	ctx.lr = 0x82608B08;
	sub_82466E20(ctx, base);
	// 82608B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82608B18 size=24
    let mut pc: u32 = 0x82608B18;
    'dispatch: loop {
        match pc {
            0x82608B18 => {
    //   block [0x82608B18..0x82608B30)
	// 82608B18: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608B1C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82608B20: 394A35D8  addi r10, r10, 0x35d8
	ctx.r[10].s64 = ctx.r[10].s64 + 13784;
	// 82608B24: 816BE274  lwz r11, -0x1d8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7564 as u32) ) } as u64;
	// 82608B28: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82608B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608B30 size=112
    let mut pc: u32 = 0x82608B30;
    'dispatch: loop {
        match pc {
            0x82608B30 => {
    //   block [0x82608B30..0x82608BA0)
	// 82608B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608B3C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608B40: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608B44: 392AD740  addi r9, r10, -0x28c0
	ctx.r[9].s64 = ctx.r[10].s64 + -10432;
	// 82608B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608B4C: 390B35D8  addi r8, r11, 0x35d8
	ctx.r[8].s64 = ctx.r[11].s64 + 13784;
	// 82608B50: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82608B54: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82608B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608B5C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608B64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608B68: 386A5DA4  addi r3, r10, 0x5da4
	ctx.r[3].s64 = ctx.r[10].s64 + 23972;
	// 82608B6C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608B70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608B84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608B8C: 4BE5E295  bl 0x82466e20
	ctx.lr = 0x82608B90;
	sub_82466E20(ctx, base);
	// 82608B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608BA0 size=108
    let mut pc: u32 = 0x82608BA0;
    'dispatch: loop {
        match pc {
            0x82608BA0 => {
    //   block [0x82608BA0..0x82608C0C)
	// 82608BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608BAC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608BB4: 38EBE444  addi r7, r11, -0x1bbc
	ctx.r[7].s64 = ctx.r[11].s64 + -7100;
	// 82608BB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608BBC: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82608BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608BC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608BC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608BD0: 386A5DD4  addi r3, r10, 0x5dd4
	ctx.r[3].s64 = ctx.r[10].s64 + 24020;
	// 82608BD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608BE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608BF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608BF8: 4BE5E229  bl 0x82466e20
	ctx.lr = 0x82608BFC;
	sub_82466E20(ctx, base);
	// 82608BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608C10 size=116
    let mut pc: u32 = 0x82608C10;
    'dispatch: loop {
        match pc {
            0x82608C10 => {
    //   block [0x82608C10..0x82608C84)
	// 82608C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608C1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608C20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608C24: 390BE478  addi r8, r11, -0x1b88
	ctx.r[8].s64 = ctx.r[11].s64 + -7048;
	// 82608C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608C2C: 392AD784  addi r9, r10, -0x287c
	ctx.r[9].s64 = ctx.r[10].s64 + -10364;
	// 82608C30: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608C34: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82608C38: 38AA5B94  addi r5, r10, 0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + 23444;
	// 82608C3C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608C44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608C4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608C54: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82608C58: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82608C5C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608C60: 386B5E04  addi r3, r11, 0x5e04
	ctx.r[3].s64 = ctx.r[11].s64 + 24068;
	// 82608C64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608C68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608C70: 4BE5E1B1  bl 0x82466e20
	ctx.lr = 0x82608C74;
	sub_82466E20(ctx, base);
	// 82608C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82608C88 size=24
    let mut pc: u32 = 0x82608C88;
    'dispatch: loop {
        match pc {
            0x82608C88 => {
    //   block [0x82608C88..0x82608CA0)
	// 82608C88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608C8C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82608C90: 394A3650  addi r10, r10, 0x3650
	ctx.r[10].s64 = ctx.r[10].s64 + 13904;
	// 82608C94: 816BE474  lwz r11, -0x1b8c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7052 as u32) ) } as u64;
	// 82608C98: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82608C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608CA0 size=112
    let mut pc: u32 = 0x82608CA0;
    'dispatch: loop {
        match pc {
            0x82608CA0 => {
    //   block [0x82608CA0..0x82608D10)
	// 82608CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608CAC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608CB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608CB4: 392AD7C0  addi r9, r10, -0x2840
	ctx.r[9].s64 = ctx.r[10].s64 + -10304;
	// 82608CB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608CBC: 390B3650  addi r8, r11, 0x3650
	ctx.r[8].s64 = ctx.r[11].s64 + 13904;
	// 82608CC0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82608CC4: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82608CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608CCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608CD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608CD8: 386A5E34  addi r3, r10, 0x5e34
	ctx.r[3].s64 = ctx.r[10].s64 + 24116;
	// 82608CDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608CE0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608CF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608CFC: 4BE5E125  bl 0x82466e20
	ctx.lr = 0x82608D00;
	sub_82466E20(ctx, base);
	// 82608D00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608D10 size=108
    let mut pc: u32 = 0x82608D10;
    'dispatch: loop {
        match pc {
            0x82608D10 => {
    //   block [0x82608D10..0x82608D7C)
	// 82608D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608D1C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608D20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608D24: 38EBE538  addi r7, r11, -0x1ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -6856;
	// 82608D28: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82608D2C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82608D30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608D34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608D38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608D3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608D40: 386A5E64  addi r3, r10, 0x5e64
	ctx.r[3].s64 = ctx.r[10].s64 + 24164;
	// 82608D44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608D48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608D4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608D50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608D58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608D60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608D64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608D68: 4BE5E0B9  bl 0x82466e20
	ctx.lr = 0x82608D6C;
	sub_82466E20(ctx, base);
	// 82608D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608D80 size=108
    let mut pc: u32 = 0x82608D80;
    'dispatch: loop {
        match pc {
            0x82608D80 => {
    //   block [0x82608D80..0x82608DEC)
	// 82608D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608D8C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608D94: 38EBE550  addi r7, r11, -0x1ab0
	ctx.r[7].s64 = ctx.r[11].s64 + -6832;
	// 82608D98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608D9C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82608DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608DA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608DA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608DB0: 386A5E94  addi r3, r10, 0x5e94
	ctx.r[3].s64 = ctx.r[10].s64 + 24212;
	// 82608DB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608DC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608DD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608DD8: 4BE5E049  bl 0x82466e20
	ctx.lr = 0x82608DDC;
	sub_82466E20(ctx, base);
	// 82608DDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608DE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608DE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82608DF0 size=24
    let mut pc: u32 = 0x82608DF0;
    'dispatch: loop {
        match pc {
            0x82608DF0 => {
    //   block [0x82608DF0..0x82608E08)
	// 82608DF0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608DF4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82608DF8: 394A3698  addi r10, r10, 0x3698
	ctx.r[10].s64 = ctx.r[10].s64 + 13976;
	// 82608DFC: 816BE580  lwz r11, -0x1a80(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6784 as u32) ) } as u64;
	// 82608E00: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82608E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608E08 size=112
    let mut pc: u32 = 0x82608E08;
    'dispatch: loop {
        match pc {
            0x82608E08 => {
    //   block [0x82608E08..0x82608E78)
	// 82608E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608E14: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82608E18: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608E1C: 392AD7FC  addi r9, r10, -0x2804
	ctx.r[9].s64 = ctx.r[10].s64 + -10244;
	// 82608E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608E24: 390B3698  addi r8, r11, 0x3698
	ctx.r[8].s64 = ctx.r[11].s64 + 13976;
	// 82608E28: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82608E2C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82608E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608E34: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608E38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608E3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608E40: 386A5EC4  addi r3, r10, 0x5ec4
	ctx.r[3].s64 = ctx.r[10].s64 + 24260;
	// 82608E44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82608E48: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82608E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608E5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608E64: 4BE5DFBD  bl 0x82466e20
	ctx.lr = 0x82608E68;
	sub_82466E20(ctx, base);
	// 82608E68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608E6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608E70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608E78 size=112
    let mut pc: u32 = 0x82608E78;
    'dispatch: loop {
        match pc {
            0x82608E78 => {
    //   block [0x82608E78..0x82608EE8)
	// 82608E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608E84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608E88: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608E8C: 38AA5B94  addi r5, r10, 0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + 23444;
	// 82608E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608E94: 390BE584  addi r8, r11, -0x1a7c
	ctx.r[8].s64 = ctx.r[11].s64 + -6780;
	// 82608E98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82608E9C: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82608EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608EA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82608EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608EB0: 386A5EF4  addi r3, r10, 0x5ef4
	ctx.r[3].s64 = ctx.r[10].s64 + 24308;
	// 82608EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82608EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608ED4: 4BE5DF4D  bl 0x82466e20
	ctx.lr = 0x82608ED8;
	sub_82466E20(ctx, base);
	// 82608ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608EE8 size=108
    let mut pc: u32 = 0x82608EE8;
    'dispatch: loop {
        match pc {
            0x82608EE8 => {
    //   block [0x82608EE8..0x82608F54)
	// 82608EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608EF4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608EFC: 38EBE5B4  addi r7, r11, -0x1a4c
	ctx.r[7].s64 = ctx.r[11].s64 + -6732;
	// 82608F00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608F04: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82608F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608F0C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608F10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608F14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608F18: 386A5F24  addi r3, r10, 0x5f24
	ctx.r[3].s64 = ctx.r[10].s64 + 24356;
	// 82608F1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608F20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608F3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608F40: 4BE5DEE1  bl 0x82466e20
	ctx.lr = 0x82608F44;
	sub_82466E20(ctx, base);
	// 82608F44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608F48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608F4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608F58 size=108
    let mut pc: u32 = 0x82608F58;
    'dispatch: loop {
        match pc {
            0x82608F58 => {
    //   block [0x82608F58..0x82608FC4)
	// 82608F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608F64: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608F68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608F6C: 38EBE5E8  addi r7, r11, -0x1a18
	ctx.r[7].s64 = ctx.r[11].s64 + -6680;
	// 82608F70: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82608F74: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82608F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608F7C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608F80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608F84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608F88: 386A5F54  addi r3, r10, 0x5f54
	ctx.r[3].s64 = ctx.r[10].s64 + 24404;
	// 82608F8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82608F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82608F94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82608F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82608F9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82608FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82608FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82608FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82608FAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82608FB0: 4BE5DE71  bl 0x82466e20
	ctx.lr = 0x82608FB4;
	sub_82466E20(ctx, base);
	// 82608FB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82608FB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82608FBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82608FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82608FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82608FC8 size=108
    let mut pc: u32 = 0x82608FC8;
    'dispatch: loop {
        match pc {
            0x82608FC8 => {
    //   block [0x82608FC8..0x82609034)
	// 82608FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82608FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82608FD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82608FD4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82608FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82608FDC: 38EBE648  addi r7, r11, -0x19b8
	ctx.r[7].s64 = ctx.r[11].s64 + -6584;
	// 82608FE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82608FE4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82608FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82608FEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82608FF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82608FF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82608FF8: 386A5F84  addi r3, r10, 0x5f84
	ctx.r[3].s64 = ctx.r[10].s64 + 24452;
	// 82608FFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260900C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260901C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609020: 4BE5DE01  bl 0x82466e20
	ctx.lr = 0x82609024;
	sub_82466E20(ctx, base);
	// 82609024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260902C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609038 size=108
    let mut pc: u32 = 0x82609038;
    'dispatch: loop {
        match pc {
            0x82609038 => {
    //   block [0x82609038..0x826090A4)
	// 82609038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260903C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609044: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260904C: 38EBE678  addi r7, r11, -0x1988
	ctx.r[7].s64 = ctx.r[11].s64 + -6536;
	// 82609050: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82609054: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82609058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260905C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609060: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609068: 386A5FB4  addi r3, r10, 0x5fb4
	ctx.r[3].s64 = ctx.r[10].s64 + 24500;
	// 8260906C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260907C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260908C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609090: 4BE5DD91  bl 0x82466e20
	ctx.lr = 0x82609094;
	sub_82466E20(ctx, base);
	// 82609094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609098: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260909C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826090A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826090A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826090A8 size=108
    let mut pc: u32 = 0x826090A8;
    'dispatch: loop {
        match pc {
            0x826090A8 => {
    //   block [0x826090A8..0x82609114)
	// 826090A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826090AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826090B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826090B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826090B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826090BC: 38EBE798  addi r7, r11, -0x1868
	ctx.r[7].s64 = ctx.r[11].s64 + -6248;
	// 826090C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826090C4: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 826090C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826090CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826090D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826090D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826090D8: 386A5FE4  addi r3, r10, 0x5fe4
	ctx.r[3].s64 = ctx.r[10].s64 + 24548;
	// 826090DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826090E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826090E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826090E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826090EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826090F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826090F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826090F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826090FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609100: 4BE5DD21  bl 0x82466e20
	ctx.lr = 0x82609104;
	sub_82466E20(ctx, base);
	// 82609104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260910C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609118 size=108
    let mut pc: u32 = 0x82609118;
    'dispatch: loop {
        match pc {
            0x82609118 => {
    //   block [0x82609118..0x82609184)
	// 82609118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260911C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609124: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260912C: 38EBE7B0  addi r7, r11, -0x1850
	ctx.r[7].s64 = ctx.r[11].s64 + -6224;
	// 82609130: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609134: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 82609138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260913C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609148: 386A6014  addi r3, r10, 0x6014
	ctx.r[3].s64 = ctx.r[10].s64 + 24596;
	// 8260914C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260915C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260916C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609170: 4BE5DCB1  bl 0x82466e20
	ctx.lr = 0x82609174;
	sub_82466E20(ctx, base);
	// 82609174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260917C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609188 size=108
    let mut pc: u32 = 0x82609188;
    'dispatch: loop {
        match pc {
            0x82609188 => {
    //   block [0x82609188..0x826091F4)
	// 82609188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260918C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609194: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260919C: 38EBE7C8  addi r7, r11, -0x1838
	ctx.r[7].s64 = ctx.r[11].s64 + -6200;
	// 826091A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826091A4: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 826091A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826091AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826091B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826091B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826091B8: 386A6044  addi r3, r10, 0x6044
	ctx.r[3].s64 = ctx.r[10].s64 + 24644;
	// 826091BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826091C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826091C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826091C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826091CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826091D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826091D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826091D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826091DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826091E0: 4BE5DC41  bl 0x82466e20
	ctx.lr = 0x826091E4;
	sub_82466E20(ctx, base);
	// 826091E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826091E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826091EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826091F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826091F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826091F8 size=108
    let mut pc: u32 = 0x826091F8;
    'dispatch: loop {
        match pc {
            0x826091F8 => {
    //   block [0x826091F8..0x82609264)
	// 826091F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826091FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609204: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260920C: 38EBE7E0  addi r7, r11, -0x1820
	ctx.r[7].s64 = ctx.r[11].s64 + -6176;
	// 82609210: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609214: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82609218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260921C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609220: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609228: 386A6074  addi r3, r10, 0x6074
	ctx.r[3].s64 = ctx.r[10].s64 + 24692;
	// 8260922C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260923C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260924C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609250: 4BE5DBD1  bl 0x82466e20
	ctx.lr = 0x82609254;
	sub_82466E20(ctx, base);
	// 82609254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260925C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609268 size=108
    let mut pc: u32 = 0x82609268;
    'dispatch: loop {
        match pc {
            0x82609268 => {
    //   block [0x82609268..0x826092D4)
	// 82609268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260926C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609274: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260927C: 38EBE7F8  addi r7, r11, -0x1808
	ctx.r[7].s64 = ctx.r[11].s64 + -6152;
	// 82609280: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609284: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 82609288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260928C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609298: 386A60A4  addi r3, r10, 0x60a4
	ctx.r[3].s64 = ctx.r[10].s64 + 24740;
	// 8260929C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826092A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826092A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826092A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826092AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826092B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826092B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826092B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826092BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826092C0: 4BE5DB61  bl 0x82466e20
	ctx.lr = 0x826092C4;
	sub_82466E20(ctx, base);
	// 826092C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826092C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826092CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826092D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826092D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826092D8 size=108
    let mut pc: u32 = 0x826092D8;
    'dispatch: loop {
        match pc {
            0x826092D8 => {
    //   block [0x826092D8..0x82609344)
	// 826092D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826092DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826092E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826092E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826092E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826092EC: 38EBE810  addi r7, r11, -0x17f0
	ctx.r[7].s64 = ctx.r[11].s64 + -6128;
	// 826092F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826092F4: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 826092F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826092FC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609300: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609304: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609308: 386A60D4  addi r3, r10, 0x60d4
	ctx.r[3].s64 = ctx.r[10].s64 + 24788;
	// 8260930C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260931C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260932C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609330: 4BE5DAF1  bl 0x82466e20
	ctx.lr = 0x82609334;
	sub_82466E20(ctx, base);
	// 82609334: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260933C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609348 size=108
    let mut pc: u32 = 0x82609348;
    'dispatch: loop {
        match pc {
            0x82609348 => {
    //   block [0x82609348..0x826093B4)
	// 82609348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260934C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609354: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260935C: 38EBE828  addi r7, r11, -0x17d8
	ctx.r[7].s64 = ctx.r[11].s64 + -6104;
	// 82609360: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82609364: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 82609368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260936C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609378: 386A6104  addi r3, r10, 0x6104
	ctx.r[3].s64 = ctx.r[10].s64 + 24836;
	// 8260937C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260938C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260939C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826093A0: 4BE5DA81  bl 0x82466e20
	ctx.lr = 0x826093A4;
	sub_82466E20(ctx, base);
	// 826093A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826093A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826093AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826093B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826093B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826093B8 size=108
    let mut pc: u32 = 0x826093B8;
    'dispatch: loop {
        match pc {
            0x826093B8 => {
    //   block [0x826093B8..0x82609424)
	// 826093B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826093BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826093C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826093C4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826093C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826093CC: 38EBE8B8  addi r7, r11, -0x1748
	ctx.r[7].s64 = ctx.r[11].s64 + -5960;
	// 826093D0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826093D4: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826093D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826093DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826093E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826093E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826093E8: 386A6134  addi r3, r10, 0x6134
	ctx.r[3].s64 = ctx.r[10].s64 + 24884;
	// 826093EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826093F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826093F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826093F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826093FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260940C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609410: 4BE5DA11  bl 0x82466e20
	ctx.lr = 0x82609414;
	sub_82466E20(ctx, base);
	// 82609414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260941C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609428 size=108
    let mut pc: u32 = 0x82609428;
    'dispatch: loop {
        match pc {
            0x82609428 => {
    //   block [0x82609428..0x82609494)
	// 82609428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260942C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609434: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260943C: 38EBE978  addi r7, r11, -0x1688
	ctx.r[7].s64 = ctx.r[11].s64 + -5768;
	// 82609440: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 82609444: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 82609448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260944C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609450: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609458: 386A6164  addi r3, r10, 0x6164
	ctx.r[3].s64 = ctx.r[10].s64 + 24932;
	// 8260945C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260946C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260947C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609480: 4BE5D9A1  bl 0x82466e20
	ctx.lr = 0x82609484;
	sub_82466E20(ctx, base);
	// 82609484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260948C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609498 size=108
    let mut pc: u32 = 0x82609498;
    'dispatch: loop {
        match pc {
            0x82609498 => {
    //   block [0x82609498..0x82609504)
	// 82609498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260949C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826094A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826094A4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826094A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826094AC: 38EBEA50  addi r7, r11, -0x15b0
	ctx.r[7].s64 = ctx.r[11].s64 + -5552;
	// 826094B0: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826094B4: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826094B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826094BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826094C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826094C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826094C8: 386A6194  addi r3, r10, 0x6194
	ctx.r[3].s64 = ctx.r[10].s64 + 24980;
	// 826094CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826094D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826094D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826094D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826094DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826094E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826094E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826094E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826094EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826094F0: 4BE5D931  bl 0x82466e20
	ctx.lr = 0x826094F4;
	sub_82466E20(ctx, base);
	// 826094F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826094F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826094FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609508 size=108
    let mut pc: u32 = 0x82609508;
    'dispatch: loop {
        match pc {
            0x82609508 => {
    //   block [0x82609508..0x82609574)
	// 82609508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260950C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609514: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260951C: 38EBEB10  addi r7, r11, -0x14f0
	ctx.r[7].s64 = ctx.r[11].s64 + -5360;
	// 82609520: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 82609524: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 82609528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260952C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609538: 386A61C4  addi r3, r10, 0x61c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25028;
	// 8260953C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260954C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260955C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609560: 4BE5D8C1  bl 0x82466e20
	ctx.lr = 0x82609564;
	sub_82466E20(ctx, base);
	// 82609564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260956C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609578 size=112
    let mut pc: u32 = 0x82609578;
    'dispatch: loop {
        match pc {
            0x82609578 => {
    //   block [0x82609578..0x826095E8)
	// 82609578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260957C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609584: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82609588: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8260958C: 38EAEBB8  addi r7, r10, -0x1448
	ctx.r[7].s64 = ctx.r[10].s64 + -5192;
	// 82609590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609594: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82609598: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8260959C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826095A0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826095A4: 396BD810  addi r11, r11, -0x27f0
	ctx.r[11].s64 = ctx.r[11].s64 + -10224;
	// 826095A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826095AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826095B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826095B4: 386A61F4  addi r3, r10, 0x61f4
	ctx.r[3].s64 = ctx.r[10].s64 + 25076;
	// 826095B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826095BC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826095C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826095C4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826095C8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826095CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826095D0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826095D4: 4BE5D84D  bl 0x82466e20
	ctx.lr = 0x826095D8;
	sub_82466E20(ctx, base);
	// 826095D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826095DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826095E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826095E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826095E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826095E8 size=108
    let mut pc: u32 = 0x826095E8;
    'dispatch: loop {
        match pc {
            0x826095E8 => {
    //   block [0x826095E8..0x82609654)
	// 826095E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826095EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826095F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826095F4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826095F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826095FC: 38EBECD8  addi r7, r11, -0x1328
	ctx.r[7].s64 = ctx.r[11].s64 + -4904;
	// 82609600: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82609604: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 82609608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260960C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609618: 386A6224  addi r3, r10, 0x6224
	ctx.r[3].s64 = ctx.r[10].s64 + 25124;
	// 8260961C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260962C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260963C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609640: 4BE5D7E1  bl 0x82466e20
	ctx.lr = 0x82609644;
	sub_82466E20(ctx, base);
	// 82609644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260964C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609658 size=108
    let mut pc: u32 = 0x82609658;
    'dispatch: loop {
        match pc {
            0x82609658 => {
    //   block [0x82609658..0x826096C4)
	// 82609658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260965C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609664: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260966C: 38EBED38  addi r7, r11, -0x12c8
	ctx.r[7].s64 = ctx.r[11].s64 + -4808;
	// 82609670: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 82609674: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 82609678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260967C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609688: 386A6254  addi r3, r10, 0x6254
	ctx.r[3].s64 = ctx.r[10].s64 + 25172;
	// 8260968C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260969C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826096A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826096A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826096A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826096AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826096B0: 4BE5D771  bl 0x82466e20
	ctx.lr = 0x826096B4;
	sub_82466E20(ctx, base);
	// 826096B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826096B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826096BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826096C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826096C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826096C8 size=108
    let mut pc: u32 = 0x826096C8;
    'dispatch: loop {
        match pc {
            0x826096C8 => {
    //   block [0x826096C8..0x82609734)
	// 826096C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826096CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826096D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826096D4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826096D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826096DC: 38EBEE40  addi r7, r11, -0x11c0
	ctx.r[7].s64 = ctx.r[11].s64 + -4544;
	// 826096E0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826096E4: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826096E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826096EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826096F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826096F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826096F8: 386A6284  addi r3, r10, 0x6284
	ctx.r[3].s64 = ctx.r[10].s64 + 25220;
	// 826096FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260970C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260971C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609720: 4BE5D701  bl 0x82466e20
	ctx.lr = 0x82609724;
	sub_82466E20(ctx, base);
	// 82609724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260972C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609738 size=108
    let mut pc: u32 = 0x82609738;
    'dispatch: loop {
        match pc {
            0x82609738 => {
    //   block [0x82609738..0x826097A4)
	// 82609738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260973C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609744: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260974C: 38EBEF18  addi r7, r11, -0x10e8
	ctx.r[7].s64 = ctx.r[11].s64 + -4328;
	// 82609750: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82609754: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 82609758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260975C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609768: 386A62B4  addi r3, r10, 0x62b4
	ctx.r[3].s64 = ctx.r[10].s64 + 25268;
	// 8260976C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260977C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260978C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609790: 4BE5D691  bl 0x82466e20
	ctx.lr = 0x82609794;
	sub_82466E20(ctx, base);
	// 82609794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260979C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826097A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826097A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826097A8 size=108
    let mut pc: u32 = 0x826097A8;
    'dispatch: loop {
        match pc {
            0x826097A8 => {
    //   block [0x826097A8..0x82609814)
	// 826097A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826097AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826097B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826097B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826097B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826097BC: 38EBEF60  addi r7, r11, -0x10a0
	ctx.r[7].s64 = ctx.r[11].s64 + -4256;
	// 826097C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826097C4: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826097C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826097CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826097D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826097D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826097D8: 386A62E4  addi r3, r10, 0x62e4
	ctx.r[3].s64 = ctx.r[10].s64 + 25316;
	// 826097DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826097E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826097E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826097E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826097EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826097F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826097F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826097F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826097FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609800: 4BE5D621  bl 0x82466e20
	ctx.lr = 0x82609804;
	sub_82466E20(ctx, base);
	// 82609804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260980C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609818 size=108
    let mut pc: u32 = 0x82609818;
    'dispatch: loop {
        match pc {
            0x82609818 => {
    //   block [0x82609818..0x82609884)
	// 82609818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260981C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609824: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260982C: 38EBEF78  addi r7, r11, -0x1088
	ctx.r[7].s64 = ctx.r[11].s64 + -4232;
	// 82609830: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82609834: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 82609838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260983C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609848: 386A6314  addi r3, r10, 0x6314
	ctx.r[3].s64 = ctx.r[10].s64 + 25364;
	// 8260984C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82609850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260985C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260986C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609870: 4BE5D5B1  bl 0x82466e20
	ctx.lr = 0x82609874;
	sub_82466E20(ctx, base);
	// 82609874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260987C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609888 size=108
    let mut pc: u32 = 0x82609888;
    'dispatch: loop {
        match pc {
            0x82609888 => {
    //   block [0x82609888..0x826098F4)
	// 82609888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260988C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609894: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260989C: 38EBEFC0  addi r7, r11, -0x1040
	ctx.r[7].s64 = ctx.r[11].s64 + -4160;
	// 826098A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826098A4: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826098A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826098AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826098B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826098B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826098B8: 386A6344  addi r3, r10, 0x6344
	ctx.r[3].s64 = ctx.r[10].s64 + 25412;
	// 826098BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826098C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826098C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826098C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826098CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826098D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826098D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826098D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826098DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826098E0: 4BE5D541  bl 0x82466e20
	ctx.lr = 0x826098E4;
	sub_82466E20(ctx, base);
	// 826098E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826098E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826098EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826098F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826098F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826098F8 size=112
    let mut pc: u32 = 0x826098F8;
    'dispatch: loop {
        match pc {
            0x826098F8 => {
    //   block [0x826098F8..0x82609968)
	// 826098F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826098FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609904: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609908: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260990C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82609910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609914: 390BEFD8  addi r8, r11, -0x1028
	ctx.r[8].s64 = ctx.r[11].s64 + -4136;
	// 82609918: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260991C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 82609920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609924: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260992C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609930: 386A6374  addi r3, r10, 0x6374
	ctx.r[3].s64 = ctx.r[10].s64 + 25460;
	// 82609934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260993C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609944: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260994C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609954: 4BE5D4CD  bl 0x82466e20
	ctx.lr = 0x82609958;
	sub_82466E20(ctx, base);
	// 82609958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260995C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609968 size=108
    let mut pc: u32 = 0x82609968;
    'dispatch: loop {
        match pc {
            0x82609968 => {
    //   block [0x82609968..0x826099D4)
	// 82609968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260996C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609974: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260997C: 38EBF020  addi r7, r11, -0xfe0
	ctx.r[7].s64 = ctx.r[11].s64 + -4064;
	// 82609980: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82609984: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 82609988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260998C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82609994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609998: 386A63A4  addi r3, r10, 0x63a4
	ctx.r[3].s64 = ctx.r[10].s64 + 25508;
	// 8260999C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826099A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826099A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826099A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826099AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826099B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826099B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826099B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826099BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826099C0: 4BE5D461  bl 0x82466e20
	ctx.lr = 0x826099C4;
	sub_82466E20(ctx, base);
	// 826099C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826099C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826099CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826099D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826099D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826099D8 size=116
    let mut pc: u32 = 0x826099D8;
    'dispatch: loop {
        match pc {
            0x826099D8 => {
    //   block [0x826099D8..0x82609A4C)
	// 826099D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826099DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826099E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826099E4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 826099E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826099EC: 390BF080  addi r8, r11, -0xf80
	ctx.r[8].s64 = ctx.r[11].s64 + -3968;
	// 826099F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826099F4: 392AD888  addi r9, r10, -0x2778
	ctx.r[9].s64 = ctx.r[10].s64 + -10104;
	// 826099F8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 826099FC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82609A00: 38AA63A4  addi r5, r10, 0x63a4
	ctx.r[5].s64 = ctx.r[10].s64 + 25508;
	// 82609A04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609A0C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609A1C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82609A20: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 82609A24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82609A28: 386B63D4  addi r3, r11, 0x63d4
	ctx.r[3].s64 = ctx.r[11].s64 + 25556;
	// 82609A2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82609A30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609A38: 4BE5D3E9  bl 0x82466e20
	ctx.lr = 0x82609A3C;
	sub_82466E20(ctx, base);
	// 82609A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609A50 size=96
    let mut pc: u32 = 0x82609A50;
    'dispatch: loop {
        match pc {
            0x82609A50 => {
    //   block [0x82609A50..0x82609AB0)
	// 82609A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609A5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609A64: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 82609A68: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609A70: 386A6404  addi r3, r10, 0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + 25604;
	// 82609A74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609A78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609A7C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609A80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609A88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609A90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609A94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609A98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609A9C: 4BE5D385  bl 0x82466e20
	ctx.lr = 0x82609AA0;
	sub_82466E20(ctx, base);
	// 82609AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609AB0 size=112
    let mut pc: u32 = 0x82609AB0;
    'dispatch: loop {
        match pc {
            0x82609AB0 => {
    //   block [0x82609AB0..0x82609B20)
	// 82609AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609ABC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 82609AC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609AC4: 38AA8414  addi r5, r10, -0x7bec
	ctx.r[5].s64 = ctx.r[10].s64 + -31724;
	// 82609AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609ACC: 390BF0F8  addi r8, r11, -0xf08
	ctx.r[8].s64 = ctx.r[11].s64 + -3848;
	// 82609AD0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82609AD4: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 82609AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609ADC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609AE8: 386A6434  addi r3, r10, 0x6434
	ctx.r[3].s64 = ctx.r[10].s64 + 25652;
	// 82609AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609B0C: 4BE5D315  bl 0x82466e20
	ctx.lr = 0x82609B10;
	sub_82466E20(ctx, base);
	// 82609B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609B20 size=96
    let mut pc: u32 = 0x82609B20;
    'dispatch: loop {
        match pc {
            0x82609B20 => {
    //   block [0x82609B20..0x82609B80)
	// 82609B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609B2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609B34: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 82609B38: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609B40: 386A6464  addi r3, r10, 0x6464
	ctx.r[3].s64 = ctx.r[10].s64 + 25700;
	// 82609B44: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609B4C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609B5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609B60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609B64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609B6C: 4BE5D2B5  bl 0x82466e20
	ctx.lr = 0x82609B70;
	sub_82466E20(ctx, base);
	// 82609B70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609B74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609B78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82609B80 size=24
    let mut pc: u32 = 0x82609B80;
    'dispatch: loop {
        match pc {
            0x82609B80 => {
    //   block [0x82609B80..0x82609B98)
	// 82609B80: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609B84: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 82609B88: 394A3710  addi r10, r10, 0x3710
	ctx.r[10].s64 = ctx.r[10].s64 + 14096;
	// 82609B8C: 816BF158  lwz r11, -0xea8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3752 as u32) ) } as u64;
	// 82609B90: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82609B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609B98 size=116
    let mut pc: u32 = 0x82609B98;
    'dispatch: loop {
        match pc {
            0x82609B98 => {
    //   block [0x82609B98..0x82609C0C)
	// 82609B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609BA4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609BA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82609BAC: 390B3710  addi r8, r11, 0x3710
	ctx.r[8].s64 = ctx.r[11].s64 + 14096;
	// 82609BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609BB4: 392AD8C4  addi r9, r10, -0x273c
	ctx.r[9].s64 = ctx.r[10].s64 + -10044;
	// 82609BB8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609BBC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82609BC0: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82609BC4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609BCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609BD4: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82609BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609BDC: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 82609BE0: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 82609BE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82609BE8: 386B6494  addi r3, r11, 0x6494
	ctx.r[3].s64 = ctx.r[11].s64 + 25748;
	// 82609BEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82609BF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609BF8: 4BE5D229  bl 0x82466e20
	ctx.lr = 0x82609BFC;
	sub_82466E20(ctx, base);
	// 82609BFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609C10 size=104
    let mut pc: u32 = 0x82609C10;
    'dispatch: loop {
        match pc {
            0x82609C10 => {
    //   block [0x82609C10..0x82609C78)
	// 82609C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609C1C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82609C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609C24: 392AD8F0  addi r9, r10, -0x2710
	ctx.r[9].s64 = ctx.r[10].s64 + -10000;
	// 82609C28: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609C2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609C30: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82609C34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609C38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609C40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609C44: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 82609C48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609C4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609C50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609C58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609C5C: 386A64C4  addi r3, r10, 0x64c4
	ctx.r[3].s64 = ctx.r[10].s64 + 25796;
	// 82609C60: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82609C64: 4BE5D1BD  bl 0x82466e20
	ctx.lr = 0x82609C68;
	sub_82466E20(ctx, base);
	// 82609C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609C78 size=96
    let mut pc: u32 = 0x82609C78;
    'dispatch: loop {
        match pc {
            0x82609C78 => {
    //   block [0x82609C78..0x82609CD8)
	// 82609C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609C84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609C8C: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 82609C90: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609C98: 386A64F4  addi r3, r10, 0x64f4
	ctx.r[3].s64 = ctx.r[10].s64 + 25844;
	// 82609C9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609CA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609CA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609CA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609CB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609CB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609CBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609CC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609CC4: 4BE5D15D  bl 0x82466e20
	ctx.lr = 0x82609CC8;
	sub_82466E20(ctx, base);
	// 82609CC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609CD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609CD8 size=100
    let mut pc: u32 = 0x82609CD8;
    'dispatch: loop {
        match pc {
            0x82609CD8 => {
    //   block [0x82609CD8..0x82609D3C)
	// 82609CD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609CDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609CE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609CE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609CEC: 38AA64C4  addi r5, r10, 0x64c4
	ctx.r[5].s64 = ctx.r[10].s64 + 25796;
	// 82609CF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609CF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609CF8: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 82609CFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609D04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609D0C: 386A6524  addi r3, r10, 0x6524
	ctx.r[3].s64 = ctx.r[10].s64 + 25892;
	// 82609D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609D14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609D18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609D20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609D24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609D28: 4BE5D0F9  bl 0x82466e20
	ctx.lr = 0x82609D2C;
	sub_82466E20(ctx, base);
	// 82609D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609D40 size=112
    let mut pc: u32 = 0x82609D40;
    'dispatch: loop {
        match pc {
            0x82609D40 => {
    //   block [0x82609D40..0x82609DB0)
	// 82609D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609D48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609D4C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609D50: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609D54: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 82609D58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609D5C: 390BF160  addi r8, r11, -0xea0
	ctx.r[8].s64 = ctx.r[11].s64 + -3744;
	// 82609D60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82609D64: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 82609D68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609D6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609D70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609D78: 386A6554  addi r3, r10, 0x6554
	ctx.r[3].s64 = ctx.r[10].s64 + 25940;
	// 82609D7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609D80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609D84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609D88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609D90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609D98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609D9C: 4BE5D085  bl 0x82466e20
	ctx.lr = 0x82609DA0;
	sub_82466E20(ctx, base);
	// 82609DA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609DA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609DA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609DAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609DB0 size=112
    let mut pc: u32 = 0x82609DB0;
    'dispatch: loop {
        match pc {
            0x82609DB0 => {
    //   block [0x82609DB0..0x82609E20)
	// 82609DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609DBC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609DC0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609DC4: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 82609DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609DCC: 390BF1A8  addi r8, r11, -0xe58
	ctx.r[8].s64 = ctx.r[11].s64 + -3672;
	// 82609DD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82609DD4: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 82609DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609DDC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609DE8: 386A6584  addi r3, r10, 0x6584
	ctx.r[3].s64 = ctx.r[10].s64 + 25988;
	// 82609DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609E0C: 4BE5D015  bl 0x82466e20
	ctx.lr = 0x82609E10;
	sub_82466E20(ctx, base);
	// 82609E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609E20 size=100
    let mut pc: u32 = 0x82609E20;
    'dispatch: loop {
        match pc {
            0x82609E20 => {
    //   block [0x82609E20..0x82609E84)
	// 82609E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609E2C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609E34: 38AA6494  addi r5, r10, 0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + 25748;
	// 82609E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609E40: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 82609E44: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609E54: 386A65B4  addi r3, r10, 0x65b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26036;
	// 82609E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609E5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609E60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609E68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609E70: 4BE5CFB1  bl 0x82466e20
	ctx.lr = 0x82609E74;
	sub_82466E20(ctx, base);
	// 82609E74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609E88 size=96
    let mut pc: u32 = 0x82609E88;
    'dispatch: loop {
        match pc {
            0x82609E88 => {
    //   block [0x82609E88..0x82609EE8)
	// 82609E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609E94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609E9C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 82609EA0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609EA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609EA8: 386A65E4  addi r3, r10, 0x65e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26084;
	// 82609EAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609EB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609EB4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609EB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609EC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609EC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609ECC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609ED0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609ED4: 4BE5CF4D  bl 0x82466e20
	ctx.lr = 0x82609ED8;
	sub_82466E20(ctx, base);
	// 82609ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609EE8 size=112
    let mut pc: u32 = 0x82609EE8;
    'dispatch: loop {
        match pc {
            0x82609EE8 => {
    //   block [0x82609EE8..0x82609F58)
	// 82609EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609EF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609EF8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609EFC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 82609F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609F04: 390BF1C0  addi r8, r11, -0xe40
	ctx.r[8].s64 = ctx.r[11].s64 + -3648;
	// 82609F08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82609F0C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 82609F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609F14: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609F18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609F20: 386A6614  addi r3, r10, 0x6614
	ctx.r[3].s64 = ctx.r[10].s64 + 26132;
	// 82609F24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609F34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82609F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609F44: 4BE5CEDD  bl 0x82466e20
	ctx.lr = 0x82609F48;
	sub_82466E20(ctx, base);
	// 82609F48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609F4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609F50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609F58 size=96
    let mut pc: u32 = 0x82609F58;
    'dispatch: loop {
        match pc {
            0x82609F58 => {
    //   block [0x82609F58..0x82609FB8)
	// 82609F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609F60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609F64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609F6C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 82609F70: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609F74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82609F78: 386A6644  addi r3, r10, 0x6644
	ctx.r[3].s64 = ctx.r[10].s64 + 26180;
	// 82609F7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82609F84: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82609F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82609F8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82609F94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82609F98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82609F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82609FA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82609FA4: 4BE5CE7D  bl 0x82466e20
	ctx.lr = 0x82609FA8;
	sub_82466E20(ctx, base);
	// 82609FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82609FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82609FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82609FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82609FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82609FB8 size=112
    let mut pc: u32 = 0x82609FB8;
    'dispatch: loop {
        match pc {
            0x82609FB8 => {
    //   block [0x82609FB8..0x8260A028)
	// 82609FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82609FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82609FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82609FC4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609FC8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 82609FCC: 38AA6644  addi r5, r10, 0x6644
	ctx.r[5].s64 = ctx.r[10].s64 + 26180;
	// 82609FD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82609FD4: 390BF1F0  addi r8, r11, -0xe10
	ctx.r[8].s64 = ctx.r[11].s64 + -3600;
	// 82609FD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82609FDC: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 82609FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82609FE4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 82609FE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82609FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82609FF0: 386A6674  addi r3, r10, 0x6674
	ctx.r[3].s64 = ctx.r[10].s64 + 26228;
	// 82609FF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82609FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82609FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A00C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A014: 4BE5CE0D  bl 0x82466e20
	ctx.lr = 0x8260A018;
	sub_82466E20(ctx, base);
	// 8260A018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A028 size=108
    let mut pc: u32 = 0x8260A028;
    'dispatch: loop {
        match pc {
            0x8260A028 => {
    //   block [0x8260A028..0x8260A094)
	// 8260A028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A034: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A03C: 38EBF208  addi r7, r11, -0xdf8
	ctx.r[7].s64 = ctx.r[11].s64 + -3576;
	// 8260A040: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260A044: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8260A048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A04C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260A054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A058: 386A66A4  addi r3, r10, 0x66a4
	ctx.r[3].s64 = ctx.r[10].s64 + 26276;
	// 8260A05C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260A060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A07C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260A080: 4BE5CDA1  bl 0x82466e20
	ctx.lr = 0x8260A084;
	sub_82466E20(ctx, base);
	// 8260A084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A098 size=112
    let mut pc: u32 = 0x8260A098;
    'dispatch: loop {
        match pc {
            0x8260A098 => {
    //   block [0x8260A098..0x8260A108)
	// 8260A098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A0A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A0A8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A0AC: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A0B4: 390BF268  addi r8, r11, -0xd98
	ctx.r[8].s64 = ctx.r[11].s64 + -3480;
	// 8260A0B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260A0BC: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8260A0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A0C4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A0C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A0CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A0D0: 386A66D4  addi r3, r10, 0x66d4
	ctx.r[3].s64 = ctx.r[10].s64 + 26324;
	// 8260A0D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A0F4: 4BE5CD2D  bl 0x82466e20
	ctx.lr = 0x8260A0F8;
	sub_82466E20(ctx, base);
	// 8260A0F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A0FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A100: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A104: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A108 size=112
    let mut pc: u32 = 0x8260A108;
    'dispatch: loop {
        match pc {
            0x8260A108 => {
    //   block [0x8260A108..0x8260A178)
	// 8260A108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A10C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A114: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A118: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A11C: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260A120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A124: 390BF280  addi r8, r11, -0xd80
	ctx.r[8].s64 = ctx.r[11].s64 + -3456;
	// 8260A128: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260A12C: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8260A130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A134: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A13C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A140: 386A6704  addi r3, r10, 0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + 26372;
	// 8260A144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A164: 4BE5CCBD  bl 0x82466e20
	ctx.lr = 0x8260A168;
	sub_82466E20(ctx, base);
	// 8260A168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A178 size=100
    let mut pc: u32 = 0x8260A178;
    'dispatch: loop {
        match pc {
            0x8260A178 => {
    //   block [0x8260A178..0x8260A1DC)
	// 8260A178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A184: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A18C: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260A190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A198: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8260A19C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A1A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A1A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A1A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A1AC: 386A6734  addi r3, r10, 0x6734
	ctx.r[3].s64 = ctx.r[10].s64 + 26420;
	// 8260A1B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A1B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A1B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260A1BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A1C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A1C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A1C8: 4BE5CC59  bl 0x82466e20
	ctx.lr = 0x8260A1CC;
	sub_82466E20(ctx, base);
	// 8260A1CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A1D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A1E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A1E0 size=116
    let mut pc: u32 = 0x8260A1E0;
    'dispatch: loop {
        match pc {
            0x8260A1E0 => {
    //   block [0x8260A1E0..0x8260A254)
	// 8260A1E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A1E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A1E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A1EC: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A1F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260A1F4: 390BF2B4  addi r8, r11, -0xd4c
	ctx.r[8].s64 = ctx.r[11].s64 + -3404;
	// 8260A1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A1FC: 392AD91C  addi r9, r10, -0x26e4
	ctx.r[9].s64 = ctx.r[10].s64 + -9956;
	// 8260A200: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A204: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8260A208: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A20C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A214: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A21C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A224: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260A228: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8260A22C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260A230: 386B6764  addi r3, r11, 0x6764
	ctx.r[3].s64 = ctx.r[11].s64 + 26468;
	// 8260A234: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260A238: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A240: 4BE5CBE1  bl 0x82466e20
	ctx.lr = 0x8260A244;
	sub_82466E20(ctx, base);
	// 8260A244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A24C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A258 size=112
    let mut pc: u32 = 0x8260A258;
    'dispatch: loop {
        match pc {
            0x8260A258 => {
    //   block [0x8260A258..0x8260A2C8)
	// 8260A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A264: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A268: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A26C: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260A270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A274: 390BF2E4  addi r8, r11, -0xd1c
	ctx.r[8].s64 = ctx.r[11].s64 + -3356;
	// 8260A278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260A27C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8260A280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A284: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A290: 386A6794  addi r3, r10, 0x6794
	ctx.r[3].s64 = ctx.r[10].s64 + 26516;
	// 8260A294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A2A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260A2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A2B4: 4BE5CB6D  bl 0x82466e20
	ctx.lr = 0x8260A2B8;
	sub_82466E20(ctx, base);
	// 8260A2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A2C8 size=116
    let mut pc: u32 = 0x8260A2C8;
    'dispatch: loop {
        match pc {
            0x8260A2C8 => {
    //   block [0x8260A2C8..0x8260A33C)
	// 8260A2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A2D4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A2D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260A2DC: 390BF300  addi r8, r11, -0xd00
	ctx.r[8].s64 = ctx.r[11].s64 + -3328;
	// 8260A2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A2E4: 392AD948  addi r9, r10, -0x26b8
	ctx.r[9].s64 = ctx.r[10].s64 + -9912;
	// 8260A2E8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A2EC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260A2F0: 38AA6DF4  addi r5, r10, 0x6df4
	ctx.r[5].s64 = ctx.r[10].s64 + 28148;
	// 8260A2F4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A2FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A30C: 3D608293  lis r11, -0x7d6d
	ctx.r[11].s64 = -2104295424;
	// 8260A310: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8260A314: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260A318: 386B67C4  addi r3, r11, 0x67c4
	ctx.r[3].s64 = ctx.r[11].s64 + 26564;
	// 8260A31C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260A320: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A328: 4BE5CAF9  bl 0x82466e20
	ctx.lr = 0x8260A32C;
	sub_82466E20(ctx, base);
	// 8260A32C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A340 size=112
    let mut pc: u32 = 0x8260A340;
    'dispatch: loop {
        match pc {
            0x8260A340 => {
    //   block [0x8260A340..0x8260A3B0)
	// 8260A340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A34C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A350: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A354: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A35C: 390BF318  addi r8, r11, -0xce8
	ctx.r[8].s64 = ctx.r[11].s64 + -3304;
	// 8260A360: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260A364: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8260A368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A36C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A378: 386A67F4  addi r3, r10, 0x67f4
	ctx.r[3].s64 = ctx.r[10].s64 + 26612;
	// 8260A37C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A38C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260A390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A39C: 4BE5CA85  bl 0x82466e20
	ctx.lr = 0x8260A3A0;
	sub_82466E20(ctx, base);
	// 8260A3A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A3A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A3A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A3B0 size=112
    let mut pc: u32 = 0x8260A3B0;
    'dispatch: loop {
        match pc {
            0x8260A3B0 => {
    //   block [0x8260A3B0..0x8260A420)
	// 8260A3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A3BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A3C0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A3C4: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260A3C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A3CC: 390BF390  addi r8, r11, -0xc70
	ctx.r[8].s64 = ctx.r[11].s64 + -3184;
	// 8260A3D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260A3D4: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8260A3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A3DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A3E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A3E8: 386A6824  addi r3, r10, 0x6824
	ctx.r[3].s64 = ctx.r[10].s64 + 26660;
	// 8260A3EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A40C: 4BE5CA15  bl 0x82466e20
	ctx.lr = 0x8260A410;
	sub_82466E20(ctx, base);
	// 8260A410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A420 size=112
    let mut pc: u32 = 0x8260A420;
    'dispatch: loop {
        match pc {
            0x8260A420 => {
    //   block [0x8260A420..0x8260A490)
	// 8260A420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A42C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A430: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A434: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A43C: 390BF3D8  addi r8, r11, -0xc28
	ctx.r[8].s64 = ctx.r[11].s64 + -3112;
	// 8260A440: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260A444: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8260A448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A44C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A458: 386A6854  addi r3, r10, 0x6854
	ctx.r[3].s64 = ctx.r[10].s64 + 26708;
	// 8260A45C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A47C: 4BE5C9A5  bl 0x82466e20
	ctx.lr = 0x8260A480;
	sub_82466E20(ctx, base);
	// 8260A480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A48C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A490 size=112
    let mut pc: u32 = 0x8260A490;
    'dispatch: loop {
        match pc {
            0x8260A490 => {
    //   block [0x8260A490..0x8260A500)
	// 8260A490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A49C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A4A0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A4A4: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A4A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A4AC: 390BF420  addi r8, r11, -0xbe0
	ctx.r[8].s64 = ctx.r[11].s64 + -3040;
	// 8260A4B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260A4B4: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8260A4B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A4BC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A4C8: 386A6884  addi r3, r10, 0x6884
	ctx.r[3].s64 = ctx.r[10].s64 + 26756;
	// 8260A4CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A4D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A4D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A4E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A4E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A4EC: 4BE5C935  bl 0x82466e20
	ctx.lr = 0x8260A4F0;
	sub_82466E20(ctx, base);
	// 8260A4F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A4F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A4F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A4FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A500 size=108
    let mut pc: u32 = 0x8260A500;
    'dispatch: loop {
        match pc {
            0x8260A500 => {
    //   block [0x8260A500..0x8260A56C)
	// 8260A500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A50C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A514: 38EBF468  addi r7, r11, -0xb98
	ctx.r[7].s64 = ctx.r[11].s64 + -2968;
	// 8260A518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260A51C: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8260A520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A524: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260A52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A530: 386A68B4  addi r3, r10, 0x68b4
	ctx.r[3].s64 = ctx.r[10].s64 + 26804;
	// 8260A534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260A538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260A558: 4BE5C8C9  bl 0x82466e20
	ctx.lr = 0x8260A55C;
	sub_82466E20(ctx, base);
	// 8260A55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A570 size=112
    let mut pc: u32 = 0x8260A570;
    'dispatch: loop {
        match pc {
            0x8260A570 => {
    //   block [0x8260A570..0x8260A5E0)
	// 8260A570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A57C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A580: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A584: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A58C: 390BF4B0  addi r8, r11, -0xb50
	ctx.r[8].s64 = ctx.r[11].s64 + -2896;
	// 8260A590: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8260A594: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8260A598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A59C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A5A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A5A8: 386A68E4  addi r3, r10, 0x68e4
	ctx.r[3].s64 = ctx.r[10].s64 + 26852;
	// 8260A5AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A5B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A5B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A5C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A5C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A5CC: 4BE5C855  bl 0x82466e20
	ctx.lr = 0x8260A5D0;
	sub_82466E20(ctx, base);
	// 8260A5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A5E0 size=116
    let mut pc: u32 = 0x8260A5E0;
    'dispatch: loop {
        match pc {
            0x8260A5E0 => {
    //   block [0x8260A5E0..0x8260A654)
	// 8260A5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A5EC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260A5F0: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A5F4: 392BD984  addi r9, r11, -0x267c
	ctx.r[9].s64 = ctx.r[11].s64 + -9852;
	// 8260A5F8: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260A5FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A600: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8260A604: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8260A608: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A60C: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8260A610: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A614: 396BF530  addi r11, r11, -0xad0
	ctx.r[11].s64 = ctx.r[11].s64 + -2768;
	// 8260A618: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260A61C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A620: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260A624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A628: 386A6914  addi r3, r10, 0x6914
	ctx.r[3].s64 = ctx.r[10].s64 + 26900;
	// 8260A62C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8260A630: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260A634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A638: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260A63C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A640: 4BE5C7E1  bl 0x82466e20
	ctx.lr = 0x8260A644;
	sub_82466E20(ctx, base);
	// 8260A644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260A658 size=36
    let mut pc: u32 = 0x8260A658;
    'dispatch: loop {
        match pc {
            0x8260A658 => {
    //   block [0x8260A658..0x8260A67C)
	// 8260A658: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A65C: 814BF5C4  lwz r10, -0xa3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2620 as u32) ) } as u64;
	// 8260A660: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A664: 396B3740  addi r11, r11, 0x3740
	ctx.r[11].s64 = ctx.r[11].s64 + 14144;
	// 8260A668: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8260A66C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260A670: 814AF52C  lwz r10, -0xad4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2772 as u32) ) } as u64;
	// 8260A674: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8260A678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A680 size=108
    let mut pc: u32 = 0x8260A680;
    'dispatch: loop {
        match pc {
            0x8260A680 => {
    //   block [0x8260A680..0x8260A6EC)
	// 8260A680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A68C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A694: 38EB3740  addi r7, r11, 0x3740
	ctx.r[7].s64 = ctx.r[11].s64 + 14144;
	// 8260A698: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8260A69C: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 8260A6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A6A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A6A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260A6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A6B0: 386A6944  addi r3, r10, 0x6944
	ctx.r[3].s64 = ctx.r[10].s64 + 26948;
	// 8260A6B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260A6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A6D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260A6D8: 4BE5C749  bl 0x82466e20
	ctx.lr = 0x8260A6DC;
	sub_82466E20(ctx, base);
	// 8260A6DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260A6F0 size=24
    let mut pc: u32 = 0x8260A6F0;
    'dispatch: loop {
        match pc {
            0x8260A6F0 => {
    //   block [0x8260A6F0..0x8260A708)
	// 8260A6F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A6F4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260A6F8: 394A37E8  addi r10, r10, 0x37e8
	ctx.r[10].s64 = ctx.r[10].s64 + 14312;
	// 8260A6FC: 816BF52C  lwz r11, -0xad4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2772 as u32) ) } as u64;
	// 8260A700: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8260A704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A708 size=116
    let mut pc: u32 = 0x8260A708;
    'dispatch: loop {
        match pc {
            0x8260A708 => {
    //   block [0x8260A708..0x8260A77C)
	// 8260A708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A714: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260A718: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8260A71C: 390A37E8  addi r8, r10, 0x37e8
	ctx.r[8].s64 = ctx.r[10].s64 + 14312;
	// 8260A720: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A724: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260A728: 38AA6944  addi r5, r10, 0x6944
	ctx.r[5].s64 = ctx.r[10].s64 + 26948;
	// 8260A72C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A730: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260A734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A738: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A73C: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 8260A740: 396BDA40  addi r11, r11, -0x25c0
	ctx.r[11].s64 = ctx.r[11].s64 + -9664;
	// 8260A744: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A748: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A74C: 386A6974  addi r3, r10, 0x6974
	ctx.r[3].s64 = ctx.r[10].s64 + 26996;
	// 8260A750: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260A754: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A758: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260A75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A768: 4BE5C6B9  bl 0x82466e20
	ctx.lr = 0x8260A76C;
	sub_82466E20(ctx, base);
	// 8260A76C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A780 size=112
    let mut pc: u32 = 0x8260A780;
    'dispatch: loop {
        match pc {
            0x8260A780 => {
    //   block [0x8260A780..0x8260A7F0)
	// 8260A780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A78C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A790: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A794: 38AA6944  addi r5, r10, 0x6944
	ctx.r[5].s64 = ctx.r[10].s64 + 26948;
	// 8260A798: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A79C: 390BF5C8  addi r8, r11, -0xa38
	ctx.r[8].s64 = ctx.r[11].s64 + -2616;
	// 8260A7A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260A7A4: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 8260A7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A7AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A7B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A7B8: 386A69A4  addi r3, r10, 0x69a4
	ctx.r[3].s64 = ctx.r[10].s64 + 27044;
	// 8260A7BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260A7C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A7DC: 4BE5C645  bl 0x82466e20
	ctx.lr = 0x8260A7E0;
	sub_82466E20(ctx, base);
	// 8260A7E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260A7F0 size=24
    let mut pc: u32 = 0x8260A7F0;
    'dispatch: loop {
        match pc {
            0x8260A7F0 => {
    //   block [0x8260A7F0..0x8260A808)
	// 8260A7F0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A7F4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260A7F8: 394A38A8  addi r10, r10, 0x38a8
	ctx.r[10].s64 = ctx.r[10].s64 + 14504;
	// 8260A7FC: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260A800: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8260A804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A808 size=116
    let mut pc: u32 = 0x8260A808;
    'dispatch: loop {
        match pc {
            0x8260A808 => {
    //   block [0x8260A808..0x8260A87C)
	// 8260A808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A814: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260A818: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A81C: 392BDA04  addi r9, r11, -0x25fc
	ctx.r[9].s64 = ctx.r[11].s64 + -9724;
	// 8260A820: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260A824: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A828: 38E90060  addi r7, r9, 0x60
	ctx.r[7].s64 = ctx.r[9].s64 + 96;
	// 8260A82C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8260A830: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A834: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 8260A838: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A83C: 396B38A8  addi r11, r11, 0x38a8
	ctx.r[11].s64 = ctx.r[11].s64 + 14504;
	// 8260A840: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260A844: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A848: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260A84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A850: 386A69D4  addi r3, r10, 0x69d4
	ctx.r[3].s64 = ctx.r[10].s64 + 27092;
	// 8260A854: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8260A858: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260A85C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A860: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260A864: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A868: 4BE5C5B9  bl 0x82466e20
	ctx.lr = 0x8260A86C;
	sub_82466E20(ctx, base);
	// 8260A86C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A880 size=100
    let mut pc: u32 = 0x8260A880;
    'dispatch: loop {
        match pc {
            0x8260A880 => {
    //   block [0x8260A880..0x8260A8E4)
	// 8260A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A88C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A894: 38AA6B24  addi r5, r10, 0x6b24
	ctx.r[5].s64 = ctx.r[10].s64 + 27428;
	// 8260A898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A89C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A8A0: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8260A8A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A8B4: 386A6A04  addi r3, r10, 0x6a04
	ctx.r[3].s64 = ctx.r[10].s64 + 27140;
	// 8260A8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A8BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A8C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260A8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A8C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A8D0: 4BE5C551  bl 0x82466e20
	ctx.lr = 0x8260A8D4;
	sub_82466E20(ctx, base);
	// 8260A8D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A8D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A8DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A8E8 size=100
    let mut pc: u32 = 0x8260A8E8;
    'dispatch: loop {
        match pc {
            0x8260A8E8 => {
    //   block [0x8260A8E8..0x8260A94C)
	// 8260A8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A8F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A8FC: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260A900: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A908: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8260A90C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A91C: 386A6A34  addi r3, r10, 0x6a34
	ctx.r[3].s64 = ctx.r[10].s64 + 27188;
	// 8260A920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A924: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A928: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260A92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A930: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260A934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A938: 4BE5C4E9  bl 0x82466e20
	ctx.lr = 0x8260A93C;
	sub_82466E20(ctx, base);
	// 8260A93C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A940: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A944: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A950 size=108
    let mut pc: u32 = 0x8260A950;
    'dispatch: loop {
        match pc {
            0x8260A950 => {
    //   block [0x8260A950..0x8260A9BC)
	// 8260A950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A95C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A964: 38EBF628  addi r7, r11, -0x9d8
	ctx.r[7].s64 = ctx.r[11].s64 + -2520;
	// 8260A968: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260A96C: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8260A970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A974: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A978: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260A97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260A980: 386A6A64  addi r3, r10, 0x6a64
	ctx.r[3].s64 = ctx.r[10].s64 + 27236;
	// 8260A984: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260A988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260A98C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260A990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260A994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260A99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260A9A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260A9A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260A9A8: 4BE5C479  bl 0x82466e20
	ctx.lr = 0x8260A9AC;
	sub_82466E20(ctx, base);
	// 8260A9AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260A9B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260A9B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260A9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260A9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260A9C0 size=112
    let mut pc: u32 = 0x8260A9C0;
    'dispatch: loop {
        match pc {
            0x8260A9C0 => {
    //   block [0x8260A9C0..0x8260AA30)
	// 8260A9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260A9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260A9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260A9CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A9D0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260A9D4: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260A9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260A9DC: 390BF688  addi r8, r11, -0x978
	ctx.r[8].s64 = ctx.r[11].s64 + -2424;
	// 8260A9E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260A9E4: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8260A9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260A9EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260A9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260A9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260A9F8: 386A6A94  addi r3, r10, 0x6a94
	ctx.r[3].s64 = ctx.r[10].s64 + 27284;
	// 8260A9FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AA1C: 4BE5C405  bl 0x82466e20
	ctx.lr = 0x8260AA20;
	sub_82466E20(ctx, base);
	// 8260AA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AA30 size=108
    let mut pc: u32 = 0x8260AA30;
    'dispatch: loop {
        match pc {
            0x8260AA30 => {
    //   block [0x8260AA30..0x8260AA9C)
	// 8260AA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AA3C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AA40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AA44: 38EBF6D0  addi r7, r11, -0x930
	ctx.r[7].s64 = ctx.r[11].s64 + -2352;
	// 8260AA48: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260AA4C: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8260AA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AA54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AA58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260AA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AA60: 386A6AC4  addi r3, r10, 0x6ac4
	ctx.r[3].s64 = ctx.r[10].s64 + 27332;
	// 8260AA64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260AA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AA84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260AA88: 4BE5C399  bl 0x82466e20
	ctx.lr = 0x8260AA8C;
	sub_82466E20(ctx, base);
	// 8260AA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260AAA0 size=28
    let mut pc: u32 = 0x8260AAA0;
    'dispatch: loop {
        match pc {
            0x8260AAA0 => {
    //   block [0x8260AAA0..0x8260AABC)
	// 8260AAA0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AAA4: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AAA8: 394A3950  addi r10, r10, 0x3950
	ctx.r[10].s64 = ctx.r[10].s64 + 14672;
	// 8260AAAC: 816BF6E8  lwz r11, -0x918(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2328 as u32) ) } as u64;
	// 8260AAB0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8260AAB4: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8260AAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AAC0 size=112
    let mut pc: u32 = 0x8260AAC0;
    'dispatch: loop {
        match pc {
            0x8260AAC0 => {
    //   block [0x8260AAC0..0x8260AB30)
	// 8260AAC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AAC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AAC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AACC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AAD0: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8260AAD4: 38EA3950  addi r7, r10, 0x3950
	ctx.r[7].s64 = ctx.r[10].s64 + 14672;
	// 8260AAD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AADC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260AAE0: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8260AAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AAE8: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260AAEC: 396BDAEC  addi r11, r11, -0x2514
	ctx.r[11].s64 = ctx.r[11].s64 + -9492;
	// 8260AAF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260AAF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AAF8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AAFC: 386A6AF4  addi r3, r10, 0x6af4
	ctx.r[3].s64 = ctx.r[10].s64 + 27380;
	// 8260AB00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AB04: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260AB08: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AB0C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260AB10: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AB14: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AB18: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260AB1C: 4BE5C305  bl 0x82466e20
	ctx.lr = 0x8260AB20;
	sub_82466E20(ctx, base);
	// 8260AB20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AB24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AB28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260AB30 size=24
    let mut pc: u32 = 0x8260AB30;
    'dispatch: loop {
        match pc {
            0x8260AB30 => {
    //   block [0x8260AB30..0x8260AB48)
	// 8260AB30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AB34: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AB38: 394A3AA0  addi r10, r10, 0x3aa0
	ctx.r[10].s64 = ctx.r[10].s64 + 15008;
	// 8260AB3C: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260AB40: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8260AB44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AB48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AB48 size=116
    let mut pc: u32 = 0x8260AB48;
    'dispatch: loop {
        match pc {
            0x8260AB48 => {
    //   block [0x8260AB48..0x8260ABBC)
	// 8260AB48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AB4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AB50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AB54: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260AB58: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AB5C: 392BDAC4  addi r9, r11, -0x253c
	ctx.r[9].s64 = ctx.r[11].s64 + -9532;
	// 8260AB60: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260AB64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AB68: 38E90064  addi r7, r9, 0x64
	ctx.r[7].s64 = ctx.r[9].s64 + 100;
	// 8260AB6C: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8260AB70: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AB74: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8260AB78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AB7C: 396B3AA0  addi r11, r11, 0x3aa0
	ctx.r[11].s64 = ctx.r[11].s64 + 15008;
	// 8260AB80: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8260AB84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AB88: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8260AB8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AB90: 386A6B24  addi r3, r10, 0x6b24
	ctx.r[3].s64 = ctx.r[10].s64 + 27428;
	// 8260AB94: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8260AB98: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8260AB9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ABA0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8260ABA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260ABA8: 4BE5C279  bl 0x82466e20
	ctx.lr = 0x8260ABAC;
	sub_82466E20(ctx, base);
	// 8260ABAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260ABB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260ABB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260ABB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ABC0 size=112
    let mut pc: u32 = 0x8260ABC0;
    'dispatch: loop {
        match pc {
            0x8260ABC0 => {
    //   block [0x8260ABC0..0x8260AC30)
	// 8260ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ABC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ABC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ABCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ABD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260ABD4: 38AA6734  addi r5, r10, 0x6734
	ctx.r[5].s64 = ctx.r[10].s64 + 26420;
	// 8260ABD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ABDC: 390BF6F0  addi r8, r11, -0x910
	ctx.r[8].s64 = ctx.r[11].s64 + -2320;
	// 8260ABE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260ABE4: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8260ABE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260ABEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ABF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260ABF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ABF8: 386A6B54  addi r3, r10, 0x6b54
	ctx.r[3].s64 = ctx.r[10].s64 + 27476;
	// 8260ABFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AC1C: 4BE5C205  bl 0x82466e20
	ctx.lr = 0x8260AC20;
	sub_82466E20(ctx, base);
	// 8260AC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260AC30 size=24
    let mut pc: u32 = 0x8260AC30;
    'dispatch: loop {
        match pc {
            0x8260AC30 => {
    //   block [0x8260AC30..0x8260AC48)
	// 8260AC30: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AC34: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AC38: 394A3B48  addi r10, r10, 0x3b48
	ctx.r[10].s64 = ctx.r[10].s64 + 15176;
	// 8260AC3C: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260AC40: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 8260AC44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AC48 size=116
    let mut pc: u32 = 0x8260AC48;
    'dispatch: loop {
        match pc {
            0x8260AC48 => {
    //   block [0x8260AC48..0x8260ACBC)
	// 8260AC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AC50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AC54: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260AC58: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8260AC5C: 390A3B48  addi r8, r10, 0x3b48
	ctx.r[8].s64 = ctx.r[10].s64 + 15176;
	// 8260AC60: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AC64: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260AC68: 38AA6734  addi r5, r10, 0x6734
	ctx.r[5].s64 = ctx.r[10].s64 + 26420;
	// 8260AC6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AC70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260AC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AC78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260AC7C: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 8260AC80: 396BDB48  addi r11, r11, -0x24b8
	ctx.r[11].s64 = ctx.r[11].s64 + -9400;
	// 8260AC84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AC88: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260AC8C: 386A6B84  addi r3, r10, 0x6b84
	ctx.r[3].s64 = ctx.r[10].s64 + 27524;
	// 8260AC90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260AC94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AC98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260AC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ACA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260ACA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ACA8: 4BE5C179  bl 0x82466e20
	ctx.lr = 0x8260ACAC;
	sub_82466E20(ctx, base);
	// 8260ACAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260ACB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260ACB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260ACB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ACC0 size=112
    let mut pc: u32 = 0x8260ACC0;
    'dispatch: loop {
        match pc {
            0x8260ACC0 => {
    //   block [0x8260ACC0..0x8260AD30)
	// 8260ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ACC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ACCC: 3D408294  lis r10, -0x7d6c
	ctx.r[10].s64 = -2104229888;
	// 8260ACD0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260ACD4: 38AA8504  addi r5, r10, -0x7afc
	ctx.r[5].s64 = ctx.r[10].s64 + -31484;
	// 8260ACD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ACDC: 390BF720  addi r8, r11, -0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + -2272;
	// 8260ACE0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260ACE4: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 8260ACE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260ACEC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ACF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260ACF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ACF8: 386A6BB4  addi r3, r10, 0x6bb4
	ctx.r[3].s64 = ctx.r[10].s64 + 27572;
	// 8260ACFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AD1C: 4BE5C105  bl 0x82466e20
	ctx.lr = 0x8260AD20;
	sub_82466E20(ctx, base);
	// 8260AD20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AD30 size=108
    let mut pc: u32 = 0x8260AD30;
    'dispatch: loop {
        match pc {
            0x8260AD30 => {
    //   block [0x8260AD30..0x8260AD9C)
	// 8260AD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AD3C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AD40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AD44: 38EBF750  addi r7, r11, -0x8b0
	ctx.r[7].s64 = ctx.r[11].s64 + -2224;
	// 8260AD48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8260AD4C: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8260AD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AD54: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AD58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260AD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AD60: 386A6BE4  addi r3, r10, 0x6be4
	ctx.r[3].s64 = ctx.r[10].s64 + 27620;
	// 8260AD64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260AD68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AD70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AD78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AD80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AD84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260AD88: 4BE5C099  bl 0x82466e20
	ctx.lr = 0x8260AD8C;
	sub_82466E20(ctx, base);
	// 8260AD8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AD90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AD94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260ADA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260ADA0 size=112
    let mut pc: u32 = 0x8260ADA0;
    'dispatch: loop {
        match pc {
            0x8260ADA0 => {
    //   block [0x8260ADA0..0x8260AE10)
	// 8260ADA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260ADA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260ADA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260ADAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ADB0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260ADB4: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260ADB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260ADBC: 390BF780  addi r8, r11, -0x880
	ctx.r[8].s64 = ctx.r[11].s64 + -2176;
	// 8260ADC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260ADC4: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8260ADC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260ADCC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260ADD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260ADD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260ADD8: 386A6C14  addi r3, r10, 0x6c14
	ctx.r[3].s64 = ctx.r[10].s64 + 27668;
	// 8260ADDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260ADE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260ADE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260ADE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260ADEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260ADF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260ADF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260ADF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260ADFC: 4BE5C025  bl 0x82466e20
	ctx.lr = 0x8260AE00;
	sub_82466E20(ctx, base);
	// 8260AE00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AE04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AE08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AE0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AE10 size=112
    let mut pc: u32 = 0x8260AE10;
    'dispatch: loop {
        match pc {
            0x8260AE10 => {
    //   block [0x8260AE10..0x8260AE80)
	// 8260AE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AE1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AE20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AE24: 38AA6DF4  addi r5, r10, 0x6df4
	ctx.r[5].s64 = ctx.r[10].s64 + 28148;
	// 8260AE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AE2C: 390BF7B0  addi r8, r11, -0x850
	ctx.r[8].s64 = ctx.r[11].s64 + -2128;
	// 8260AE30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260AE34: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8260AE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AE3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AE40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260AE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AE48: 386A6C44  addi r3, r10, 0x6c44
	ctx.r[3].s64 = ctx.r[10].s64 + 27716;
	// 8260AE4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AE54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AE5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AE6C: 4BE5BFB5  bl 0x82466e20
	ctx.lr = 0x8260AE70;
	sub_82466E20(ctx, base);
	// 8260AE70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AE74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AE78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AE7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AE80 size=108
    let mut pc: u32 = 0x8260AE80;
    'dispatch: loop {
        match pc {
            0x8260AE80 => {
    //   block [0x8260AE80..0x8260AEEC)
	// 8260AE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AE88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AE8C: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AE94: 38EBF7E0  addi r7, r11, -0x820
	ctx.r[7].s64 = ctx.r[11].s64 + -2080;
	// 8260AE98: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8260AE9C: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 8260AEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AEA4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AEA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260AEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AEB0: 386A6C74  addi r3, r10, 0x6c74
	ctx.r[3].s64 = ctx.r[10].s64 + 27764;
	// 8260AEB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260AEB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260AED8: 4BE5BF49  bl 0x82466e20
	ctx.lr = 0x8260AEDC;
	sub_82466E20(ctx, base);
	// 8260AEDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AEF0 size=112
    let mut pc: u32 = 0x8260AEF0;
    'dispatch: loop {
        match pc {
            0x8260AEF0 => {
    //   block [0x8260AEF0..0x8260AF60)
	// 8260AEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AEF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AEFC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AF00: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AF04: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260AF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AF0C: 390BF828  addi r8, r11, -0x7d8
	ctx.r[8].s64 = ctx.r[11].s64 + -2008;
	// 8260AF10: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260AF14: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 8260AF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AF1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AF20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260AF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AF28: 386A6CA4  addi r3, r10, 0x6ca4
	ctx.r[3].s64 = ctx.r[10].s64 + 27812;
	// 8260AF2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260AF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AF3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AF4C: 4BE5BED5  bl 0x82466e20
	ctx.lr = 0x8260AF50;
	sub_82466E20(ctx, base);
	// 8260AF50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AF60 size=100
    let mut pc: u32 = 0x8260AF60;
    'dispatch: loop {
        match pc {
            0x8260AF60 => {
    //   block [0x8260AF60..0x8260AFC4)
	// 8260AF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AF6C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AF74: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260AF78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AF7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260AF80: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8260AF84: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AF88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260AF8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260AF90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260AF94: 386A6CD4  addi r3, r10, 0x6cd4
	ctx.r[3].s64 = ctx.r[10].s64 + 27860;
	// 8260AF98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260AF9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260AFA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260AFA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260AFA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260AFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260AFB0: 4BE5BE71  bl 0x82466e20
	ctx.lr = 0x8260AFB4;
	sub_82466E20(ctx, base);
	// 8260AFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260AFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260AFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260AFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260AFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260AFC8 size=112
    let mut pc: u32 = 0x8260AFC8;
    'dispatch: loop {
        match pc {
            0x8260AFC8 => {
    //   block [0x8260AFC8..0x8260B038)
	// 8260AFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260AFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260AFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260AFD4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AFD8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260AFDC: 38AA6A34  addi r5, r10, 0x6a34
	ctx.r[5].s64 = ctx.r[10].s64 + 27188;
	// 8260AFE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260AFE4: 390BF888  addi r8, r11, -0x778
	ctx.r[8].s64 = ctx.r[11].s64 + -1912;
	// 8260AFE8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260AFEC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8260AFF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260AFF4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260AFF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260AFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B000: 386A6D04  addi r3, r10, 0x6d04
	ctx.r[3].s64 = ctx.r[10].s64 + 27908;
	// 8260B004: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B024: 4BE5BDFD  bl 0x82466e20
	ctx.lr = 0x8260B028;
	sub_82466E20(ctx, base);
	// 8260B028: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B02C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B030: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B038 size=112
    let mut pc: u32 = 0x8260B038;
    'dispatch: loop {
        match pc {
            0x8260B038 => {
    //   block [0x8260B038..0x8260B0A8)
	// 8260B038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B044: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B048: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B04C: 38AA6A34  addi r5, r10, 0x6a34
	ctx.r[5].s64 = ctx.r[10].s64 + 27188;
	// 8260B050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B054: 390BF8D0  addi r8, r11, -0x730
	ctx.r[8].s64 = ctx.r[11].s64 + -1840;
	// 8260B058: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8260B05C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8260B060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B064: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B070: 386A6D34  addi r3, r10, 0x6d34
	ctx.r[3].s64 = ctx.r[10].s64 + 27956;
	// 8260B074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B094: 4BE5BD8D  bl 0x82466e20
	ctx.lr = 0x8260B098;
	sub_82466E20(ctx, base);
	// 8260B098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B0A8 size=108
    let mut pc: u32 = 0x8260B0A8;
    'dispatch: loop {
        match pc {
            0x8260B0A8 => {
    //   block [0x8260B0A8..0x8260B114)
	// 8260B0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B0B4: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B0BC: 38EBF978  addi r7, r11, -0x688
	ctx.r[7].s64 = ctx.r[11].s64 + -1672;
	// 8260B0C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8260B0C4: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8260B0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B0CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B0D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260B0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B0D8: 386A6D64  addi r3, r10, 0x6d64
	ctx.r[3].s64 = ctx.r[10].s64 + 28004;
	// 8260B0DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260B0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B0FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260B100: 4BE5BD21  bl 0x82466e20
	ctx.lr = 0x8260B104;
	sub_82466E20(ctx, base);
	// 8260B104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260B118 size=24
    let mut pc: u32 = 0x8260B118;
    'dispatch: loop {
        match pc {
            0x8260B118 => {
    //   block [0x8260B118..0x8260B130)
	// 8260B118: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B11C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B120: 394A3C80  addi r10, r10, 0x3c80
	ctx.r[10].s64 = ctx.r[10].s64 + 15488;
	// 8260B124: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260B128: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8260B12C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B130 size=116
    let mut pc: u32 = 0x8260B130;
    'dispatch: loop {
        match pc {
            0x8260B130 => {
    //   block [0x8260B130..0x8260B1A4)
	// 8260B130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B13C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B140: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8260B144: 390A3C80  addi r8, r10, 0x3c80
	ctx.r[8].s64 = ctx.r[10].s64 + 15488;
	// 8260B148: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B14C: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260B150: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260B154: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B158: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260B15C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B160: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B164: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8260B168: 396BDB80  addi r11, r11, -0x2480
	ctx.r[11].s64 = ctx.r[11].s64 + -9344;
	// 8260B16C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B170: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B174: 386A6D94  addi r3, r10, 0x6d94
	ctx.r[3].s64 = ctx.r[10].s64 + 28052;
	// 8260B178: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260B17C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B180: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260B184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B190: 4BE5BC91  bl 0x82466e20
	ctx.lr = 0x8260B194;
	sub_82466E20(ctx, base);
	// 8260B194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B1A8 size=100
    let mut pc: u32 = 0x8260B1A8;
    'dispatch: loop {
        match pc {
            0x8260B1A8 => {
    //   block [0x8260B1A8..0x8260B20C)
	// 8260B1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B1B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B1BC: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260B1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B1C8: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8260B1CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B1D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B1DC: 386A6DC4  addi r3, r10, 0x6dc4
	ctx.r[3].s64 = ctx.r[10].s64 + 28100;
	// 8260B1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B1E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B1E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260B1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B1F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260B1F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B1F8: 4BE5BC29  bl 0x82466e20
	ctx.lr = 0x8260B1FC;
	sub_82466E20(ctx, base);
	// 8260B1FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B210 size=100
    let mut pc: u32 = 0x8260B210;
    'dispatch: loop {
        match pc {
            0x8260B210 => {
    //   block [0x8260B210..0x8260B274)
	// 8260B210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B218: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B21C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B224: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260B228: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B230: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8260B234: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B244: 386A6DF4  addi r3, r10, 0x6df4
	ctx.r[3].s64 = ctx.r[10].s64 + 28148;
	// 8260B248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B24C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B250: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260B254: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B258: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260B25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B260: 4BE5BBC1  bl 0x82466e20
	ctx.lr = 0x8260B264;
	sub_82466E20(ctx, base);
	// 8260B264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B278 size=112
    let mut pc: u32 = 0x8260B278;
    'dispatch: loop {
        match pc {
            0x8260B278 => {
    //   block [0x8260B278..0x8260B2E8)
	// 8260B278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B284: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B288: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B28C: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260B290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B294: 390BF9D8  addi r8, r11, -0x628
	ctx.r[8].s64 = ctx.r[11].s64 + -1576;
	// 8260B298: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260B29C: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 8260B2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B2A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B2A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B2B0: 386A6E24  addi r3, r10, 0x6e24
	ctx.r[3].s64 = ctx.r[10].s64 + 28196;
	// 8260B2B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B2B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B2C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B2C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B2D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B2D4: 4BE5BB4D  bl 0x82466e20
	ctx.lr = 0x8260B2D8;
	sub_82466E20(ctx, base);
	// 8260B2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B2E8 size=112
    let mut pc: u32 = 0x8260B2E8;
    'dispatch: loop {
        match pc {
            0x8260B2E8 => {
    //   block [0x8260B2E8..0x8260B358)
	// 8260B2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B2F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B2F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B2FC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260B300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B304: 390BFA68  addi r8, r11, -0x598
	ctx.r[8].s64 = ctx.r[11].s64 + -1432;
	// 8260B308: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8260B30C: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 8260B310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B314: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B31C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B320: 386A6E54  addi r3, r10, 0x6e54
	ctx.r[3].s64 = ctx.r[10].s64 + 28244;
	// 8260B324: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B328: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B32C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B33C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B344: 4BE5BADD  bl 0x82466e20
	ctx.lr = 0x8260B348;
	sub_82466E20(ctx, base);
	// 8260B348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B34C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B358 size=112
    let mut pc: u32 = 0x8260B358;
    'dispatch: loop {
        match pc {
            0x8260B358 => {
    //   block [0x8260B358..0x8260B3C8)
	// 8260B358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B364: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B368: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B36C: 38AA69D4  addi r5, r10, 0x69d4
	ctx.r[5].s64 = ctx.r[10].s64 + 27092;
	// 8260B370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B374: 390BFAC8  addi r8, r11, -0x538
	ctx.r[8].s64 = ctx.r[11].s64 + -1336;
	// 8260B378: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B37C: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 8260B380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B384: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B390: 386A6E84  addi r3, r10, 0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + 28292;
	// 8260B394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B3B4: 4BE5BA6D  bl 0x82466e20
	ctx.lr = 0x8260B3B8;
	sub_82466E20(ctx, base);
	// 8260B3B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B3C8 size=112
    let mut pc: u32 = 0x8260B3C8;
    'dispatch: loop {
        match pc {
            0x8260B3C8 => {
    //   block [0x8260B3C8..0x8260B438)
	// 8260B3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B3D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B3D8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B3DC: 38AA5774  addi r5, r10, 0x5774
	ctx.r[5].s64 = ctx.r[10].s64 + 22388;
	// 8260B3E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B3E4: 390BFAF8  addi r8, r11, -0x508
	ctx.r[8].s64 = ctx.r[11].s64 + -1288;
	// 8260B3E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8260B3EC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8260B3F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B3F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B3F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B3FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B400: 386A6EB4  addi r3, r10, 0x6eb4
	ctx.r[3].s64 = ctx.r[10].s64 + 28340;
	// 8260B404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B40C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B41C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B424: 4BE5B9FD  bl 0x82466e20
	ctx.lr = 0x8260B428;
	sub_82466E20(ctx, base);
	// 8260B428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B42C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B438 size=112
    let mut pc: u32 = 0x8260B438;
    'dispatch: loop {
        match pc {
            0x8260B438 => {
    //   block [0x8260B438..0x8260B4A8)
	// 8260B438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B444: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B448: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B44C: 38AA6B24  addi r5, r10, 0x6b24
	ctx.r[5].s64 = ctx.r[10].s64 + 27428;
	// 8260B450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B454: 390BFB88  addi r8, r11, -0x478
	ctx.r[8].s64 = ctx.r[11].s64 + -1144;
	// 8260B458: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260B45C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8260B460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B464: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B470: 386A6EE4  addi r3, r10, 0x6ee4
	ctx.r[3].s64 = ctx.r[10].s64 + 28388;
	// 8260B474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B47C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B48C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B494: 4BE5B98D  bl 0x82466e20
	ctx.lr = 0x8260B498;
	sub_82466E20(ctx, base);
	// 8260B498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B4A8 size=112
    let mut pc: u32 = 0x8260B4A8;
    'dispatch: loop {
        match pc {
            0x8260B4A8 => {
    //   block [0x8260B4A8..0x8260B518)
	// 8260B4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B4B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B4B8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B4BC: 38AA6D34  addi r5, r10, 0x6d34
	ctx.r[5].s64 = ctx.r[10].s64 + 27956;
	// 8260B4C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B4C4: 390BFBA0  addi r8, r11, -0x460
	ctx.r[8].s64 = ctx.r[11].s64 + -1120;
	// 8260B4C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B4CC: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8260B4D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B4D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B4D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B4DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B4E0: 386A6F14  addi r3, r10, 0x6f14
	ctx.r[3].s64 = ctx.r[10].s64 + 28436;
	// 8260B4E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B4E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B4EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B4F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B4F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B4FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B504: 4BE5B91D  bl 0x82466e20
	ctx.lr = 0x8260B508;
	sub_82466E20(ctx, base);
	// 8260B508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B50C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B518 size=112
    let mut pc: u32 = 0x8260B518;
    'dispatch: loop {
        match pc {
            0x8260B518 => {
    //   block [0x8260B518..0x8260B588)
	// 8260B518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B524: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B528: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B52C: 38AA6614  addi r5, r10, 0x6614
	ctx.r[5].s64 = ctx.r[10].s64 + 26132;
	// 8260B530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B534: 390BFBD0  addi r8, r11, -0x430
	ctx.r[8].s64 = ctx.r[11].s64 + -1072;
	// 8260B538: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8260B53C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8260B540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B544: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B550: 386A6F44  addi r3, r10, 0x6f44
	ctx.r[3].s64 = ctx.r[10].s64 + 28484;
	// 8260B554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B55C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B56C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B574: 4BE5B8AD  bl 0x82466e20
	ctx.lr = 0x8260B578;
	sub_82466E20(ctx, base);
	// 8260B578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B57C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260B588 size=24
    let mut pc: u32 = 0x8260B588;
    'dispatch: loop {
        match pc {
            0x8260B588 => {
    //   block [0x8260B588..0x8260B5A0)
	// 8260B588: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B58C: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B590: 394A3CF8  addi r10, r10, 0x3cf8
	ctx.r[10].s64 = ctx.r[10].s64 + 15608;
	// 8260B594: 816BFC78  lwz r11, -0x388(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-904 as u32) ) } as u64;
	// 8260B598: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8260B59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B5A0 size=116
    let mut pc: u32 = 0x8260B5A0;
    'dispatch: loop {
        match pc {
            0x8260B5A0 => {
    //   block [0x8260B5A0..0x8260B614)
	// 8260B5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B5AC: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B5B0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8260B5B4: 390A3CF8  addi r8, r10, 0x3cf8
	ctx.r[8].s64 = ctx.r[10].s64 + 15608;
	// 8260B5B8: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B5BC: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 8260B5C0: 38AA67C4  addi r5, r10, 0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + 26564;
	// 8260B5C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B5C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260B5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B5D4: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8260B5D8: 396BDB98  addi r11, r11, -0x2468
	ctx.r[11].s64 = ctx.r[11].s64 + -9320;
	// 8260B5DC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B5E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B5E4: 386A6F74  addi r3, r10, 0x6f74
	ctx.r[3].s64 = ctx.r[10].s64 + 28532;
	// 8260B5E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8260B5EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B5F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8260B5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B600: 4BE5B821  bl 0x82466e20
	ctx.lr = 0x8260B604;
	sub_82466E20(ctx, base);
	// 8260B604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B618 size=112
    let mut pc: u32 = 0x8260B618;
    'dispatch: loop {
        match pc {
            0x8260B618 => {
    //   block [0x8260B618..0x8260B688)
	// 8260B618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B624: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B628: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B62C: 38AA6734  addi r5, r10, 0x6734
	ctx.r[5].s64 = ctx.r[10].s64 + 26420;
	// 8260B630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B634: 390BFC18  addi r8, r11, -0x3e8
	ctx.r[8].s64 = ctx.r[11].s64 + -1000;
	// 8260B638: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B63C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8260B640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B644: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B650: 386A6FA4  addi r3, r10, 0x6fa4
	ctx.r[3].s64 = ctx.r[10].s64 + 28580;
	// 8260B654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B674: 4BE5B7AD  bl 0x82466e20
	ctx.lr = 0x8260B678;
	sub_82466E20(ctx, base);
	// 8260B678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B67C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B688 size=112
    let mut pc: u32 = 0x8260B688;
    'dispatch: loop {
        match pc {
            0x8260B688 => {
    //   block [0x8260B688..0x8260B6F8)
	// 8260B688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B694: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B698: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B69C: 38AA6794  addi r5, r10, 0x6794
	ctx.r[5].s64 = ctx.r[10].s64 + 26516;
	// 8260B6A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B6A4: 390BFC48  addi r8, r11, -0x3b8
	ctx.r[8].s64 = ctx.r[11].s64 + -952;
	// 8260B6A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B6AC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8260B6B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B6B4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B6B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B6C0: 386A6FD4  addi r3, r10, 0x6fd4
	ctx.r[3].s64 = ctx.r[10].s64 + 28628;
	// 8260B6C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B6C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B6D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B6D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B6E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B6E4: 4BE5B73D  bl 0x82466e20
	ctx.lr = 0x8260B6E8;
	sub_82466E20(ctx, base);
	// 8260B6E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B6F8 size=100
    let mut pc: u32 = 0x8260B6F8;
    'dispatch: loop {
        match pc {
            0x8260B6F8 => {
    //   block [0x8260B6F8..0x8260B75C)
	// 8260B6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B6FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B704: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260B708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B70C: 392ADC08  addi r9, r10, -0x23f8
	ctx.r[9].s64 = ctx.r[10].s64 + -9208;
	// 8260B710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B718: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 8260B71C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B720: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B728: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B72C: 386A7004  addi r3, r10, 0x7004
	ctx.r[3].s64 = ctx.r[10].s64 + 28676;
	// 8260B730: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B734: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8260B738: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260B73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B740: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260B744: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260B748: 4BE5B6D9  bl 0x82466e20
	ctx.lr = 0x8260B74C;
	sub_82466E20(ctx, base);
	// 8260B74C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8260B760 size=24
    let mut pc: u32 = 0x8260B760;
    'dispatch: loop {
        match pc {
            0x8260B760 => {
    //   block [0x8260B760..0x8260B778)
	// 8260B760: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B764: 3D408275  lis r10, -0x7d8b
	ctx.r[10].s64 = -2106261504;
	// 8260B768: 394A3DA0  addi r10, r10, 0x3da0
	ctx.r[10].s64 = ctx.r[10].s64 + 15776;
	// 8260B76C: 816BFC84  lwz r11, -0x37c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-892 as u32) ) } as u64;
	// 8260B770: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8260B774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B778 size=112
    let mut pc: u32 = 0x8260B778;
    'dispatch: loop {
        match pc {
            0x8260B778 => {
    //   block [0x8260B778..0x8260B7E8)
	// 8260B778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B784: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8260B788: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B78C: 392ADD48  addi r9, r10, -0x22b8
	ctx.r[9].s64 = ctx.r[10].s64 + -8888;
	// 8260B790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B794: 390B3DA0  addi r8, r11, 0x3da0
	ctx.r[8].s64 = ctx.r[11].s64 + 15776;
	// 8260B798: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8260B79C: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 8260B7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B7A4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B7A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B7B0: 386A7034  addi r3, r10, 0x7034
	ctx.r[3].s64 = ctx.r[10].s64 + 28724;
	// 8260B7B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8260B7B8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8260B7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B7CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260B7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B7D4: 4BE5B64D  bl 0x82466e20
	ctx.lr = 0x8260B7D8;
	sub_82466E20(ctx, base);
	// 8260B7D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B7E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B7E8 size=112
    let mut pc: u32 = 0x8260B7E8;
    'dispatch: loop {
        match pc {
            0x8260B7E8 => {
    //   block [0x8260B7E8..0x8260B858)
	// 8260B7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B7F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B7F4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B7F8: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B7FC: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260B800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B804: 390BFC8C  addi r8, r11, -0x374
	ctx.r[8].s64 = ctx.r[11].s64 + -884;
	// 8260B808: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B80C: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 8260B810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B814: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B81C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B820: 386A7064  addi r3, r10, 0x7064
	ctx.r[3].s64 = ctx.r[10].s64 + 28772;
	// 8260B824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B82C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B844: 4BE5B5DD  bl 0x82466e20
	ctx.lr = 0x8260B848;
	sub_82466E20(ctx, base);
	// 8260B848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B84C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B858 size=108
    let mut pc: u32 = 0x8260B858;
    'dispatch: loop {
        match pc {
            0x8260B858 => {
    //   block [0x8260B858..0x8260B8C4)
	// 8260B858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B864: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B86C: 38EBFCBC  addi r7, r11, -0x344
	ctx.r[7].s64 = ctx.r[11].s64 + -836;
	// 8260B870: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8260B874: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 8260B878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B87C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B880: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8260B884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B888: 386A7094  addi r3, r10, 0x7094
	ctx.r[3].s64 = ctx.r[10].s64 + 28820;
	// 8260B88C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8260B890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B8AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8260B8B0: 4BE5B571  bl 0x82466e20
	ctx.lr = 0x8260B8B4;
	sub_82466E20(ctx, base);
	// 8260B8B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B8B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B8BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B8C8 size=100
    let mut pc: u32 = 0x8260B8C8;
    'dispatch: loop {
        match pc {
            0x8260B8C8 => {
    //   block [0x8260B8C8..0x8260B92C)
	// 8260B8C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B8CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B8D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B8D4: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B8DC: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260B8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B8E8: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 8260B8EC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B8F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B8FC: 386A70C4  addi r3, r10, 0x70c4
	ctx.r[3].s64 = ctx.r[10].s64 + 28868;
	// 8260B900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B904: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B908: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8260B90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B910: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8260B914: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B918: 4BE5B509  bl 0x82466e20
	ctx.lr = 0x8260B91C;
	sub_82466E20(ctx, base);
	// 8260B91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B930 size=112
    let mut pc: u32 = 0x8260B930;
    'dispatch: loop {
        match pc {
            0x8260B930 => {
    //   block [0x8260B930..0x8260B9A0)
	// 8260B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B93C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B940: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B944: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260B948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B94C: 390BFCD4  addi r8, r11, -0x32c
	ctx.r[8].s64 = ctx.r[11].s64 + -812;
	// 8260B950: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8260B954: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 8260B958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B95C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B968: 386A70F4  addi r3, r10, 0x70f4
	ctx.r[3].s64 = ctx.r[10].s64 + 28916;
	// 8260B96C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B98C: 4BE5B495  bl 0x82466e20
	ctx.lr = 0x8260B990;
	sub_82466E20(ctx, base);
	// 8260B990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260B994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260B998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260B99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260B9A0 size=112
    let mut pc: u32 = 0x8260B9A0;
    'dispatch: loop {
        match pc {
            0x8260B9A0 => {
    //   block [0x8260B9A0..0x8260BA10)
	// 8260B9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260B9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260B9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260B9AC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B9B0: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260B9B4: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260B9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260B9BC: 390BFCEC  addi r8, r11, -0x314
	ctx.r[8].s64 = ctx.r[11].s64 + -788;
	// 8260B9C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260B9C4: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 8260B9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260B9CC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260B9D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260B9D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260B9D8: 386A7124  addi r3, r10, 0x7124
	ctx.r[3].s64 = ctx.r[10].s64 + 28964;
	// 8260B9DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260B9E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260B9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260B9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260B9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260B9FC: 4BE5B425  bl 0x82466e20
	ctx.lr = 0x8260BA00;
	sub_82466E20(ctx, base);
	// 8260BA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BA10 size=112
    let mut pc: u32 = 0x8260BA10;
    'dispatch: loop {
        match pc {
            0x8260BA10 => {
    //   block [0x8260BA10..0x8260BA80)
	// 8260BA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BA1C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BA20: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BA24: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BA2C: 390BFD1C  addi r8, r11, -0x2e4
	ctx.r[8].s64 = ctx.r[11].s64 + -740;
	// 8260BA30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260BA34: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 8260BA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BA3C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BA48: 386A7154  addi r3, r10, 0x7154
	ctx.r[3].s64 = ctx.r[10].s64 + 29012;
	// 8260BA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BA6C: 4BE5B3B5  bl 0x82466e20
	ctx.lr = 0x8260BA70;
	sub_82466E20(ctx, base);
	// 8260BA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8260BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8260BA80 size=112
    let mut pc: u32 = 0x8260BA80;
    'dispatch: loop {
        match pc {
            0x8260BA80 => {
    //   block [0x8260BA80..0x8260BAF0)
	// 8260BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8260BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8260BA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8260BA8C: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BA90: 3D608275  lis r11, -0x7d8b
	ctx.r[11].s64 = -2106261504;
	// 8260BA94: 38AA7034  addi r5, r10, 0x7034
	ctx.r[5].s64 = ctx.r[10].s64 + 28724;
	// 8260BA98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8260BA9C: 390BFD4C  addi r8, r11, -0x2b4
	ctx.r[8].s64 = ctx.r[11].s64 + -692;
	// 8260BAA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8260BAA4: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 8260BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8260BAAC: 3D408293  lis r10, -0x7d6d
	ctx.r[10].s64 = -2104295424;
	// 8260BAB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8260BAB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8260BAB8: 386A7184  addi r3, r10, 0x7184
	ctx.r[3].s64 = ctx.r[10].s64 + 29060;
	// 8260BABC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8260BAC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8260BAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8260BAC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8260BACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8260BAD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8260BAD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8260BAD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8260BADC: 4BE5B345  bl 0x82466e20
	ctx.lr = 0x8260BAE0;
	sub_82466E20(ctx, base);
	// 8260BAE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8260BAE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8260BAE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8260BAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


