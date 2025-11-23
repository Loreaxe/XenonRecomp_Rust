pub fn sub_82659368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659368 size=116
    let mut pc: u32 = 0x82659368;
    'dispatch: loop {
        match pc {
            0x82659368 => {
    //   block [0x82659368..0x826593DC)
	// 82659368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265936C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659374: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659378: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265937C: 392BD410  addi r9, r11, -0x2bf0
	ctx.r[9].s64 = ctx.r[11].s64 + -11248;
	// 82659380: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 82659384: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659388: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 8265938C: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 82659390: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659394: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 82659398: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265939C: 396B1E00  addi r11, r11, 0x1e00
	ctx.r[11].s64 = ctx.r[11].s64 + 7680;
	// 826593A0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826593A4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826593A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826593AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826593B0: 386A7E28  addi r3, r10, 0x7e28
	ctx.r[3].s64 = ctx.r[10].s64 + 32296;
	// 826593B4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826593B8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826593BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826593C0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826593C4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826593C8: 4BE0DA59  bl 0x82466e20
	ctx.lr = 0x826593CC;
	sub_82466E20(ctx, base);
	// 826593CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826593D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826593D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826593D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826593E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826593E0 size=100
    let mut pc: u32 = 0x826593E0;
    'dispatch: loop {
        match pc {
            0x826593E0 => {
    //   block [0x826593E0..0x82659444)
	// 826593E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826593E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826593E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826593EC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826593F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826593F4: 38AA7F78  addi r5, r10, 0x7f78
	ctx.r[5].s64 = ctx.r[10].s64 + 32632;
	// 826593F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826593FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659400: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 82659404: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265940C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659414: 386A7E58  addi r3, r10, 0x7e58
	ctx.r[3].s64 = ctx.r[10].s64 + 32344;
	// 82659418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265941C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659420: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82659424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659428: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265942C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659430: 4BE0D9F1  bl 0x82466e20
	ctx.lr = 0x82659434;
	sub_82466E20(ctx, base);
	// 82659434: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265943C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659448 size=100
    let mut pc: u32 = 0x82659448;
    'dispatch: loop {
        match pc {
            0x82659448 => {
    //   block [0x82659448..0x826594AC)
	// 82659448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265944C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659454: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265945C: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82659460: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659468: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8265946C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659474: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265947C: 386A7E88  addi r3, r10, 0x7e88
	ctx.r[3].s64 = ctx.r[10].s64 + 32392;
	// 82659480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659484: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659488: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265948C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659490: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659498: 4BE0D989  bl 0x82466e20
	ctx.lr = 0x8265949C;
	sub_82466E20(ctx, base);
	// 8265949C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826594A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826594A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826594A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826594B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826594B0 size=108
    let mut pc: u32 = 0x826594B0;
    'dispatch: loop {
        match pc {
            0x826594B0 => {
    //   block [0x826594B0..0x8265951C)
	// 826594B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826594B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826594B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826594BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826594C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826594C4: 38EBD578  addi r7, r11, -0x2a88
	ctx.r[7].s64 = ctx.r[11].s64 + -10888;
	// 826594C8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826594CC: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826594D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826594D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826594D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826594DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826594E0: 386A7EB8  addi r3, r10, 0x7eb8
	ctx.r[3].s64 = ctx.r[10].s64 + 32440;
	// 826594E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826594E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826594EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826594F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826594F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826594F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826594FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659504: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659508: 4BE0D919  bl 0x82466e20
	ctx.lr = 0x8265950C;
	sub_82466E20(ctx, base);
	// 8265950C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659520 size=112
    let mut pc: u32 = 0x82659520;
    'dispatch: loop {
        match pc {
            0x82659520 => {
    //   block [0x82659520..0x82659590)
	// 82659520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265952C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659530: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659534: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 82659538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265953C: 390BD5D8  addi r8, r11, -0x2a28
	ctx.r[8].s64 = ctx.r[11].s64 + -10792;
	// 82659540: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82659544: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 82659548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265954C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659550: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659558: 386A7EE8  addi r3, r10, 0x7ee8
	ctx.r[3].s64 = ctx.r[10].s64 + 32488;
	// 8265955C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265956C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265957C: 4BE0D8A5  bl 0x82466e20
	ctx.lr = 0x82659580;
	sub_82466E20(ctx, base);
	// 82659580: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265958C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659590 size=108
    let mut pc: u32 = 0x82659590;
    'dispatch: loop {
        match pc {
            0x82659590 => {
    //   block [0x82659590..0x826595FC)
	// 82659590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659598: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265959C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826595A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826595A4: 38EBD638  addi r7, r11, -0x29c8
	ctx.r[7].s64 = ctx.r[11].s64 + -10696;
	// 826595A8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826595AC: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826595B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826595B4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826595B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826595BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826595C0: 386A7F18  addi r3, r10, 0x7f18
	ctx.r[3].s64 = ctx.r[10].s64 + 32536;
	// 826595C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826595C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826595CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826595D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826595D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826595D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826595DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826595E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826595E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826595E8: 4BE0D839  bl 0x82466e20
	ctx.lr = 0x826595EC;
	sub_82466E20(ctx, base);
	// 826595EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826595F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826595F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826595F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659600 size=28
    let mut pc: u32 = 0x82659600;
    'dispatch: loop {
        match pc {
            0x82659600 => {
    //   block [0x82659600..0x8265961C)
	// 82659600: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659604: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659608: 394A1ED8  addi r10, r10, 0x1ed8
	ctx.r[10].s64 = ctx.r[10].s64 + 7896;
	// 8265960C: 816BD4FC  lwz r11, -0x2b04(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-11012 as u32) ) } as u64;
	// 82659610: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82659614: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82659618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659620 size=112
    let mut pc: u32 = 0x82659620;
    'dispatch: loop {
        match pc {
            0x82659620 => {
    //   block [0x82659620..0x82659690)
	// 82659620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265962C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659630: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 82659634: 38EA1ED8  addi r7, r10, 0x1ed8
	ctx.r[7].s64 = ctx.r[10].s64 + 7896;
	// 82659638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265963C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659640: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 82659644: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659648: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265964C: 396BD508  addi r11, r11, -0x2af8
	ctx.r[11].s64 = ctx.r[11].s64 + -11000;
	// 82659650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82659654: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659658: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265965C: 386A7F48  addi r3, r10, 0x7f48
	ctx.r[3].s64 = ctx.r[10].s64 + 32584;
	// 82659660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659664: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82659668: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265966C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82659670: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659674: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659678: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265967C: 4BE0D7A5  bl 0x82466e20
	ctx.lr = 0x82659680;
	sub_82466E20(ctx, base);
	// 82659680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265968C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659690 size=24
    let mut pc: u32 = 0x82659690;
    'dispatch: loop {
        match pc {
            0x82659690 => {
    //   block [0x82659690..0x826596A8)
	// 82659690: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659694: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659698: 394A2040  addi r10, r10, 0x2040
	ctx.r[10].s64 = ctx.r[10].s64 + 8256;
	// 8265969C: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 826596A0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826596A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826596A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826596A8 size=116
    let mut pc: u32 = 0x826596A8;
    'dispatch: loop {
        match pc {
            0x826596A8 => {
    //   block [0x826596A8..0x8265971C)
	// 826596A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826596AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826596B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826596B4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826596B8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826596BC: 392BD4E0  addi r9, r11, -0x2b20
	ctx.r[9].s64 = ctx.r[11].s64 + -11040;
	// 826596C0: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 826596C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826596C8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 826596CC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826596D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826596D4: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826596D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826596DC: 396B2040  addi r11, r11, 0x2040
	ctx.r[11].s64 = ctx.r[11].s64 + 8256;
	// 826596E0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826596E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826596E8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826596EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826596F0: 386A7F78  addi r3, r10, 0x7f78
	ctx.r[3].s64 = ctx.r[10].s64 + 32632;
	// 826596F4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826596F8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826596FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659700: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82659704: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659708: 4BE0D719  bl 0x82466e20
	ctx.lr = 0x8265970C;
	sub_82466E20(ctx, base);
	// 8265970C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659720 size=112
    let mut pc: u32 = 0x82659720;
    'dispatch: loop {
        match pc {
            0x82659720 => {
    //   block [0x82659720..0x82659790)
	// 82659720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265972C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659730: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659734: 38AA7B88  addi r5, r10, 0x7b88
	ctx.r[5].s64 = ctx.r[10].s64 + 31624;
	// 82659738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265973C: 390BD658  addi r8, r11, -0x29a8
	ctx.r[8].s64 = ctx.r[11].s64 + -10664;
	// 82659740: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82659744: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 82659748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265974C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659750: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659758: 386A7FA8  addi r3, r10, 0x7fa8
	ctx.r[3].s64 = ctx.r[10].s64 + 32680;
	// 8265975C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265976C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265977C: 4BE0D6A5  bl 0x82466e20
	ctx.lr = 0x82659780;
	sub_82466E20(ctx, base);
	// 82659780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265978C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659790 size=24
    let mut pc: u32 = 0x82659790;
    'dispatch: loop {
        match pc {
            0x82659790 => {
    //   block [0x82659790..0x826597A8)
	// 82659790: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659794: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659798: 394A20E8  addi r10, r10, 0x20e8
	ctx.r[10].s64 = ctx.r[10].s64 + 8424;
	// 8265979C: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 826597A0: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826597A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826597A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826597A8 size=116
    let mut pc: u32 = 0x826597A8;
    'dispatch: loop {
        match pc {
            0x826597A8 => {
    //   block [0x826597A8..0x8265981C)
	// 826597A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826597AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826597B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826597B4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 826597B8: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826597BC: 390A20E8  addi r8, r10, 0x20e8
	ctx.r[8].s64 = ctx.r[10].s64 + 8424;
	// 826597C0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826597C4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826597C8: 38AA7B88  addi r5, r10, 0x7b88
	ctx.r[5].s64 = ctx.r[10].s64 + 31624;
	// 826597CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826597D0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826597D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826597D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826597DC: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 826597E0: 396BD568  addi r11, r11, -0x2a98
	ctx.r[11].s64 = ctx.r[11].s64 + -10904;
	// 826597E4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 826597E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826597EC: 386A7FD8  addi r3, r10, 0x7fd8
	ctx.r[3].s64 = ctx.r[10].s64 + 32728;
	// 826597F0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826597F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826597F8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826597FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659808: 4BE0D619  bl 0x82466e20
	ctx.lr = 0x8265980C;
	sub_82466E20(ctx, base);
	// 8265980C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659820 size=112
    let mut pc: u32 = 0x82659820;
    'dispatch: loop {
        match pc {
            0x82659820 => {
    //   block [0x82659820..0x82659890)
	// 82659820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265982C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659830: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659834: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82659838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265983C: 390BD6E8  addi r8, r11, -0x2918
	ctx.r[8].s64 = ctx.r[11].s64 + -10520;
	// 82659840: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82659844: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 82659848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265984C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659858: 386A8008  addi r3, r10, -0x7ff8
	ctx.r[3].s64 = ctx.r[10].s64 + -32760;
	// 8265985C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659860: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265986C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82659870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659874: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265987C: 4BE0D5A5  bl 0x82466e20
	ctx.lr = 0x82659880;
	sub_82466E20(ctx, base);
	// 82659880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265988C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659890 size=108
    let mut pc: u32 = 0x82659890;
    'dispatch: loop {
        match pc {
            0x82659890 => {
    //   block [0x82659890..0x826598FC)
	// 82659890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265989C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826598A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826598A4: 38EBD718  addi r7, r11, -0x28e8
	ctx.r[7].s64 = ctx.r[11].s64 + -10472;
	// 826598A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826598AC: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826598B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826598B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826598B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826598BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826598C0: 386A8038  addi r3, r10, -0x7fc8
	ctx.r[3].s64 = ctx.r[10].s64 + -32712;
	// 826598C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826598C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826598CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826598D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826598D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826598D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826598DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826598E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826598E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826598E8: 4BE0D539  bl 0x82466e20
	ctx.lr = 0x826598EC;
	sub_82466E20(ctx, base);
	// 826598EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826598F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826598F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826598F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659900 size=112
    let mut pc: u32 = 0x82659900;
    'dispatch: loop {
        match pc {
            0x82659900 => {
    //   block [0x82659900..0x82659970)
	// 82659900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265990C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659910: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659914: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82659918: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265991C: 390BD748  addi r8, r11, -0x28b8
	ctx.r[8].s64 = ctx.r[11].s64 + -10424;
	// 82659920: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82659924: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 82659928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265992C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659930: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659934: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659938: 386A8068  addi r3, r10, -0x7f98
	ctx.r[3].s64 = ctx.r[10].s64 + -32664;
	// 8265993C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659940: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659948: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265994C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659950: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659958: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265995C: 4BE0D4C5  bl 0x82466e20
	ctx.lr = 0x82659960;
	sub_82466E20(ctx, base);
	// 82659960: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265996C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659970 size=112
    let mut pc: u32 = 0x82659970;
    'dispatch: loop {
        match pc {
            0x82659970 => {
    //   block [0x82659970..0x826599E0)
	// 82659970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265997C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659980: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659984: 38AA8278  addi r5, r10, -0x7d88
	ctx.r[5].s64 = ctx.r[10].s64 + -32136;
	// 82659988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265998C: 390BD778  addi r8, r11, -0x2888
	ctx.r[8].s64 = ctx.r[11].s64 + -10376;
	// 82659990: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82659994: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 82659998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265999C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826599A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826599A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826599A8: 386A8098  addi r3, r10, -0x7f68
	ctx.r[3].s64 = ctx.r[10].s64 + -32616;
	// 826599AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826599B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826599B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826599B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826599BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826599C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826599C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826599C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826599CC: 4BE0D455  bl 0x82466e20
	ctx.lr = 0x826599D0;
	sub_82466E20(ctx, base);
	// 826599D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826599D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826599D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826599DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826599E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826599E0 size=108
    let mut pc: u32 = 0x826599E0;
    'dispatch: loop {
        match pc {
            0x826599E0 => {
    //   block [0x826599E0..0x82659A4C)
	// 826599E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826599E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826599E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826599EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 826599F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826599F4: 38EBD7A8  addi r7, r11, -0x2858
	ctx.r[7].s64 = ctx.r[11].s64 + -10328;
	// 826599F8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826599FC: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 82659A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659A04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659A08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82659A0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659A10: 386A80C8  addi r3, r10, -0x7f38
	ctx.r[3].s64 = ctx.r[10].s64 + -32568;
	// 82659A14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659A24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659A34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659A38: 4BE0D3E9  bl 0x82466e20
	ctx.lr = 0x82659A3C;
	sub_82466E20(ctx, base);
	// 82659A3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659A40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659A44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659A48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659A50 size=108
    let mut pc: u32 = 0x82659A50;
    'dispatch: loop {
        match pc {
            0x82659A50 => {
    //   block [0x82659A50..0x82659ABC)
	// 82659A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659A58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659A5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659A60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82659A64: 38EBD7F0  addi r7, r11, -0x2810
	ctx.r[7].s64 = ctx.r[11].s64 + -10256;
	// 82659A68: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82659A6C: 388A0DDC  addi r4, r10, 0xddc
	ctx.r[4].s64 = ctx.r[10].s64 + 3548;
	// 82659A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659A74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659A78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82659A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659A80: 386A80F8  addi r3, r10, -0x7f08
	ctx.r[3].s64 = ctx.r[10].s64 + -32520;
	// 82659A84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659AA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659AA8: 4BE0D379  bl 0x82466e20
	ctx.lr = 0x82659AAC;
	sub_82466E20(ctx, base);
	// 82659AAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659AB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659AC0 size=112
    let mut pc: u32 = 0x82659AC0;
    'dispatch: loop {
        match pc {
            0x82659AC0 => {
    //   block [0x82659AC0..0x82659B30)
	// 82659AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659AC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659AC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659ACC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659AD0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659AD4: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82659AD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659ADC: 390BD850  addi r8, r11, -0x27b0
	ctx.r[8].s64 = ctx.r[11].s64 + -10160;
	// 82659AE0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82659AE4: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 82659AE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659AEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659AF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659AF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659AF8: 386A8128  addi r3, r10, -0x7ed8
	ctx.r[3].s64 = ctx.r[10].s64 + -32472;
	// 82659AFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659B00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659B08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659B0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659B10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659B18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659B1C: 4BE0D305  bl 0x82466e20
	ctx.lr = 0x82659B20;
	sub_82466E20(ctx, base);
	// 82659B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659B30 size=100
    let mut pc: u32 = 0x82659B30;
    'dispatch: loop {
        match pc {
            0x82659B30 => {
    //   block [0x82659B30..0x82659B94)
	// 82659B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659B3C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659B44: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82659B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659B50: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 82659B54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659B58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659B60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659B64: 386A8158  addi r3, r10, -0x7ea8
	ctx.r[3].s64 = ctx.r[10].s64 + -32424;
	// 82659B68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659B6C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659B70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82659B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659B78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659B80: 4BE0D2A1  bl 0x82466e20
	ctx.lr = 0x82659B84;
	sub_82466E20(ctx, base);
	// 82659B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659B98 size=112
    let mut pc: u32 = 0x82659B98;
    'dispatch: loop {
        match pc {
            0x82659B98 => {
    //   block [0x82659B98..0x82659C08)
	// 82659B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659BA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659BA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659BAC: 38AA7E88  addi r5, r10, 0x7e88
	ctx.r[5].s64 = ctx.r[10].s64 + 32392;
	// 82659BB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659BB4: 390BD8B0  addi r8, r11, -0x2750
	ctx.r[8].s64 = ctx.r[11].s64 + -10064;
	// 82659BB8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82659BBC: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 82659BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659BC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659BC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659BD0: 386A8188  addi r3, r10, -0x7e78
	ctx.r[3].s64 = ctx.r[10].s64 + -32376;
	// 82659BD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659BD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659BDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659BE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659BE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659BF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659BF4: 4BE0D22D  bl 0x82466e20
	ctx.lr = 0x82659BF8;
	sub_82466E20(ctx, base);
	// 82659BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659C08 size=112
    let mut pc: u32 = 0x82659C08;
    'dispatch: loop {
        match pc {
            0x82659C08 => {
    //   block [0x82659C08..0x82659C78)
	// 82659C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659C14: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659C18: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659C1C: 38AA7E88  addi r5, r10, 0x7e88
	ctx.r[5].s64 = ctx.r[10].s64 + 32392;
	// 82659C20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659C24: 390BD8F8  addi r8, r11, -0x2708
	ctx.r[8].s64 = ctx.r[11].s64 + -9992;
	// 82659C28: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 82659C2C: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 82659C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659C34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659C38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659C3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659C40: 386A81B8  addi r3, r10, -0x7e48
	ctx.r[3].s64 = ctx.r[10].s64 + -32328;
	// 82659C44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659C4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659C50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659C58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659C5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659C60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659C64: 4BE0D1BD  bl 0x82466e20
	ctx.lr = 0x82659C68;
	sub_82466E20(ctx, base);
	// 82659C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659C74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659C78 size=108
    let mut pc: u32 = 0x82659C78;
    'dispatch: loop {
        match pc {
            0x82659C78 => {
    //   block [0x82659C78..0x82659CE4)
	// 82659C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659C84: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659C8C: 38EBD9A0  addi r7, r11, -0x2660
	ctx.r[7].s64 = ctx.r[11].s64 + -9824;
	// 82659C90: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82659C94: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 82659C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659C9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659CA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82659CA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659CA8: 386A81E8  addi r3, r10, -0x7e18
	ctx.r[3].s64 = ctx.r[10].s64 + -32280;
	// 82659CAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82659CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659CB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659CBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659CCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82659CD0: 4BE0D151  bl 0x82466e20
	ctx.lr = 0x82659CD4;
	sub_82466E20(ctx, base);
	// 82659CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82659CE8 size=24
    let mut pc: u32 = 0x82659CE8;
    'dispatch: loop {
        match pc {
            0x82659CE8 => {
    //   block [0x82659CE8..0x82659D00)
	// 82659CE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659CEC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659CF0: 394A2220  addi r10, r10, 0x2220
	ctx.r[10].s64 = ctx.r[10].s64 + 8736;
	// 82659CF4: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 82659CF8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82659CFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659D00 size=116
    let mut pc: u32 = 0x82659D00;
    'dispatch: loop {
        match pc {
            0x82659D00 => {
    //   block [0x82659D00..0x82659D74)
	// 82659D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659D08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659D0C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 82659D10: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82659D14: 390A2220  addi r8, r10, 0x2220
	ctx.r[8].s64 = ctx.r[10].s64 + 8736;
	// 82659D18: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659D1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82659D20: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 82659D24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659D28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82659D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659D30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659D34: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 82659D38: 396BD5A0  addi r11, r11, -0x2a60
	ctx.r[11].s64 = ctx.r[11].s64 + -10848;
	// 82659D3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659D40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659D44: 386A8218  addi r3, r10, -0x7de8
	ctx.r[3].s64 = ctx.r[10].s64 + -32232;
	// 82659D48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82659D4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659D50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82659D54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659D60: 4BE0D0C1  bl 0x82466e20
	ctx.lr = 0x82659D64;
	sub_82466E20(ctx, base);
	// 82659D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659D78 size=100
    let mut pc: u32 = 0x82659D78;
    'dispatch: loop {
        match pc {
            0x82659D78 => {
    //   block [0x82659D78..0x82659DDC)
	// 82659D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659D84: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659D88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659D8C: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 82659D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659D98: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 82659D9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659DAC: 386A8248  addi r3, r10, -0x7db8
	ctx.r[3].s64 = ctx.r[10].s64 + -32184;
	// 82659DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659DB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659DB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82659DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659DC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659DC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659DC8: 4BE0D059  bl 0x82466e20
	ctx.lr = 0x82659DCC;
	sub_82466E20(ctx, base);
	// 82659DCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659DD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659DD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659DD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659DE0 size=100
    let mut pc: u32 = 0x82659DE0;
    'dispatch: loop {
        match pc {
            0x82659DE0 => {
    //   block [0x82659DE0..0x82659E44)
	// 82659DE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659DE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659DE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659DEC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659DF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659DF4: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 82659DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659DFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659E00: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 82659E04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659E14: 386A8278  addi r3, r10, -0x7d88
	ctx.r[3].s64 = ctx.r[10].s64 + -32136;
	// 82659E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659E1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659E20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82659E24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659E28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82659E2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659E30: 4BE0CFF1  bl 0x82466e20
	ctx.lr = 0x82659E34;
	sub_82466E20(ctx, base);
	// 82659E34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659E38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659E3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659E48 size=112
    let mut pc: u32 = 0x82659E48;
    'dispatch: loop {
        match pc {
            0x82659E48 => {
    //   block [0x82659E48..0x82659EB8)
	// 82659E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659E54: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659E58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659E5C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82659E60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659E64: 390BDA00  addi r8, r11, -0x2600
	ctx.r[8].s64 = ctx.r[11].s64 + -9728;
	// 82659E68: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82659E6C: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 82659E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659E74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659E78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659E7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659E80: 386A82A8  addi r3, r10, -0x7d58
	ctx.r[3].s64 = ctx.r[10].s64 + -32088;
	// 82659E84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659E88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659E8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659E90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659E98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659E9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659EA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659EA4: 4BE0CF7D  bl 0x82466e20
	ctx.lr = 0x82659EA8;
	sub_82466E20(ctx, base);
	// 82659EA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659EAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659EB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659EB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659EB8 size=112
    let mut pc: u32 = 0x82659EB8;
    'dispatch: loop {
        match pc {
            0x82659EB8 => {
    //   block [0x82659EB8..0x82659F28)
	// 82659EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659EC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659EC4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659EC8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659ECC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82659ED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659ED4: 390BDA90  addi r8, r11, -0x2570
	ctx.r[8].s64 = ctx.r[11].s64 + -9584;
	// 82659ED8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82659EDC: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 82659EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659EE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659EE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659EF0: 386A82D8  addi r3, r10, -0x7d28
	ctx.r[3].s64 = ctx.r[10].s64 + -32040;
	// 82659EF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659EF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659EFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659F00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659F08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659F0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659F10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659F14: 4BE0CF0D  bl 0x82466e20
	ctx.lr = 0x82659F18;
	sub_82466E20(ctx, base);
	// 82659F18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659F24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659F28 size=112
    let mut pc: u32 = 0x82659F28;
    'dispatch: loop {
        match pc {
            0x82659F28 => {
    //   block [0x82659F28..0x82659F98)
	// 82659F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659F30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659F34: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659F38: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659F3C: 38AA7E28  addi r5, r10, 0x7e28
	ctx.r[5].s64 = ctx.r[10].s64 + 32296;
	// 82659F40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659F44: 390BDAF0  addi r8, r11, -0x2510
	ctx.r[8].s64 = ctx.r[11].s64 + -9488;
	// 82659F48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82659F4C: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 82659F50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659F54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659F58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659F5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659F60: 386A8308  addi r3, r10, -0x7cf8
	ctx.r[3].s64 = ctx.r[10].s64 + -31992;
	// 82659F64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659F68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659F6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659F70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659F74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659F78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659F80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659F84: 4BE0CE9D  bl 0x82466e20
	ctx.lr = 0x82659F88;
	sub_82466E20(ctx, base);
	// 82659F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82659F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82659F94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82659F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82659F98 size=112
    let mut pc: u32 = 0x82659F98;
    'dispatch: loop {
        match pc {
            0x82659F98 => {
    //   block [0x82659F98..0x8265A008)
	// 82659F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82659F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82659FA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82659FA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 82659FA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 82659FAC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 82659FB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82659FB4: 390BDB20  addi r8, r11, -0x24e0
	ctx.r[8].s64 = ctx.r[11].s64 + -9440;
	// 82659FB8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82659FBC: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 82659FC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82659FC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82659FC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82659FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82659FD0: 386A8338  addi r3, r10, -0x7cc8
	ctx.r[3].s64 = ctx.r[10].s64 + -31944;
	// 82659FD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82659FD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82659FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82659FE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82659FE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82659FE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82659FEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82659FF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82659FF4: 4BE0CE2D  bl 0x82466e20
	ctx.lr = 0x82659FF8;
	sub_82466E20(ctx, base);
	// 82659FF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82659FFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A008 size=112
    let mut pc: u32 = 0x8265A008;
    'dispatch: loop {
        match pc {
            0x8265A008 => {
    //   block [0x8265A008..0x8265A078)
	// 8265A008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A014: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A018: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A01C: 38AA7F78  addi r5, r10, 0x7f78
	ctx.r[5].s64 = ctx.r[10].s64 + 32632;
	// 8265A020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A024: 390BDBB0  addi r8, r11, -0x2450
	ctx.r[8].s64 = ctx.r[11].s64 + -9296;
	// 8265A028: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A02C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8265A030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A034: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A040: 386A8368  addi r3, r10, -0x7c98
	ctx.r[3].s64 = ctx.r[10].s64 + -31896;
	// 8265A044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A04C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A05C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A064: 4BE0CDBD  bl 0x82466e20
	ctx.lr = 0x8265A068;
	sub_82466E20(ctx, base);
	// 8265A068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A06C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A078 size=112
    let mut pc: u32 = 0x8265A078;
    'dispatch: loop {
        match pc {
            0x8265A078 => {
    //   block [0x8265A078..0x8265A0E8)
	// 8265A078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A084: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A088: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A08C: 38AA81B8  addi r5, r10, -0x7e48
	ctx.r[5].s64 = ctx.r[10].s64 + -32328;
	// 8265A090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A094: 390BDBC8  addi r8, r11, -0x2438
	ctx.r[8].s64 = ctx.r[11].s64 + -9272;
	// 8265A098: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A09C: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8265A0A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A0A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A0A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A0AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A0B0: 386A8398  addi r3, r10, -0x7c68
	ctx.r[3].s64 = ctx.r[10].s64 + -31848;
	// 8265A0B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A0B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A0BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A0C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A0C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A0C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A0CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A0D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A0D4: 4BE0CD4D  bl 0x82466e20
	ctx.lr = 0x8265A0D8;
	sub_82466E20(ctx, base);
	// 8265A0D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A0DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A0E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A0E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A0E8 size=112
    let mut pc: u32 = 0x8265A0E8;
    'dispatch: loop {
        match pc {
            0x8265A0E8 => {
    //   block [0x8265A0E8..0x8265A158)
	// 8265A0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A0F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A0F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A0F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A0FC: 38AA7A68  addi r5, r10, 0x7a68
	ctx.r[5].s64 = ctx.r[10].s64 + 31336;
	// 8265A100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A104: 390BDBF8  addi r8, r11, -0x2408
	ctx.r[8].s64 = ctx.r[11].s64 + -9224;
	// 8265A108: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265A10C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8265A110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A114: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A11C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A120: 386A83C8  addi r3, r10, -0x7c38
	ctx.r[3].s64 = ctx.r[10].s64 + -31800;
	// 8265A124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A144: 4BE0CCDD  bl 0x82466e20
	ctx.lr = 0x8265A148;
	sub_82466E20(ctx, base);
	// 8265A148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A14C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265A158 size=24
    let mut pc: u32 = 0x8265A158;
    'dispatch: loop {
        match pc {
            0x8265A158 => {
    //   block [0x8265A158..0x8265A170)
	// 8265A158: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A15C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265A160: 394A2298  addi r10, r10, 0x2298
	ctx.r[10].s64 = ctx.r[10].s64 + 8856;
	// 8265A164: 816BD654  lwz r11, -0x29ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10668 as u32) ) } as u64;
	// 8265A168: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8265A16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A170 size=116
    let mut pc: u32 = 0x8265A170;
    'dispatch: loop {
        match pc {
            0x8265A170 => {
    //   block [0x8265A170..0x8265A1E4)
	// 8265A170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A178: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A17C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265A180: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265A184: 390A2298  addi r8, r10, 0x2298
	ctx.r[8].s64 = ctx.r[10].s64 + 8856;
	// 8265A188: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A18C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265A190: 38AA7C18  addi r5, r10, 0x7c18
	ctx.r[5].s64 = ctx.r[10].s64 + 31768;
	// 8265A194: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A198: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265A19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A1A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A1A4: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8265A1A8: 396BD5B8  addi r11, r11, -0x2a48
	ctx.r[11].s64 = ctx.r[11].s64 + -10824;
	// 8265A1AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A1B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A1B4: 386A83F8  addi r3, r10, -0x7c08
	ctx.r[3].s64 = ctx.r[10].s64 + -31752;
	// 8265A1B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265A1BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A1C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265A1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A1C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A1D0: 4BE0CC51  bl 0x82466e20
	ctx.lr = 0x8265A1D4;
	sub_82466E20(ctx, base);
	// 8265A1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A1E8 size=112
    let mut pc: u32 = 0x8265A1E8;
    'dispatch: loop {
        match pc {
            0x8265A1E8 => {
    //   block [0x8265A1E8..0x8265A258)
	// 8265A1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A1F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A1F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A1FC: 38AA7B88  addi r5, r10, 0x7b88
	ctx.r[5].s64 = ctx.r[10].s64 + 31624;
	// 8265A200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A204: 390BDC40  addi r8, r11, -0x23c0
	ctx.r[8].s64 = ctx.r[11].s64 + -9152;
	// 8265A208: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A20C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8265A210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A220: 386A8428  addi r3, r10, -0x7bd8
	ctx.r[3].s64 = ctx.r[10].s64 + -31704;
	// 8265A224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A244: 4BE0CBDD  bl 0x82466e20
	ctx.lr = 0x8265A248;
	sub_82466E20(ctx, base);
	// 8265A248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A258 size=112
    let mut pc: u32 = 0x8265A258;
    'dispatch: loop {
        match pc {
            0x8265A258 => {
    //   block [0x8265A258..0x8265A2C8)
	// 8265A258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265A268: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A26C: 38AA7BE8  addi r5, r10, 0x7be8
	ctx.r[5].s64 = ctx.r[10].s64 + 31720;
	// 8265A270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A274: 390BDC70  addi r8, r11, -0x2390
	ctx.r[8].s64 = ctx.r[11].s64 + -9104;
	// 8265A278: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A27C: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8265A280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A290: 386A8458  addi r3, r10, -0x7ba8
	ctx.r[3].s64 = ctx.r[10].s64 + -31656;
	// 8265A294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A2B4: 4BE0CB6D  bl 0x82466e20
	ctx.lr = 0x8265A2B8;
	sub_82466E20(ctx, base);
	// 8265A2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A2C8 size=100
    let mut pc: u32 = 0x8265A2C8;
    'dispatch: loop {
        match pc {
            0x8265A2C8 => {
    //   block [0x8265A2C8..0x8265A32C)
	// 8265A2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A2D4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265A2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A2DC: 392AD628  addi r9, r10, -0x29d8
	ctx.r[9].s64 = ctx.r[10].s64 + -10712;
	// 8265A2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A2E8: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 8265A2EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A2F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A2F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A2F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A2FC: 386A8488  addi r3, r10, -0x7b78
	ctx.r[3].s64 = ctx.r[10].s64 + -31608;
	// 8265A300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A304: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8265A308: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265A30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A310: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265A314: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265A318: 4BE0CB09  bl 0x82466e20
	ctx.lr = 0x8265A31C;
	sub_82466E20(ctx, base);
	// 8265A31C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265A330 size=24
    let mut pc: u32 = 0x8265A330;
    'dispatch: loop {
        match pc {
            0x8265A330 => {
    //   block [0x8265A330..0x8265A348)
	// 8265A330: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A334: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265A338: 394A2340  addi r10, r10, 0x2340
	ctx.r[10].s64 = ctx.r[10].s64 + 9024;
	// 8265A33C: 816BDCA8  lwz r11, -0x2358(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9048 as u32) ) } as u64;
	// 8265A340: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265A344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A348 size=112
    let mut pc: u32 = 0x8265A348;
    'dispatch: loop {
        match pc {
            0x8265A348 => {
    //   block [0x8265A348..0x8265A3B8)
	// 8265A348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A354: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265A358: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A35C: 392AD768  addi r9, r10, -0x2898
	ctx.r[9].s64 = ctx.r[10].s64 + -10392;
	// 8265A360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A364: 390B2340  addi r8, r11, 0x2340
	ctx.r[8].s64 = ctx.r[11].s64 + 9024;
	// 8265A368: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265A36C: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 8265A370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A374: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A378: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A380: 386A84B8  addi r3, r10, -0x7b48
	ctx.r[3].s64 = ctx.r[10].s64 + -31560;
	// 8265A384: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265A388: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8265A38C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A39C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265A3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A3A4: 4BE0CA7D  bl 0x82466e20
	ctx.lr = 0x8265A3A8;
	sub_82466E20(ctx, base);
	// 8265A3A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A3B8 size=112
    let mut pc: u32 = 0x8265A3B8;
    'dispatch: loop {
        match pc {
            0x8265A3B8 => {
    //   block [0x8265A3B8..0x8265A428)
	// 8265A3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A3C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A3C8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A3CC: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A3D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A3D4: 390BDCB0  addi r8, r11, -0x2350
	ctx.r[8].s64 = ctx.r[11].s64 + -9040;
	// 8265A3D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A3DC: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 8265A3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A3E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A3E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A3F0: 386A84E8  addi r3, r10, -0x7b18
	ctx.r[3].s64 = ctx.r[10].s64 + -31512;
	// 8265A3F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A3F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A3FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A414: 4BE0CA0D  bl 0x82466e20
	ctx.lr = 0x8265A418;
	sub_82466E20(ctx, base);
	// 8265A418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A428 size=108
    let mut pc: u32 = 0x8265A428;
    'dispatch: loop {
        match pc {
            0x8265A428 => {
    //   block [0x8265A428..0x8265A494)
	// 8265A428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A434: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A43C: 38EBDCE0  addi r7, r11, -0x2320
	ctx.r[7].s64 = ctx.r[11].s64 + -8992;
	// 8265A440: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265A444: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 8265A448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A44C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A450: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265A454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A458: 386A8518  addi r3, r10, -0x7ae8
	ctx.r[3].s64 = ctx.r[10].s64 + -31464;
	// 8265A45C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265A460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A47C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265A480: 4BE0C9A1  bl 0x82466e20
	ctx.lr = 0x8265A484;
	sub_82466E20(ctx, base);
	// 8265A484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A48C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A498 size=112
    let mut pc: u32 = 0x8265A498;
    'dispatch: loop {
        match pc {
            0x8265A498 => {
    //   block [0x8265A498..0x8265A508)
	// 8265A498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A49C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A4A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A4A8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A4AC: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A4B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265A4B4: 390BDCF8  addi r8, r11, -0x2308
	ctx.r[8].s64 = ctx.r[11].s64 + -8968;
	// 8265A4B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265A4BC: 388A0E04  addi r4, r10, 0xe04
	ctx.r[4].s64 = ctx.r[10].s64 + 3588;
	// 8265A4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A4C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A4C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A4CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A4D0: 386A8548  addi r3, r10, -0x7ab8
	ctx.r[3].s64 = ctx.r[10].s64 + -31416;
	// 8265A4D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A4DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A4F4: 4BE0C92D  bl 0x82466e20
	ctx.lr = 0x8265A4F8;
	sub_82466E20(ctx, base);
	// 8265A4F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A4FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A500: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A508 size=100
    let mut pc: u32 = 0x8265A508;
    'dispatch: loop {
        match pc {
            0x8265A508 => {
    //   block [0x8265A508..0x8265A56C)
	// 8265A508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A514: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A51C: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A528: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 8265A52C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A53C: 386A8578  addi r3, r10, -0x7a88
	ctx.r[3].s64 = ctx.r[10].s64 + -31368;
	// 8265A540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A544: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A548: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265A54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A550: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265A554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A558: 4BE0C8C9  bl 0x82466e20
	ctx.lr = 0x8265A55C;
	sub_82466E20(ctx, base);
	// 8265A55C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A570 size=112
    let mut pc: u32 = 0x8265A570;
    'dispatch: loop {
        match pc {
            0x8265A570 => {
    //   block [0x8265A570..0x8265A5E0)
	// 8265A570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A57C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A580: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A584: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A58C: 390BDD58  addi r8, r11, -0x22a8
	ctx.r[8].s64 = ctx.r[11].s64 + -8872;
	// 8265A590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A594: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 8265A598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A59C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A5A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A5A8: 386A85A8  addi r3, r10, -0x7a58
	ctx.r[3].s64 = ctx.r[10].s64 + -31320;
	// 8265A5AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A5B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A5B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A5B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A5C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A5C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A5C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A5CC: 4BE0C855  bl 0x82466e20
	ctx.lr = 0x8265A5D0;
	sub_82466E20(ctx, base);
	// 8265A5D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A5E0 size=112
    let mut pc: u32 = 0x8265A5E0;
    'dispatch: loop {
        match pc {
            0x8265A5E0 => {
    //   block [0x8265A5E0..0x8265A650)
	// 8265A5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A5E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A5EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A5F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A5F4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A5FC: 390BDD70  addi r8, r11, -0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + -8848;
	// 8265A600: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A604: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 8265A608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A60C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A614: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A618: 386A85D8  addi r3, r10, -0x7a28
	ctx.r[3].s64 = ctx.r[10].s64 + -31272;
	// 8265A61C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A63C: 4BE0C7E5  bl 0x82466e20
	ctx.lr = 0x8265A640;
	sub_82466E20(ctx, base);
	// 8265A640: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A64C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A650 size=112
    let mut pc: u32 = 0x8265A650;
    'dispatch: loop {
        match pc {
            0x8265A650 => {
    //   block [0x8265A650..0x8265A6C0)
	// 8265A650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A65C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A660: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A664: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A66C: 390BDDA0  addi r8, r11, -0x2260
	ctx.r[8].s64 = ctx.r[11].s64 + -8800;
	// 8265A670: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A674: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 8265A678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A67C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A688: 386A8608  addi r3, r10, -0x79f8
	ctx.r[3].s64 = ctx.r[10].s64 + -31224;
	// 8265A68C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A6AC: 4BE0C775  bl 0x82466e20
	ctx.lr = 0x8265A6B0;
	sub_82466E20(ctx, base);
	// 8265A6B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A6C0 size=112
    let mut pc: u32 = 0x8265A6C0;
    'dispatch: loop {
        match pc {
            0x8265A6C0 => {
    //   block [0x8265A6C0..0x8265A730)
	// 8265A6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A6CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A6D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A6D4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A6DC: 390BDDD0  addi r8, r11, -0x2230
	ctx.r[8].s64 = ctx.r[11].s64 + -8752;
	// 8265A6E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A6E4: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 8265A6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A6EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A6F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A6F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A6F8: 386A8638  addi r3, r10, -0x79c8
	ctx.r[3].s64 = ctx.r[10].s64 + -31176;
	// 8265A6FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A71C: 4BE0C705  bl 0x82466e20
	ctx.lr = 0x8265A720;
	sub_82466E20(ctx, base);
	// 8265A720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A72C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A730 size=112
    let mut pc: u32 = 0x8265A730;
    'dispatch: loop {
        match pc {
            0x8265A730 => {
    //   block [0x8265A730..0x8265A7A0)
	// 8265A730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A73C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A740: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A744: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A74C: 390BDE00  addi r8, r11, -0x2200
	ctx.r[8].s64 = ctx.r[11].s64 + -8704;
	// 8265A750: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A754: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 8265A758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A75C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A768: 386A8668  addi r3, r10, -0x7998
	ctx.r[3].s64 = ctx.r[10].s64 + -31128;
	// 8265A76C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A78C: 4BE0C695  bl 0x82466e20
	ctx.lr = 0x8265A790;
	sub_82466E20(ctx, base);
	// 8265A790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A79C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A7A0 size=112
    let mut pc: u32 = 0x8265A7A0;
    'dispatch: loop {
        match pc {
            0x8265A7A0 => {
    //   block [0x8265A7A0..0x8265A810)
	// 8265A7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A7AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A7B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A7B4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A7BC: 390BDE18  addi r8, r11, -0x21e8
	ctx.r[8].s64 = ctx.r[11].s64 + -8680;
	// 8265A7C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A7C4: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 8265A7C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A7CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A7D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A7D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A7D8: 386A8698  addi r3, r10, -0x7968
	ctx.r[3].s64 = ctx.r[10].s64 + -31080;
	// 8265A7DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A7EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A7FC: 4BE0C625  bl 0x82466e20
	ctx.lr = 0x8265A800;
	sub_82466E20(ctx, base);
	// 8265A800: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A80C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A810 size=112
    let mut pc: u32 = 0x8265A810;
    'dispatch: loop {
        match pc {
            0x8265A810 => {
    //   block [0x8265A810..0x8265A880)
	// 8265A810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A81C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A820: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A824: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A82C: 390BDE30  addi r8, r11, -0x21d0
	ctx.r[8].s64 = ctx.r[11].s64 + -8656;
	// 8265A830: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265A834: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 8265A838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A83C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A848: 386A86C8  addi r3, r10, -0x7938
	ctx.r[3].s64 = ctx.r[10].s64 + -31032;
	// 8265A84C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A85C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A86C: 4BE0C5B5  bl 0x82466e20
	ctx.lr = 0x8265A870;
	sub_82466E20(ctx, base);
	// 8265A870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A87C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A880 size=112
    let mut pc: u32 = 0x8265A880;
    'dispatch: loop {
        match pc {
            0x8265A880 => {
    //   block [0x8265A880..0x8265A8F0)
	// 8265A880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A88C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A890: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A894: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A89C: 390BDE78  addi r8, r11, -0x2188
	ctx.r[8].s64 = ctx.r[11].s64 + -8584;
	// 8265A8A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265A8A4: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 8265A8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A8AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A8B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A8B8: 386A86F8  addi r3, r10, -0x7908
	ctx.r[3].s64 = ctx.r[10].s64 + -30984;
	// 8265A8BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A8C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A8CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A8DC: 4BE0C545  bl 0x82466e20
	ctx.lr = 0x8265A8E0;
	sub_82466E20(ctx, base);
	// 8265A8E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A8E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A8E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A8EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A8F0 size=112
    let mut pc: u32 = 0x8265A8F0;
    'dispatch: loop {
        match pc {
            0x8265A8F0 => {
    //   block [0x8265A8F0..0x8265A960)
	// 8265A8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A8FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A900: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A904: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A90C: 390BDEC0  addi r8, r11, -0x2140
	ctx.r[8].s64 = ctx.r[11].s64 + -8512;
	// 8265A910: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265A914: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 8265A918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A91C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A928: 386A8728  addi r3, r10, -0x78d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30936;
	// 8265A92C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A94C: 4BE0C4D5  bl 0x82466e20
	ctx.lr = 0x8265A950;
	sub_82466E20(ctx, base);
	// 8265A950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A960 size=112
    let mut pc: u32 = 0x8265A960;
    'dispatch: loop {
        match pc {
            0x8265A960 => {
    //   block [0x8265A960..0x8265A9D0)
	// 8265A960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A96C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A970: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265A974: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A97C: 390BDED8  addi r8, r11, -0x2128
	ctx.r[8].s64 = ctx.r[11].s64 + -8488;
	// 8265A980: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265A984: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 8265A988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265A98C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265A994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265A998: 386A8758  addi r3, r10, -0x78a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30888;
	// 8265A99C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265A9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265A9A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265A9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265A9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265A9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265A9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265A9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265A9BC: 4BE0C465  bl 0x82466e20
	ctx.lr = 0x8265A9C0;
	sub_82466E20(ctx, base);
	// 8265A9C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265A9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265A9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265A9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265A9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265A9D0 size=116
    let mut pc: u32 = 0x8265A9D0;
    'dispatch: loop {
        match pc {
            0x8265A9D0 => {
    //   block [0x8265A9D0..0x8265AA44)
	// 8265A9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265A9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265A9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265A9DC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265A9E0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8265A9E4: 390ADF08  addi r8, r10, -0x20f8
	ctx.r[8].s64 = ctx.r[10].s64 + -8440;
	// 8265A9E8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265A9EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265A9F0: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265A9F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265A9F8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265A9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AA00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AA04: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 8265AA08: 396BD790  addi r11, r11, -0x2870
	ctx.r[11].s64 = ctx.r[11].s64 + -10352;
	// 8265AA0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AA10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AA14: 386A8788  addi r3, r10, -0x7878
	ctx.r[3].s64 = ctx.r[10].s64 + -30840;
	// 8265AA18: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265AA1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AA20: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265AA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AA30: 4BE0C3F1  bl 0x82466e20
	ctx.lr = 0x8265AA34;
	sub_82466E20(ctx, base);
	// 8265AA34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AA38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AA3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AA40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AA48 size=116
    let mut pc: u32 = 0x8265AA48;
    'dispatch: loop {
        match pc {
            0x8265AA48 => {
    //   block [0x8265AA48..0x8265AABC)
	// 8265AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AA54: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265AA58: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8265AA5C: 390ADF80  addi r8, r10, -0x2080
	ctx.r[8].s64 = ctx.r[10].s64 + -8320;
	// 8265AA60: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AA64: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265AA68: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AA6C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AA70: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265AA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AA78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AA7C: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 8265AA80: 396BD7A8  addi r11, r11, -0x2858
	ctx.r[11].s64 = ctx.r[11].s64 + -10328;
	// 8265AA84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AA88: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AA8C: 386A87B8  addi r3, r10, -0x7848
	ctx.r[3].s64 = ctx.r[10].s64 + -30792;
	// 8265AA90: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265AA94: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AA98: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265AA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AAA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AAA8: 4BE0C379  bl 0x82466e20
	ctx.lr = 0x8265AAAC;
	sub_82466E20(ctx, base);
	// 8265AAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AAC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265AAC0 size=24
    let mut pc: u32 = 0x8265AAC0;
    'dispatch: loop {
        match pc {
            0x8265AAC0 => {
    //   block [0x8265AAC0..0x8265AAD8)
	// 8265AAC0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AAC4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265AAC8: 394A2358  addi r10, r10, 0x2358
	ctx.r[10].s64 = ctx.r[10].s64 + 9048;
	// 8265AACC: 816BE010  lwz r11, -0x1ff0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8176 as u32) ) } as u64;
	// 8265AAD0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8265AAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AAD8 size=116
    let mut pc: u32 = 0x8265AAD8;
    'dispatch: loop {
        match pc {
            0x8265AAD8 => {
    //   block [0x8265AAD8..0x8265AB4C)
	// 8265AAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AAE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AAE4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265AAE8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AAEC: 392BD7D4  addi r9, r11, -0x282c
	ctx.r[9].s64 = ctx.r[11].s64 + -10284;
	// 8265AAF0: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AAF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AAF8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8265AAFC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8265AB00: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AB04: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 8265AB08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AB0C: 396B2358  addi r11, r11, 0x2358
	ctx.r[11].s64 = ctx.r[11].s64 + 9048;
	// 8265AB10: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265AB14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AB18: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8265AB1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AB20: 386A87E8  addi r3, r10, -0x7818
	ctx.r[3].s64 = ctx.r[10].s64 + -30744;
	// 8265AB24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265AB28: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8265AB2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AB30: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8265AB34: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265AB38: 4BE0C2E9  bl 0x82466e20
	ctx.lr = 0x8265AB3C;
	sub_82466E20(ctx, base);
	// 8265AB3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AB40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AB44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AB48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AB50 size=112
    let mut pc: u32 = 0x8265AB50;
    'dispatch: loop {
        match pc {
            0x8265AB50 => {
    //   block [0x8265AB50..0x8265ABC0)
	// 8265AB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AB5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AB60: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AB64: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AB68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AB6C: 390BE018  addi r8, r11, -0x1fe8
	ctx.r[8].s64 = ctx.r[11].s64 + -8168;
	// 8265AB70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265AB74: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 8265AB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AB7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AB80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AB88: 386A8818  addi r3, r10, -0x77e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30696;
	// 8265AB8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ABA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ABA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ABA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ABAC: 4BE0C275  bl 0x82466e20
	ctx.lr = 0x8265ABB0;
	sub_82466E20(ctx, base);
	// 8265ABB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ABB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ABB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ABBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ABC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ABC0 size=112
    let mut pc: u32 = 0x8265ABC0;
    'dispatch: loop {
        match pc {
            0x8265ABC0 => {
    //   block [0x8265ABC0..0x8265AC30)
	// 8265ABC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ABC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ABC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ABCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ABD0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265ABD4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265ABD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265ABDC: 390BE078  addi r8, r11, -0x1f88
	ctx.r[8].s64 = ctx.r[11].s64 + -8072;
	// 8265ABE0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8265ABE4: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 8265ABE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ABEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ABF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265ABF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ABF8: 386A8848  addi r3, r10, -0x77b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30648;
	// 8265ABFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AC00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AC04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AC08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AC10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AC14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AC18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AC1C: 4BE0C205  bl 0x82466e20
	ctx.lr = 0x8265AC20;
	sub_82466E20(ctx, base);
	// 8265AC20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AC24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AC28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AC30 size=112
    let mut pc: u32 = 0x8265AC30;
    'dispatch: loop {
        match pc {
            0x8265AC30 => {
    //   block [0x8265AC30..0x8265ACA0)
	// 8265AC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AC3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AC40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AC44: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AC4C: 390BE120  addi r8, r11, -0x1ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -7904;
	// 8265AC50: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8265AC54: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 8265AC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AC5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AC68: 386A8878  addi r3, r10, -0x7788
	ctx.r[3].s64 = ctx.r[10].s64 + -30600;
	// 8265AC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AC8C: 4BE0C195  bl 0x82466e20
	ctx.lr = 0x8265AC90;
	sub_82466E20(ctx, base);
	// 8265AC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ACA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ACA0 size=112
    let mut pc: u32 = 0x8265ACA0;
    'dispatch: loop {
        match pc {
            0x8265ACA0 => {
    //   block [0x8265ACA0..0x8265AD10)
	// 8265ACA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ACA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ACA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ACAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ACB0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265ACB4: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265ACB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265ACBC: 390BE198  addi r8, r11, -0x1e68
	ctx.r[8].s64 = ctx.r[11].s64 + -7784;
	// 8265ACC0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265ACC4: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 8265ACC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ACCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ACD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265ACD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ACD8: 386A88A8  addi r3, r10, -0x7758
	ctx.r[3].s64 = ctx.r[10].s64 + -30552;
	// 8265ACDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265ACE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265ACE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265ACE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265ACEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ACF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ACF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ACF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ACFC: 4BE0C125  bl 0x82466e20
	ctx.lr = 0x8265AD00;
	sub_82466E20(ctx, base);
	// 8265AD00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AD04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AD08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AD10 size=112
    let mut pc: u32 = 0x8265AD10;
    'dispatch: loop {
        match pc {
            0x8265AD10 => {
    //   block [0x8265AD10..0x8265AD80)
	// 8265AD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AD1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AD20: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AD24: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AD2C: 390BE1E0  addi r8, r11, -0x1e20
	ctx.r[8].s64 = ctx.r[11].s64 + -7712;
	// 8265AD30: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8265AD34: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 8265AD38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AD3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AD40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AD44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AD48: 386A88D8  addi r3, r10, -0x7728
	ctx.r[3].s64 = ctx.r[10].s64 + -30504;
	// 8265AD4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AD50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AD54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AD58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AD5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AD60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AD68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AD6C: 4BE0C0B5  bl 0x82466e20
	ctx.lr = 0x8265AD70;
	sub_82466E20(ctx, base);
	// 8265AD70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AD74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AD78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AD80 size=112
    let mut pc: u32 = 0x8265AD80;
    'dispatch: loop {
        match pc {
            0x8265AD80 => {
    //   block [0x8265AD80..0x8265ADF0)
	// 8265AD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AD8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AD90: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AD94: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AD98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AD9C: 390BE270  addi r8, r11, -0x1d90
	ctx.r[8].s64 = ctx.r[11].s64 + -7568;
	// 8265ADA0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265ADA4: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 8265ADA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ADAC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ADB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265ADB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ADB8: 386A8908  addi r3, r10, -0x76f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30456;
	// 8265ADBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265ADC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265ADC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265ADC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265ADCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ADD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ADD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ADD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ADDC: 4BE0C045  bl 0x82466e20
	ctx.lr = 0x8265ADE0;
	sub_82466E20(ctx, base);
	// 8265ADE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ADE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ADE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ADEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ADF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ADF0 size=112
    let mut pc: u32 = 0x8265ADF0;
    'dispatch: loop {
        match pc {
            0x8265ADF0 => {
    //   block [0x8265ADF0..0x8265AE60)
	// 8265ADF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ADF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ADF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ADFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AE00: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AE04: 38AA84B8  addi r5, r10, -0x7b48
	ctx.r[5].s64 = ctx.r[10].s64 + -31560;
	// 8265AE08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AE0C: 390BE2D0  addi r8, r11, -0x1d30
	ctx.r[8].s64 = ctx.r[11].s64 + -7472;
	// 8265AE10: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265AE14: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 8265AE18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AE1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AE20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AE28: 386A8938  addi r3, r10, -0x76c8
	ctx.r[3].s64 = ctx.r[10].s64 + -30408;
	// 8265AE2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AE30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AE44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AE4C: 4BE0BFD5  bl 0x82466e20
	ctx.lr = 0x8265AE50;
	sub_82466E20(ctx, base);
	// 8265AE50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AE5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AE60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AE60 size=112
    let mut pc: u32 = 0x8265AE60;
    'dispatch: loop {
        match pc {
            0x8265AE60 => {
    //   block [0x8265AE60..0x8265AED0)
	// 8265AE60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AE64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AE68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AE6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AE70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AE74: 38AA8938  addi r5, r10, -0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + -30408;
	// 8265AE78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AE7C: 390BE330  addi r8, r11, -0x1cd0
	ctx.r[8].s64 = ctx.r[11].s64 + -7376;
	// 8265AE80: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265AE84: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 8265AE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AE8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AE90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AE94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AE98: 386A8968  addi r3, r10, -0x7698
	ctx.r[3].s64 = ctx.r[10].s64 + -30360;
	// 8265AE9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AEA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AEA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AEA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AEAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AEB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AEB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AEB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AEBC: 4BE0BF65  bl 0x82466e20
	ctx.lr = 0x8265AEC0;
	sub_82466E20(ctx, base);
	// 8265AEC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AED0 size=112
    let mut pc: u32 = 0x8265AED0;
    'dispatch: loop {
        match pc {
            0x8265AED0 => {
    //   block [0x8265AED0..0x8265AF40)
	// 8265AED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AED4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AED8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AEDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AEE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AEE4: 38AA8938  addi r5, r10, -0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + -30408;
	// 8265AEE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AEEC: 390BE360  addi r8, r11, -0x1ca0
	ctx.r[8].s64 = ctx.r[11].s64 + -7328;
	// 8265AEF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265AEF4: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 8265AEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AEFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AF00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AF08: 386A8998  addi r3, r10, -0x7668
	ctx.r[3].s64 = ctx.r[10].s64 + -30312;
	// 8265AF0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AF10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AF14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AF18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AF20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AF24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AF28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AF2C: 4BE0BEF5  bl 0x82466e20
	ctx.lr = 0x8265AF30;
	sub_82466E20(ctx, base);
	// 8265AF30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AF34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AF38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AF3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AF40 size=100
    let mut pc: u32 = 0x8265AF40;
    'dispatch: loop {
        match pc {
            0x8265AF40 => {
    //   block [0x8265AF40..0x8265AFA4)
	// 8265AF40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AF44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AF48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AF4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AF50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AF54: 38AA8938  addi r5, r10, -0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + -30408;
	// 8265AF58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AF5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AF60: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8265AF64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AF74: 386A89C8  addi r3, r10, -0x7638
	ctx.r[3].s64 = ctx.r[10].s64 + -30264;
	// 8265AF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265AF7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AF80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265AF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AF88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265AF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265AF90: 4BE0BE91  bl 0x82466e20
	ctx.lr = 0x8265AF94;
	sub_82466E20(ctx, base);
	// 8265AF94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265AF98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265AF9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265AFA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265AFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265AFA8 size=112
    let mut pc: u32 = 0x8265AFA8;
    'dispatch: loop {
        match pc {
            0x8265AFA8 => {
    //   block [0x8265AFA8..0x8265B018)
	// 8265AFA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265AFAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265AFB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265AFB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AFB8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265AFBC: 38AA8938  addi r5, r10, -0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + -30408;
	// 8265AFC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265AFC4: 390BE390  addi r8, r11, -0x1c70
	ctx.r[8].s64 = ctx.r[11].s64 + -7280;
	// 8265AFC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265AFCC: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 8265AFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265AFD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265AFD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265AFDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265AFE0: 386A89F8  addi r3, r10, -0x7608
	ctx.r[3].s64 = ctx.r[10].s64 + -30216;
	// 8265AFE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265AFE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265AFEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265AFF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265AFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265AFF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265AFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B004: 4BE0BE1D  bl 0x82466e20
	ctx.lr = 0x8265B008;
	sub_82466E20(ctx, base);
	// 8265B008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B00C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B018 size=108
    let mut pc: u32 = 0x8265B018;
    'dispatch: loop {
        match pc {
            0x8265B018 => {
    //   block [0x8265B018..0x8265B084)
	// 8265B018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B01C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B020: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B024: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B028: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265B02C: 38EBE3A8  addi r7, r11, -0x1c58
	ctx.r[7].s64 = ctx.r[11].s64 + -7256;
	// 8265B030: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265B034: 388A0E24  addi r4, r10, 0xe24
	ctx.r[4].s64 = ctx.r[10].s64 + 3620;
	// 8265B038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B03C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B040: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B048: 386A8A28  addi r3, r10, -0x75d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30168;
	// 8265B04C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B05C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B06C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B070: 4BE0BDB1  bl 0x82466e20
	ctx.lr = 0x8265B074;
	sub_82466E20(ctx, base);
	// 8265B074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B078: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B07C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B088 size=112
    let mut pc: u32 = 0x8265B088;
    'dispatch: loop {
        match pc {
            0x8265B088 => {
    //   block [0x8265B088..0x8265B0F8)
	// 8265B088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B094: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265B098: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B09C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265B0A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B0A4: 390BE3F0  addi r8, r11, -0x1c10
	ctx.r[8].s64 = ctx.r[11].s64 + -7184;
	// 8265B0A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265B0AC: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 8265B0B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B0B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B0B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B0BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B0C0: 386A8A58  addi r3, r10, -0x75a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30120;
	// 8265B0C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B0C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B0D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B0D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B0D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B0DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B0E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B0E4: 4BE0BD3D  bl 0x82466e20
	ctx.lr = 0x8265B0E8;
	sub_82466E20(ctx, base);
	// 8265B0E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B0EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B0F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B0F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B0F8 size=112
    let mut pc: u32 = 0x8265B0F8;
    'dispatch: loop {
        match pc {
            0x8265B0F8 => {
    //   block [0x8265B0F8..0x8265B168)
	// 8265B0F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B0FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B104: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B108: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B10C: 38AA8A58  addi r5, r10, -0x75a8
	ctx.r[5].s64 = ctx.r[10].s64 + -30120;
	// 8265B110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B114: 390BE450  addi r8, r11, -0x1bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -7088;
	// 8265B118: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B11C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 8265B120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B124: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B12C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B130: 386A8A88  addi r3, r10, -0x7578
	ctx.r[3].s64 = ctx.r[10].s64 + -30072;
	// 8265B134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B154: 4BE0BCCD  bl 0x82466e20
	ctx.lr = 0x8265B158;
	sub_82466E20(ctx, base);
	// 8265B158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B15C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B168 size=112
    let mut pc: u32 = 0x8265B168;
    'dispatch: loop {
        match pc {
            0x8265B168 => {
    //   block [0x8265B168..0x8265B1D8)
	// 8265B168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B16C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B174: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B178: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B17C: 38AA8A58  addi r5, r10, -0x75a8
	ctx.r[5].s64 = ctx.r[10].s64 + -30120;
	// 8265B180: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B184: 390BE468  addi r8, r11, -0x1b98
	ctx.r[8].s64 = ctx.r[11].s64 + -7064;
	// 8265B188: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265B18C: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 8265B190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B194: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B198: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B19C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B1A0: 386A8AB8  addi r3, r10, -0x7548
	ctx.r[3].s64 = ctx.r[10].s64 + -30024;
	// 8265B1A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B1A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B1B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B1B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B1B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B1BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B1C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B1C4: 4BE0BC5D  bl 0x82466e20
	ctx.lr = 0x8265B1C8;
	sub_82466E20(ctx, base);
	// 8265B1C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B1CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B1D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B1D8 size=112
    let mut pc: u32 = 0x8265B1D8;
    'dispatch: loop {
        match pc {
            0x8265B1D8 => {
    //   block [0x8265B1D8..0x8265B248)
	// 8265B1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B1E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B1E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B1E8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B1EC: 38AA8A58  addi r5, r10, -0x75a8
	ctx.r[5].s64 = ctx.r[10].s64 + -30120;
	// 8265B1F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B1F4: 390BE498  addi r8, r11, -0x1b68
	ctx.r[8].s64 = ctx.r[11].s64 + -7016;
	// 8265B1F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B1FC: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 8265B200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B204: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B208: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B20C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B210: 386A8AE8  addi r3, r10, -0x7518
	ctx.r[3].s64 = ctx.r[10].s64 + -29976;
	// 8265B214: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B218: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B220: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B228: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B22C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B230: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B234: 4BE0BBED  bl 0x82466e20
	ctx.lr = 0x8265B238;
	sub_82466E20(ctx, base);
	// 8265B238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265B248 size=24
    let mut pc: u32 = 0x8265B248;
    'dispatch: loop {
        match pc {
            0x8265B248 => {
    //   block [0x8265B248..0x8265B260)
	// 8265B248: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B24C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265B250: 394A2400  addi r10, r10, 0x2400
	ctx.r[10].s64 = ctx.r[10].s64 + 9216;
	// 8265B254: 816BE014  lwz r11, -0x1fec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8172 as u32) ) } as u64;
	// 8265B258: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265B25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B260 size=112
    let mut pc: u32 = 0x8265B260;
    'dispatch: loop {
        match pc {
            0x8265B260 => {
    //   block [0x8265B260..0x8265B2D0)
	// 8265B260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B26C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B270: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B274: 392AD830  addi r9, r10, -0x27d0
	ctx.r[9].s64 = ctx.r[10].s64 + -10192;
	// 8265B278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B27C: 390B2400  addi r8, r11, 0x2400
	ctx.r[8].s64 = ctx.r[11].s64 + 9216;
	// 8265B280: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265B284: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 8265B288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B28C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B298: 386A8B18  addi r3, r10, -0x74e8
	ctx.r[3].s64 = ctx.r[10].s64 + -29928;
	// 8265B29C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B2A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B2A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B2BC: 4BE0BB65  bl 0x82466e20
	ctx.lr = 0x8265B2C0;
	sub_82466E20(ctx, base);
	// 8265B2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B2D0 size=108
    let mut pc: u32 = 0x8265B2D0;
    'dispatch: loop {
        match pc {
            0x8265B2D0 => {
    //   block [0x8265B2D0..0x8265B33C)
	// 8265B2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B2DC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B2E4: 38EBE4B0  addi r7, r11, -0x1b50
	ctx.r[7].s64 = ctx.r[11].s64 + -6992;
	// 8265B2E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265B2EC: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 8265B2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B2F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B2F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B2FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B300: 386A8B48  addi r3, r10, -0x74b8
	ctx.r[3].s64 = ctx.r[10].s64 + -29880;
	// 8265B304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B30C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B328: 4BE0BAF9  bl 0x82466e20
	ctx.lr = 0x8265B32C;
	sub_82466E20(ctx, base);
	// 8265B32C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B340 size=108
    let mut pc: u32 = 0x8265B340;
    'dispatch: loop {
        match pc {
            0x8265B340 => {
    //   block [0x8265B340..0x8265B3AC)
	// 8265B340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B34C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B354: 38EBE4C8  addi r7, r11, -0x1b38
	ctx.r[7].s64 = ctx.r[11].s64 + -6968;
	// 8265B358: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265B35C: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 8265B360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B370: 386A8B78  addi r3, r10, -0x7488
	ctx.r[3].s64 = ctx.r[10].s64 + -29832;
	// 8265B374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B398: 4BE0BA89  bl 0x82466e20
	ctx.lr = 0x8265B39C;
	sub_82466E20(ctx, base);
	// 8265B39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B3B0 size=116
    let mut pc: u32 = 0x8265B3B0;
    'dispatch: loop {
        match pc {
            0x8265B3B0 => {
    //   block [0x8265B3B0..0x8265B424)
	// 8265B3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B3BC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B3C0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B3C4: 390BE514  addi r8, r11, -0x1aec
	ctx.r[8].s64 = ctx.r[11].s64 + -6892;
	// 8265B3C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B3CC: 392AD8E8  addi r9, r10, -0x2718
	ctx.r[9].s64 = ctx.r[10].s64 + -10008;
	// 8265B3D0: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265B3D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265B3D8: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265B3DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B3E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B3F4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265B3F8: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 8265B3FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B400: 386B8BA8  addi r3, r11, -0x7458
	ctx.r[3].s64 = ctx.r[11].s64 + -29784;
	// 8265B404: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B408: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B410: 4BE0BA11  bl 0x82466e20
	ctx.lr = 0x8265B414;
	sub_82466E20(ctx, base);
	// 8265B414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B418: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B41C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265B428 size=24
    let mut pc: u32 = 0x8265B428;
    'dispatch: loop {
        match pc {
            0x8265B428 => {
    //   block [0x8265B428..0x8265B440)
	// 8265B428: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B42C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265B430: 394A2448  addi r10, r10, 0x2448
	ctx.r[10].s64 = ctx.r[10].s64 + 9288;
	// 8265B434: 816BE52C  lwz r11, -0x1ad4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6868 as u32) ) } as u64;
	// 8265B438: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8265B43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B440 size=116
    let mut pc: u32 = 0x8265B440;
    'dispatch: loop {
        match pc {
            0x8265B440 => {
    //   block [0x8265B440..0x8265B4B4)
	// 8265B440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B44C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B450: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B454: 390B2448  addi r8, r11, 0x2448
	ctx.r[8].s64 = ctx.r[11].s64 + 9288;
	// 8265B458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B45C: 392AD944  addi r9, r10, -0x26bc
	ctx.r[9].s64 = ctx.r[10].s64 + -9916;
	// 8265B460: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265B464: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8265B468: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265B46C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B474: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B47C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B484: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265B488: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 8265B48C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B490: 386B8BD8  addi r3, r11, -0x7428
	ctx.r[3].s64 = ctx.r[11].s64 + -29736;
	// 8265B494: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8265B498: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B49C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B4A0: 4BE0B981  bl 0x82466e20
	ctx.lr = 0x8265B4A4;
	sub_82466E20(ctx, base);
	// 8265B4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B4B8 size=108
    let mut pc: u32 = 0x8265B4B8;
    'dispatch: loop {
        match pc {
            0x8265B4B8 => {
    //   block [0x8265B4B8..0x8265B524)
	// 8265B4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B4C4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B4C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B4CC: 38EBE538  addi r7, r11, -0x1ac8
	ctx.r[7].s64 = ctx.r[11].s64 + -6856;
	// 8265B4D0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265B4D4: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 8265B4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B4DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B4E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B4E8: 386A8C08  addi r3, r10, -0x73f8
	ctx.r[3].s64 = ctx.r[10].s64 + -29688;
	// 8265B4EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B50C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B510: 4BE0B911  bl 0x82466e20
	ctx.lr = 0x8265B514;
	sub_82466E20(ctx, base);
	// 8265B514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B528 size=112
    let mut pc: u32 = 0x8265B528;
    'dispatch: loop {
        match pc {
            0x8265B528 => {
    //   block [0x8265B528..0x8265B598)
	// 8265B528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B534: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B538: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B53C: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B540: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B544: 390BE568  addi r8, r11, -0x1a98
	ctx.r[8].s64 = ctx.r[11].s64 + -6808;
	// 8265B548: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B54C: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 8265B550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B554: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B560: 386A8C38  addi r3, r10, -0x73c8
	ctx.r[3].s64 = ctx.r[10].s64 + -29640;
	// 8265B564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B584: 4BE0B89D  bl 0x82466e20
	ctx.lr = 0x8265B588;
	sub_82466E20(ctx, base);
	// 8265B588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B58C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B598 size=112
    let mut pc: u32 = 0x8265B598;
    'dispatch: loop {
        match pc {
            0x8265B598 => {
    //   block [0x8265B598..0x8265B608)
	// 8265B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B5A4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B5A8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B5AC: 392AD988  addi r9, r10, -0x2678
	ctx.r[9].s64 = ctx.r[10].s64 + -9848;
	// 8265B5B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B5B4: 390BE588  addi r8, r11, -0x1a78
	ctx.r[8].s64 = ctx.r[11].s64 + -6776;
	// 8265B5B8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265B5BC: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 8265B5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B5C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B5D0: 386A8C68  addi r3, r10, -0x7398
	ctx.r[3].s64 = ctx.r[10].s64 + -29592;
	// 8265B5D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B5D8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B5DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B5EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B5F4: 4BE0B82D  bl 0x82466e20
	ctx.lr = 0x8265B5F8;
	sub_82466E20(ctx, base);
	// 8265B5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B608 size=112
    let mut pc: u32 = 0x8265B608;
    'dispatch: loop {
        match pc {
            0x8265B608 => {
    //   block [0x8265B608..0x8265B678)
	// 8265B608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B614: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B618: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B61C: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B624: 390BE5D0  addi r8, r11, -0x1a30
	ctx.r[8].s64 = ctx.r[11].s64 + -6704;
	// 8265B628: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B62C: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 8265B630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B634: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B640: 386A8C98  addi r3, r10, -0x7368
	ctx.r[3].s64 = ctx.r[10].s64 + -29544;
	// 8265B644: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B664: 4BE0B7BD  bl 0x82466e20
	ctx.lr = 0x8265B668;
	sub_82466E20(ctx, base);
	// 8265B668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B678 size=112
    let mut pc: u32 = 0x8265B678;
    'dispatch: loop {
        match pc {
            0x8265B678 => {
    //   block [0x8265B678..0x8265B6E8)
	// 8265B678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B684: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B688: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B68C: 392AD9B4  addi r9, r10, -0x264c
	ctx.r[9].s64 = ctx.r[10].s64 + -9804;
	// 8265B690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B694: 390BE5E8  addi r8, r11, -0x1a18
	ctx.r[8].s64 = ctx.r[11].s64 + -6680;
	// 8265B698: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8265B69C: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 8265B6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B6A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B6A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B6B0: 386A8CC8  addi r3, r10, -0x7338
	ctx.r[3].s64 = ctx.r[10].s64 + -29496;
	// 8265B6B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B6B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B6D4: 4BE0B74D  bl 0x82466e20
	ctx.lr = 0x8265B6D8;
	sub_82466E20(ctx, base);
	// 8265B6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B6E8 size=112
    let mut pc: u32 = 0x8265B6E8;
    'dispatch: loop {
        match pc {
            0x8265B6E8 => {
    //   block [0x8265B6E8..0x8265B758)
	// 8265B6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B6F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B6F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B6FC: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B704: 390BE678  addi r8, r11, -0x1988
	ctx.r[8].s64 = ctx.r[11].s64 + -6536;
	// 8265B708: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B70C: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 8265B710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B714: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B71C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B720: 386A8CF8  addi r3, r10, -0x7308
	ctx.r[3].s64 = ctx.r[10].s64 + -29448;
	// 8265B724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B744: 4BE0B6DD  bl 0x82466e20
	ctx.lr = 0x8265B748;
	sub_82466E20(ctx, base);
	// 8265B748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B758 size=112
    let mut pc: u32 = 0x8265B758;
    'dispatch: loop {
        match pc {
            0x8265B758 => {
    //   block [0x8265B758..0x8265B7C8)
	// 8265B758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B764: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B768: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B76C: 38AA8D58  addi r5, r10, -0x72a8
	ctx.r[5].s64 = ctx.r[10].s64 + -29352;
	// 8265B770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B774: 390BE690  addi r8, r11, -0x1970
	ctx.r[8].s64 = ctx.r[11].s64 + -6512;
	// 8265B778: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8265B77C: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 8265B780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B784: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B790: 386A8D28  addi r3, r10, -0x72d8
	ctx.r[3].s64 = ctx.r[10].s64 + -29400;
	// 8265B794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B7B4: 4BE0B66D  bl 0x82466e20
	ctx.lr = 0x8265B7B8;
	sub_82466E20(ctx, base);
	// 8265B7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B7C8 size=100
    let mut pc: u32 = 0x8265B7C8;
    'dispatch: loop {
        match pc {
            0x8265B7C8 => {
    //   block [0x8265B7C8..0x8265B82C)
	// 8265B7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B7D4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265B7D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B7DC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265B7E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B7E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B7E8: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 8265B7EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B7FC: 386A8D58  addi r3, r10, -0x72a8
	ctx.r[3].s64 = ctx.r[10].s64 + -29352;
	// 8265B800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B804: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B808: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265B80C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B810: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265B814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B818: 4BE0B609  bl 0x82466e20
	ctx.lr = 0x8265B81C;
	sub_82466E20(ctx, base);
	// 8265B81C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265B830 size=24
    let mut pc: u32 = 0x8265B830;
    'dispatch: loop {
        match pc {
            0x8265B830 => {
    //   block [0x8265B830..0x8265B848)
	// 8265B830: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B834: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265B838: 394A2520  addi r10, r10, 0x2520
	ctx.r[10].s64 = ctx.r[10].s64 + 9504;
	// 8265B83C: 816BE708  lwz r11, -0x18f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6392 as u32) ) } as u64;
	// 8265B840: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8265B844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B848 size=116
    let mut pc: u32 = 0x8265B848;
    'dispatch: loop {
        match pc {
            0x8265B848 => {
    //   block [0x8265B848..0x8265B8BC)
	// 8265B848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B84C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B854: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B858: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B85C: 390B2520  addi r8, r11, 0x2520
	ctx.r[8].s64 = ctx.r[11].s64 + 9504;
	// 8265B860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B864: 392AD9F0  addi r9, r10, -0x2610
	ctx.r[9].s64 = ctx.r[10].s64 + -9744;
	// 8265B868: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B86C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8265B870: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B874: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B87C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B88C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265B890: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 8265B894: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B898: 386B8D88  addi r3, r11, -0x7278
	ctx.r[3].s64 = ctx.r[11].s64 + -29304;
	// 8265B89C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B8A0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B8A8: 4BE0B579  bl 0x82466e20
	ctx.lr = 0x8265B8AC;
	sub_82466E20(ctx, base);
	// 8265B8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B8C0 size=108
    let mut pc: u32 = 0x8265B8C0;
    'dispatch: loop {
        match pc {
            0x8265B8C0 => {
    //   block [0x8265B8C0..0x8265B92C)
	// 8265B8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B8CC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B8D4: 38EBE70C  addi r7, r11, -0x18f4
	ctx.r[7].s64 = ctx.r[11].s64 + -6388;
	// 8265B8D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265B8DC: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 8265B8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265B8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B8F0: 386A8DB8  addi r3, r10, -0x7248
	ctx.r[3].s64 = ctx.r[10].s64 + -29256;
	// 8265B8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265B8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B918: 4BE0B509  bl 0x82466e20
	ctx.lr = 0x8265B91C;
	sub_82466E20(ctx, base);
	// 8265B91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B930 size=112
    let mut pc: u32 = 0x8265B930;
    'dispatch: loop {
        match pc {
            0x8265B930 => {
    //   block [0x8265B930..0x8265B9A0)
	// 8265B930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B93C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B940: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B944: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265B948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B94C: 390BE73C  addi r8, r11, -0x18c4
	ctx.r[8].s64 = ctx.r[11].s64 + -6340;
	// 8265B950: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265B954: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 8265B958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B95C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B968: 386A8DE8  addi r3, r10, -0x7218
	ctx.r[3].s64 = ctx.r[10].s64 + -29208;
	// 8265B96C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265B970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265B974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265B978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B98C: 4BE0B495  bl 0x82466e20
	ctx.lr = 0x8265B990;
	sub_82466E20(ctx, base);
	// 8265B990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265B994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265B998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265B99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265B9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265B9A0 size=112
    let mut pc: u32 = 0x8265B9A0;
    'dispatch: loop {
        match pc {
            0x8265B9A0 => {
    //   block [0x8265B9A0..0x8265BA10)
	// 8265B9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265B9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265B9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265B9AC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265B9B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265B9B4: 392ADA14  addi r9, r10, -0x25ec
	ctx.r[9].s64 = ctx.r[10].s64 + -9708;
	// 8265B9B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265B9BC: 390BE758  addi r8, r11, -0x18a8
	ctx.r[8].s64 = ctx.r[11].s64 + -6312;
	// 8265B9C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265B9C4: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 8265B9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265B9CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265B9D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265B9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265B9D8: 386A8E18  addi r3, r10, -0x71e8
	ctx.r[3].s64 = ctx.r[10].s64 + -29160;
	// 8265B9DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265B9E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265B9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265B9E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265B9EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265B9F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265B9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265B9F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265B9FC: 4BE0B425  bl 0x82466e20
	ctx.lr = 0x8265BA00;
	sub_82466E20(ctx, base);
	// 8265BA00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BA04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BA08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BA0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BA10 size=112
    let mut pc: u32 = 0x8265BA10;
    'dispatch: loop {
        match pc {
            0x8265BA10 => {
    //   block [0x8265BA10..0x8265BA80)
	// 8265BA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BA14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BA18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BA1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BA20: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BA24: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BA28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BA2C: 390BE800  addi r8, r11, -0x1800
	ctx.r[8].s64 = ctx.r[11].s64 + -6144;
	// 8265BA30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265BA34: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 8265BA38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BA3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BA40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BA44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BA48: 386A8E48  addi r3, r10, -0x71b8
	ctx.r[3].s64 = ctx.r[10].s64 + -29112;
	// 8265BA4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BA50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BA6C: 4BE0B3B5  bl 0x82466e20
	ctx.lr = 0x8265BA70;
	sub_82466E20(ctx, base);
	// 8265BA70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BA74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BA78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BA80 size=108
    let mut pc: u32 = 0x8265BA80;
    'dispatch: loop {
        match pc {
            0x8265BA80 => {
    //   block [0x8265BA80..0x8265BAEC)
	// 8265BA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BA88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BA8C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BA90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BA94: 38EBE818  addi r7, r11, -0x17e8
	ctx.r[7].s64 = ctx.r[11].s64 + -6120;
	// 8265BA98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265BA9C: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 8265BAA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BAA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BAA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265BAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BAB0: 386A8E78  addi r3, r10, -0x7188
	ctx.r[3].s64 = ctx.r[10].s64 + -29064;
	// 8265BAB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265BAB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BAC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BAD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265BAD8: 4BE0B349  bl 0x82466e20
	ctx.lr = 0x8265BADC;
	sub_82466E20(ctx, base);
	// 8265BADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BAE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BAE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BAE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BAF0 size=112
    let mut pc: u32 = 0x8265BAF0;
    'dispatch: loop {
        match pc {
            0x8265BAF0 => {
    //   block [0x8265BAF0..0x8265BB60)
	// 8265BAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BAFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BB00: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BB04: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BB0C: 390BE848  addi r8, r11, -0x17b8
	ctx.r[8].s64 = ctx.r[11].s64 + -6072;
	// 8265BB10: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265BB14: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 8265BB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BB1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BB20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BB24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BB28: 386A8EA8  addi r3, r10, -0x7158
	ctx.r[3].s64 = ctx.r[10].s64 + -29016;
	// 8265BB2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BB4C: 4BE0B2D5  bl 0x82466e20
	ctx.lr = 0x8265BB50;
	sub_82466E20(ctx, base);
	// 8265BB50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BB54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BB58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BB5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BB60 size=112
    let mut pc: u32 = 0x8265BB60;
    'dispatch: loop {
        match pc {
            0x8265BB60 => {
    //   block [0x8265BB60..0x8265BBD0)
	// 8265BB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BB64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BB68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BB6C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265BB70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BB74: 392ADA48  addi r9, r10, -0x25b8
	ctx.r[9].s64 = ctx.r[10].s64 + -9656;
	// 8265BB78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BB7C: 390BE868  addi r8, r11, -0x1798
	ctx.r[8].s64 = ctx.r[11].s64 + -6040;
	// 8265BB80: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265BB84: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 8265BB88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BB8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BB90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BB98: 386A8ED8  addi r3, r10, -0x7128
	ctx.r[3].s64 = ctx.r[10].s64 + -28968;
	// 8265BB9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265BBA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265BBA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BBA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BBB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BBB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265BBB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BBBC: 4BE0B265  bl 0x82466e20
	ctx.lr = 0x8265BBC0;
	sub_82466E20(ctx, base);
	// 8265BBC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BBC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BBC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BBCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BBD0 size=112
    let mut pc: u32 = 0x8265BBD0;
    'dispatch: loop {
        match pc {
            0x8265BBD0 => {
    //   block [0x8265BBD0..0x8265BC40)
	// 8265BBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BBDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BBE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BBE4: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BBE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BBEC: 390BE910  addi r8, r11, -0x16f0
	ctx.r[8].s64 = ctx.r[11].s64 + -5872;
	// 8265BBF0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265BBF4: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 8265BBF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BBFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BC00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BC08: 386A8F08  addi r3, r10, -0x70f8
	ctx.r[3].s64 = ctx.r[10].s64 + -28920;
	// 8265BC0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BC10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BC18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BC20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BC24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BC28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BC2C: 4BE0B1F5  bl 0x82466e20
	ctx.lr = 0x8265BC30;
	sub_82466E20(ctx, base);
	// 8265BC30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BC3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BC40 size=112
    let mut pc: u32 = 0x8265BC40;
    'dispatch: loop {
        match pc {
            0x8265BC40 => {
    //   block [0x8265BC40..0x8265BCB0)
	// 8265BC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BC44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BC48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BC4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BC50: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BC54: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BC58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BC5C: 390BE958  addi r8, r11, -0x16a8
	ctx.r[8].s64 = ctx.r[11].s64 + -5800;
	// 8265BC60: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8265BC64: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 8265BC68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BC6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BC70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BC74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BC78: 386A8F38  addi r3, r10, -0x70c8
	ctx.r[3].s64 = ctx.r[10].s64 + -28872;
	// 8265BC7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BC80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BC94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BC9C: 4BE0B185  bl 0x82466e20
	ctx.lr = 0x8265BCA0;
	sub_82466E20(ctx, base);
	// 8265BCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BCA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BCA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BCAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BCB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BCB0 size=100
    let mut pc: u32 = 0x8265BCB0;
    'dispatch: loop {
        match pc {
            0x8265BCB0 => {
    //   block [0x8265BCB0..0x8265BD14)
	// 8265BCB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BCB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BCB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BCBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BCC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BCC4: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BCC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BCCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BCD0: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8265BCD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BCE4: 386A8F68  addi r3, r10, -0x7098
	ctx.r[3].s64 = ctx.r[10].s64 + -28824;
	// 8265BCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BCEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BCF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265BCF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BCF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265BCFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BD00: 4BE0B121  bl 0x82466e20
	ctx.lr = 0x8265BD04;
	sub_82466E20(ctx, base);
	// 8265BD04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BD18 size=112
    let mut pc: u32 = 0x8265BD18;
    'dispatch: loop {
        match pc {
            0x8265BD18 => {
    //   block [0x8265BD18..0x8265BD88)
	// 8265BD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BD20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BD24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BD28: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BD2C: 38AA8BD8  addi r5, r10, -0x7428
	ctx.r[5].s64 = ctx.r[10].s64 + -29736;
	// 8265BD30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BD34: 390BEA18  addi r8, r11, -0x15e8
	ctx.r[8].s64 = ctx.r[11].s64 + -5608;
	// 8265BD38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265BD3C: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 8265BD40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BD44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BD48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BD4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BD50: 386A8F98  addi r3, r10, -0x7068
	ctx.r[3].s64 = ctx.r[10].s64 + -28776;
	// 8265BD54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BD58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BD60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BD64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BD68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BD6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BD70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BD74: 4BE0B0AD  bl 0x82466e20
	ctx.lr = 0x8265BD78;
	sub_82466E20(ctx, base);
	// 8265BD78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BD7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BD80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BD88 size=112
    let mut pc: u32 = 0x8265BD88;
    'dispatch: loop {
        match pc {
            0x8265BD88 => {
    //   block [0x8265BD88..0x8265BDF8)
	// 8265BD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BD90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BD94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BD98: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BD9C: 38AA8A58  addi r5, r10, -0x75a8
	ctx.r[5].s64 = ctx.r[10].s64 + -30120;
	// 8265BDA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BDA4: 390BEA48  addi r8, r11, -0x15b8
	ctx.r[8].s64 = ctx.r[11].s64 + -5560;
	// 8265BDA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265BDAC: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 8265BDB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BDB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BDB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BDC0: 386A8FC8  addi r3, r10, -0x7038
	ctx.r[3].s64 = ctx.r[10].s64 + -28728;
	// 8265BDC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BDC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BDD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BDD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BDD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BDDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BDE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BDE4: 4BE0B03D  bl 0x82466e20
	ctx.lr = 0x8265BDE8;
	sub_82466E20(ctx, base);
	// 8265BDE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BDEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BDF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BDF8 size=108
    let mut pc: u32 = 0x8265BDF8;
    'dispatch: loop {
        match pc {
            0x8265BDF8 => {
    //   block [0x8265BDF8..0x8265BE64)
	// 8265BDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BE00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BE04: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BE08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BE0C: 38EBEA60  addi r7, r11, -0x15a0
	ctx.r[7].s64 = ctx.r[11].s64 + -5536;
	// 8265BE10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265BE14: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 8265BE18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BE1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BE20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265BE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BE28: 386A8FF8  addi r3, r10, -0x7008
	ctx.r[3].s64 = ctx.r[10].s64 + -28680;
	// 8265BE2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265BE30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BE34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BE38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BE3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BE40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BE44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BE48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BE4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265BE50: 4BE0AFD1  bl 0x82466e20
	ctx.lr = 0x8265BE54;
	sub_82466E20(ctx, base);
	// 8265BE54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BE60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BE68 size=112
    let mut pc: u32 = 0x8265BE68;
    'dispatch: loop {
        match pc {
            0x8265BE68 => {
    //   block [0x8265BE68..0x8265BED8)
	// 8265BE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BE70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BE74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BE78: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BE7C: 38AA8F68  addi r5, r10, -0x7098
	ctx.r[5].s64 = ctx.r[10].s64 + -28824;
	// 8265BE80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BE84: 390BEA90  addi r8, r11, -0x1570
	ctx.r[8].s64 = ctx.r[11].s64 + -5488;
	// 8265BE88: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8265BE8C: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 8265BE90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BE94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BE98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BE9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BEA0: 386A9028  addi r3, r10, -0x6fd8
	ctx.r[3].s64 = ctx.r[10].s64 + -28632;
	// 8265BEA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BEA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BEB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BEB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BEB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BEC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BEC4: 4BE0AF5D  bl 0x82466e20
	ctx.lr = 0x8265BEC8;
	sub_82466E20(ctx, base);
	// 8265BEC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BECC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BED0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BED8 size=112
    let mut pc: u32 = 0x8265BED8;
    'dispatch: loop {
        match pc {
            0x8265BED8 => {
    //   block [0x8265BED8..0x8265BF48)
	// 8265BED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BEE4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265BEE8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BEEC: 392ADA74  addi r9, r10, -0x258c
	ctx.r[9].s64 = ctx.r[10].s64 + -9612;
	// 8265BEF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BEF4: 390BEB20  addi r8, r11, -0x14e0
	ctx.r[8].s64 = ctx.r[11].s64 + -5344;
	// 8265BEF8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265BEFC: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 8265BF00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BF04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BF08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BF10: 386A9058  addi r3, r10, -0x6fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -28584;
	// 8265BF14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265BF18: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265BF1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BF20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BF28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BF2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265BF30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BF34: 4BE0AEED  bl 0x82466e20
	ctx.lr = 0x8265BF38;
	sub_82466E20(ctx, base);
	// 8265BF38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BF3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BF40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BF48 size=112
    let mut pc: u32 = 0x8265BF48;
    'dispatch: loop {
        match pc {
            0x8265BF48 => {
    //   block [0x8265BF48..0x8265BFB8)
	// 8265BF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BF54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BF58: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BF5C: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265BF60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BF64: 390BEB68  addi r8, r11, -0x1498
	ctx.r[8].s64 = ctx.r[11].s64 + -5272;
	// 8265BF68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265BF6C: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 8265BF70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BF74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BF78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265BF7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265BF80: 386A9088  addi r3, r10, -0x6f78
	ctx.r[3].s64 = ctx.r[10].s64 + -28536;
	// 8265BF84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265BF88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BF90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BF98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265BF9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265BFA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265BFA4: 4BE0AE7D  bl 0x82466e20
	ctx.lr = 0x8265BFA8;
	sub_82466E20(ctx, base);
	// 8265BFA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265BFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265BFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265BFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265BFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265BFB8 size=108
    let mut pc: u32 = 0x8265BFB8;
    'dispatch: loop {
        match pc {
            0x8265BFB8 => {
    //   block [0x8265BFB8..0x8265C024)
	// 8265BFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265BFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265BFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265BFC4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265BFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265BFCC: 38EBEB80  addi r7, r11, -0x1480
	ctx.r[7].s64 = ctx.r[11].s64 + -5248;
	// 8265BFD0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265BFD4: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 8265BFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265BFDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265BFE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265BFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265BFE8: 386A90B8  addi r3, r10, -0x6f48
	ctx.r[3].s64 = ctx.r[10].s64 + -28488;
	// 8265BFEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265BFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265BFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265BFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265BFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C010: 4BE0AE11  bl 0x82466e20
	ctx.lr = 0x8265C014;
	sub_82466E20(ctx, base);
	// 8265C014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C028 size=116
    let mut pc: u32 = 0x8265C028;
    'dispatch: loop {
        match pc {
            0x8265C028 => {
    //   block [0x8265C028..0x8265C09C)
	// 8265C028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C034: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265C038: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8265C03C: 390AEC10  addi r8, r10, -0x13f0
	ctx.r[8].s64 = ctx.r[10].s64 + -5104;
	// 8265C040: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C044: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265C048: 38AA8F68  addi r5, r10, -0x7098
	ctx.r[5].s64 = ctx.r[10].s64 + -28824;
	// 8265C04C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C050: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265C054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C058: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C05C: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 8265C060: 396BDA88  addi r11, r11, -0x2578
	ctx.r[11].s64 = ctx.r[11].s64 + -9592;
	// 8265C064: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C068: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C06C: 386A90E8  addi r3, r10, -0x6f18
	ctx.r[3].s64 = ctx.r[10].s64 + -28440;
	// 8265C070: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265C074: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C078: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265C07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C088: 4BE0AD99  bl 0x82466e20
	ctx.lr = 0x8265C08C;
	sub_82466E20(ctx, base);
	// 8265C08C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C0A0 size=108
    let mut pc: u32 = 0x8265C0A0;
    'dispatch: loop {
        match pc {
            0x8265C0A0 => {
    //   block [0x8265C0A0..0x8265C10C)
	// 8265C0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C0AC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C0B4: 38EBECE8  addi r7, r11, -0x1318
	ctx.r[7].s64 = ctx.r[11].s64 + -4888;
	// 8265C0B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265C0BC: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 8265C0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C0C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C0C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C0D0: 386A9118  addi r3, r10, -0x6ee8
	ctx.r[3].s64 = ctx.r[10].s64 + -28392;
	// 8265C0D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C0E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C0F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C0F8: 4BE0AD29  bl 0x82466e20
	ctx.lr = 0x8265C0FC;
	sub_82466E20(ctx, base);
	// 8265C0FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C110 size=112
    let mut pc: u32 = 0x8265C110;
    'dispatch: loop {
        match pc {
            0x8265C110 => {
    //   block [0x8265C110..0x8265C180)
	// 8265C110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C11C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C120: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C124: 38AA8F68  addi r5, r10, -0x7098
	ctx.r[5].s64 = ctx.r[10].s64 + -28824;
	// 8265C128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C12C: 390BED30  addi r8, r11, -0x12d0
	ctx.r[8].s64 = ctx.r[11].s64 + -4816;
	// 8265C130: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8265C134: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 8265C138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C13C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C148: 386A9148  addi r3, r10, -0x6eb8
	ctx.r[3].s64 = ctx.r[10].s64 + -28344;
	// 8265C14C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C16C: 4BE0ACB5  bl 0x82466e20
	ctx.lr = 0x8265C170;
	sub_82466E20(ctx, base);
	// 8265C170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C180 size=112
    let mut pc: u32 = 0x8265C180;
    'dispatch: loop {
        match pc {
            0x8265C180 => {
    //   block [0x8265C180..0x8265C1F0)
	// 8265C180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C18C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C190: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C194: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265C198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C19C: 390BEDA8  addi r8, r11, -0x1258
	ctx.r[8].s64 = ctx.r[11].s64 + -4696;
	// 8265C1A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265C1A4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 8265C1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C1AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C1B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C1B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C1B8: 386A9178  addi r3, r10, -0x6e88
	ctx.r[3].s64 = ctx.r[10].s64 + -28296;
	// 8265C1BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C1C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C1C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C1C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C1D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C1D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C1DC: 4BE0AC45  bl 0x82466e20
	ctx.lr = 0x8265C1E0;
	sub_82466E20(ctx, base);
	// 8265C1E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C1E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C1E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C1EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C1F0 size=108
    let mut pc: u32 = 0x8265C1F0;
    'dispatch: loop {
        match pc {
            0x8265C1F0 => {
    //   block [0x8265C1F0..0x8265C25C)
	// 8265C1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C1FC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C204: 38EBEDD8  addi r7, r11, -0x1228
	ctx.r[7].s64 = ctx.r[11].s64 + -4648;
	// 8265C208: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265C20C: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 8265C210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C218: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C220: 386A91A8  addi r3, r10, -0x6e58
	ctx.r[3].s64 = ctx.r[10].s64 + -28248;
	// 8265C224: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C244: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C248: 4BE0ABD9  bl 0x82466e20
	ctx.lr = 0x8265C24C;
	sub_82466E20(ctx, base);
	// 8265C24C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C260 size=108
    let mut pc: u32 = 0x8265C260;
    'dispatch: loop {
        match pc {
            0x8265C260 => {
    //   block [0x8265C260..0x8265C2CC)
	// 8265C260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C26C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C274: 38EBEE38  addi r7, r11, -0x11c8
	ctx.r[7].s64 = ctx.r[11].s64 + -4552;
	// 8265C278: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8265C27C: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 8265C280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C290: 386A91D8  addi r3, r10, -0x6e28
	ctx.r[3].s64 = ctx.r[10].s64 + -28200;
	// 8265C294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C29C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C2B8: 4BE0AB69  bl 0x82466e20
	ctx.lr = 0x8265C2BC;
	sub_82466E20(ctx, base);
	// 8265C2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C2D0 size=112
    let mut pc: u32 = 0x8265C2D0;
    'dispatch: loop {
        match pc {
            0x8265C2D0 => {
    //   block [0x8265C2D0..0x8265C340)
	// 8265C2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C2DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C2E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C2E4: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265C2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C2EC: 390BEEB0  addi r8, r11, -0x1150
	ctx.r[8].s64 = ctx.r[11].s64 + -4432;
	// 8265C2F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265C2F4: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 8265C2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C2FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C308: 386A9208  addi r3, r10, -0x6df8
	ctx.r[3].s64 = ctx.r[10].s64 + -28152;
	// 8265C30C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C32C: 4BE0AAF5  bl 0x82466e20
	ctx.lr = 0x8265C330;
	sub_82466E20(ctx, base);
	// 8265C330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265C340 size=24
    let mut pc: u32 = 0x8265C340;
    'dispatch: loop {
        match pc {
            0x8265C340 => {
    //   block [0x8265C340..0x8265C358)
	// 8265C340: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C344: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265C348: 394A2598  addi r10, r10, 0x2598
	ctx.r[10].s64 = ctx.r[10].s64 + 9624;
	// 8265C34C: 816BEEF8  lwz r11, -0x1108(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4360 as u32) ) } as u64;
	// 8265C350: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265C354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C358 size=116
    let mut pc: u32 = 0x8265C358;
    'dispatch: loop {
        match pc {
            0x8265C358 => {
    //   block [0x8265C358..0x8265C3CC)
	// 8265C358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C364: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C368: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265C36C: 390B2598  addi r8, r11, 0x2598
	ctx.r[8].s64 = ctx.r[11].s64 + 9624;
	// 8265C370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C374: 392ADAEC  addi r9, r10, -0x2514
	ctx.r[9].s64 = ctx.r[10].s64 + -9492;
	// 8265C378: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265C37C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265C380: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265C384: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C38C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C39C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265C3A0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 8265C3A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265C3A8: 386B9238  addi r3, r11, -0x6dc8
	ctx.r[3].s64 = ctx.r[11].s64 + -28104;
	// 8265C3AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265C3B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C3B8: 4BE0AA69  bl 0x82466e20
	ctx.lr = 0x8265C3BC;
	sub_82466E20(ctx, base);
	// 8265C3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C3D0 size=112
    let mut pc: u32 = 0x8265C3D0;
    'dispatch: loop {
        match pc {
            0x8265C3D0 => {
    //   block [0x8265C3D0..0x8265C440)
	// 8265C3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C3DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C3E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C3E4: 38AA9238  addi r5, r10, -0x6dc8
	ctx.r[5].s64 = ctx.r[10].s64 + -28104;
	// 8265C3E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C3EC: 390BEEFC  addi r8, r11, -0x1104
	ctx.r[8].s64 = ctx.r[11].s64 + -4356;
	// 8265C3F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265C3F4: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 8265C3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C3FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C408: 386A9268  addi r3, r10, -0x6d98
	ctx.r[3].s64 = ctx.r[10].s64 + -28056;
	// 8265C40C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C42C: 4BE0A9F5  bl 0x82466e20
	ctx.lr = 0x8265C430;
	sub_82466E20(ctx, base);
	// 8265C430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265C440 size=24
    let mut pc: u32 = 0x8265C440;
    'dispatch: loop {
        match pc {
            0x8265C440 => {
    //   block [0x8265C440..0x8265C458)
	// 8265C440: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C444: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265C448: 394A25B0  addi r10, r10, 0x25b0
	ctx.r[10].s64 = ctx.r[10].s64 + 9648;
	// 8265C44C: 816BEF2C  lwz r11, -0x10d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4308 as u32) ) } as u64;
	// 8265C450: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8265C454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C458 size=116
    let mut pc: u32 = 0x8265C458;
    'dispatch: loop {
        match pc {
            0x8265C458 => {
    //   block [0x8265C458..0x8265C4CC)
	// 8265C458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C464: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C468: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265C46C: 390B25B0  addi r8, r11, 0x25b0
	ctx.r[8].s64 = ctx.r[11].s64 + 9648;
	// 8265C470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C474: 392ADB28  addi r9, r10, -0x24d8
	ctx.r[9].s64 = ctx.r[10].s64 + -9432;
	// 8265C478: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C47C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8265C480: 38AA9268  addi r5, r10, -0x6d98
	ctx.r[5].s64 = ctx.r[10].s64 + -28056;
	// 8265C484: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C48C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C494: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C49C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265C4A0: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 8265C4A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265C4A8: 386B9298  addi r3, r11, -0x6d68
	ctx.r[3].s64 = ctx.r[11].s64 + -28008;
	// 8265C4AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265C4B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C4B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C4B8: 4BE0A969  bl 0x82466e20
	ctx.lr = 0x8265C4BC;
	sub_82466E20(ctx, base);
	// 8265C4BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C4C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C4C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C4C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C4D0 size=112
    let mut pc: u32 = 0x8265C4D0;
    'dispatch: loop {
        match pc {
            0x8265C4D0 => {
    //   block [0x8265C4D0..0x8265C540)
	// 8265C4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C4D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C4DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C4E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C4E4: 38AA9268  addi r5, r10, -0x6d98
	ctx.r[5].s64 = ctx.r[10].s64 + -28056;
	// 8265C4E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C4EC: 390BEF30  addi r8, r11, -0x10d0
	ctx.r[8].s64 = ctx.r[11].s64 + -4304;
	// 8265C4F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265C4F4: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 8265C4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C4FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C500: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C508: 386A92C8  addi r3, r10, -0x6d38
	ctx.r[3].s64 = ctx.r[10].s64 + -27960;
	// 8265C50C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C51C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C52C: 4BE0A8F5  bl 0x82466e20
	ctx.lr = 0x8265C530;
	sub_82466E20(ctx, base);
	// 8265C530: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C53C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C540 size=112
    let mut pc: u32 = 0x8265C540;
    'dispatch: loop {
        match pc {
            0x8265C540 => {
    //   block [0x8265C540..0x8265C5B0)
	// 8265C540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C54C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C550: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C554: 38AA9268  addi r5, r10, -0x6d98
	ctx.r[5].s64 = ctx.r[10].s64 + -28056;
	// 8265C558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C55C: 390BEF90  addi r8, r11, -0x1070
	ctx.r[8].s64 = ctx.r[11].s64 + -4208;
	// 8265C560: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265C564: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 8265C568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C56C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C578: 386A92F8  addi r3, r10, -0x6d08
	ctx.r[3].s64 = ctx.r[10].s64 + -27912;
	// 8265C57C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C58C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C59C: 4BE0A885  bl 0x82466e20
	ctx.lr = 0x8265C5A0;
	sub_82466E20(ctx, base);
	// 8265C5A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C5B0 size=112
    let mut pc: u32 = 0x8265C5B0;
    'dispatch: loop {
        match pc {
            0x8265C5B0 => {
    //   block [0x8265C5B0..0x8265C620)
	// 8265C5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C5BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C5C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C5C4: 38AA9268  addi r5, r10, -0x6d98
	ctx.r[5].s64 = ctx.r[10].s64 + -28056;
	// 8265C5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C5CC: 390BEFC0  addi r8, r11, -0x1040
	ctx.r[8].s64 = ctx.r[11].s64 + -4160;
	// 8265C5D0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265C5D4: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 8265C5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C5DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C5E8: 386A9328  addi r3, r10, -0x6cd8
	ctx.r[3].s64 = ctx.r[10].s64 + -27864;
	// 8265C5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C60C: 4BE0A815  bl 0x82466e20
	ctx.lr = 0x8265C610;
	sub_82466E20(ctx, base);
	// 8265C610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C620 size=108
    let mut pc: u32 = 0x8265C620;
    'dispatch: loop {
        match pc {
            0x8265C620 => {
    //   block [0x8265C620..0x8265C68C)
	// 8265C620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C62C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C634: 38EBF008  addi r7, r11, -0xff8
	ctx.r[7].s64 = ctx.r[11].s64 + -4088;
	// 8265C638: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265C63C: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 8265C640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C644: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C650: 386A9358  addi r3, r10, -0x6ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -27816;
	// 8265C654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C678: 4BE0A7A9  bl 0x82466e20
	ctx.lr = 0x8265C67C;
	sub_82466E20(ctx, base);
	// 8265C67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C690 size=112
    let mut pc: u32 = 0x8265C690;
    'dispatch: loop {
        match pc {
            0x8265C690 => {
    //   block [0x8265C690..0x8265C700)
	// 8265C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C69C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C6A0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C6A4: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265C6A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C6AC: 390BF038  addi r8, r11, -0xfc8
	ctx.r[8].s64 = ctx.r[11].s64 + -4040;
	// 8265C6B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265C6B4: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 8265C6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C6BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C6C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265C6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C6C8: 386A9388  addi r3, r10, -0x6c78
	ctx.r[3].s64 = ctx.r[10].s64 + -27768;
	// 8265C6CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265C6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C6EC: 4BE0A735  bl 0x82466e20
	ctx.lr = 0x8265C6F0;
	sub_82466E20(ctx, base);
	// 8265C6F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C700 size=108
    let mut pc: u32 = 0x8265C700;
    'dispatch: loop {
        match pc {
            0x8265C700 => {
    //   block [0x8265C700..0x8265C76C)
	// 8265C700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C70C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C710: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C714: 38EBF058  addi r7, r11, -0xfa8
	ctx.r[7].s64 = ctx.r[11].s64 + -4008;
	// 8265C718: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265C71C: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 8265C720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C724: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C728: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C730: 386A93B8  addi r3, r10, -0x6c48
	ctx.r[3].s64 = ctx.r[10].s64 + -27720;
	// 8265C734: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C738: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C740: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C748: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C74C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C750: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C754: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C758: 4BE0A6C9  bl 0x82466e20
	ctx.lr = 0x8265C75C;
	sub_82466E20(ctx, base);
	// 8265C75C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C760: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C764: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C770 size=108
    let mut pc: u32 = 0x8265C770;
    'dispatch: loop {
        match pc {
            0x8265C770 => {
    //   block [0x8265C770..0x8265C7DC)
	// 8265C770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C77C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C784: 38EBF0A0  addi r7, r11, -0xf60
	ctx.r[7].s64 = ctx.r[11].s64 + -3936;
	// 8265C788: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265C78C: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 8265C790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C794: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C798: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265C79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C7A0: 386A93E8  addi r3, r10, -0x6c18
	ctx.r[3].s64 = ctx.r[10].s64 + -27672;
	// 8265C7A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265C7A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C7B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C7B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C7C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265C7C8: 4BE0A659  bl 0x82466e20
	ctx.lr = 0x8265C7CC;
	sub_82466E20(ctx, base);
	// 8265C7CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C7D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C7D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C7D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C7E0 size=116
    let mut pc: u32 = 0x8265C7E0;
    'dispatch: loop {
        match pc {
            0x8265C7E0 => {
    //   block [0x8265C7E0..0x8265C854)
	// 8265C7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C7EC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265C7F0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C7F4: 392BDB5C  addi r9, r11, -0x24a4
	ctx.r[9].s64 = ctx.r[11].s64 + -9380;
	// 8265C7F8: 38AA9868  addi r5, r10, -0x6798
	ctx.r[5].s64 = ctx.r[10].s64 + -26520;
	// 8265C7FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C800: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8265C804: 38C00012  li r6, 0x12
	ctx.r[6].s64 = 18;
	// 8265C808: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265C80C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 8265C810: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C814: 396BF100  addi r11, r11, -0xf00
	ctx.r[11].s64 = ctx.r[11].s64 + -3840;
	// 8265C818: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265C81C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C820: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8265C824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C828: 386A9418  addi r3, r10, -0x6be8
	ctx.r[3].s64 = ctx.r[10].s64 + -27624;
	// 8265C82C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265C830: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8265C834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C838: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8265C83C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C840: 4BE0A5E1  bl 0x82466e20
	ctx.lr = 0x8265C844;
	sub_82466E20(ctx, base);
	// 8265C844: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C84C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C858 size=100
    let mut pc: u32 = 0x8265C858;
    'dispatch: loop {
        match pc {
            0x8265C858 => {
    //   block [0x8265C858..0x8265C8BC)
	// 8265C858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C864: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265C868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C86C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265C870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C878: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 8265C87C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C88C: 386A9448  addi r3, r10, -0x6bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -27576;
	// 8265C890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C894: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C898: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265C89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C8A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C8A8: 4BE0A579  bl 0x82466e20
	ctx.lr = 0x8265C8AC;
	sub_82466E20(ctx, base);
	// 8265C8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C8C0 size=100
    let mut pc: u32 = 0x8265C8C0;
    'dispatch: loop {
        match pc {
            0x8265C8C0 => {
    //   block [0x8265C8C0..0x8265C924)
	// 8265C8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C8CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C8D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C8D4: 38AA94D8  addi r5, r10, -0x6b28
	ctx.r[5].s64 = ctx.r[10].s64 + -27432;
	// 8265C8D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C8DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C8E0: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 8265C8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C8F4: 386A9478  addi r3, r10, -0x6b88
	ctx.r[3].s64 = ctx.r[10].s64 + -27528;
	// 8265C8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C8FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C900: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265C904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C908: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C910: 4BE0A511  bl 0x82466e20
	ctx.lr = 0x8265C914;
	sub_82466E20(ctx, base);
	// 8265C914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C928 size=100
    let mut pc: u32 = 0x8265C928;
    'dispatch: loop {
        match pc {
            0x8265C928 => {
    //   block [0x8265C928..0x8265C98C)
	// 8265C928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C934: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C93C: 38AA9418  addi r5, r10, -0x6be8
	ctx.r[5].s64 = ctx.r[10].s64 + -27624;
	// 8265C940: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C944: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265C948: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 8265C94C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C95C: 386A94A8  addi r3, r10, -0x6b58
	ctx.r[3].s64 = ctx.r[10].s64 + -27480;
	// 8265C960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C964: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265C968: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265C96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C970: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C978: 4BE0A4A9  bl 0x82466e20
	ctx.lr = 0x8265C97C;
	sub_82466E20(ctx, base);
	// 8265C97C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C990 size=104
    let mut pc: u32 = 0x8265C990;
    'dispatch: loop {
        match pc {
            0x8265C990 => {
    //   block [0x8265C990..0x8265C9F8)
	// 8265C990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265C998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265C99C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265C9A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265C9A4: 392ADBDC  addi r9, r10, -0x2424
	ctx.r[9].s64 = ctx.r[10].s64 + -9252;
	// 8265C9A8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265C9B0: 38AA9448  addi r5, r10, -0x6bb8
	ctx.r[5].s64 = ctx.r[10].s64 + -27576;
	// 8265C9B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265C9B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265C9BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265C9C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265C9C4: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 8265C9C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265C9CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265C9D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265C9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265C9D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265C9DC: 386A94D8  addi r3, r10, -0x6b28
	ctx.r[3].s64 = ctx.r[10].s64 + -27432;
	// 8265C9E0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265C9E4: 4BE0A43D  bl 0x82466e20
	ctx.lr = 0x8265C9E8;
	sub_82466E20(ctx, base);
	// 8265C9E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265C9EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265C9F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265C9F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265C9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265C9F8 size=108
    let mut pc: u32 = 0x8265C9F8;
    'dispatch: loop {
        match pc {
            0x8265C9F8 => {
    //   block [0x8265C9F8..0x8265CA64)
	// 8265C9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265C9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CA00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CA04: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CA0C: 38EBF2B0  addi r7, r11, -0xd50
	ctx.r[7].s64 = ctx.r[11].s64 + -3408;
	// 8265CA10: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265CA14: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 8265CA18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CA1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265CA24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CA28: 386A9508  addi r3, r10, -0x6af8
	ctx.r[3].s64 = ctx.r[10].s64 + -27384;
	// 8265CA2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265CA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CA34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CA38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CA3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CA40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CA44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CA48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CA4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265CA50: 4BE0A3D1  bl 0x82466e20
	ctx.lr = 0x8265CA54;
	sub_82466E20(ctx, base);
	// 8265CA54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CA58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CA5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CA60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CA68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CA68 size=112
    let mut pc: u32 = 0x8265CA68;
    'dispatch: loop {
        match pc {
            0x8265CA68 => {
    //   block [0x8265CA68..0x8265CAD8)
	// 8265CA68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CA6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CA70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CA74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CA78: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CA7C: 38AA94D8  addi r5, r10, -0x6b28
	ctx.r[5].s64 = ctx.r[10].s64 + -27432;
	// 8265CA80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CA84: 390BF2E0  addi r8, r11, -0xd20
	ctx.r[8].s64 = ctx.r[11].s64 + -3360;
	// 8265CA88: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8265CA8C: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 8265CA90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CA94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CA98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CA9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CAA0: 386A9538  addi r3, r10, -0x6ac8
	ctx.r[3].s64 = ctx.r[10].s64 + -27336;
	// 8265CAA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CAA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CAAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CAB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CAB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CAB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CABC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CAC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CAC4: 4BE0A35D  bl 0x82466e20
	ctx.lr = 0x8265CAC8;
	sub_82466E20(ctx, base);
	// 8265CAC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CAD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265CAD8 size=24
    let mut pc: u32 = 0x8265CAD8;
    'dispatch: loop {
        match pc {
            0x8265CAD8 => {
    //   block [0x8265CAD8..0x8265CAF0)
	// 8265CAD8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CADC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265CAE0: 394A2628  addi r10, r10, 0x2628
	ctx.r[10].s64 = ctx.r[10].s64 + 9768;
	// 8265CAE4: 816BF388  lwz r11, -0xc78(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3192 as u32) ) } as u64;
	// 8265CAE8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265CAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CAF0 size=116
    let mut pc: u32 = 0x8265CAF0;
    'dispatch: loop {
        match pc {
            0x8265CAF0 => {
    //   block [0x8265CAF0..0x8265CB64)
	// 8265CAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CAF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CAFC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CB00: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265CB04: 390B2628  addi r8, r11, 0x2628
	ctx.r[8].s64 = ctx.r[11].s64 + 9768;
	// 8265CB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CB0C: 392ADC40  addi r9, r10, -0x23c0
	ctx.r[9].s64 = ctx.r[10].s64 + -9152;
	// 8265CB10: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265CB14: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8265CB18: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265CB1C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CB20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CB24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CB28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CB30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CB34: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265CB38: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8265CB3C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265CB40: 386B9568  addi r3, r11, -0x6a98
	ctx.r[3].s64 = ctx.r[11].s64 + -27288;
	// 8265CB44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265CB48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CB50: 4BE0A2D1  bl 0x82466e20
	ctx.lr = 0x8265CB54;
	sub_82466E20(ctx, base);
	// 8265CB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CB68 size=100
    let mut pc: u32 = 0x8265CB68;
    'dispatch: loop {
        match pc {
            0x8265CB68 => {
    //   block [0x8265CB68..0x8265CBCC)
	// 8265CB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CB74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CB7C: 38AA9568  addi r5, r10, -0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + -27288;
	// 8265CB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CB88: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 8265CB8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CB90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CB94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CB98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CB9C: 386A9598  addi r3, r10, -0x6a68
	ctx.r[3].s64 = ctx.r[10].s64 + -27240;
	// 8265CBA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CBA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CBA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CBB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CBB8: 4BE0A269  bl 0x82466e20
	ctx.lr = 0x8265CBBC;
	sub_82466E20(ctx, base);
	// 8265CBBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CBC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CBC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CBD0 size=100
    let mut pc: u32 = 0x8265CBD0;
    'dispatch: loop {
        match pc {
            0x8265CBD0 => {
    //   block [0x8265CBD0..0x8265CC34)
	// 8265CBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CBD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CBD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CBDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CBE4: 38AA95F8  addi r5, r10, -0x6a08
	ctx.r[5].s64 = ctx.r[10].s64 + -27144;
	// 8265CBE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CBEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CBF0: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 8265CBF4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CBF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CC00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CC04: 386A95C8  addi r3, r10, -0x6a38
	ctx.r[3].s64 = ctx.r[10].s64 + -27192;
	// 8265CC08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CC0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CC10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CC14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CC18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CC20: 4BE0A201  bl 0x82466e20
	ctx.lr = 0x8265CC24;
	sub_82466E20(ctx, base);
	// 8265CC24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CC28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CC2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CC30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CC38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CC38 size=112
    let mut pc: u32 = 0x8265CC38;
    'dispatch: loop {
        match pc {
            0x8265CC38 => {
    //   block [0x8265CC38..0x8265CCA8)
	// 8265CC38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CC3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CC40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CC44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CC48: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CC4C: 38AA9568  addi r5, r10, -0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + -27288;
	// 8265CC50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CC54: 390BF38C  addi r8, r11, -0xc74
	ctx.r[8].s64 = ctx.r[11].s64 + -3188;
	// 8265CC58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265CC5C: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 8265CC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CC64: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CC68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CC6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CC70: 386A95F8  addi r3, r10, -0x6a08
	ctx.r[3].s64 = ctx.r[10].s64 + -27144;
	// 8265CC74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CC78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CC80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CC84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CC88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CC8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CC90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CC94: 4BE0A18D  bl 0x82466e20
	ctx.lr = 0x8265CC98;
	sub_82466E20(ctx, base);
	// 8265CC98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CC9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CCA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CCA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CCA8 size=100
    let mut pc: u32 = 0x8265CCA8;
    'dispatch: loop {
        match pc {
            0x8265CCA8 => {
    //   block [0x8265CCA8..0x8265CD0C)
	// 8265CCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CCAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CCB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CCB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CCBC: 38AA95F8  addi r5, r10, -0x6a08
	ctx.r[5].s64 = ctx.r[10].s64 + -27144;
	// 8265CCC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CCC8: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 8265CCCC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CCD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CCD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CCD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CCDC: 386A9628  addi r3, r10, -0x69d8
	ctx.r[3].s64 = ctx.r[10].s64 + -27096;
	// 8265CCE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CCE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CCE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CCEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CCF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CCF8: 4BE0A129  bl 0x82466e20
	ctx.lr = 0x8265CCFC;
	sub_82466E20(ctx, base);
	// 8265CCFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CD00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CD04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CD08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CD10 size=100
    let mut pc: u32 = 0x8265CD10;
    'dispatch: loop {
        match pc {
            0x8265CD10 => {
    //   block [0x8265CD10..0x8265CD74)
	// 8265CD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CD18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CD1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CD24: 38AA9568  addi r5, r10, -0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + -27288;
	// 8265CD28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CD30: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 8265CD34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CD38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CD40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CD44: 386A9658  addi r3, r10, -0x69a8
	ctx.r[3].s64 = ctx.r[10].s64 + -27048;
	// 8265CD48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CD4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CD50: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CD54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CD58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CD5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CD60: 4BE0A0C1  bl 0x82466e20
	ctx.lr = 0x8265CD64;
	sub_82466E20(ctx, base);
	// 8265CD64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CD68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CD6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CD70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CD78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CD78 size=100
    let mut pc: u32 = 0x8265CD78;
    'dispatch: loop {
        match pc {
            0x8265CD78 => {
    //   block [0x8265CD78..0x8265CDDC)
	// 8265CD78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CD7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CD80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CD84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CD88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CD8C: 38AA9598  addi r5, r10, -0x6a68
	ctx.r[5].s64 = ctx.r[10].s64 + -27240;
	// 8265CD90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CD98: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 8265CD9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CDA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CDAC: 386A9688  addi r3, r10, -0x6978
	ctx.r[3].s64 = ctx.r[10].s64 + -27000;
	// 8265CDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CDB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CDB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CDC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CDC8: 4BE0A059  bl 0x82466e20
	ctx.lr = 0x8265CDCC;
	sub_82466E20(ctx, base);
	// 8265CDCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CDD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CDD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CDD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CDE0 size=100
    let mut pc: u32 = 0x8265CDE0;
    'dispatch: loop {
        match pc {
            0x8265CDE0 => {
    //   block [0x8265CDE0..0x8265CE44)
	// 8265CDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CDEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CDF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CDF4: 38AA9658  addi r5, r10, -0x69a8
	ctx.r[5].s64 = ctx.r[10].s64 + -27048;
	// 8265CDF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CE00: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 8265CE04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CE0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CE14: 386A96B8  addi r3, r10, -0x6948
	ctx.r[3].s64 = ctx.r[10].s64 + -26952;
	// 8265CE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CE1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CE20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CE24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CE28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CE2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CE30: 4BE09FF1  bl 0x82466e20
	ctx.lr = 0x8265CE34;
	sub_82466E20(ctx, base);
	// 8265CE34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CE38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CE3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CE48 size=100
    let mut pc: u32 = 0x8265CE48;
    'dispatch: loop {
        match pc {
            0x8265CE48 => {
    //   block [0x8265CE48..0x8265CEAC)
	// 8265CE48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CE4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CE50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CE54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CE5C: 38AA9598  addi r5, r10, -0x6a68
	ctx.r[5].s64 = ctx.r[10].s64 + -27240;
	// 8265CE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CE68: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 8265CE6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CE70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CE78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CE7C: 386A96E8  addi r3, r10, -0x6918
	ctx.r[3].s64 = ctx.r[10].s64 + -26904;
	// 8265CE80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CE84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CE88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265CE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CE90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265CE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CE98: 4BE09F89  bl 0x82466e20
	ctx.lr = 0x8265CE9C;
	sub_82466E20(ctx, base);
	// 8265CE9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CEA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CEA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CEA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CEB0 size=112
    let mut pc: u32 = 0x8265CEB0;
    'dispatch: loop {
        match pc {
            0x8265CEB0 => {
    //   block [0x8265CEB0..0x8265CF20)
	// 8265CEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CEBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CEC0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CEC4: 38AA9778  addi r5, r10, -0x6888
	ctx.r[5].s64 = ctx.r[10].s64 + -26760;
	// 8265CEC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CECC: 390BF3BC  addi r8, r11, -0xc44
	ctx.r[8].s64 = ctx.r[11].s64 + -3140;
	// 8265CED0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265CED4: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 8265CED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CEDC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CEE8: 386A9718  addi r3, r10, -0x68e8
	ctx.r[3].s64 = ctx.r[10].s64 + -26856;
	// 8265CEEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CF0C: 4BE09F15  bl 0x82466e20
	ctx.lr = 0x8265CF10;
	sub_82466E20(ctx, base);
	// 8265CF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CF20 size=112
    let mut pc: u32 = 0x8265CF20;
    'dispatch: loop {
        match pc {
            0x8265CF20 => {
    //   block [0x8265CF20..0x8265CF90)
	// 8265CF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CF2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CF30: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CF34: 38AA97A8  addi r5, r10, -0x6858
	ctx.r[5].s64 = ctx.r[10].s64 + -26712;
	// 8265CF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CF3C: 390BF3EC  addi r8, r11, -0xc14
	ctx.r[8].s64 = ctx.r[11].s64 + -3092;
	// 8265CF40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265CF44: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 8265CF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CF4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CF58: 386A9748  addi r3, r10, -0x68b8
	ctx.r[3].s64 = ctx.r[10].s64 + -26808;
	// 8265CF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CF7C: 4BE09EA5  bl 0x82466e20
	ctx.lr = 0x8265CF80;
	sub_82466E20(ctx, base);
	// 8265CF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265CF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265CF90 size=112
    let mut pc: u32 = 0x8265CF90;
    'dispatch: loop {
        match pc {
            0x8265CF90 => {
    //   block [0x8265CF90..0x8265D000)
	// 8265CF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265CF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265CF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265CF9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CFA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265CFA4: 38AA9868  addi r5, r10, -0x6798
	ctx.r[5].s64 = ctx.r[10].s64 + -26520;
	// 8265CFA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265CFAC: 390BF404  addi r8, r11, -0xbfc
	ctx.r[8].s64 = ctx.r[11].s64 + -3068;
	// 8265CFB0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265CFB4: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 8265CFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265CFBC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265CFC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265CFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265CFC8: 386A9778  addi r3, r10, -0x6888
	ctx.r[3].s64 = ctx.r[10].s64 + -26760;
	// 8265CFCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265CFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265CFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265CFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265CFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265CFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265CFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265CFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265CFEC: 4BE09E35  bl 0x82466e20
	ctx.lr = 0x8265CFF0;
	sub_82466E20(ctx, base);
	// 8265CFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265CFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265CFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265CFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D000 size=112
    let mut pc: u32 = 0x8265D000;
    'dispatch: loop {
        match pc {
            0x8265D000 => {
    //   block [0x8265D000..0x8265D070)
	// 8265D000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D00C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D010: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D014: 38AA9778  addi r5, r10, -0x6888
	ctx.r[5].s64 = ctx.r[10].s64 + -26760;
	// 8265D018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D01C: 390BF434  addi r8, r11, -0xbcc
	ctx.r[8].s64 = ctx.r[11].s64 + -3020;
	// 8265D020: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D024: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 8265D028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D02C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D038: 386A97A8  addi r3, r10, -0x6858
	ctx.r[3].s64 = ctx.r[10].s64 + -26712;
	// 8265D03C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D05C: 4BE09DC5  bl 0x82466e20
	ctx.lr = 0x8265D060;
	sub_82466E20(ctx, base);
	// 8265D060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D070 size=112
    let mut pc: u32 = 0x8265D070;
    'dispatch: loop {
        match pc {
            0x8265D070 => {
    //   block [0x8265D070..0x8265D0E0)
	// 8265D070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D07C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D080: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D084: 38AA97A8  addi r5, r10, -0x6858
	ctx.r[5].s64 = ctx.r[10].s64 + -26712;
	// 8265D088: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D08C: 390BF44C  addi r8, r11, -0xbb4
	ctx.r[8].s64 = ctx.r[11].s64 + -2996;
	// 8265D090: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D094: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 8265D098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D09C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D0A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D0A8: 386A97D8  addi r3, r10, -0x6828
	ctx.r[3].s64 = ctx.r[10].s64 + -26664;
	// 8265D0AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D0B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D0CC: 4BE09D55  bl 0x82466e20
	ctx.lr = 0x8265D0D0;
	sub_82466E20(ctx, base);
	// 8265D0D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D0E0 size=116
    let mut pc: u32 = 0x8265D0E0;
    'dispatch: loop {
        match pc {
            0x8265D0E0 => {
    //   block [0x8265D0E0..0x8265D154)
	// 8265D0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D0EC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265D0F0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265D0F4: 390AF468  addi r8, r10, -0xb98
	ctx.r[8].s64 = ctx.r[10].s64 + -2968;
	// 8265D0F8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D0FC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265D100: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D104: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D108: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D114: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 8265D118: 396BDC54  addi r11, r11, -0x23ac
	ctx.r[11].s64 = ctx.r[11].s64 + -9132;
	// 8265D11C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D120: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D124: 386A9808  addi r3, r10, -0x67f8
	ctx.r[3].s64 = ctx.r[10].s64 + -26616;
	// 8265D128: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265D12C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D130: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265D134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D13C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D140: 4BE09CE1  bl 0x82466e20
	ctx.lr = 0x8265D144;
	sub_82466E20(ctx, base);
	// 8265D144: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D14C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265D158 size=48
    let mut pc: u32 = 0x8265D158;
    'dispatch: loop {
        match pc {
            0x8265D158 => {
    //   block [0x8265D158..0x8265D188)
	// 8265D158: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D15C: 814BF518  lwz r10, -0xae8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2792 as u32) ) } as u64;
	// 8265D160: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D164: 396B26E8  addi r11, r11, 0x26e8
	ctx.r[11].s64 = ctx.r[11].s64 + 9960;
	// 8265D168: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8265D16C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265D170: 814AF514  lwz r10, -0xaec(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2796 as u32) ) } as u64;
	// 8265D174: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8265D178: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265D17C: 814AF510  lwz r10, -0xaf0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2800 as u32) ) } as u64;
	// 8265D180: 914B0350  stw r10, 0x350(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(848 as u32), ctx.r[10].u32 ) };
	// 8265D184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D188 size=116
    let mut pc: u32 = 0x8265D188;
    'dispatch: loop {
        match pc {
            0x8265D188 => {
    //   block [0x8265D188..0x8265D1FC)
	// 8265D188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D18C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D194: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265D198: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D19C: 392BDD28  addi r9, r11, -0x22d8
	ctx.r[9].s64 = ctx.r[11].s64 + -8920;
	// 8265D1A0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D1A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D1A8: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8265D1AC: 38C00028  li r6, 0x28
	ctx.r[6].s64 = 40;
	// 8265D1B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D1B4: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 8265D1B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D1BC: 396B26E8  addi r11, r11, 0x26e8
	ctx.r[11].s64 = ctx.r[11].s64 + 9960;
	// 8265D1C0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8265D1C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D1C8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8265D1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D1D0: 386A9838  addi r3, r10, -0x67c8
	ctx.r[3].s64 = ctx.r[10].s64 + -26568;
	// 8265D1D4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8265D1D8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8265D1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D1E0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8265D1E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265D1E8: 4BE09C39  bl 0x82466e20
	ctx.lr = 0x8265D1EC;
	sub_82466E20(ctx, base);
	// 8265D1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D200 size=116
    let mut pc: u32 = 0x8265D200;
    'dispatch: loop {
        match pc {
            0x8265D200 => {
    //   block [0x8265D200..0x8265D274)
	// 8265D200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D20C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D210: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265D214: 390BF528  addi r8, r11, -0xad8
	ctx.r[8].s64 = ctx.r[11].s64 + -2776;
	// 8265D218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D21C: 392ADEC8  addi r9, r10, -0x2138
	ctx.r[9].s64 = ctx.r[10].s64 + -8504;
	// 8265D220: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D224: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8265D228: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D22C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D234: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D244: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265D248: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 8265D24C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D250: 386B9868  addi r3, r11, -0x6798
	ctx.r[3].s64 = ctx.r[11].s64 + -26520;
	// 8265D254: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265D258: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D260: 4BE09BC1  bl 0x82466e20
	ctx.lr = 0x8265D264;
	sub_82466E20(ctx, base);
	// 8265D264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D26C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D278 size=112
    let mut pc: u32 = 0x8265D278;
    'dispatch: loop {
        match pc {
            0x8265D278 => {
    //   block [0x8265D278..0x8265D2E8)
	// 8265D278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D284: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D288: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D28C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D294: 390BF5B8  addi r8, r11, -0xa48
	ctx.r[8].s64 = ctx.r[11].s64 + -2632;
	// 8265D298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D29C: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 8265D2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D2A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D2A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D2B0: 386A9898  addi r3, r10, -0x6768
	ctx.r[3].s64 = ctx.r[10].s64 + -26472;
	// 8265D2B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D2B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D2BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D2C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D2C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D2C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D2D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D2D4: 4BE09B4D  bl 0x82466e20
	ctx.lr = 0x8265D2D8;
	sub_82466E20(ctx, base);
	// 8265D2D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D2DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D2E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D2E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D2E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D2E8 size=108
    let mut pc: u32 = 0x8265D2E8;
    'dispatch: loop {
        match pc {
            0x8265D2E8 => {
    //   block [0x8265D2E8..0x8265D354)
	// 8265D2E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D2EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D2F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D2F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D2F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265D2FC: 38EBF5D0  addi r7, r11, -0xa30
	ctx.r[7].s64 = ctx.r[11].s64 + -2608;
	// 8265D300: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8265D304: 388A0E48  addi r4, r10, 0xe48
	ctx.r[4].s64 = ctx.r[10].s64 + 3656;
	// 8265D308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D30C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D310: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265D314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D318: 386A98C8  addi r3, r10, -0x6738
	ctx.r[3].s64 = ctx.r[10].s64 + -26424;
	// 8265D31C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265D320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D33C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D340: 4BE09AE1  bl 0x82466e20
	ctx.lr = 0x8265D344;
	sub_82466E20(ctx, base);
	// 8265D344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D358 size=112
    let mut pc: u32 = 0x8265D358;
    'dispatch: loop {
        match pc {
            0x8265D358 => {
    //   block [0x8265D358..0x8265D3C8)
	// 8265D358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D364: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D368: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D36C: 38AA7828  addi r5, r10, 0x7828
	ctx.r[5].s64 = ctx.r[10].s64 + 30760;
	// 8265D370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D374: 390BF648  addi r8, r11, -0x9b8
	ctx.r[8].s64 = ctx.r[11].s64 + -2488;
	// 8265D378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D37C: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 8265D380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D384: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D390: 386A98F8  addi r3, r10, -0x6708
	ctx.r[3].s64 = ctx.r[10].s64 + -26376;
	// 8265D394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D39C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D3A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D3A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D3B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D3B4: 4BE09A6D  bl 0x82466e20
	ctx.lr = 0x8265D3B8;
	sub_82466E20(ctx, base);
	// 8265D3B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D3BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D3C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D3C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D3C8 size=108
    let mut pc: u32 = 0x8265D3C8;
    'dispatch: loop {
        match pc {
            0x8265D3C8 => {
    //   block [0x8265D3C8..0x8265D434)
	// 8265D3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D3D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D3D4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D3DC: 38EBF660  addi r7, r11, -0x9a0
	ctx.r[7].s64 = ctx.r[11].s64 + -2464;
	// 8265D3E0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265D3E4: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 8265D3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D3EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D3F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265D3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D3F8: 386A9928  addi r3, r10, -0x66d8
	ctx.r[3].s64 = ctx.r[10].s64 + -26328;
	// 8265D3FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265D400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D41C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D420: 4BE09A01  bl 0x82466e20
	ctx.lr = 0x8265D424;
	sub_82466E20(ctx, base);
	// 8265D424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D42C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D438 size=112
    let mut pc: u32 = 0x8265D438;
    'dispatch: loop {
        match pc {
            0x8265D438 => {
    //   block [0x8265D438..0x8265D4A8)
	// 8265D438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D43C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D444: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D448: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D44C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D454: 390BF678  addi r8, r11, -0x988
	ctx.r[8].s64 = ctx.r[11].s64 + -2440;
	// 8265D458: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265D45C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 8265D460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D464: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D470: 386A9958  addi r3, r10, -0x66a8
	ctx.r[3].s64 = ctx.r[10].s64 + -26280;
	// 8265D474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D47C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D48C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D494: 4BE0998D  bl 0x82466e20
	ctx.lr = 0x8265D498;
	sub_82466E20(ctx, base);
	// 8265D498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D4A8 size=108
    let mut pc: u32 = 0x8265D4A8;
    'dispatch: loop {
        match pc {
            0x8265D4A8 => {
    //   block [0x8265D4A8..0x8265D514)
	// 8265D4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D4B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D4B4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D4B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D4BC: 38EBF6C0  addi r7, r11, -0x940
	ctx.r[7].s64 = ctx.r[11].s64 + -2368;
	// 8265D4C0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265D4C4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 8265D4C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D4CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D4D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265D4D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D4D8: 386A9988  addi r3, r10, -0x6678
	ctx.r[3].s64 = ctx.r[10].s64 + -26232;
	// 8265D4DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265D4E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D4E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D4E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D4EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D4F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D4F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D4F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D4FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D500: 4BE09921  bl 0x82466e20
	ctx.lr = 0x8265D504;
	sub_82466E20(ctx, base);
	// 8265D504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D510: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D518 size=108
    let mut pc: u32 = 0x8265D518;
    'dispatch: loop {
        match pc {
            0x8265D518 => {
    //   block [0x8265D518..0x8265D584)
	// 8265D518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D524: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D52C: 38EBF6F0  addi r7, r11, -0x910
	ctx.r[7].s64 = ctx.r[11].s64 + -2320;
	// 8265D530: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265D534: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 8265D538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D53C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D540: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265D544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D548: 386A99B8  addi r3, r10, -0x6648
	ctx.r[3].s64 = ctx.r[10].s64 + -26184;
	// 8265D54C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265D550: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D55C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D564: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D56C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D570: 4BE098B1  bl 0x82466e20
	ctx.lr = 0x8265D574;
	sub_82466E20(ctx, base);
	// 8265D574: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D57C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D588 size=112
    let mut pc: u32 = 0x8265D588;
    'dispatch: loop {
        match pc {
            0x8265D588 => {
    //   block [0x8265D588..0x8265D5F8)
	// 8265D588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D58C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D594: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265D598: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D59C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265D5A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D5A4: 390BF708  addi r8, r11, -0x8f8
	ctx.r[8].s64 = ctx.r[11].s64 + -2296;
	// 8265D5A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265D5AC: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 8265D5B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D5B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D5B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D5BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D5C0: 386A99E8  addi r3, r10, -0x6618
	ctx.r[3].s64 = ctx.r[10].s64 + -26136;
	// 8265D5C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D5C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D5CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D5D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D5D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D5DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D5E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D5E4: 4BE0983D  bl 0x82466e20
	ctx.lr = 0x8265D5E8;
	sub_82466E20(ctx, base);
	// 8265D5E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D5EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D5F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D5F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D5F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D5F8 size=112
    let mut pc: u32 = 0x8265D5F8;
    'dispatch: loop {
        match pc {
            0x8265D5F8 => {
    //   block [0x8265D5F8..0x8265D668)
	// 8265D5F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D5FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D604: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265D608: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D60C: 392ADF20  addi r9, r10, -0x20e0
	ctx.r[9].s64 = ctx.r[10].s64 + -8416;
	// 8265D610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D614: 390BF740  addi r8, r11, -0x8c0
	ctx.r[8].s64 = ctx.r[11].s64 + -2240;
	// 8265D618: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8265D61C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 8265D620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D624: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D628: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D630: 386A9A18  addi r3, r10, -0x65e8
	ctx.r[3].s64 = ctx.r[10].s64 + -26088;
	// 8265D634: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D638: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265D63C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D644: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D64C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D654: 4BE097CD  bl 0x82466e20
	ctx.lr = 0x8265D658;
	sub_82466E20(ctx, base);
	// 8265D658: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D65C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D668 size=116
    let mut pc: u32 = 0x8265D668;
    'dispatch: loop {
        match pc {
            0x8265D668 => {
    //   block [0x8265D668..0x8265D6DC)
	// 8265D668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D670: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D674: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D678: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265D67C: 390BF7E8  addi r8, r11, -0x818
	ctx.r[8].s64 = ctx.r[11].s64 + -2072;
	// 8265D680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D684: 392ADEF4  addi r9, r10, -0x210c
	ctx.r[9].s64 = ctx.r[10].s64 + -8460;
	// 8265D688: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D68C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265D690: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265D694: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D69C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D6AC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265D6B0: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 8265D6B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D6B8: 386B9A48  addi r3, r11, -0x65b8
	ctx.r[3].s64 = ctx.r[11].s64 + -26040;
	// 8265D6BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265D6C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D6C8: 4BE09759  bl 0x82466e20
	ctx.lr = 0x8265D6CC;
	sub_82466E20(ctx, base);
	// 8265D6CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D6D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D6D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D6D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D6E0 size=112
    let mut pc: u32 = 0x8265D6E0;
    'dispatch: loop {
        match pc {
            0x8265D6E0 => {
    //   block [0x8265D6E0..0x8265D750)
	// 8265D6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D6E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D6E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D6EC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265D6F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D6F4: 392ADF4C  addi r9, r10, -0x20b4
	ctx.r[9].s64 = ctx.r[10].s64 + -8372;
	// 8265D6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D6FC: 390BF800  addi r8, r11, -0x800
	ctx.r[8].s64 = ctx.r[11].s64 + -2048;
	// 8265D700: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8265D704: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 8265D708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D70C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D710: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D718: 386A9A78  addi r3, r10, -0x6588
	ctx.r[3].s64 = ctx.r[10].s64 + -25992;
	// 8265D71C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265D720: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265D724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D734: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265D738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D73C: 4BE096E5  bl 0x82466e20
	ctx.lr = 0x8265D740;
	sub_82466E20(ctx, base);
	// 8265D740: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D750 size=112
    let mut pc: u32 = 0x8265D750;
    'dispatch: loop {
        match pc {
            0x8265D750 => {
    //   block [0x8265D750..0x8265D7C0)
	// 8265D750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D758: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D75C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D760: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D764: 38AA8BA8  addi r5, r10, -0x7458
	ctx.r[5].s64 = ctx.r[10].s64 + -29784;
	// 8265D768: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D76C: 390BF860  addi r8, r11, -0x7a0
	ctx.r[8].s64 = ctx.r[11].s64 + -1952;
	// 8265D770: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265D774: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 8265D778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D77C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D780: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D788: 386A9AA8  addi r3, r10, -0x6558
	ctx.r[3].s64 = ctx.r[10].s64 + -25944;
	// 8265D78C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D790: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D794: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D7AC: 4BE09675  bl 0x82466e20
	ctx.lr = 0x8265D7B0;
	sub_82466E20(ctx, base);
	// 8265D7B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D7C0 size=112
    let mut pc: u32 = 0x8265D7C0;
    'dispatch: loop {
        match pc {
            0x8265D7C0 => {
    //   block [0x8265D7C0..0x8265D830)
	// 8265D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D7C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D7CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D7D0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D7D4: 38AA8AB8  addi r5, r10, -0x7548
	ctx.r[5].s64 = ctx.r[10].s64 + -30024;
	// 8265D7D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D7DC: 390BF878  addi r8, r11, -0x788
	ctx.r[8].s64 = ctx.r[11].s64 + -1928;
	// 8265D7E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265D7E4: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 8265D7E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D7EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D7F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D7F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D7F8: 386A9AD8  addi r3, r10, -0x6528
	ctx.r[3].s64 = ctx.r[10].s64 + -25896;
	// 8265D7FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D800: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D804: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D81C: 4BE09605  bl 0x82466e20
	ctx.lr = 0x8265D820;
	sub_82466E20(ctx, base);
	// 8265D820: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D82C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D830 size=112
    let mut pc: u32 = 0x8265D830;
    'dispatch: loop {
        match pc {
            0x8265D830 => {
    //   block [0x8265D830..0x8265D8A0)
	// 8265D830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D83C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D840: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D844: 38AA8AB8  addi r5, r10, -0x7548
	ctx.r[5].s64 = ctx.r[10].s64 + -30024;
	// 8265D848: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D84C: 390BF8C0  addi r8, r11, -0x740
	ctx.r[8].s64 = ctx.r[11].s64 + -1856;
	// 8265D850: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265D854: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 8265D858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D85C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D860: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D868: 386A9B08  addi r3, r10, -0x64f8
	ctx.r[3].s64 = ctx.r[10].s64 + -25848;
	// 8265D86C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D870: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D874: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D878: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D87C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D880: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D888: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D88C: 4BE09595  bl 0x82466e20
	ctx.lr = 0x8265D890;
	sub_82466E20(ctx, base);
	// 8265D890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D8A0 size=112
    let mut pc: u32 = 0x8265D8A0;
    'dispatch: loop {
        match pc {
            0x8265D8A0 => {
    //   block [0x8265D8A0..0x8265D910)
	// 8265D8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D8AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D8B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D8B4: 38AA8AE8  addi r5, r10, -0x7518
	ctx.r[5].s64 = ctx.r[10].s64 + -29976;
	// 8265D8B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D8BC: 390BF920  addi r8, r11, -0x6e0
	ctx.r[8].s64 = ctx.r[11].s64 + -1760;
	// 8265D8C0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265D8C4: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 8265D8C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D8CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D8D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D8D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D8D8: 386A9B38  addi r3, r10, -0x64c8
	ctx.r[3].s64 = ctx.r[10].s64 + -25800;
	// 8265D8DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D8E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D8E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D8E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D8EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D8F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D8F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D8F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D8FC: 4BE09525  bl 0x82466e20
	ctx.lr = 0x8265D900;
	sub_82466E20(ctx, base);
	// 8265D900: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D910 size=112
    let mut pc: u32 = 0x8265D910;
    'dispatch: loop {
        match pc {
            0x8265D910 => {
    //   block [0x8265D910..0x8265D980)
	// 8265D910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D91C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D920: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D924: 38AA8AE8  addi r5, r10, -0x7518
	ctx.r[5].s64 = ctx.r[10].s64 + -29976;
	// 8265D928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D92C: 390BF980  addi r8, r11, -0x680
	ctx.r[8].s64 = ctx.r[11].s64 + -1664;
	// 8265D930: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265D934: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 8265D938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D93C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D948: 386A9B68  addi r3, r10, -0x6498
	ctx.r[3].s64 = ctx.r[10].s64 + -25752;
	// 8265D94C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D95C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D96C: 4BE094B5  bl 0x82466e20
	ctx.lr = 0x8265D970;
	sub_82466E20(ctx, base);
	// 8265D970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D980 size=112
    let mut pc: u32 = 0x8265D980;
    'dispatch: loop {
        match pc {
            0x8265D980 => {
    //   block [0x8265D980..0x8265D9F0)
	// 8265D980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D98C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D990: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265D994: 38AA8AB8  addi r5, r10, -0x7548
	ctx.r[5].s64 = ctx.r[10].s64 + -30024;
	// 8265D998: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265D99C: 390BF9E0  addi r8, r11, -0x620
	ctx.r[8].s64 = ctx.r[11].s64 + -1568;
	// 8265D9A0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8265D9A4: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 8265D9A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265D9AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265D9B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265D9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265D9B8: 386A9B98  addi r3, r10, -0x6468
	ctx.r[3].s64 = ctx.r[10].s64 + -25704;
	// 8265D9BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265D9C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265D9C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265D9C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265D9CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265D9D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265D9D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265D9D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265D9DC: 4BE09445  bl 0x82466e20
	ctx.lr = 0x8265D9E0;
	sub_82466E20(ctx, base);
	// 8265D9E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265D9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265D9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265D9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265D9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265D9F0 size=112
    let mut pc: u32 = 0x8265D9F0;
    'dispatch: loop {
        match pc {
            0x8265D9F0 => {
    //   block [0x8265D9F0..0x8265DA60)
	// 8265D9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265D9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265D9F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265D9FC: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265DA00: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8265DA04: 38EAFAA0  addi r7, r10, -0x560
	ctx.r[7].s64 = ctx.r[10].s64 + -1376;
	// 8265DA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DA0C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265DA10: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 8265DA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DA18: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DA1C: 396BDF60  addi r11, r11, -0x20a0
	ctx.r[11].s64 = ctx.r[11].s64 + -8352;
	// 8265DA20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DA24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DA28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DA2C: 386A9BC8  addi r3, r10, -0x6438
	ctx.r[3].s64 = ctx.r[10].s64 + -25656;
	// 8265DA30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DA34: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265DA38: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DA3C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265DA40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DA44: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DA48: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DA4C: 4BE093D5  bl 0x82466e20
	ctx.lr = 0x8265DA50;
	sub_82466E20(ctx, base);
	// 8265DA50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DA54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DA58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DA60 size=112
    let mut pc: u32 = 0x8265DA60;
    'dispatch: loop {
        match pc {
            0x8265DA60 => {
    //   block [0x8265DA60..0x8265DAD0)
	// 8265DA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DA68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DA6C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DA70: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DA74: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 8265DA78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DA7C: 390BFC68  addi r8, r11, -0x398
	ctx.r[8].s64 = ctx.r[11].s64 + -920;
	// 8265DA80: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265DA84: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 8265DA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DA8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DA90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DA98: 386A9BF8  addi r3, r10, -0x6408
	ctx.r[3].s64 = ctx.r[10].s64 + -25608;
	// 8265DA9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DAA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DAA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DAA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DAAC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265DAB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DAB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DAB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DABC: 4BE09365  bl 0x82466e20
	ctx.lr = 0x8265DAC0;
	sub_82466E20(ctx, base);
	// 8265DAC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DAC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DAC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DAD0 size=112
    let mut pc: u32 = 0x8265DAD0;
    'dispatch: loop {
        match pc {
            0x8265DAD0 => {
    //   block [0x8265DAD0..0x8265DB40)
	// 8265DAD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DAD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DAD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DADC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DAE0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DAE4: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 8265DAE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DAEC: 390BFC80  addi r8, r11, -0x380
	ctx.r[8].s64 = ctx.r[11].s64 + -896;
	// 8265DAF0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265DAF4: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 8265DAF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DAFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DB00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DB04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DB08: 386A9C28  addi r3, r10, -0x63d8
	ctx.r[3].s64 = ctx.r[10].s64 + -25560;
	// 8265DB0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DB10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DB14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DB18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DB1C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265DB20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DB24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DB28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DB2C: 4BE092F5  bl 0x82466e20
	ctx.lr = 0x8265DB30;
	sub_82466E20(ctx, base);
	// 8265DB30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DB34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DB38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DB40 size=112
    let mut pc: u32 = 0x8265DB40;
    'dispatch: loop {
        match pc {
            0x8265DB40 => {
    //   block [0x8265DB40..0x8265DBB0)
	// 8265DB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DB48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DB4C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DB50: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DB54: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 8265DB58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DB5C: 390BFC98  addi r8, r11, -0x368
	ctx.r[8].s64 = ctx.r[11].s64 + -872;
	// 8265DB60: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265DB64: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 8265DB68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DB6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DB70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DB78: 386A9C58  addi r3, r10, -0x63a8
	ctx.r[3].s64 = ctx.r[10].s64 + -25512;
	// 8265DB7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DB80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DB88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DB8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DB90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DB98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DB9C: 4BE09285  bl 0x82466e20
	ctx.lr = 0x8265DBA0;
	sub_82466E20(ctx, base);
	// 8265DBA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DBA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DBA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DBB0 size=108
    let mut pc: u32 = 0x8265DBB0;
    'dispatch: loop {
        match pc {
            0x8265DBB0 => {
    //   block [0x8265DBB0..0x8265DC1C)
	// 8265DBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DBB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DBB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DBBC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DBC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DBC4: 38EBFCC8  addi r7, r11, -0x338
	ctx.r[7].s64 = ctx.r[11].s64 + -824;
	// 8265DBC8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265DBCC: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 8265DBD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DBD4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DBD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DBDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DBE0: 386A9C88  addi r3, r10, -0x6378
	ctx.r[3].s64 = ctx.r[10].s64 + -25464;
	// 8265DBE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DBE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DBEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DBF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DBF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DBF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DBFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DC00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DC04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DC08: 4BE09219  bl 0x82466e20
	ctx.lr = 0x8265DC0C;
	sub_82466E20(ctx, base);
	// 8265DC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DC20 size=112
    let mut pc: u32 = 0x8265DC20;
    'dispatch: loop {
        match pc {
            0x8265DC20 => {
    //   block [0x8265DC20..0x8265DC90)
	// 8265DC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DC2C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DC30: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DC34: 38AA78E8  addi r5, r10, 0x78e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30952;
	// 8265DC38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DC3C: 390BFCF8  addi r8, r11, -0x308
	ctx.r[8].s64 = ctx.r[11].s64 + -776;
	// 8265DC40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265DC44: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 8265DC48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DC4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DC50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DC58: 386A9CB8  addi r3, r10, -0x6348
	ctx.r[3].s64 = ctx.r[10].s64 + -25416;
	// 8265DC5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DC60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DC64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DC68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DC6C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265DC70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DC78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DC7C: 4BE091A5  bl 0x82466e20
	ctx.lr = 0x8265DC80;
	sub_82466E20(ctx, base);
	// 8265DC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DC90 size=108
    let mut pc: u32 = 0x8265DC90;
    'dispatch: loop {
        match pc {
            0x8265DC90 => {
    //   block [0x8265DC90..0x8265DCFC)
	// 8265DC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DC9C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DCA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DCA4: 38EBFD10  addi r7, r11, -0x2f0
	ctx.r[7].s64 = ctx.r[11].s64 + -752;
	// 8265DCA8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265DCAC: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 8265DCB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DCB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DCB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DCBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DCC0: 386A9CE8  addi r3, r10, -0x6318
	ctx.r[3].s64 = ctx.r[10].s64 + -25368;
	// 8265DCC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DCC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DCCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DCD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DCD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DCD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DCDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DCE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DCE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DCE8: 4BE09139  bl 0x82466e20
	ctx.lr = 0x8265DCEC;
	sub_82466E20(ctx, base);
	// 8265DCEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DCF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DCF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DCF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DD00 size=108
    let mut pc: u32 = 0x8265DD00;
    'dispatch: loop {
        match pc {
            0x8265DD00 => {
    //   block [0x8265DD00..0x8265DD6C)
	// 8265DD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DD0C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DD10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DD14: 38EBFD40  addi r7, r11, -0x2c0
	ctx.r[7].s64 = ctx.r[11].s64 + -704;
	// 8265DD18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265DD1C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 8265DD20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DD24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DD28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DD2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DD30: 386A9D18  addi r3, r10, -0x62e8
	ctx.r[3].s64 = ctx.r[10].s64 + -25320;
	// 8265DD34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DD38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DD3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DD40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DD44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DD48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DD4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DD50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DD54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DD58: 4BE090C9  bl 0x82466e20
	ctx.lr = 0x8265DD5C;
	sub_82466E20(ctx, base);
	// 8265DD5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DD60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DD64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DD68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DD70 size=112
    let mut pc: u32 = 0x8265DD70;
    'dispatch: loop {
        match pc {
            0x8265DD70 => {
    //   block [0x8265DD70..0x8265DDE0)
	// 8265DD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DD7C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DD80: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DD84: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265DD88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265DD8C: 390BFD88  addi r8, r11, -0x278
	ctx.r[8].s64 = ctx.r[11].s64 + -632;
	// 8265DD90: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265DD94: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 8265DD98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DD9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DDA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DDA8: 386A9D48  addi r3, r10, -0x62b8
	ctx.r[3].s64 = ctx.r[10].s64 + -25272;
	// 8265DDAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DDB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DDB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DDCC: 4BE09055  bl 0x82466e20
	ctx.lr = 0x8265DDD0;
	sub_82466E20(ctx, base);
	// 8265DDD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DDE0 size=112
    let mut pc: u32 = 0x8265DDE0;
    'dispatch: loop {
        match pc {
            0x8265DDE0 => {
    //   block [0x8265DDE0..0x8265DE50)
	// 8265DDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DDEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DDF0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DDF4: 38AA9808  addi r5, r10, -0x67f8
	ctx.r[5].s64 = ctx.r[10].s64 + -26616;
	// 8265DDF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DDFC: 390BFDD0  addi r8, r11, -0x230
	ctx.r[8].s64 = ctx.r[11].s64 + -560;
	// 8265DE00: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265DE04: 388A0E58  addi r4, r10, 0xe58
	ctx.r[4].s64 = ctx.r[10].s64 + 3672;
	// 8265DE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DE0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DE10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DE18: 386A9D78  addi r3, r10, -0x6288
	ctx.r[3].s64 = ctx.r[10].s64 + -25224;
	// 8265DE1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DE20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DE28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DE30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DE38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DE3C: 4BE08FE5  bl 0x82466e20
	ctx.lr = 0x8265DE40;
	sub_82466E20(ctx, base);
	// 8265DE40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DE50 size=108
    let mut pc: u32 = 0x8265DE50;
    'dispatch: loop {
        match pc {
            0x8265DE50 => {
    //   block [0x8265DE50..0x8265DEBC)
	// 8265DE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DE5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DE60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DE64: 38EBFDE8  addi r7, r11, -0x218
	ctx.r[7].s64 = ctx.r[11].s64 + -536;
	// 8265DE68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265DE6C: 388A0E74  addi r4, r10, 0xe74
	ctx.r[4].s64 = ctx.r[10].s64 + 3700;
	// 8265DE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DE74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DE78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DE80: 386A9DA8  addi r3, r10, -0x6258
	ctx.r[3].s64 = ctx.r[10].s64 + -25176;
	// 8265DE84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DE94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DEA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DEA8: 4BE08F79  bl 0x82466e20
	ctx.lr = 0x8265DEAC;
	sub_82466E20(ctx, base);
	// 8265DEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DEC0 size=108
    let mut pc: u32 = 0x8265DEC0;
    'dispatch: loop {
        match pc {
            0x8265DEC0 => {
    //   block [0x8265DEC0..0x8265DF2C)
	// 8265DEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DECC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DED0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DED4: 38EBFE00  addi r7, r11, -0x200
	ctx.r[7].s64 = ctx.r[11].s64 + -512;
	// 8265DED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265DEDC: 388A0E9C  addi r4, r10, 0xe9c
	ctx.r[4].s64 = ctx.r[10].s64 + 3740;
	// 8265DEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DEE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DEE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265DEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DEF0: 386A9DD8  addi r3, r10, -0x6228
	ctx.r[3].s64 = ctx.r[10].s64 + -25128;
	// 8265DEF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265DEF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DF14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265DF18: 4BE08F09  bl 0x82466e20
	ctx.lr = 0x8265DF1C;
	sub_82466E20(ctx, base);
	// 8265DF1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DF30 size=112
    let mut pc: u32 = 0x8265DF30;
    'dispatch: loop {
        match pc {
            0x8265DF30 => {
    //   block [0x8265DF30..0x8265DFA0)
	// 8265DF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DF3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DF40: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DF44: 38AA9DD8  addi r5, r10, -0x6228
	ctx.r[5].s64 = ctx.r[10].s64 + -25128;
	// 8265DF48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DF4C: 390BFE30  addi r8, r11, -0x1d0
	ctx.r[8].s64 = ctx.r[11].s64 + -464;
	// 8265DF50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265DF54: 388A0EB4  addi r4, r10, 0xeb4
	ctx.r[4].s64 = ctx.r[10].s64 + 3764;
	// 8265DF58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DF5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265DF60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DF64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265DF68: 386A9E08  addi r3, r10, -0x61f8
	ctx.r[3].s64 = ctx.r[10].s64 + -25080;
	// 8265DF6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265DF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265DF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265DF78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DF7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DF80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DF84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265DF88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DF8C: 4BE08E95  bl 0x82466e20
	ctx.lr = 0x8265DF90;
	sub_82466E20(ctx, base);
	// 8265DF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265DF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265DF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265DF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265DFA0 size=24
    let mut pc: u32 = 0x8265DFA0;
    'dispatch: loop {
        match pc {
            0x8265DFA0 => {
    //   block [0x8265DFA0..0x8265DFB8)
	// 8265DFA0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DFA4: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265DFA8: 394A2AA8  addi r10, r10, 0x2aa8
	ctx.r[10].s64 = ctx.r[10].s64 + 10920;
	// 8265DFAC: 816BFE60  lwz r11, -0x1a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-416 as u32) ) } as u64;
	// 8265DFB0: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8265DFB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265DFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265DFB8 size=116
    let mut pc: u32 = 0x8265DFB8;
    'dispatch: loop {
        match pc {
            0x8265DFB8 => {
    //   block [0x8265DFB8..0x8265E02C)
	// 8265DFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265DFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265DFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265DFC4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265DFC8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265DFCC: 390B2AA8  addi r8, r11, 0x2aa8
	ctx.r[8].s64 = ctx.r[11].s64 + 10920;
	// 8265DFD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265DFD4: 392ADFF8  addi r9, r10, -0x2008
	ctx.r[9].s64 = ctx.r[10].s64 + -8200;
	// 8265DFD8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265DFDC: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 8265DFE0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265DFE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265DFE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265DFEC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8265DFF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265DFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265DFF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265DFFC: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265E000: 388A0EDC  addi r4, r10, 0xedc
	ctx.r[4].s64 = ctx.r[10].s64 + 3804;
	// 8265E004: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265E008: 386B9E38  addi r3, r11, -0x61c8
	ctx.r[3].s64 = ctx.r[11].s64 + -25032;
	// 8265E00C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265E010: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E018: 4BE08E09  bl 0x82466e20
	ctx.lr = 0x8265E01C;
	sub_82466E20(ctx, base);
	// 8265E01C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E030 size=112
    let mut pc: u32 = 0x8265E030;
    'dispatch: loop {
        match pc {
            0x8265E030 => {
    //   block [0x8265E030..0x8265E0A0)
	// 8265E030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E03C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E040: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E044: 38AA8AE8  addi r5, r10, -0x7518
	ctx.r[5].s64 = ctx.r[10].s64 + -29976;
	// 8265E048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E04C: 390BFE68  addi r8, r11, -0x198
	ctx.r[8].s64 = ctx.r[11].s64 + -408;
	// 8265E050: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8265E054: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 8265E058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E05C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E068: 386A9E68  addi r3, r10, -0x6198
	ctx.r[3].s64 = ctx.r[10].s64 + -24984;
	// 8265E06C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E08C: 4BE08D95  bl 0x82466e20
	ctx.lr = 0x8265E090;
	sub_82466E20(ctx, base);
	// 8265E090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E0A0 size=108
    let mut pc: u32 = 0x8265E0A0;
    'dispatch: loop {
        match pc {
            0x8265E0A0 => {
    //   block [0x8265E0A0..0x8265E10C)
	// 8265E0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E0AC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E0B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E0B4: 38EBFEF8  addi r7, r11, -0x108
	ctx.r[7].s64 = ctx.r[11].s64 + -264;
	// 8265E0B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265E0BC: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 8265E0C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E0C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E0C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E0CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E0D0: 386A9E98  addi r3, r10, -0x6168
	ctx.r[3].s64 = ctx.r[10].s64 + -24936;
	// 8265E0D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E0D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E0DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E0E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E0E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E0E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E0EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E0F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E0F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E0F8: 4BE08D29  bl 0x82466e20
	ctx.lr = 0x8265E0FC;
	sub_82466E20(ctx, base);
	// 8265E0FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E100: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E104: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E108: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E110 size=108
    let mut pc: u32 = 0x8265E110;
    'dispatch: loop {
        match pc {
            0x8265E110 => {
    //   block [0x8265E110..0x8265E17C)
	// 8265E110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E11C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E120: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E124: 38EBFF40  addi r7, r11, -0xc0
	ctx.r[7].s64 = ctx.r[11].s64 + -192;
	// 8265E128: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265E12C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 8265E130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E138: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E13C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E140: 386A9EC8  addi r3, r10, -0x6138
	ctx.r[3].s64 = ctx.r[10].s64 + -24888;
	// 8265E144: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E14C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E164: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E168: 4BE08CB9  bl 0x82466e20
	ctx.lr = 0x8265E16C;
	sub_82466E20(ctx, base);
	// 8265E16C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E180 size=108
    let mut pc: u32 = 0x8265E180;
    'dispatch: loop {
        match pc {
            0x8265E180 => {
    //   block [0x8265E180..0x8265E1EC)
	// 8265E180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E18C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E194: 38EBFF70  addi r7, r11, -0x90
	ctx.r[7].s64 = ctx.r[11].s64 + -144;
	// 8265E198: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265E19C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 8265E1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E1A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E1A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E1AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E1B0: 386A9EF8  addi r3, r10, -0x6108
	ctx.r[3].s64 = ctx.r[10].s64 + -24840;
	// 8265E1B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E1BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E1D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E1D8: 4BE08C49  bl 0x82466e20
	ctx.lr = 0x8265E1DC;
	sub_82466E20(ctx, base);
	// 8265E1DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E1E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E1F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E1F0 size=112
    let mut pc: u32 = 0x8265E1F0;
    'dispatch: loop {
        match pc {
            0x8265E1F0 => {
    //   block [0x8265E1F0..0x8265E260)
	// 8265E1F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E1F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E1F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E1FC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E200: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E204: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E20C: 390BFFA0  addi r8, r11, -0x60
	ctx.r[8].s64 = ctx.r[11].s64 + -96;
	// 8265E210: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265E214: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 8265E218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E21C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E228: 386A9F28  addi r3, r10, -0x60d8
	ctx.r[3].s64 = ctx.r[10].s64 + -24792;
	// 8265E22C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E24C: 4BE08BD5  bl 0x82466e20
	ctx.lr = 0x8265E250;
	sub_82466E20(ctx, base);
	// 8265E250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E260 size=112
    let mut pc: u32 = 0x8265E260;
    'dispatch: loop {
        match pc {
            0x8265E260 => {
    //   block [0x8265E260..0x8265E2D0)
	// 8265E260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E26C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E270: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E274: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E27C: 390BFFD0  addi r8, r11, -0x30
	ctx.r[8].s64 = ctx.r[11].s64 + -48;
	// 8265E280: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265E284: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 8265E288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E28C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E298: 386A9F58  addi r3, r10, -0x60a8
	ctx.r[3].s64 = ctx.r[10].s64 + -24744;
	// 8265E29C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E2A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E2A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E2A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E2B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E2B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E2BC: 4BE08B65  bl 0x82466e20
	ctx.lr = 0x8265E2C0;
	sub_82466E20(ctx, base);
	// 8265E2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E2D0 size=112
    let mut pc: u32 = 0x8265E2D0;
    'dispatch: loop {
        match pc {
            0x8265E2D0 => {
    //   block [0x8265E2D0..0x8265E340)
	// 8265E2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E2DC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E2E0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E2E4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E2EC: 390BFFE8  addi r8, r11, -0x18
	ctx.r[8].s64 = ctx.r[11].s64 + -24;
	// 8265E2F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265E2F4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 8265E2F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E2FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E308: 386A9F88  addi r3, r10, -0x6078
	ctx.r[3].s64 = ctx.r[10].s64 + -24696;
	// 8265E30C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E32C: 4BE08AF5  bl 0x82466e20
	ctx.lr = 0x8265E330;
	sub_82466E20(ctx, base);
	// 8265E330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E340 size=108
    let mut pc: u32 = 0x8265E340;
    'dispatch: loop {
        match pc {
            0x8265E340 => {
    //   block [0x8265E340..0x8265E3AC)
	// 8265E340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E34C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E354: 38EB0000  addi r7, r11, 0
	ctx.r[7].s64 = ctx.r[11].s64 + 0;
	// 8265E358: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265E35C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 8265E360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E370: 386A9FB8  addi r3, r10, -0x6048
	ctx.r[3].s64 = ctx.r[10].s64 + -24648;
	// 8265E374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E37C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E398: 4BE08A89  bl 0x82466e20
	ctx.lr = 0x8265E39C;
	sub_82466E20(ctx, base);
	// 8265E39C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E3A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E3A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E3A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E3B0 size=112
    let mut pc: u32 = 0x8265E3B0;
    'dispatch: loop {
        match pc {
            0x8265E3B0 => {
    //   block [0x8265E3B0..0x8265E420)
	// 8265E3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E3B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E3B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E3BC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E3C0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E3C4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E3C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E3CC: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 8265E3D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265E3D4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 8265E3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E3DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E3E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E3E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E3E8: 386A9FE8  addi r3, r10, -0x6018
	ctx.r[3].s64 = ctx.r[10].s64 + -24600;
	// 8265E3EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E3F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E3F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E3F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E40C: 4BE08A15  bl 0x82466e20
	ctx.lr = 0x8265E410;
	sub_82466E20(ctx, base);
	// 8265E410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E41C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E420 size=108
    let mut pc: u32 = 0x8265E420;
    'dispatch: loop {
        match pc {
            0x8265E420 => {
    //   block [0x8265E420..0x8265E48C)
	// 8265E420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E42C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E434: 38EB0048  addi r7, r11, 0x48
	ctx.r[7].s64 = ctx.r[11].s64 + 72;
	// 8265E438: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8265E43C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 8265E440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E444: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E450: 386AA018  addi r3, r10, -0x5fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -24552;
	// 8265E454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E46C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E478: 4BE089A9  bl 0x82466e20
	ctx.lr = 0x8265E47C;
	sub_82466E20(ctx, base);
	// 8265E47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E490 size=116
    let mut pc: u32 = 0x8265E490;
    'dispatch: loop {
        match pc {
            0x8265E490 => {
    //   block [0x8265E490..0x8265E504)
	// 8265E490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E49C: 3D408279  lis r10, -0x7d87
	ctx.r[10].s64 = -2105999360;
	// 8265E4A0: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 8265E4A4: 390A0138  addi r8, r10, 0x138
	ctx.r[8].s64 = ctx.r[10].s64 + 312;
	// 8265E4A8: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E4AC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265E4B0: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E4B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E4B8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265E4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E4C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E4C4: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 8265E4C8: 396BE010  addi r11, r11, -0x1ff0
	ctx.r[11].s64 = ctx.r[11].s64 + -8176;
	// 8265E4CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E4D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E4D4: 386AA048  addi r3, r10, -0x5fb8
	ctx.r[3].s64 = ctx.r[10].s64 + -24504;
	// 8265E4D8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8265E4DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E4E0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8265E4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E4E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E4F0: 4BE08931  bl 0x82466e20
	ctx.lr = 0x8265E4F4;
	sub_82466E20(ctx, base);
	// 8265E4F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E4F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E4FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E508 size=108
    let mut pc: u32 = 0x8265E508;
    'dispatch: loop {
        match pc {
            0x8265E508 => {
    //   block [0x8265E508..0x8265E574)
	// 8265E508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E50C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E514: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E51C: 38EB0300  addi r7, r11, 0x300
	ctx.r[7].s64 = ctx.r[11].s64 + 768;
	// 8265E520: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 8265E524: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 8265E528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E52C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E530: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E538: 386AA078  addi r3, r10, -0x5f88
	ctx.r[3].s64 = ctx.r[10].s64 + -24456;
	// 8265E53C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E54C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E55C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E560: 4BE088C1  bl 0x82466e20
	ctx.lr = 0x8265E564;
	sub_82466E20(ctx, base);
	// 8265E564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E578 size=112
    let mut pc: u32 = 0x8265E578;
    'dispatch: loop {
        match pc {
            0x8265E578 => {
    //   block [0x8265E578..0x8265E5E8)
	// 8265E578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E584: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E588: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E58C: 38AA8AE8  addi r5, r10, -0x7518
	ctx.r[5].s64 = ctx.r[10].s64 + -29976;
	// 8265E590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E594: 390B0498  addi r8, r11, 0x498
	ctx.r[8].s64 = ctx.r[11].s64 + 1176;
	// 8265E598: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 8265E59C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 8265E5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E5A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E5AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E5B0: 386AA0A8  addi r3, r10, -0x5f58
	ctx.r[3].s64 = ctx.r[10].s64 + -24408;
	// 8265E5B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E5BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E5C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E5D4: 4BE0884D  bl 0x82466e20
	ctx.lr = 0x8265E5D8;
	sub_82466E20(ctx, base);
	// 8265E5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E5E8 size=100
    let mut pc: u32 = 0x8265E5E8;
    'dispatch: loop {
        match pc {
            0x8265E5E8 => {
    //   block [0x8265E5E8..0x8265E64C)
	// 8265E5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E5F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E5FC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E608: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 8265E60C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E61C: 386AA0D8  addi r3, r10, -0x5f28
	ctx.r[3].s64 = ctx.r[10].s64 + -24360;
	// 8265E620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E624: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E628: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E630: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265E634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E638: 4BE087E9  bl 0x82466e20
	ctx.lr = 0x8265E63C;
	sub_82466E20(ctx, base);
	// 8265E63C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E650 size=112
    let mut pc: u32 = 0x8265E650;
    'dispatch: loop {
        match pc {
            0x8265E650 => {
    //   block [0x8265E650..0x8265E6C0)
	// 8265E650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E65C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E660: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E664: 38AAA0D8  addi r5, r10, -0x5f28
	ctx.r[5].s64 = ctx.r[10].s64 + -24360;
	// 8265E668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E66C: 390B06F0  addi r8, r11, 0x6f0
	ctx.r[8].s64 = ctx.r[11].s64 + 1776;
	// 8265E670: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8265E674: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 8265E678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E67C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E688: 386AA108  addi r3, r10, -0x5ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -24312;
	// 8265E68C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E6AC: 4BE08775  bl 0x82466e20
	ctx.lr = 0x8265E6B0;
	sub_82466E20(ctx, base);
	// 8265E6B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E6B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E6B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E6BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E6C0 size=100
    let mut pc: u32 = 0x8265E6C0;
    'dispatch: loop {
        match pc {
            0x8265E6C0 => {
    //   block [0x8265E6C0..0x8265E724)
	// 8265E6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E6C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E6CC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E6D4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E6DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E6E0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 8265E6E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E6F4: 386AA138  addi r3, r10, -0x5ec8
	ctx.r[3].s64 = ctx.r[10].s64 + -24264;
	// 8265E6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E6FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E700: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E708: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265E70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E710: 4BE08711  bl 0x82466e20
	ctx.lr = 0x8265E714;
	sub_82466E20(ctx, base);
	// 8265E714: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E71C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E728 size=108
    let mut pc: u32 = 0x8265E728;
    'dispatch: loop {
        match pc {
            0x8265E728 => {
    //   block [0x8265E728..0x8265E794)
	// 8265E728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E734: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E738: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E73C: 38EB0768  addi r7, r11, 0x768
	ctx.r[7].s64 = ctx.r[11].s64 + 1896;
	// 8265E740: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265E744: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 8265E748: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E74C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E750: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265E754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E758: 386AA168  addi r3, r10, -0x5e98
	ctx.r[3].s64 = ctx.r[10].s64 + -24216;
	// 8265E75C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265E760: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E76C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E77C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265E780: 4BE086A1  bl 0x82466e20
	ctx.lr = 0x8265E784;
	sub_82466E20(ctx, base);
	// 8265E784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E78C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E798 size=112
    let mut pc: u32 = 0x8265E798;
    'dispatch: loop {
        match pc {
            0x8265E798 => {
    //   block [0x8265E798..0x8265E808)
	// 8265E798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E7A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E7A8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E7AC: 38AAA138  addi r5, r10, -0x5ec8
	ctx.r[5].s64 = ctx.r[10].s64 + -24264;
	// 8265E7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E7B4: 390B07B0  addi r8, r11, 0x7b0
	ctx.r[8].s64 = ctx.r[11].s64 + 1968;
	// 8265E7B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8265E7BC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 8265E7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E7C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E7C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E7D0: 386AA198  addi r3, r10, -0x5e68
	ctx.r[3].s64 = ctx.r[10].s64 + -24168;
	// 8265E7D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E7D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E7F4: 4BE0862D  bl 0x82466e20
	ctx.lr = 0x8265E7F8;
	sub_82466E20(ctx, base);
	// 8265E7F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E808 size=100
    let mut pc: u32 = 0x8265E808;
    'dispatch: loop {
        match pc {
            0x8265E808 => {
    //   block [0x8265E808..0x8265E86C)
	// 8265E808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E814: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E81C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E828: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 8265E82C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E83C: 386AA1C8  addi r3, r10, -0x5e38
	ctx.r[3].s64 = ctx.r[10].s64 + -24120;
	// 8265E840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E844: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E848: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E850: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265E854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E858: 4BE085C9  bl 0x82466e20
	ctx.lr = 0x8265E85C;
	sub_82466E20(ctx, base);
	// 8265E85C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E870 size=100
    let mut pc: u32 = 0x8265E870;
    'dispatch: loop {
        match pc {
            0x8265E870 => {
    //   block [0x8265E870..0x8265E8D4)
	// 8265E870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E87C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E884: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E890: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 8265E894: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E8A4: 386AA1F8  addi r3, r10, -0x5e08
	ctx.r[3].s64 = ctx.r[10].s64 + -24072;
	// 8265E8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E8AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E8B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E8B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E8B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265E8BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E8C0: 4BE08561  bl 0x82466e20
	ctx.lr = 0x8265E8C4;
	sub_82466E20(ctx, base);
	// 8265E8C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E8C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E8CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E8D8 size=112
    let mut pc: u32 = 0x8265E8D8;
    'dispatch: loop {
        match pc {
            0x8265E8D8 => {
    //   block [0x8265E8D8..0x8265E948)
	// 8265E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E8E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E8E8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E8EC: 38AAA1C8  addi r5, r10, -0x5e38
	ctx.r[5].s64 = ctx.r[10].s64 + -24120;
	// 8265E8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E8F4: 390B07E0  addi r8, r11, 0x7e0
	ctx.r[8].s64 = ctx.r[11].s64 + 2016;
	// 8265E8F8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265E8FC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 8265E900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E904: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E908: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E90C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E910: 386AA228  addi r3, r10, -0x5dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -24024;
	// 8265E914: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E918: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E91C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E934: 4BE084ED  bl 0x82466e20
	ctx.lr = 0x8265E938;
	sub_82466E20(ctx, base);
	// 8265E938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E948 size=112
    let mut pc: u32 = 0x8265E948;
    'dispatch: loop {
        match pc {
            0x8265E948 => {
    //   block [0x8265E948..0x8265E9B8)
	// 8265E948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E954: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E958: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265E95C: 38AAA1F8  addi r5, r10, -0x5e08
	ctx.r[5].s64 = ctx.r[10].s64 + -24072;
	// 8265E960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E964: 390B0840  addi r8, r11, 0x840
	ctx.r[8].s64 = ctx.r[11].s64 + 2112;
	// 8265E968: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8265E96C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 8265E970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E974: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E978: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265E97C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265E980: 386AA258  addi r3, r10, -0x5da8
	ctx.r[3].s64 = ctx.r[10].s64 + -23976;
	// 8265E984: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265E988: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E990: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E998: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E99C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265E9A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E9A4: 4BE0847D  bl 0x82466e20
	ctx.lr = 0x8265E9A8;
	sub_82466E20(ctx, base);
	// 8265E9A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265E9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265E9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265E9B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265E9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265E9B8 size=100
    let mut pc: u32 = 0x8265E9B8;
    'dispatch: loop {
        match pc {
            0x8265E9B8 => {
    //   block [0x8265E9B8..0x8265EA1C)
	// 8265E9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265E9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265E9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265E9C4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265E9C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265E9CC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265E9D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265E9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265E9D8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 8265E9DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265E9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265E9E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265E9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265E9EC: 386AA288  addi r3, r10, -0x5d78
	ctx.r[3].s64 = ctx.r[10].s64 + -23928;
	// 8265E9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265E9F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265E9F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265E9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EA00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265EA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EA08: 4BE08419  bl 0x82466e20
	ctx.lr = 0x8265EA0C;
	sub_82466E20(ctx, base);
	// 8265EA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EA20 size=112
    let mut pc: u32 = 0x8265EA20;
    'dispatch: loop {
        match pc {
            0x8265EA20 => {
    //   block [0x8265EA20..0x8265EA90)
	// 8265EA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EA2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EA30: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EA34: 38AAA288  addi r5, r10, -0x5d78
	ctx.r[5].s64 = ctx.r[10].s64 + -23928;
	// 8265EA38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EA3C: 390B08A0  addi r8, r11, 0x8a0
	ctx.r[8].s64 = ctx.r[11].s64 + 2208;
	// 8265EA40: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8265EA44: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 8265EA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EA4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EA50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265EA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EA58: 386AA2B8  addi r3, r10, -0x5d48
	ctx.r[3].s64 = ctx.r[10].s64 + -23880;
	// 8265EA5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265EA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EA7C: 4BE083A5  bl 0x82466e20
	ctx.lr = 0x8265EA80;
	sub_82466E20(ctx, base);
	// 8265EA80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EA90 size=108
    let mut pc: u32 = 0x8265EA90;
    'dispatch: loop {
        match pc {
            0x8265EA90 => {
    //   block [0x8265EA90..0x8265EAFC)
	// 8265EA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EA9C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EAA4: 38EB0990  addi r7, r11, 0x990
	ctx.r[7].s64 = ctx.r[11].s64 + 2448;
	// 8265EAA8: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 8265EAAC: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 8265EAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EAB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EAB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EAC0: 386AA2E8  addi r3, r10, -0x5d18
	ctx.r[3].s64 = ctx.r[10].s64 + -23832;
	// 8265EAC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EAE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EAE8: 4BE08339  bl 0x82466e20
	ctx.lr = 0x8265EAEC;
	sub_82466E20(ctx, base);
	// 8265EAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EB00 size=108
    let mut pc: u32 = 0x8265EB00;
    'dispatch: loop {
        match pc {
            0x8265EB00 => {
    //   block [0x8265EB00..0x8265EB6C)
	// 8265EB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EB0C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EB14: 38EB0A80  addi r7, r11, 0xa80
	ctx.r[7].s64 = ctx.r[11].s64 + 2688;
	// 8265EB18: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265EB1C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 8265EB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EB24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EB30: 386AA318  addi r3, r10, -0x5ce8
	ctx.r[3].s64 = ctx.r[10].s64 + -23784;
	// 8265EB34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EB54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EB58: 4BE082C9  bl 0x82466e20
	ctx.lr = 0x8265EB5C;
	sub_82466E20(ctx, base);
	// 8265EB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EB70 size=108
    let mut pc: u32 = 0x8265EB70;
    'dispatch: loop {
        match pc {
            0x8265EB70 => {
    //   block [0x8265EB70..0x8265EBDC)
	// 8265EB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EB7C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EB84: 38EB0AC8  addi r7, r11, 0xac8
	ctx.r[7].s64 = ctx.r[11].s64 + 2760;
	// 8265EB88: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8265EB8C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 8265EB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EB94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EB98: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EB9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EBA0: 386AA348  addi r3, r10, -0x5cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -23736;
	// 8265EBA4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EBAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EBB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EBC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EBC8: 4BE08259  bl 0x82466e20
	ctx.lr = 0x8265EBCC;
	sub_82466E20(ctx, base);
	// 8265EBCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EBD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EBD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EBD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EBE0 size=108
    let mut pc: u32 = 0x8265EBE0;
    'dispatch: loop {
        match pc {
            0x8265EBE0 => {
    //   block [0x8265EBE0..0x8265EC4C)
	// 8265EBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EBEC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EBF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EBF4: 38EB0BA0  addi r7, r11, 0xba0
	ctx.r[7].s64 = ctx.r[11].s64 + 2976;
	// 8265EBF8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265EBFC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 8265EC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EC04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EC08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EC10: 386AA378  addi r3, r10, -0x5c88
	ctx.r[3].s64 = ctx.r[10].s64 + -23688;
	// 8265EC14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EC38: 4BE081E9  bl 0x82466e20
	ctx.lr = 0x8265EC3C;
	sub_82466E20(ctx, base);
	// 8265EC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EC50 size=100
    let mut pc: u32 = 0x8265EC50;
    'dispatch: loop {
        match pc {
            0x8265EC50 => {
    //   block [0x8265EC50..0x8265ECB4)
	// 8265EC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EC5C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265EC60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EC64: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265EC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EC6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EC70: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 8265EC74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EC84: 386AA3A8  addi r3, r10, -0x5c58
	ctx.r[3].s64 = ctx.r[10].s64 + -23640;
	// 8265EC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EC8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EC90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265EC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EC98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265EC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ECA0: 4BE08181  bl 0x82466e20
	ctx.lr = 0x8265ECA4;
	sub_82466E20(ctx, base);
	// 8265ECA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ECA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ECAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ECB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ECB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ECB8 size=112
    let mut pc: u32 = 0x8265ECB8;
    'dispatch: loop {
        match pc {
            0x8265ECB8 => {
    //   block [0x8265ECB8..0x8265ED28)
	// 8265ECB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ECBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ECC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ECC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ECC8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265ECCC: 38AAA3A8  addi r5, r10, -0x5c58
	ctx.r[5].s64 = ctx.r[10].s64 + -23640;
	// 8265ECD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265ECD4: 390B0BB8  addi r8, r11, 0xbb8
	ctx.r[8].s64 = ctx.r[11].s64 + 3000;
	// 8265ECD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265ECDC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 8265ECE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ECE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ECE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265ECEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ECF0: 386AA3D8  addi r3, r10, -0x5c28
	ctx.r[3].s64 = ctx.r[10].s64 + -23592;
	// 8265ECF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265ECF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265ECFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265ED00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265ED04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ED08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ED0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ED10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ED14: 4BE0810D  bl 0x82466e20
	ctx.lr = 0x8265ED18;
	sub_82466E20(ctx, base);
	// 8265ED18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ED1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ED20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ED24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ED28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ED28 size=108
    let mut pc: u32 = 0x8265ED28;
    'dispatch: loop {
        match pc {
            0x8265ED28 => {
    //   block [0x8265ED28..0x8265ED94)
	// 8265ED28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ED2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265ED30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265ED34: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265ED38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265ED3C: 38EB0C00  addi r7, r11, 0xc00
	ctx.r[7].s64 = ctx.r[11].s64 + 3072;
	// 8265ED40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265ED44: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 8265ED48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265ED4C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265ED50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265ED54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265ED58: 386AA408  addi r3, r10, -0x5bf8
	ctx.r[3].s64 = ctx.r[10].s64 + -23544;
	// 8265ED5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265ED60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265ED64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265ED68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265ED6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265ED70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265ED74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265ED78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265ED7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265ED80: 4BE080A1  bl 0x82466e20
	ctx.lr = 0x8265ED84;
	sub_82466E20(ctx, base);
	// 8265ED84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265ED88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265ED8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265ED90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265ED98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265ED98 size=112
    let mut pc: u32 = 0x8265ED98;
    'dispatch: loop {
        match pc {
            0x8265ED98 => {
    //   block [0x8265ED98..0x8265EE08)
	// 8265ED98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265ED9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EDA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EDA4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265EDA8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EDAC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265EDB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EDB4: 390B0C48  addi r8, r11, 0xc48
	ctx.r[8].s64 = ctx.r[11].s64 + 3144;
	// 8265EDB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265EDBC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 8265EDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EDC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EDC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265EDCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EDD0: 386AA438  addi r3, r10, -0x5bc8
	ctx.r[3].s64 = ctx.r[10].s64 + -23496;
	// 8265EDD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265EDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EDDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EDF4: 4BE0802D  bl 0x82466e20
	ctx.lr = 0x8265EDF8;
	sub_82466E20(ctx, base);
	// 8265EDF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EDFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EE00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EE04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EE08 size=108
    let mut pc: u32 = 0x8265EE08;
    'dispatch: loop {
        match pc {
            0x8265EE08 => {
    //   block [0x8265EE08..0x8265EE74)
	// 8265EE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EE0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EE10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EE14: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EE18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EE1C: 38EB0C60  addi r7, r11, 0xc60
	ctx.r[7].s64 = ctx.r[11].s64 + 3168;
	// 8265EE20: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265EE24: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 8265EE28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EE2C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EE30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EE34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EE38: 386AA468  addi r3, r10, -0x5b98
	ctx.r[3].s64 = ctx.r[10].s64 + -23448;
	// 8265EE3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EE44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EE54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EE5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265EE60: 4BE07FC1  bl 0x82466e20
	ctx.lr = 0x8265EE64;
	sub_82466E20(ctx, base);
	// 8265EE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EE78 size=112
    let mut pc: u32 = 0x8265EE78;
    'dispatch: loop {
        match pc {
            0x8265EE78 => {
    //   block [0x8265EE78..0x8265EEE8)
	// 8265EE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EE84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EE88: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EE8C: 38AAA438  addi r5, r10, -0x5bc8
	ctx.r[5].s64 = ctx.r[10].s64 + -23496;
	// 8265EE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EE94: 390B0CA8  addi r8, r11, 0xca8
	ctx.r[8].s64 = ctx.r[11].s64 + 3240;
	// 8265EE98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265EE9C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 8265EEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EEA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EEA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265EEAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EEB0: 386AA498  addi r3, r10, -0x5b68
	ctx.r[3].s64 = ctx.r[10].s64 + -23400;
	// 8265EEB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265EEB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EEBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EED4: 4BE07F4D  bl 0x82466e20
	ctx.lr = 0x8265EED8;
	sub_82466E20(ctx, base);
	// 8265EED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EEE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EEE8 size=100
    let mut pc: u32 = 0x8265EEE8;
    'dispatch: loop {
        match pc {
            0x8265EEE8 => {
    //   block [0x8265EEE8..0x8265EF4C)
	// 8265EEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EEF4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265EEF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EEFC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265EF00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EF08: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 8265EF0C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EF1C: 386AA4C8  addi r3, r10, -0x5b38
	ctx.r[3].s64 = ctx.r[10].s64 + -23352;
	// 8265EF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EF24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EF28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265EF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EF30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265EF34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EF38: 4BE07EE9  bl 0x82466e20
	ctx.lr = 0x8265EF3C;
	sub_82466E20(ctx, base);
	// 8265EF3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EF40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EF44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EF48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EF50 size=112
    let mut pc: u32 = 0x8265EF50;
    'dispatch: loop {
        match pc {
            0x8265EF50 => {
    //   block [0x8265EF50..0x8265EFC0)
	// 8265EF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EF58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EF5C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EF60: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EF64: 38AAA4C8  addi r5, r10, -0x5b38
	ctx.r[5].s64 = ctx.r[10].s64 + -23352;
	// 8265EF68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EF6C: 390B0CC0  addi r8, r11, 0xcc0
	ctx.r[8].s64 = ctx.r[11].s64 + 3264;
	// 8265EF70: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8265EF74: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 8265EF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EF7C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EF80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265EF84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265EF88: 386AA4F8  addi r3, r10, -0x5b08
	ctx.r[3].s64 = ctx.r[10].s64 + -23304;
	// 8265EF8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265EF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265EF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265EFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265EFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265EFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265EFAC: 4BE07E75  bl 0x82466e20
	ctx.lr = 0x8265EFB0;
	sub_82466E20(ctx, base);
	// 8265EFB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265EFB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265EFB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265EFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265EFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265EFC0 size=108
    let mut pc: u32 = 0x8265EFC0;
    'dispatch: loop {
        match pc {
            0x8265EFC0 => {
    //   block [0x8265EFC0..0x8265F02C)
	// 8265EFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265EFC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265EFC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265EFCC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265EFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265EFD4: 38EB0D68  addi r7, r11, 0xd68
	ctx.r[7].s64 = ctx.r[11].s64 + 3432;
	// 8265EFD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265EFDC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 8265EFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265EFE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265EFE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265EFEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265EFF0: 386AA528  addi r3, r10, -0x5ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -23256;
	// 8265EFF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265EFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265EFFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F00C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F018: 4BE07E09  bl 0x82466e20
	ctx.lr = 0x8265F01C;
	sub_82466E20(ctx, base);
	// 8265F01C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F030 size=112
    let mut pc: u32 = 0x8265F030;
    'dispatch: loop {
        match pc {
            0x8265F030 => {
    //   block [0x8265F030..0x8265F0A0)
	// 8265F030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F03C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F040: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F044: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F04C: 390B0D98  addi r8, r11, 0xd98
	ctx.r[8].s64 = ctx.r[11].s64 + 3480;
	// 8265F050: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265F054: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 8265F058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F05C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F068: 386AA558  addi r3, r10, -0x5aa8
	ctx.r[3].s64 = ctx.r[10].s64 + -23208;
	// 8265F06C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F08C: 4BE07D95  bl 0x82466e20
	ctx.lr = 0x8265F090;
	sub_82466E20(ctx, base);
	// 8265F090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F0A0 size=112
    let mut pc: u32 = 0x8265F0A0;
    'dispatch: loop {
        match pc {
            0x8265F0A0 => {
    //   block [0x8265F0A0..0x8265F110)
	// 8265F0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F0AC: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F0B0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F0B4: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F0BC: 390B0DE0  addi r8, r11, 0xde0
	ctx.r[8].s64 = ctx.r[11].s64 + 3552;
	// 8265F0C0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265F0C4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 8265F0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F0CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F0D8: 386AA588  addi r3, r10, -0x5a78
	ctx.r[3].s64 = ctx.r[10].s64 + -23160;
	// 8265F0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F0FC: 4BE07D25  bl 0x82466e20
	ctx.lr = 0x8265F100;
	sub_82466E20(ctx, base);
	// 8265F100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F110 size=100
    let mut pc: u32 = 0x8265F110;
    'dispatch: loop {
        match pc {
            0x8265F110 => {
    //   block [0x8265F110..0x8265F174)
	// 8265F110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F11C: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F124: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F12C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F130: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 8265F134: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F13C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F144: 386AA5B8  addi r3, r10, -0x5a48
	ctx.r[3].s64 = ctx.r[10].s64 + -23112;
	// 8265F148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F14C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F150: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8265F154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F158: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8265F15C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F160: 4BE07CC1  bl 0x82466e20
	ctx.lr = 0x8265F164;
	sub_82466E20(ctx, base);
	// 8265F164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F16C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F178 size=112
    let mut pc: u32 = 0x8265F178;
    'dispatch: loop {
        match pc {
            0x8265F178 => {
    //   block [0x8265F178..0x8265F1E8)
	// 8265F178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F17C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F184: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F188: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F18C: 38AAA5B8  addi r5, r10, -0x5a48
	ctx.r[5].s64 = ctx.r[10].s64 + -23112;
	// 8265F190: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F194: 390B0E28  addi r8, r11, 0xe28
	ctx.r[8].s64 = ctx.r[11].s64 + 3624;
	// 8265F198: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265F19C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 8265F1A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F1A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F1A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F1AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F1B0: 386AA5E8  addi r3, r10, -0x5a18
	ctx.r[3].s64 = ctx.r[10].s64 + -23064;
	// 8265F1B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F1B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F1D4: 4BE07C4D  bl 0x82466e20
	ctx.lr = 0x8265F1D8;
	sub_82466E20(ctx, base);
	// 8265F1D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F1DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F1E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F1E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F1E8 size=112
    let mut pc: u32 = 0x8265F1E8;
    'dispatch: loop {
        match pc {
            0x8265F1E8 => {
    //   block [0x8265F1E8..0x8265F258)
	// 8265F1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F1F4: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F1F8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F1FC: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F200: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F204: 390B0E70  addi r8, r11, 0xe70
	ctx.r[8].s64 = ctx.r[11].s64 + 3696;
	// 8265F208: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265F20C: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 8265F210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F214: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F218: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F220: 386AA618  addi r3, r10, -0x59e8
	ctx.r[3].s64 = ctx.r[10].s64 + -23016;
	// 8265F224: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F228: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F234: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F23C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F244: 4BE07BDD  bl 0x82466e20
	ctx.lr = 0x8265F248;
	sub_82466E20(ctx, base);
	// 8265F248: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F24C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F258 size=112
    let mut pc: u32 = 0x8265F258;
    'dispatch: loop {
        match pc {
            0x8265F258 => {
    //   block [0x8265F258..0x8265F2C8)
	// 8265F258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F264: 3D408295  lis r10, -0x7d6b
	ctx.r[10].s64 = -2104164352;
	// 8265F268: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F26C: 38AA6868  addi r5, r10, 0x6868
	ctx.r[5].s64 = ctx.r[10].s64 + 26728;
	// 8265F270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F274: 390B0E88  addi r8, r11, 0xe88
	ctx.r[8].s64 = ctx.r[11].s64 + 3720;
	// 8265F278: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265F27C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 8265F280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F284: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F288: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F28C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F290: 386AA648  addi r3, r10, -0x59b8
	ctx.r[3].s64 = ctx.r[10].s64 + -22968;
	// 8265F294: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F2A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F2A4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265F2A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F2B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F2B4: 4BE07B6D  bl 0x82466e20
	ctx.lr = 0x8265F2B8;
	sub_82466E20(ctx, base);
	// 8265F2B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F2BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F2C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F2C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F2C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F2C8 size=112
    let mut pc: u32 = 0x8265F2C8;
    'dispatch: loop {
        match pc {
            0x8265F2C8 => {
    //   block [0x8265F2C8..0x8265F338)
	// 8265F2C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F2CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F2D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F2D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F2D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F2DC: 38AAA618  addi r5, r10, -0x59e8
	ctx.r[5].s64 = ctx.r[10].s64 + -23016;
	// 8265F2E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F2E4: 390B0EA0  addi r8, r11, 0xea0
	ctx.r[8].s64 = ctx.r[11].s64 + 3744;
	// 8265F2E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8265F2EC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 8265F2F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F2F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F2F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F300: 386AA678  addi r3, r10, -0x5988
	ctx.r[3].s64 = ctx.r[10].s64 + -22920;
	// 8265F304: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F314: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F324: 4BE07AFD  bl 0x82466e20
	ctx.lr = 0x8265F328;
	sub_82466E20(ctx, base);
	// 8265F328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F338 size=72
    let mut pc: u32 = 0x8265F338;
    'dispatch: loop {
        match pc {
            0x8265F338 => {
    //   block [0x8265F338..0x8265F380)
	// 8265F338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F344: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265F348: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8265F34C: 38CBC808  addi r6, r11, -0x37f8
	ctx.r[6].s64 = ctx.r[11].s64 + -14328;
	// 8265F350: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8265F354: 388BE060  addi r4, r11, -0x1fa0
	ctx.r[4].s64 = ctx.r[11].s64 + -8096;
	// 8265F358: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265F35C: 386BA6A8  addi r3, r11, -0x5958
	ctx.r[3].s64 = ctx.r[11].s64 + -22872;
	// 8265F360: 4BE1C729  bl 0x8247ba88
	ctx.lr = 0x8265F364;
	sub_8247BA88(ctx, base);
	// 8265F364: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 8265F368: 386BCDE0  addi r3, r11, -0x3220
	ctx.r[3].s64 = ctx.r[11].s64 + -12832;
	// 8265F36C: 4BED37CD  bl 0x82532b38
	ctx.lr = 0x8265F370;
	sub_82532B38(ctx, base);
	// 8265F370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8265F374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F37C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F380 size=108
    let mut pc: u32 = 0x8265F380;
    'dispatch: loop {
        match pc {
            0x8265F380 => {
    //   block [0x8265F380..0x8265F3EC)
	// 8265F380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F38C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F390: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F394: 38EB2BE0  addi r7, r11, 0x2be0
	ctx.r[7].s64 = ctx.r[11].s64 + 11232;
	// 8265F398: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8265F39C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 8265F3A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F3A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F3A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F3B0: 386AA6C0  addi r3, r10, -0x5940
	ctx.r[3].s64 = ctx.r[10].s64 + -22848;
	// 8265F3B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F3B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F3BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F3C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F3C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F3C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F3CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F3D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F3D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F3D8: 4BE07A49  bl 0x82466e20
	ctx.lr = 0x8265F3DC;
	sub_82466E20(ctx, base);
	// 8265F3DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F3E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F3E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F3E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265F3F0 size=24
    let mut pc: u32 = 0x8265F3F0;
    'dispatch: loop {
        match pc {
            0x8265F3F0 => {
    //   block [0x8265F3F0..0x8265F408)
	// 8265F3F0: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F3F4: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8265F3F8: 394AB1B0  addi r10, r10, -0x4e50
	ctx.r[10].s64 = ctx.r[10].s64 + -20048;
	// 8265F3FC: 816B2C58  lwz r11, 0x2c58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11352 as u32) ) } as u64;
	// 8265F400: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8265F404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F408 size=112
    let mut pc: u32 = 0x8265F408;
    'dispatch: loop {
        match pc {
            0x8265F408 => {
    //   block [0x8265F408..0x8265F478)
	// 8265F408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F414: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265F418: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8265F41C: 392AE774  addi r9, r10, -0x188c
	ctx.r[9].s64 = ctx.r[10].s64 + -6284;
	// 8265F420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F424: 390BB1B0  addi r8, r11, -0x4e50
	ctx.r[8].s64 = ctx.r[11].s64 + -20048;
	// 8265F428: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8265F42C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 8265F430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F434: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F43C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F440: 386AA6F0  addi r3, r10, -0x5910
	ctx.r[3].s64 = ctx.r[10].s64 + -22800;
	// 8265F444: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265F448: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265F44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F45C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F464: 4BE079BD  bl 0x82466e20
	ctx.lr = 0x8265F468;
	sub_82466E20(ctx, base);
	// 8265F468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F478 size=108
    let mut pc: u32 = 0x8265F478;
    'dispatch: loop {
        match pc {
            0x8265F478 => {
    //   block [0x8265F478..0x8265F4E4)
	// 8265F478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F484: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F48C: 38EB2C5C  addi r7, r11, 0x2c5c
	ctx.r[7].s64 = ctx.r[11].s64 + 11356;
	// 8265F490: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265F494: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 8265F498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F49C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F4A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F4A8: 386AA720  addi r3, r10, -0x58e0
	ctx.r[3].s64 = ctx.r[10].s64 + -22752;
	// 8265F4AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F4CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F4D0: 4BE07951  bl 0x82466e20
	ctx.lr = 0x8265F4D4;
	sub_82466E20(ctx, base);
	// 8265F4D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F4D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F4DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F4E8 size=108
    let mut pc: u32 = 0x8265F4E8;
    'dispatch: loop {
        match pc {
            0x8265F4E8 => {
    //   block [0x8265F4E8..0x8265F554)
	// 8265F4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F4F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F4FC: 38EB2C8C  addi r7, r11, 0x2c8c
	ctx.r[7].s64 = ctx.r[11].s64 + 11404;
	// 8265F500: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8265F504: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 8265F508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F50C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F510: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F514: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F518: 386AA750  addi r3, r10, -0x58b0
	ctx.r[3].s64 = ctx.r[10].s64 + -22704;
	// 8265F51C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F524: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F53C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F540: 4BE078E1  bl 0x82466e20
	ctx.lr = 0x8265F544;
	sub_82466E20(ctx, base);
	// 8265F544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F54C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8265F558 size=24
    let mut pc: u32 = 0x8265F558;
    'dispatch: loop {
        match pc {
            0x8265F558 => {
    //   block [0x8265F558..0x8265F570)
	// 8265F558: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F55C: 3D40827A  lis r10, -0x7d86
	ctx.r[10].s64 = -2105933824;
	// 8265F560: 394AB1F8  addi r10, r10, -0x4e08
	ctx.r[10].s64 = ctx.r[10].s64 + -19976;
	// 8265F564: 816B2CBC  lwz r11, 0x2cbc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11452 as u32) ) } as u64;
	// 8265F568: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8265F56C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F570 size=116
    let mut pc: u32 = 0x8265F570;
    'dispatch: loop {
        match pc {
            0x8265F570 => {
    //   block [0x8265F570..0x8265F5E4)
	// 8265F570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F57C: 3D60827A  lis r11, -0x7d86
	ctx.r[11].s64 = -2105933824;
	// 8265F580: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265F584: 390BB1F8  addi r8, r11, -0x4e08
	ctx.r[8].s64 = ctx.r[11].s64 + -19976;
	// 8265F588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F58C: 392AE7A8  addi r9, r10, -0x1858
	ctx.r[9].s64 = ctx.r[10].s64 + -6232;
	// 8265F590: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F594: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8265F598: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265F59C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F5A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F5AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F5B4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265F5B8: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 8265F5BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265F5C0: 386BA780  addi r3, r11, -0x5880
	ctx.r[3].s64 = ctx.r[11].s64 + -22656;
	// 8265F5C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265F5C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F5D0: 4BE07851  bl 0x82466e20
	ctx.lr = 0x8265F5D4;
	sub_82466E20(ctx, base);
	// 8265F5D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F5D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F5DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F5E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F5E8 size=108
    let mut pc: u32 = 0x8265F5E8;
    'dispatch: loop {
        match pc {
            0x8265F5E8 => {
    //   block [0x8265F5E8..0x8265F654)
	// 8265F5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F5F4: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F5FC: 38EB2CC0  addi r7, r11, 0x2cc0
	ctx.r[7].s64 = ctx.r[11].s64 + 11456;
	// 8265F600: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265F604: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 8265F608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F60C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F618: 386AA7B0  addi r3, r10, -0x5850
	ctx.r[3].s64 = ctx.r[10].s64 + -22608;
	// 8265F61C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F63C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F640: 4BE077E1  bl 0x82466e20
	ctx.lr = 0x8265F644;
	sub_82466E20(ctx, base);
	// 8265F644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F658 size=112
    let mut pc: u32 = 0x8265F658;
    'dispatch: loop {
        match pc {
            0x8265F658 => {
    //   block [0x8265F658..0x8265F6C8)
	// 8265F658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F664: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F668: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F66C: 38AAA780  addi r5, r10, -0x5880
	ctx.r[5].s64 = ctx.r[10].s64 + -22656;
	// 8265F670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F674: 390B2D50  addi r8, r11, 0x2d50
	ctx.r[8].s64 = ctx.r[11].s64 + 11600;
	// 8265F678: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 8265F67C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 8265F680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F684: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F68C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F690: 386AA7E0  addi r3, r10, -0x5820
	ctx.r[3].s64 = ctx.r[10].s64 + -22560;
	// 8265F694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F69C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F6A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F6B4: 4BE0776D  bl 0x82466e20
	ctx.lr = 0x8265F6B8;
	sub_82466E20(ctx, base);
	// 8265F6B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F6BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F6C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F6C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F6C8 size=112
    let mut pc: u32 = 0x8265F6C8;
    'dispatch: loop {
        match pc {
            0x8265F6C8 => {
    //   block [0x8265F6C8..0x8265F738)
	// 8265F6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F6D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F6D8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F6DC: 38AAA780  addi r5, r10, -0x5880
	ctx.r[5].s64 = ctx.r[10].s64 + -22656;
	// 8265F6E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F6E4: 390B2E70  addi r8, r11, 0x2e70
	ctx.r[8].s64 = ctx.r[11].s64 + 11888;
	// 8265F6E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8265F6EC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 8265F6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F6F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F6F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F700: 386AA810  addi r3, r10, -0x57f0
	ctx.r[3].s64 = ctx.r[10].s64 + -22512;
	// 8265F704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F70C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F724: 4BE076FD  bl 0x82466e20
	ctx.lr = 0x8265F728;
	sub_82466E20(ctx, base);
	// 8265F728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F72C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F738 size=108
    let mut pc: u32 = 0x8265F738;
    'dispatch: loop {
        match pc {
            0x8265F738 => {
    //   block [0x8265F738..0x8265F7A4)
	// 8265F738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F744: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F74C: 38EB2E88  addi r7, r11, 0x2e88
	ctx.r[7].s64 = ctx.r[11].s64 + 11912;
	// 8265F750: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265F754: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 8265F758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F75C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F768: 386AA840  addi r3, r10, -0x57c0
	ctx.r[3].s64 = ctx.r[10].s64 + -22464;
	// 8265F76C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F78C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F790: 4BE07691  bl 0x82466e20
	ctx.lr = 0x8265F794;
	sub_82466E20(ctx, base);
	// 8265F794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F7A8 size=112
    let mut pc: u32 = 0x8265F7A8;
    'dispatch: loop {
        match pc {
            0x8265F7A8 => {
    //   block [0x8265F7A8..0x8265F818)
	// 8265F7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F7B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F7B8: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F7BC: 38AAA780  addi r5, r10, -0x5880
	ctx.r[5].s64 = ctx.r[10].s64 + -22656;
	// 8265F7C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F7C4: 390B2F18  addi r8, r11, 0x2f18
	ctx.r[8].s64 = ctx.r[11].s64 + 12056;
	// 8265F7C8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 8265F7CC: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 8265F7D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F7D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F7D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F7DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F7E0: 386AA870  addi r3, r10, -0x5790
	ctx.r[3].s64 = ctx.r[10].s64 + -22416;
	// 8265F7E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8265F7E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F7EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F7F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F7F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F7F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F7FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F804: 4BE0761D  bl 0x82466e20
	ctx.lr = 0x8265F808;
	sub_82466E20(ctx, base);
	// 8265F808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F818 size=108
    let mut pc: u32 = 0x8265F818;
    'dispatch: loop {
        match pc {
            0x8265F818 => {
    //   block [0x8265F818..0x8265F884)
	// 8265F818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F824: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F82C: 38EB3008  addi r7, r11, 0x3008
	ctx.r[7].s64 = ctx.r[11].s64 + 12296;
	// 8265F830: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8265F834: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 8265F838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F83C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F848: 386AA8A0  addi r3, r10, -0x5760
	ctx.r[3].s64 = ctx.r[10].s64 + -22368;
	// 8265F84C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F86C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F870: 4BE075B1  bl 0x82466e20
	ctx.lr = 0x8265F874;
	sub_82466E20(ctx, base);
	// 8265F874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F888 size=108
    let mut pc: u32 = 0x8265F888;
    'dispatch: loop {
        match pc {
            0x8265F888 => {
    //   block [0x8265F888..0x8265F8F4)
	// 8265F888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F894: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F89C: 38EB3020  addi r7, r11, 0x3020
	ctx.r[7].s64 = ctx.r[11].s64 + 12320;
	// 8265F8A0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8265F8A4: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 8265F8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F8AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F8B8: 386AA8D0  addi r3, r10, -0x5730
	ctx.r[3].s64 = ctx.r[10].s64 + -22320;
	// 8265F8BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F8E0: 4BE07541  bl 0x82466e20
	ctx.lr = 0x8265F8E4;
	sub_82466E20(ctx, base);
	// 8265F8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F8F8 size=116
    let mut pc: u32 = 0x8265F8F8;
    'dispatch: loop {
        match pc {
            0x8265F8F8 => {
    //   block [0x8265F8F8..0x8265F96C)
	// 8265F8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F904: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F908: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8265F90C: 390B3084  addi r8, r11, 0x3084
	ctx.r[8].s64 = ctx.r[11].s64 + 12420;
	// 8265F910: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F914: 392AE7D4  addi r9, r10, -0x182c
	ctx.r[9].s64 = ctx.r[10].s64 + -6188;
	// 8265F918: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F91C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8265F920: 38AABB60  addi r5, r10, -0x44a0
	ctx.r[5].s64 = ctx.r[10].s64 + -17568;
	// 8265F924: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8265F928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F92C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F93C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8265F940: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 8265F944: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8265F948: 386BA900  addi r3, r11, -0x5700
	ctx.r[3].s64 = ctx.r[11].s64 + -22272;
	// 8265F94C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8265F950: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F958: 4BE074C9  bl 0x82466e20
	ctx.lr = 0x8265F95C;
	sub_82466E20(ctx, base);
	// 8265F95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F970 size=108
    let mut pc: u32 = 0x8265F970;
    'dispatch: loop {
        match pc {
            0x8265F970 => {
    //   block [0x8265F970..0x8265F9DC)
	// 8265F970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F97C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F984: 38EB30A0  addi r7, r11, 0x30a0
	ctx.r[7].s64 = ctx.r[11].s64 + 12448;
	// 8265F988: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8265F98C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 8265F990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265F994: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265F998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265F99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265F9A0: 386AA930  addi r3, r10, -0x56d0
	ctx.r[3].s64 = ctx.r[10].s64 + -22224;
	// 8265F9A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265F9A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265F9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265F9B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265F9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265F9B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265F9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265F9C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265F9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265F9C8: 4BE07459  bl 0x82466e20
	ctx.lr = 0x8265F9CC;
	sub_82466E20(ctx, base);
	// 8265F9CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265F9D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265F9D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265F9D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265F9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265F9E0 size=108
    let mut pc: u32 = 0x8265F9E0;
    'dispatch: loop {
        match pc {
            0x8265F9E0 => {
    //   block [0x8265F9E0..0x8265FA4C)
	// 8265F9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265F9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265F9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265F9EC: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265F9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265F9F4: 38EB30E8  addi r7, r11, 0x30e8
	ctx.r[7].s64 = ctx.r[11].s64 + 12520;
	// 8265F9F8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265F9FC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 8265FA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FA04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FA0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FA10: 386AA960  addi r3, r10, -0x56a0
	ctx.r[3].s64 = ctx.r[10].s64 + -22176;
	// 8265FA14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FA20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FA24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FA28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FA2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FA30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FA34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FA38: 4BE073E9  bl 0x82466e20
	ctx.lr = 0x8265FA3C;
	sub_82466E20(ctx, base);
	// 8265FA3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FA40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FA44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FA48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8265FA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8265FA50 size=108
    let mut pc: u32 = 0x8265FA50;
    'dispatch: loop {
        match pc {
            0x8265FA50 => {
    //   block [0x8265FA50..0x8265FABC)
	// 8265FA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8265FA54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8265FA58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8265FA5C: 3D608279  lis r11, -0x7d87
	ctx.r[11].s64 = -2105999360;
	// 8265FA60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8265FA64: 38EB3178  addi r7, r11, 0x3178
	ctx.r[7].s64 = ctx.r[11].s64 + 12664;
	// 8265FA68: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8265FA6C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 8265FA70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8265FA74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8265FA78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8265FA7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8265FA80: 386AA990  addi r3, r10, -0x5670
	ctx.r[3].s64 = ctx.r[10].s64 + -22128;
	// 8265FA84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8265FA88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8265FA8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8265FA90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8265FA94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8265FA98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8265FA9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8265FAA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8265FAA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8265FAA8: 4BE07379  bl 0x82466e20
	ctx.lr = 0x8265FAAC;
	sub_82466E20(ctx, base);
	// 8265FAAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8265FAB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8265FAB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8265FAB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


