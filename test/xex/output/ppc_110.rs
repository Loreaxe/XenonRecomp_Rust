pub fn sub_826CD2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD2A8 size=112
    let mut pc: u32 = 0x826CD2A8;
    'dispatch: loop {
        match pc {
            0x826CD2A8 => {
    //   block [0x826CD2A8..0x826CD318)
	// 826CD2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD2B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD2B4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CD2B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD2BC: 392A3540  addi r9, r10, 0x3540
	ctx.r[9].s64 = ctx.r[10].s64 + 13632;
	// 826CD2C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD2C4: 390BF4B0  addi r8, r11, -0xb50
	ctx.r[8].s64 = ctx.r[11].s64 + -2896;
	// 826CD2C8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826CD2CC: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826CD2D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD2D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD2D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CD2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD2E0: 386A8C3C  addi r3, r10, -0x73c4
	ctx.r[3].s64 = ctx.r[10].s64 + -29636;
	// 826CD2E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CD2E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CD2EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD2F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD2F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD2F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD2FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD304: 4BD99B1D  bl 0x82466e20
	ctx.lr = 0x826CD308;
	sub_82466E20(ctx, base);
	// 826CD308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD30C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD318 size=108
    let mut pc: u32 = 0x826CD318;
    'dispatch: loop {
        match pc {
            0x826CD318 => {
    //   block [0x826CD318..0x826CD384)
	// 826CD318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD31C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD324: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD32C: 38EBB688  addi r7, r11, -0x4978
	ctx.r[7].s64 = ctx.r[11].s64 + -18808;
	// 826CD330: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CD334: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826CD338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD33C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD348: 386A8C6C  addi r3, r10, -0x7394
	ctx.r[3].s64 = ctx.r[10].s64 + -29588;
	// 826CD34C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD36C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD370: 4BD99AB1  bl 0x82466e20
	ctx.lr = 0x826CD374;
	sub_82466E20(ctx, base);
	// 826CD374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD37C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD388 size=112
    let mut pc: u32 = 0x826CD388;
    'dispatch: loop {
        match pc {
            0x826CD388 => {
    //   block [0x826CD388..0x826CD3F8)
	// 826CD388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD38C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD394: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CD398: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD39C: 392A3584  addi r9, r10, 0x3584
	ctx.r[9].s64 = ctx.r[10].s64 + 13700;
	// 826CD3A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD3A4: 390BB6B8  addi r8, r11, -0x4948
	ctx.r[8].s64 = ctx.r[11].s64 + -18760;
	// 826CD3A8: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826CD3AC: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826CD3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD3B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD3B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CD3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD3C0: 386A8C9C  addi r3, r10, -0x7364
	ctx.r[3].s64 = ctx.r[10].s64 + -29540;
	// 826CD3C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CD3C8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CD3CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD3D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD3D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD3DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD3E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD3E4: 4BD99A3D  bl 0x82466e20
	ctx.lr = 0x826CD3E8;
	sub_82466E20(ctx, base);
	// 826CD3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD3F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD3F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CD3F8 size=24
    let mut pc: u32 = 0x826CD3F8;
    'dispatch: loop {
        match pc {
            0x826CD3F8 => {
    //   block [0x826CD3F8..0x826CD410)
	// 826CD3F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD3FC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CD400: 394AF528  addi r10, r10, -0xad8
	ctx.r[10].s64 = ctx.r[10].s64 + -2776;
	// 826CD404: 816BB778  lwz r11, -0x4888(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18568 as u32) ) } as u64;
	// 826CD408: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826CD40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD410 size=112
    let mut pc: u32 = 0x826CD410;
    'dispatch: loop {
        match pc {
            0x826CD410 => {
    //   block [0x826CD410..0x826CD480)
	// 826CD410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD41C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CD420: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD424: 392A35C0  addi r9, r10, 0x35c0
	ctx.r[9].s64 = ctx.r[10].s64 + 13760;
	// 826CD428: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD42C: 390BF528  addi r8, r11, -0xad8
	ctx.r[8].s64 = ctx.r[11].s64 + -2776;
	// 826CD430: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826CD434: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826CD438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD43C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CD444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD448: 386A8CCC  addi r3, r10, -0x7334
	ctx.r[3].s64 = ctx.r[10].s64 + -29492;
	// 826CD44C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CD450: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CD454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD464: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD46C: 4BD999B5  bl 0x82466e20
	ctx.lr = 0x826CD470;
	sub_82466E20(ctx, base);
	// 826CD470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD480 size=108
    let mut pc: u32 = 0x826CD480;
    'dispatch: loop {
        match pc {
            0x826CD480 => {
    //   block [0x826CD480..0x826CD4EC)
	// 826CD480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD48C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD494: 38EBB77C  addi r7, r11, -0x4884
	ctx.r[7].s64 = ctx.r[11].s64 + -18564;
	// 826CD498: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CD49C: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826CD4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD4A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD4A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD4B0: 386A8CFC  addi r3, r10, -0x7304
	ctx.r[3].s64 = ctx.r[10].s64 + -29444;
	// 826CD4B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD4C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD4D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD4D8: 4BD99949  bl 0x82466e20
	ctx.lr = 0x826CD4DC;
	sub_82466E20(ctx, base);
	// 826CD4DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD4E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD4E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD4E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD4F0 size=108
    let mut pc: u32 = 0x826CD4F0;
    'dispatch: loop {
        match pc {
            0x826CD4F0 => {
    //   block [0x826CD4F0..0x826CD55C)
	// 826CD4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD4FC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD504: 38EBB794  addi r7, r11, -0x486c
	ctx.r[7].s64 = ctx.r[11].s64 + -18540;
	// 826CD508: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CD50C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826CD510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD514: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD518: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD51C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD520: 386A8D2C  addi r3, r10, -0x72d4
	ctx.r[3].s64 = ctx.r[10].s64 + -29396;
	// 826CD524: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD544: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD548: 4BD998D9  bl 0x82466e20
	ctx.lr = 0x826CD54C;
	sub_82466E20(ctx, base);
	// 826CD54C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD550: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD554: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD558: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CD560 size=24
    let mut pc: u32 = 0x826CD560;
    'dispatch: loop {
        match pc {
            0x826CD560 => {
    //   block [0x826CD560..0x826CD578)
	// 826CD560: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD564: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CD568: 394AF570  addi r10, r10, -0xa90
	ctx.r[10].s64 = ctx.r[10].s64 + -2704;
	// 826CD56C: 816BB7C4  lwz r11, -0x483c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-18492 as u32) ) } as u64;
	// 826CD570: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826CD574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD578 size=112
    let mut pc: u32 = 0x826CD578;
    'dispatch: loop {
        match pc {
            0x826CD578 => {
    //   block [0x826CD578..0x826CD5E8)
	// 826CD578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD584: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CD588: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD58C: 392A35FC  addi r9, r10, 0x35fc
	ctx.r[9].s64 = ctx.r[10].s64 + 13820;
	// 826CD590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD594: 390BF570  addi r8, r11, -0xa90
	ctx.r[8].s64 = ctx.r[11].s64 + -2704;
	// 826CD598: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826CD59C: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826CD5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD5A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD5A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CD5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD5B0: 386A8D5C  addi r3, r10, -0x72a4
	ctx.r[3].s64 = ctx.r[10].s64 + -29348;
	// 826CD5B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CD5B8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CD5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD5CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD5D4: 4BD9984D  bl 0x82466e20
	ctx.lr = 0x826CD5D8;
	sub_82466E20(ctx, base);
	// 826CD5D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD5DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD5E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD5E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD5E8 size=108
    let mut pc: u32 = 0x826CD5E8;
    'dispatch: loop {
        match pc {
            0x826CD5E8 => {
    //   block [0x826CD5E8..0x826CD654)
	// 826CD5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD5EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD5F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD5F4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD5F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD5FC: 38EBB7C8  addi r7, r11, -0x4838
	ctx.r[7].s64 = ctx.r[11].s64 + -18488;
	// 826CD600: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CD604: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826CD608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD60C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD618: 386A8D8C  addi r3, r10, -0x7274
	ctx.r[3].s64 = ctx.r[10].s64 + -29300;
	// 826CD61C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD62C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD63C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD640: 4BD997E1  bl 0x82466e20
	ctx.lr = 0x826CD644;
	sub_82466E20(ctx, base);
	// 826CD644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD64C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD658 size=108
    let mut pc: u32 = 0x826CD658;
    'dispatch: loop {
        match pc {
            0x826CD658 => {
    //   block [0x826CD658..0x826CD6C4)
	// 826CD658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD65C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD664: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD668: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD66C: 38EBB7E0  addi r7, r11, -0x4820
	ctx.r[7].s64 = ctx.r[11].s64 + -18464;
	// 826CD670: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CD674: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826CD678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD67C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD688: 386A8DBC  addi r3, r10, -0x7244
	ctx.r[3].s64 = ctx.r[10].s64 + -29252;
	// 826CD68C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD69C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD6A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD6A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD6A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD6AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD6B0: 4BD99771  bl 0x82466e20
	ctx.lr = 0x826CD6B4;
	sub_82466E20(ctx, base);
	// 826CD6B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD6B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD6BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD6C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD6C8 size=108
    let mut pc: u32 = 0x826CD6C8;
    'dispatch: loop {
        match pc {
            0x826CD6C8 => {
    //   block [0x826CD6C8..0x826CD734)
	// 826CD6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD6CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD6D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD6D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD6D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD6DC: 38EBB828  addi r7, r11, -0x47d8
	ctx.r[7].s64 = ctx.r[11].s64 + -18392;
	// 826CD6E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CD6E4: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826CD6E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD6EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD6F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD6F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD6F8: 386A8DEC  addi r3, r10, -0x7214
	ctx.r[3].s64 = ctx.r[10].s64 + -29204;
	// 826CD6FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD70C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD71C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD720: 4BD99701  bl 0x82466e20
	ctx.lr = 0x826CD724;
	sub_82466E20(ctx, base);
	// 826CD724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD72C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD738 size=108
    let mut pc: u32 = 0x826CD738;
    'dispatch: loop {
        match pc {
            0x826CD738 => {
    //   block [0x826CD738..0x826CD7A4)
	// 826CD738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD744: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD74C: 38EBB858  addi r7, r11, -0x47a8
	ctx.r[7].s64 = ctx.r[11].s64 + -18344;
	// 826CD750: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826CD754: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826CD758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD75C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD768: 386A8E1C  addi r3, r10, -0x71e4
	ctx.r[3].s64 = ctx.r[10].s64 + -29156;
	// 826CD76C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD78C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD790: 4BD99691  bl 0x82466e20
	ctx.lr = 0x826CD794;
	sub_82466E20(ctx, base);
	// 826CD794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD79C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD7A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD7A8 size=108
    let mut pc: u32 = 0x826CD7A8;
    'dispatch: loop {
        match pc {
            0x826CD7A8 => {
    //   block [0x826CD7A8..0x826CD814)
	// 826CD7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD7AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD7B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD7B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD7BC: 38EBB978  addi r7, r11, -0x4688
	ctx.r[7].s64 = ctx.r[11].s64 + -18056;
	// 826CD7C0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826CD7C4: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826CD7C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD7CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD7D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD7D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD7D8: 386A8E4C  addi r3, r10, -0x71b4
	ctx.r[3].s64 = ctx.r[10].s64 + -29108;
	// 826CD7DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD7E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD7E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD7F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD7F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD7F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD7FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD800: 4BD99621  bl 0x82466e20
	ctx.lr = 0x826CD804;
	sub_82466E20(ctx, base);
	// 826CD804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD818 size=108
    let mut pc: u32 = 0x826CD818;
    'dispatch: loop {
        match pc {
            0x826CD818 => {
    //   block [0x826CD818..0x826CD884)
	// 826CD818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD824: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD82C: 38EBBA08  addi r7, r11, -0x45f8
	ctx.r[7].s64 = ctx.r[11].s64 + -17912;
	// 826CD830: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826CD834: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826CD838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD83C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD848: 386A8E7C  addi r3, r10, -0x7184
	ctx.r[3].s64 = ctx.r[10].s64 + -29060;
	// 826CD84C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD86C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD870: 4BD995B1  bl 0x82466e20
	ctx.lr = 0x826CD874;
	sub_82466E20(ctx, base);
	// 826CD874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD888 size=108
    let mut pc: u32 = 0x826CD888;
    'dispatch: loop {
        match pc {
            0x826CD888 => {
    //   block [0x826CD888..0x826CD8F4)
	// 826CD888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD894: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD89C: 38EBBAC8  addi r7, r11, -0x4538
	ctx.r[7].s64 = ctx.r[11].s64 + -17720;
	// 826CD8A0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826CD8A4: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826CD8A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD8AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD8B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD8B8: 386A8EAC  addi r3, r10, -0x7154
	ctx.r[3].s64 = ctx.r[10].s64 + -29012;
	// 826CD8BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD8C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD8C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD8C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD8CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD8D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD8D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD8DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD8E0: 4BD99541  bl 0x82466e20
	ctx.lr = 0x826CD8E4;
	sub_82466E20(ctx, base);
	// 826CD8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD8E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD8EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD8F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD8F8 size=108
    let mut pc: u32 = 0x826CD8F8;
    'dispatch: loop {
        match pc {
            0x826CD8F8 => {
    //   block [0x826CD8F8..0x826CD964)
	// 826CD8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD8FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD904: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD90C: 38EBBBA0  addi r7, r11, -0x4460
	ctx.r[7].s64 = ctx.r[11].s64 + -17504;
	// 826CD910: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826CD914: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826CD918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD91C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD928: 386A8EDC  addi r3, r10, -0x7124
	ctx.r[3].s64 = ctx.r[10].s64 + -28964;
	// 826CD92C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD93C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD94C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD950: 4BD994D1  bl 0x82466e20
	ctx.lr = 0x826CD954;
	sub_82466E20(ctx, base);
	// 826CD954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD95C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD968 size=108
    let mut pc: u32 = 0x826CD968;
    'dispatch: loop {
        match pc {
            0x826CD968 => {
    //   block [0x826CD968..0x826CD9D4)
	// 826CD968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD96C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD974: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CD978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD97C: 38EBBC60  addi r7, r11, -0x43a0
	ctx.r[7].s64 = ctx.r[11].s64 + -17312;
	// 826CD980: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826CD984: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826CD988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CD98C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CD990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CD994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CD998: 386A8F0C  addi r3, r10, -0x70f4
	ctx.r[3].s64 = ctx.r[10].s64 + -28916;
	// 826CD99C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CD9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CD9A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CD9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CD9AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CD9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CD9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CD9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CD9BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CD9C0: 4BD99461  bl 0x82466e20
	ctx.lr = 0x826CD9C4;
	sub_82466E20(ctx, base);
	// 826CD9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CD9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CD9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CD9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CD9D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CD9D8 size=112
    let mut pc: u32 = 0x826CD9D8;
    'dispatch: loop {
        match pc {
            0x826CD9D8 => {
    //   block [0x826CD9D8..0x826CDA48)
	// 826CD9D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CD9DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CD9E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CD9E4: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CD9E8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826CD9EC: 38EABD08  addi r7, r10, -0x42f8
	ctx.r[7].s64 = ctx.r[10].s64 + -17144;
	// 826CD9F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CD9F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826CD9F8: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826CD9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDA00: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDA04: 396B3610  addi r11, r11, 0x3610
	ctx.r[11].s64 = ctx.r[11].s64 + 13840;
	// 826CDA08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDA0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDA10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDA14: 386A8F3C  addi r3, r10, -0x70c4
	ctx.r[3].s64 = ctx.r[10].s64 + -28868;
	// 826CDA18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDA1C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826CDA20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDA24: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826CDA28: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDA2C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDA30: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDA34: 4BD993ED  bl 0x82466e20
	ctx.lr = 0x826CDA38;
	sub_82466E20(ctx, base);
	// 826CDA38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDA3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDA40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDA44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDA48 size=108
    let mut pc: u32 = 0x826CDA48;
    'dispatch: loop {
        match pc {
            0x826CDA48 => {
    //   block [0x826CDA48..0x826CDAB4)
	// 826CDA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDA4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDA50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDA54: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDA58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDA5C: 38EBBE28  addi r7, r11, -0x41d8
	ctx.r[7].s64 = ctx.r[11].s64 + -16856;
	// 826CDA60: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CDA64: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826CDA68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDA6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDA70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDA78: 386A8F6C  addi r3, r10, -0x7094
	ctx.r[3].s64 = ctx.r[10].s64 + -28820;
	// 826CDA7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDA80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDA84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDA88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDA8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDA90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDA94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDA98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDA9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDAA0: 4BD99381  bl 0x82466e20
	ctx.lr = 0x826CDAA4;
	sub_82466E20(ctx, base);
	// 826CDAA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDAA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDAAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDAB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDAB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDAB8 size=108
    let mut pc: u32 = 0x826CDAB8;
    'dispatch: loop {
        match pc {
            0x826CDAB8 => {
    //   block [0x826CDAB8..0x826CDB24)
	// 826CDAB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDAC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDAC4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDAC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDACC: 38EBBE88  addi r7, r11, -0x4178
	ctx.r[7].s64 = ctx.r[11].s64 + -16760;
	// 826CDAD0: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826CDAD4: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826CDAD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDADC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDAE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDAE8: 386A8F9C  addi r3, r10, -0x7064
	ctx.r[3].s64 = ctx.r[10].s64 + -28772;
	// 826CDAEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDAF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDAF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDAFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDB00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDB08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDB0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDB10: 4BD99311  bl 0x82466e20
	ctx.lr = 0x826CDB14;
	sub_82466E20(ctx, base);
	// 826CDB14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDB18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDB1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDB20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDB28 size=108
    let mut pc: u32 = 0x826CDB28;
    'dispatch: loop {
        match pc {
            0x826CDB28 => {
    //   block [0x826CDB28..0x826CDB94)
	// 826CDB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDB34: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDB38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDB3C: 38EBBF90  addi r7, r11, -0x4070
	ctx.r[7].s64 = ctx.r[11].s64 + -16496;
	// 826CDB40: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826CDB44: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826CDB48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDB4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDB50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDB58: 386A8FCC  addi r3, r10, -0x7034
	ctx.r[3].s64 = ctx.r[10].s64 + -28724;
	// 826CDB5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDB68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDB6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDB70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDB78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDB7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDB80: 4BD992A1  bl 0x82466e20
	ctx.lr = 0x826CDB84;
	sub_82466E20(ctx, base);
	// 826CDB84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDB88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDB8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDB90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDB98 size=108
    let mut pc: u32 = 0x826CDB98;
    'dispatch: loop {
        match pc {
            0x826CDB98 => {
    //   block [0x826CDB98..0x826CDC04)
	// 826CDB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDBA4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDBA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDBAC: 38EBC068  addi r7, r11, -0x3f98
	ctx.r[7].s64 = ctx.r[11].s64 + -16280;
	// 826CDBB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CDBB4: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826CDBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDBBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDBC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDBC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDBC8: 386A8FFC  addi r3, r10, -0x7004
	ctx.r[3].s64 = ctx.r[10].s64 + -28676;
	// 826CDBCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDBD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDBD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDBDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDBE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDBE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDBE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDBEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDBF0: 4BD99231  bl 0x82466e20
	ctx.lr = 0x826CDBF4;
	sub_82466E20(ctx, base);
	// 826CDBF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDBF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDBFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDC08 size=108
    let mut pc: u32 = 0x826CDC08;
    'dispatch: loop {
        match pc {
            0x826CDC08 => {
    //   block [0x826CDC08..0x826CDC74)
	// 826CDC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDC14: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDC18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDC1C: 38EBC098  addi r7, r11, -0x3f68
	ctx.r[7].s64 = ctx.r[11].s64 + -16232;
	// 826CDC20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CDC24: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826CDC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDC2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDC30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDC38: 386A902C  addi r3, r10, -0x6fd4
	ctx.r[3].s64 = ctx.r[10].s64 + -28628;
	// 826CDC3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDC40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDC4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDC54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDC60: 4BD991C1  bl 0x82466e20
	ctx.lr = 0x826CDC64;
	sub_82466E20(ctx, base);
	// 826CDC64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDC78 size=108
    let mut pc: u32 = 0x826CDC78;
    'dispatch: loop {
        match pc {
            0x826CDC78 => {
    //   block [0x826CDC78..0x826CDCE4)
	// 826CDC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDC84: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDC88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDC8C: 38EBC0B0  addi r7, r11, -0x3f50
	ctx.r[7].s64 = ctx.r[11].s64 + -16208;
	// 826CDC90: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CDC94: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 826CDC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDC9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDCA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDCA8: 386A905C  addi r3, r10, -0x6fa4
	ctx.r[3].s64 = ctx.r[10].s64 + -28580;
	// 826CDCAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDCB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDCB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDCB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDCBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDCC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDCC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDCCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDCD0: 4BD99151  bl 0x82466e20
	ctx.lr = 0x826CDCD4;
	sub_82466E20(ctx, base);
	// 826CDCD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDCD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDCDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDCE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDCE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDCE8 size=108
    let mut pc: u32 = 0x826CDCE8;
    'dispatch: loop {
        match pc {
            0x826CDCE8 => {
    //   block [0x826CDCE8..0x826CDD54)
	// 826CDCE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDCEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDCF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDCF4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDCF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDCFC: 38EBC0F8  addi r7, r11, -0x3f08
	ctx.r[7].s64 = ctx.r[11].s64 + -16136;
	// 826CDD00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CDD04: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826CDD08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDD0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDD10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDD14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDD18: 386A908C  addi r3, r10, -0x6f74
	ctx.r[3].s64 = ctx.r[10].s64 + -28532;
	// 826CDD1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDD20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDD24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDD28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDD2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDD30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDD38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDD3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDD40: 4BD990E1  bl 0x82466e20
	ctx.lr = 0x826CDD44;
	sub_82466E20(ctx, base);
	// 826CDD44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDD48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDD4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDD58 size=112
    let mut pc: u32 = 0x826CDD58;
    'dispatch: loop {
        match pc {
            0x826CDD58 => {
    //   block [0x826CDD58..0x826CDDC8)
	// 826CDD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDD64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDD68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDD6C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CDD70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDD74: 390BC110  addi r8, r11, -0x3ef0
	ctx.r[8].s64 = ctx.r[11].s64 + -16112;
	// 826CDD78: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CDD7C: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826CDD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDD84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDD88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CDD8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDD90: 386A90BC  addi r3, r10, -0x6f44
	ctx.r[3].s64 = ctx.r[10].s64 + -28484;
	// 826CDD94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CDD98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDD9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDDA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDDA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDDA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDDAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDDB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDDB4: 4BD9906D  bl 0x82466e20
	ctx.lr = 0x826CDDB8;
	sub_82466E20(ctx, base);
	// 826CDDB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDDBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDDC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDDC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDDC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDDC8 size=108
    let mut pc: u32 = 0x826CDDC8;
    'dispatch: loop {
        match pc {
            0x826CDDC8 => {
    //   block [0x826CDDC8..0x826CDE34)
	// 826CDDC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDDCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDDD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDDD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDDD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDDDC: 38EBC158  addi r7, r11, -0x3ea8
	ctx.r[7].s64 = ctx.r[11].s64 + -16040;
	// 826CDDE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CDDE4: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826CDDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDDEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDDF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CDDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDDF8: 386A90EC  addi r3, r10, -0x6f14
	ctx.r[3].s64 = ctx.r[10].s64 + -28436;
	// 826CDDFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CDE00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDE1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDE20: 4BD99001  bl 0x82466e20
	ctx.lr = 0x826CDE24;
	sub_82466E20(ctx, base);
	// 826CDE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDE38 size=112
    let mut pc: u32 = 0x826CDE38;
    'dispatch: loop {
        match pc {
            0x826CDE38 => {
    //   block [0x826CDE38..0x826CDEA8)
	// 826CDE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDE44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDE48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDE4C: 38AA90EC  addi r5, r10, -0x6f14
	ctx.r[5].s64 = ctx.r[10].s64 + -28436;
	// 826CDE50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDE54: 390BC1B8  addi r8, r11, -0x3e48
	ctx.r[8].s64 = ctx.r[11].s64 + -15944;
	// 826CDE58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CDE5C: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826CDE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDE64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDE68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CDE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDE70: 386A911C  addi r3, r10, -0x6ee4
	ctx.r[3].s64 = ctx.r[10].s64 + -28388;
	// 826CDE74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CDE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDE94: 4BD98F8D  bl 0x82466e20
	ctx.lr = 0x826CDE98;
	sub_82466E20(ctx, base);
	// 826CDE98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDEA8 size=96
    let mut pc: u32 = 0x826CDEA8;
    'dispatch: loop {
        match pc {
            0x826CDEA8 => {
    //   block [0x826CDEA8..0x826CDF08)
	// 826CDEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDEB4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDEBC: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826CDEC0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDEC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDEC8: 386A914C  addi r3, r10, -0x6eb4
	ctx.r[3].s64 = ctx.r[10].s64 + -28340;
	// 826CDECC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDED4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CDED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDEE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDEE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CDEEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDEF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CDEF4: 4BD98F2D  bl 0x82466e20
	ctx.lr = 0x826CDEF8;
	sub_82466E20(ctx, base);
	// 826CDEF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDF04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDF08 size=112
    let mut pc: u32 = 0x826CDF08;
    'dispatch: loop {
        match pc {
            0x826CDF08 => {
    //   block [0x826CDF08..0x826CDF78)
	// 826CDF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDF0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDF10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDF14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDF18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CDF1C: 38AAA76C  addi r5, r10, -0x5894
	ctx.r[5].s64 = ctx.r[10].s64 + -22676;
	// 826CDF20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDF24: 390BC200  addi r8, r11, -0x3e00
	ctx.r[8].s64 = ctx.r[11].s64 + -15872;
	// 826CDF28: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CDF2C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826CDF30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDF34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDF38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CDF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDF40: 386A917C  addi r3, r10, -0x6e84
	ctx.r[3].s64 = ctx.r[10].s64 + -28292;
	// 826CDF44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CDF48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDF4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDF50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CDF58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDF5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDF60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDF64: 4BD98EBD  bl 0x82466e20
	ctx.lr = 0x826CDF68;
	sub_82466E20(ctx, base);
	// 826CDF68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDF6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDF70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDF78 size=96
    let mut pc: u32 = 0x826CDF78;
    'dispatch: loop {
        match pc {
            0x826CDF78 => {
    //   block [0x826CDF78..0x826CDFD8)
	// 826CDF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDF84: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDF8C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826CDF90: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDF98: 386A91AC  addi r3, r10, -0x6e54
	ctx.r[3].s64 = ctx.r[10].s64 + -28244;
	// 826CDF9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CDFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CDFA4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CDFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CDFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CDFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CDFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CDFB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CDFBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CDFC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CDFC4: 4BD98E5D  bl 0x82466e20
	ctx.lr = 0x826CDFC8;
	sub_82466E20(ctx, base);
	// 826CDFC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CDFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CDFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CDFD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CDFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CDFD8 size=100
    let mut pc: u32 = 0x826CDFD8;
    'dispatch: loop {
        match pc {
            0x826CDFD8 => {
    //   block [0x826CDFD8..0x826CE03C)
	// 826CDFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CDFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CDFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CDFE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CDFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CDFEC: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CDFF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CDFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CDFF8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826CDFFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE004: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CE008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE00C: 386A91DC  addi r3, r10, -0x6e24
	ctx.r[3].s64 = ctx.r[10].s64 + -28196;
	// 826CE010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE014: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE018: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE020: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE028: 4BD98DF9  bl 0x82466e20
	ctx.lr = 0x826CE02C;
	sub_82466E20(ctx, base);
	// 826CE02C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE040 size=96
    let mut pc: u32 = 0x826CE040;
    'dispatch: loop {
        match pc {
            0x826CE040 => {
    //   block [0x826CE040..0x826CE0A0)
	// 826CE040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE04C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE054: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826CE058: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE05C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE060: 386A920C  addi r3, r10, -0x6df4
	ctx.r[3].s64 = ctx.r[10].s64 + -28148;
	// 826CE064: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE06C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CE070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE07C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE080: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CE088: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE08C: 4BD98D95  bl 0x82466e20
	ctx.lr = 0x826CE090;
	sub_82466E20(ctx, base);
	// 826CE090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE09C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE0A0 size=112
    let mut pc: u32 = 0x826CE0A0;
    'dispatch: loop {
        match pc {
            0x826CE0A0 => {
    //   block [0x826CE0A0..0x826CE110)
	// 826CE0A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE0A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE0A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE0AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE0B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE0B4: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826CE0B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE0BC: 390BC260  addi r8, r11, -0x3da0
	ctx.r[8].s64 = ctx.r[11].s64 + -15776;
	// 826CE0C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE0C4: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826CE0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE0CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE0D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE0D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE0D8: 386A923C  addi r3, r10, -0x6dc4
	ctx.r[3].s64 = ctx.r[10].s64 + -28100;
	// 826CE0DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE0FC: 4BD98D25  bl 0x82466e20
	ctx.lr = 0x826CE100;
	sub_82466E20(ctx, base);
	// 826CE100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE10C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE110 size=112
    let mut pc: u32 = 0x826CE110;
    'dispatch: loop {
        match pc {
            0x826CE110 => {
    //   block [0x826CE110..0x826CE180)
	// 826CE110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE11C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE120: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE124: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826CE128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE12C: 390BC290  addi r8, r11, -0x3d70
	ctx.r[8].s64 = ctx.r[11].s64 + -15728;
	// 826CE130: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE134: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826CE138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE13C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE148: 386A926C  addi r3, r10, -0x6d94
	ctx.r[3].s64 = ctx.r[10].s64 + -28052;
	// 826CE14C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE16C: 4BD98CB5  bl 0x82466e20
	ctx.lr = 0x826CE170;
	sub_82466E20(ctx, base);
	// 826CE170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE180 size=100
    let mut pc: u32 = 0x826CE180;
    'dispatch: loop {
        match pc {
            0x826CE180 => {
    //   block [0x826CE180..0x826CE1E4)
	// 826CE180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE18C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE194: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826CE198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE19C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE1A0: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826CE1A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE1A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE1AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE1B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE1B4: 386A929C  addi r3, r10, -0x6d64
	ctx.r[3].s64 = ctx.r[10].s64 + -28004;
	// 826CE1B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE1BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE1C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE1C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE1CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE1D0: 4BD98C51  bl 0x82466e20
	ctx.lr = 0x826CE1D4;
	sub_82466E20(ctx, base);
	// 826CE1D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE1E8 size=96
    let mut pc: u32 = 0x826CE1E8;
    'dispatch: loop {
        match pc {
            0x826CE1E8 => {
    //   block [0x826CE1E8..0x826CE248)
	// 826CE1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE1F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE1F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE1FC: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826CE200: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE208: 386A92CC  addi r3, r10, -0x6d34
	ctx.r[3].s64 = ctx.r[10].s64 + -27956;
	// 826CE20C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE214: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CE218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE21C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE228: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE22C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CE230: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE234: 4BD98BED  bl 0x82466e20
	ctx.lr = 0x826CE238;
	sub_82466E20(ctx, base);
	// 826CE238: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE23C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE240: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE248 size=112
    let mut pc: u32 = 0x826CE248;
    'dispatch: loop {
        match pc {
            0x826CE248 => {
    //   block [0x826CE248..0x826CE2B8)
	// 826CE248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE258: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE25C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CE260: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE264: 390BC2A8  addi r8, r11, -0x3d58
	ctx.r[8].s64 = ctx.r[11].s64 + -15704;
	// 826CE268: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE26C: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826CE270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE274: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE278: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE27C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE280: 386A92FC  addi r3, r10, -0x6d04
	ctx.r[3].s64 = ctx.r[10].s64 + -27908;
	// 826CE284: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE288: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE2A4: 4BD98B7D  bl 0x82466e20
	ctx.lr = 0x826CE2A8;
	sub_82466E20(ctx, base);
	// 826CE2A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE2AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE2B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE2B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE2B8 size=108
    let mut pc: u32 = 0x826CE2B8;
    'dispatch: loop {
        match pc {
            0x826CE2B8 => {
    //   block [0x826CE2B8..0x826CE324)
	// 826CE2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE2C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE2C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE2CC: 38EBC2C0  addi r7, r11, -0x3d40
	ctx.r[7].s64 = ctx.r[11].s64 + -15680;
	// 826CE2D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CE2D4: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826CE2D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE2DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE2E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CE2E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE2E8: 386A932C  addi r3, r10, -0x6cd4
	ctx.r[3].s64 = ctx.r[10].s64 + -27860;
	// 826CE2EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CE2F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE2F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE2FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE30C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CE310: 4BD98B11  bl 0x82466e20
	ctx.lr = 0x826CE314;
	sub_82466E20(ctx, base);
	// 826CE314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE31C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE328 size=112
    let mut pc: u32 = 0x826CE328;
    'dispatch: loop {
        match pc {
            0x826CE328 => {
    //   block [0x826CE328..0x826CE398)
	// 826CE328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE338: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE33C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE344: 390BC320  addi r8, r11, -0x3ce0
	ctx.r[8].s64 = ctx.r[11].s64 + -15584;
	// 826CE348: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE34C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826CE350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE354: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE35C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE360: 386A935C  addi r3, r10, -0x6ca4
	ctx.r[3].s64 = ctx.r[10].s64 + -27812;
	// 826CE364: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE368: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE36C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE37C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE384: 4BD98A9D  bl 0x82466e20
	ctx.lr = 0x826CE388;
	sub_82466E20(ctx, base);
	// 826CE388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE38C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE398 size=112
    let mut pc: u32 = 0x826CE398;
    'dispatch: loop {
        match pc {
            0x826CE398 => {
    //   block [0x826CE398..0x826CE408)
	// 826CE398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE3A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE3A8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE3AC: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CE3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE3B4: 390BC338  addi r8, r11, -0x3cc8
	ctx.r[8].s64 = ctx.r[11].s64 + -15560;
	// 826CE3B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE3BC: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826CE3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE3C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE3C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE3D0: 386A938C  addi r3, r10, -0x6c74
	ctx.r[3].s64 = ctx.r[10].s64 + -27764;
	// 826CE3D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE3E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE3F4: 4BD98A2D  bl 0x82466e20
	ctx.lr = 0x826CE3F8;
	sub_82466E20(ctx, base);
	// 826CE3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE408 size=112
    let mut pc: u32 = 0x826CE408;
    'dispatch: loop {
        match pc {
            0x826CE408 => {
    //   block [0x826CE408..0x826CE478)
	// 826CE408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE418: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE41C: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CE420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE424: 390BC368  addi r8, r11, -0x3c98
	ctx.r[8].s64 = ctx.r[11].s64 + -15512;
	// 826CE428: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE42C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826CE430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE434: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE440: 386A93BC  addi r3, r10, -0x6c44
	ctx.r[3].s64 = ctx.r[10].s64 + -27716;
	// 826CE444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE464: 4BD989BD  bl 0x82466e20
	ctx.lr = 0x826CE468;
	sub_82466E20(ctx, base);
	// 826CE468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE478 size=112
    let mut pc: u32 = 0x826CE478;
    'dispatch: loop {
        match pc {
            0x826CE478 => {
    //   block [0x826CE478..0x826CE4E8)
	// 826CE478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE484: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE488: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE48C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE494: 390BC380  addi r8, r11, -0x3c80
	ctx.r[8].s64 = ctx.r[11].s64 + -15488;
	// 826CE498: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE49C: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826CE4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE4A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE4A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE4AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE4B0: 386A93EC  addi r3, r10, -0x6c14
	ctx.r[3].s64 = ctx.r[10].s64 + -27668;
	// 826CE4B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE4B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE4BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE4C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE4C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE4C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE4CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE4D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE4D4: 4BD9894D  bl 0x82466e20
	ctx.lr = 0x826CE4D8;
	sub_82466E20(ctx, base);
	// 826CE4D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE4E8 size=112
    let mut pc: u32 = 0x826CE4E8;
    'dispatch: loop {
        match pc {
            0x826CE4E8 => {
    //   block [0x826CE4E8..0x826CE558)
	// 826CE4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE4F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE4F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE4FC: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CE500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE504: 390BC3B0  addi r8, r11, -0x3c50
	ctx.r[8].s64 = ctx.r[11].s64 + -15440;
	// 826CE508: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE50C: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826CE510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE514: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE520: 386A941C  addi r3, r10, -0x6be4
	ctx.r[3].s64 = ctx.r[10].s64 + -27620;
	// 826CE524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE544: 4BD988DD  bl 0x82466e20
	ctx.lr = 0x826CE548;
	sub_82466E20(ctx, base);
	// 826CE548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE558 size=112
    let mut pc: u32 = 0x826CE558;
    'dispatch: loop {
        match pc {
            0x826CE558 => {
    //   block [0x826CE558..0x826CE5C8)
	// 826CE558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE568: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE56C: 38AA98FC  addi r5, r10, -0x6704
	ctx.r[5].s64 = ctx.r[10].s64 + -26372;
	// 826CE570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE574: 390BC3C8  addi r8, r11, -0x3c38
	ctx.r[8].s64 = ctx.r[11].s64 + -15416;
	// 826CE578: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE57C: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826CE580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE584: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE588: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE58C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE590: 386A944C  addi r3, r10, -0x6bb4
	ctx.r[3].s64 = ctx.r[10].s64 + -27572;
	// 826CE594: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE59C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE5A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE5B4: 4BD9886D  bl 0x82466e20
	ctx.lr = 0x826CE5B8;
	sub_82466E20(ctx, base);
	// 826CE5B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE5C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE5C8 size=112
    let mut pc: u32 = 0x826CE5C8;
    'dispatch: loop {
        match pc {
            0x826CE5C8 => {
    //   block [0x826CE5C8..0x826CE638)
	// 826CE5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE5D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE5D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE5D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE5DC: 38AA965C  addi r5, r10, -0x69a4
	ctx.r[5].s64 = ctx.r[10].s64 + -27044;
	// 826CE5E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE5E4: 390BC3E0  addi r8, r11, -0x3c20
	ctx.r[8].s64 = ctx.r[11].s64 + -15392;
	// 826CE5E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CE5EC: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826CE5F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE5F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE5F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE5FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE600: 386A947C  addi r3, r10, -0x6b84
	ctx.r[3].s64 = ctx.r[10].s64 + -27524;
	// 826CE604: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE608: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE60C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE610: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE614: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE618: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE61C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE620: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE624: 4BD987FD  bl 0x82466e20
	ctx.lr = 0x826CE628;
	sub_82466E20(ctx, base);
	// 826CE628: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE638 size=112
    let mut pc: u32 = 0x826CE638;
    'dispatch: loop {
        match pc {
            0x826CE638 => {
    //   block [0x826CE638..0x826CE6A8)
	// 826CE638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE644: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE648: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE64C: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CE650: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE654: 390BC3F8  addi r8, r11, -0x3c08
	ctx.r[8].s64 = ctx.r[11].s64 + -15368;
	// 826CE658: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CE65C: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826CE660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE668: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE66C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE670: 386A94AC  addi r3, r10, -0x6b54
	ctx.r[3].s64 = ctx.r[10].s64 + -27476;
	// 826CE674: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE678: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE67C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE680: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE688: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE68C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE690: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE694: 4BD9878D  bl 0x82466e20
	ctx.lr = 0x826CE698;
	sub_82466E20(ctx, base);
	// 826CE698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE6A8 size=112
    let mut pc: u32 = 0x826CE6A8;
    'dispatch: loop {
        match pc {
            0x826CE6A8 => {
    //   block [0x826CE6A8..0x826CE718)
	// 826CE6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE6AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE6B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE6B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE6BC: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE6C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE6C4: 390BC440  addi r8, r11, -0x3bc0
	ctx.r[8].s64 = ctx.r[11].s64 + -15296;
	// 826CE6C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE6CC: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826CE6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE6D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE6D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE6E0: 386A94DC  addi r3, r10, -0x6b24
	ctx.r[3].s64 = ctx.r[10].s64 + -27428;
	// 826CE6E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE6E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE6EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE6F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE6F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE6F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE6FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE700: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE704: 4BD9871D  bl 0x82466e20
	ctx.lr = 0x826CE708;
	sub_82466E20(ctx, base);
	// 826CE708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE718 size=112
    let mut pc: u32 = 0x826CE718;
    'dispatch: loop {
        match pc {
            0x826CE718 => {
    //   block [0x826CE718..0x826CE788)
	// 826CE718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE71C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE720: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE724: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE728: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE72C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE730: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE734: 390BC470  addi r8, r11, -0x3b90
	ctx.r[8].s64 = ctx.r[11].s64 + -15248;
	// 826CE738: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CE73C: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826CE740: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE748: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE74C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE750: 386A950C  addi r3, r10, -0x6af4
	ctx.r[3].s64 = ctx.r[10].s64 + -27380;
	// 826CE754: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE758: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE75C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE760: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE764: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE768: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE76C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE770: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE774: 4BD986AD  bl 0x82466e20
	ctx.lr = 0x826CE778;
	sub_82466E20(ctx, base);
	// 826CE778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE788 size=108
    let mut pc: u32 = 0x826CE788;
    'dispatch: loop {
        match pc {
            0x826CE788 => {
    //   block [0x826CE788..0x826CE7F4)
	// 826CE788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE794: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE798: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE79C: 38EBC4A0  addi r7, r11, -0x3b60
	ctx.r[7].s64 = ctx.r[11].s64 + -15200;
	// 826CE7A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CE7A4: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826CE7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE7AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE7B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CE7B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE7B8: 386A953C  addi r3, r10, -0x6ac4
	ctx.r[3].s64 = ctx.r[10].s64 + -27332;
	// 826CE7BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CE7C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE7C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE7DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CE7E0: 4BD98641  bl 0x82466e20
	ctx.lr = 0x826CE7E4;
	sub_82466E20(ctx, base);
	// 826CE7E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE7E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE7EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE7F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE7F8 size=112
    let mut pc: u32 = 0x826CE7F8;
    'dispatch: loop {
        match pc {
            0x826CE7F8 => {
    //   block [0x826CE7F8..0x826CE868)
	// 826CE7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE7FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE800: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE804: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE808: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE80C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE810: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE814: 390BC4E8  addi r8, r11, -0x3b18
	ctx.r[8].s64 = ctx.r[11].s64 + -15128;
	// 826CE818: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826CE81C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826CE820: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE824: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE828: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE830: 386A956C  addi r3, r10, -0x6a94
	ctx.r[3].s64 = ctx.r[10].s64 + -27284;
	// 826CE834: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CE838: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE83C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE840: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE844: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE848: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE84C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE850: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE854: 4BD985CD  bl 0x82466e20
	ctx.lr = 0x826CE858;
	sub_82466E20(ctx, base);
	// 826CE858: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE868 size=116
    let mut pc: u32 = 0x826CE868;
    'dispatch: loop {
        match pc {
            0x826CE868 => {
    //   block [0x826CE868..0x826CE8DC)
	// 826CE868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE86C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE874: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE878: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CE87C: 390BC568  addi r8, r11, -0x3a98
	ctx.r[8].s64 = ctx.r[11].s64 + -15000;
	// 826CE880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE884: 392A3698  addi r9, r10, 0x3698
	ctx.r[9].s64 = ctx.r[10].s64 + 13976;
	// 826CE888: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE88C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826CE890: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CE894: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CE898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE89C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE8A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE8AC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CE8B0: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826CE8B4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CE8B8: 386B959C  addi r3, r11, -0x6a64
	ctx.r[3].s64 = ctx.r[11].s64 + -27236;
	// 826CE8BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CE8C0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE8C8: 4BD98559  bl 0x82466e20
	ctx.lr = 0x826CE8CC;
	sub_82466E20(ctx, base);
	// 826CE8CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE8D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE8D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE8D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE8E0 size=100
    let mut pc: u32 = 0x826CE8E0;
    'dispatch: loop {
        match pc {
            0x826CE8E0 => {
    //   block [0x826CE8E0..0x826CE944)
	// 826CE8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE8EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE8F4: 38AA96EC  addi r5, r10, -0x6914
	ctx.r[5].s64 = ctx.r[10].s64 + -26900;
	// 826CE8F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE900: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826CE904: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE908: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE910: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE914: 386A95CC  addi r3, r10, -0x6a34
	ctx.r[3].s64 = ctx.r[10].s64 + -27188;
	// 826CE918: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE91C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE920: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE928: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE92C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE930: 4BD984F1  bl 0x82466e20
	ctx.lr = 0x826CE934;
	sub_82466E20(ctx, base);
	// 826CE934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE93C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE948 size=100
    let mut pc: u32 = 0x826CE948;
    'dispatch: loop {
        match pc {
            0x826CE948 => {
    //   block [0x826CE948..0x826CE9AC)
	// 826CE948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE954: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE95C: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CE960: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE968: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826CE96C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE97C: 386A95FC  addi r3, r10, -0x6a04
	ctx.r[3].s64 = ctx.r[10].s64 + -27140;
	// 826CE980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CE984: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE988: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CE98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE990: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CE994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CE998: 4BD98489  bl 0x82466e20
	ctx.lr = 0x826CE99C;
	sub_82466E20(ctx, base);
	// 826CE99C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CE9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CE9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CE9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CE9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CE9B0 size=108
    let mut pc: u32 = 0x826CE9B0;
    'dispatch: loop {
        match pc {
            0x826CE9B0 => {
    //   block [0x826CE9B0..0x826CEA1C)
	// 826CE9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CE9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CE9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CE9BC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CE9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CE9C4: 38EBC5E0  addi r7, r11, -0x3a20
	ctx.r[7].s64 = ctx.r[11].s64 + -14880;
	// 826CE9C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CE9CC: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826CE9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CE9D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CE9D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CE9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CE9E0: 386A962C  addi r3, r10, -0x69d4
	ctx.r[3].s64 = ctx.r[10].s64 + -27092;
	// 826CE9E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CE9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CE9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CE9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CE9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CE9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CE9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEA04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CEA08: 4BD98419  bl 0x82466e20
	ctx.lr = 0x826CEA0C;
	sub_82466E20(ctx, base);
	// 826CEA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEA20 size=112
    let mut pc: u32 = 0x826CEA20;
    'dispatch: loop {
        match pc {
            0x826CEA20 => {
    //   block [0x826CEA20..0x826CEA90)
	// 826CEA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEA2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEA30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEA34: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CEA38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEA3C: 390BC610  addi r8, r11, -0x39f0
	ctx.r[8].s64 = ctx.r[11].s64 + -14832;
	// 826CEA40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CEA44: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826CEA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEA4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEA50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEA54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEA58: 386A965C  addi r3, r10, -0x69a4
	ctx.r[3].s64 = ctx.r[10].s64 + -27044;
	// 826CEA5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEA7C: 4BD983A5  bl 0x82466e20
	ctx.lr = 0x826CEA80;
	sub_82466E20(ctx, base);
	// 826CEA80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEA84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEA88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEA90 size=108
    let mut pc: u32 = 0x826CEA90;
    'dispatch: loop {
        match pc {
            0x826CEA90 => {
    //   block [0x826CEA90..0x826CEAFC)
	// 826CEA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEA9C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEAA4: 38EBC628  addi r7, r11, -0x39d8
	ctx.r[7].s64 = ctx.r[11].s64 + -14808;
	// 826CEAA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CEAAC: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826CEAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEAB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEAB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CEABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEAC0: 386A968C  addi r3, r10, -0x6974
	ctx.r[3].s64 = ctx.r[10].s64 + -26996;
	// 826CEAC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CEAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEAE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CEAE8: 4BD98339  bl 0x82466e20
	ctx.lr = 0x826CEAEC;
	sub_82466E20(ctx, base);
	// 826CEAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CEB00 size=28
    let mut pc: u32 = 0x826CEB00;
    'dispatch: loop {
        match pc {
            0x826CEB00 => {
    //   block [0x826CEB00..0x826CEB1C)
	// 826CEB00: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEB04: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CEB08: 394AF5E8  addi r10, r10, -0xa18
	ctx.r[10].s64 = ctx.r[10].s64 + -2584;
	// 826CEB0C: 816BC564  lwz r11, -0x3a9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15004 as u32) ) } as u64;
	// 826CEB10: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826CEB14: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826CEB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEB20 size=108
    let mut pc: u32 = 0x826CEB20;
    'dispatch: loop {
        match pc {
            0x826CEB20 => {
    //   block [0x826CEB20..0x826CEB8C)
	// 826CEB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEB2C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEB30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEB34: 38EBF5E8  addi r7, r11, -0xa18
	ctx.r[7].s64 = ctx.r[11].s64 + -2584;
	// 826CEB38: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826CEB3C: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826CEB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEB44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEB48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CEB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEB50: 386A96BC  addi r3, r10, -0x6944
	ctx.r[3].s64 = ctx.r[10].s64 + -26948;
	// 826CEB54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CEB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CEB78: 4BD982A9  bl 0x82466e20
	ctx.lr = 0x826CEB7C;
	sub_82466E20(ctx, base);
	// 826CEB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEB90 size=116
    let mut pc: u32 = 0x826CEB90;
    'dispatch: loop {
        match pc {
            0x826CEB90 => {
    //   block [0x826CEB90..0x826CEC04)
	// 826CEB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEB9C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEBA0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CEBA4: 390BC648  addi r8, r11, -0x39b8
	ctx.r[8].s64 = ctx.r[11].s64 + -14776;
	// 826CEBA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEBAC: 392A36EC  addi r9, r10, 0x36ec
	ctx.r[9].s64 = ctx.r[10].s64 + 14060;
	// 826CEBB0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEBB4: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826CEBB8: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CEBBC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEBC4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEBD4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CEBD8: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826CEBDC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CEBE0: 386B96EC  addi r3, r11, -0x6914
	ctx.r[3].s64 = ctx.r[11].s64 + -26900;
	// 826CEBE4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826CEBE8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEBEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEBF0: 4BD98231  bl 0x82466e20
	ctx.lr = 0x826CEBF4;
	sub_82466E20(ctx, base);
	// 826CEBF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEBF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEBFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEC00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEC08 size=112
    let mut pc: u32 = 0x826CEC08;
    'dispatch: loop {
        match pc {
            0x826CEC08 => {
    //   block [0x826CEC08..0x826CEC78)
	// 826CEC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEC14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEC18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEC1C: 38AA93BC  addi r5, r10, -0x6c44
	ctx.r[5].s64 = ctx.r[10].s64 + -27716;
	// 826CEC20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEC24: 390BC6A8  addi r8, r11, -0x3958
	ctx.r[8].s64 = ctx.r[11].s64 + -14680;
	// 826CEC28: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CEC2C: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826CEC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEC34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEC38: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEC3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEC40: 386A971C  addi r3, r10, -0x68e4
	ctx.r[3].s64 = ctx.r[10].s64 + -26852;
	// 826CEC44: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEC48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEC4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEC50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEC58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEC5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEC60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEC64: 4BD981BD  bl 0x82466e20
	ctx.lr = 0x826CEC68;
	sub_82466E20(ctx, base);
	// 826CEC68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEC6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEC70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEC74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEC78 size=108
    let mut pc: u32 = 0x826CEC78;
    'dispatch: loop {
        match pc {
            0x826CEC78 => {
    //   block [0x826CEC78..0x826CECE4)
	// 826CEC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEC84: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEC88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEC8C: 38EBC6C0  addi r7, r11, -0x3940
	ctx.r[7].s64 = ctx.r[11].s64 + -14656;
	// 826CEC90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CEC94: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826CEC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEC9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CECA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CECA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CECA8: 386A974C  addi r3, r10, -0x68b4
	ctx.r[3].s64 = ctx.r[10].s64 + -26804;
	// 826CECAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CECB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CECB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CECB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CECBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CECC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CECC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CECC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CECCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CECD0: 4BD98151  bl 0x82466e20
	ctx.lr = 0x826CECD4;
	sub_82466E20(ctx, base);
	// 826CECD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CECD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CECDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CECE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CECE8 size=112
    let mut pc: u32 = 0x826CECE8;
    'dispatch: loop {
        match pc {
            0x826CECE8 => {
    //   block [0x826CECE8..0x826CED58)
	// 826CECE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CECEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CECF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CECF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CECF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CECFC: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CED00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CED04: 390BC6F0  addi r8, r11, -0x3910
	ctx.r[8].s64 = ctx.r[11].s64 + -14608;
	// 826CED08: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CED0C: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826CED10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CED14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CED18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CED1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CED20: 386A977C  addi r3, r10, -0x6884
	ctx.r[3].s64 = ctx.r[10].s64 + -26756;
	// 826CED24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CED28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CED2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CED30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CED34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CED38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CED3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CED40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CED44: 4BD980DD  bl 0x82466e20
	ctx.lr = 0x826CED48;
	sub_82466E20(ctx, base);
	// 826CED48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CED4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CED50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CED54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CED58 size=116
    let mut pc: u32 = 0x826CED58;
    'dispatch: loop {
        match pc {
            0x826CED58 => {
    //   block [0x826CED58..0x826CEDCC)
	// 826CED58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CED5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CED60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CED64: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CED68: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CED6C: 390BC720  addi r8, r11, -0x38e0
	ctx.r[8].s64 = ctx.r[11].s64 + -14560;
	// 826CED70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CED74: 392A371C  addi r9, r10, 0x371c
	ctx.r[9].s64 = ctx.r[10].s64 + 14108;
	// 826CED78: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CED7C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826CED80: 38AA98FC  addi r5, r10, -0x6704
	ctx.r[5].s64 = ctx.r[10].s64 + -26372;
	// 826CED84: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CED88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CED8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CED90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CED94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CED98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CED9C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CEDA0: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826CEDA4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CEDA8: 386B97AC  addi r3, r11, -0x6854
	ctx.r[3].s64 = ctx.r[11].s64 + -26708;
	// 826CEDAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CEDB0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEDB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEDB8: 4BD98069  bl 0x82466e20
	ctx.lr = 0x826CEDBC;
	sub_82466E20(ctx, base);
	// 826CEDBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEDC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEDC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEDC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEDD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEDD0 size=100
    let mut pc: u32 = 0x826CEDD0;
    'dispatch: loop {
        match pc {
            0x826CEDD0 => {
    //   block [0x826CEDD0..0x826CEE34)
	// 826CEDD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEDD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEDD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEDDC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEDE4: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CEDE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEDEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEDF0: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826CEDF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEE04: 386A97DC  addi r3, r10, -0x6824
	ctx.r[3].s64 = ctx.r[10].s64 + -26660;
	// 826CEE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEE0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEE10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CEE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEE18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CEE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEE20: 4BD98001  bl 0x82466e20
	ctx.lr = 0x826CEE24;
	sub_82466E20(ctx, base);
	// 826CEE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEE38 size=112
    let mut pc: u32 = 0x826CEE38;
    'dispatch: loop {
        match pc {
            0x826CEE38 => {
    //   block [0x826CEE38..0x826CEEA8)
	// 826CEE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEE44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEE48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEE4C: 38AA95FC  addi r5, r10, -0x6a04
	ctx.r[5].s64 = ctx.r[10].s64 + -27140;
	// 826CEE50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEE54: 390BC750  addi r8, r11, -0x38b0
	ctx.r[8].s64 = ctx.r[11].s64 + -14512;
	// 826CEE58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CEE5C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826CEE60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEE64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEE68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEE70: 386A980C  addi r3, r10, -0x67f4
	ctx.r[3].s64 = ctx.r[10].s64 + -26612;
	// 826CEE74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEE78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEE80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEE84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEE88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEE8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEE90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEE94: 4BD97F8D  bl 0x82466e20
	ctx.lr = 0x826CEE98;
	sub_82466E20(ctx, base);
	// 826CEE98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEE9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEEA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEEA8 size=112
    let mut pc: u32 = 0x826CEEA8;
    'dispatch: loop {
        match pc {
            0x826CEEA8 => {
    //   block [0x826CEEA8..0x826CEF18)
	// 826CEEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEEB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEEB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEEBC: 38AA95FC  addi r5, r10, -0x6a04
	ctx.r[5].s64 = ctx.r[10].s64 + -27140;
	// 826CEEC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEEC4: 390BC798  addi r8, r11, -0x3868
	ctx.r[8].s64 = ctx.r[11].s64 + -14440;
	// 826CEEC8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826CEECC: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826CEED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEED4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEED8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEEDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEEE0: 386A983C  addi r3, r10, -0x67c4
	ctx.r[3].s64 = ctx.r[10].s64 + -26564;
	// 826CEEE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEEE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEEF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEEF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEEF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEEFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEF00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEF04: 4BD97F1D  bl 0x82466e20
	ctx.lr = 0x826CEF08;
	sub_82466E20(ctx, base);
	// 826CEF08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEF0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEF10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEF14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEF18 size=108
    let mut pc: u32 = 0x826CEF18;
    'dispatch: loop {
        match pc {
            0x826CEF18 => {
    //   block [0x826CEF18..0x826CEF84)
	// 826CEF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEF1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEF20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEF24: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEF28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEF2C: 38EBC840  addi r7, r11, -0x37c0
	ctx.r[7].s64 = ctx.r[11].s64 + -14272;
	// 826CEF30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CEF34: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826CEF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEF3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEF40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CEF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEF48: 386A986C  addi r3, r10, -0x6794
	ctx.r[3].s64 = ctx.r[10].s64 + -26516;
	// 826CEF4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CEF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEF54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEF6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CEF70: 4BD97EB1  bl 0x82466e20
	ctx.lr = 0x826CEF74;
	sub_82466E20(ctx, base);
	// 826CEF74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEF80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEF88 size=112
    let mut pc: u32 = 0x826CEF88;
    'dispatch: loop {
        match pc {
            0x826CEF88 => {
    //   block [0x826CEF88..0x826CEFF8)
	// 826CEF88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEF8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CEF90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CEF94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEF98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CEF9C: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CEFA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CEFA4: 390BC888  addi r8, r11, -0x3778
	ctx.r[8].s64 = ctx.r[11].s64 + -14200;
	// 826CEFA8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CEFAC: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826CEFB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CEFB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CEFB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CEFBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CEFC0: 386A989C  addi r3, r10, -0x6764
	ctx.r[3].s64 = ctx.r[10].s64 + -26468;
	// 826CEFC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CEFC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CEFCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CEFD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CEFD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CEFD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CEFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CEFE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CEFE4: 4BD97E3D  bl 0x82466e20
	ctx.lr = 0x826CEFE8;
	sub_82466E20(ctx, base);
	// 826CEFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CEFEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CEFF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CEFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CEFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CEFF8 size=100
    let mut pc: u32 = 0x826CEFF8;
    'dispatch: loop {
        match pc {
            0x826CEFF8 => {
    //   block [0x826CEFF8..0x826CF05C)
	// 826CEFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CEFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF00C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CF010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF018: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826CF01C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF02C: 386A98CC  addi r3, r10, -0x6734
	ctx.r[3].s64 = ctx.r[10].s64 + -26420;
	// 826CF030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF034: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF038: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CF03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF040: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CF044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF048: 4BD97DD9  bl 0x82466e20
	ctx.lr = 0x826CF04C;
	sub_82466E20(ctx, base);
	// 826CF04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF060 size=100
    let mut pc: u32 = 0x826CF060;
    'dispatch: loop {
        match pc {
            0x826CF060 => {
    //   block [0x826CF060..0x826CF0C4)
	// 826CF060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF06C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF074: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CF078: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF080: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826CF084: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF094: 386A98FC  addi r3, r10, -0x6704
	ctx.r[3].s64 = ctx.r[10].s64 + -26372;
	// 826CF098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF0A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CF0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CF0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF0B0: 4BD97D71  bl 0x82466e20
	ctx.lr = 0x826CF0B4;
	sub_82466E20(ctx, base);
	// 826CF0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF0C8 size=108
    let mut pc: u32 = 0x826CF0C8;
    'dispatch: loop {
        match pc {
            0x826CF0C8 => {
    //   block [0x826CF0C8..0x826CF134)
	// 826CF0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF0D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF0D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF0DC: 38EBC8E8  addi r7, r11, -0x3718
	ctx.r[7].s64 = ctx.r[11].s64 + -14104;
	// 826CF0E0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826CF0E4: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826CF0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF0EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF0F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF0F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF0F8: 386A992C  addi r3, r10, -0x66d4
	ctx.r[3].s64 = ctx.r[10].s64 + -26324;
	// 826CF0FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF10C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF11C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF120: 4BD97D01  bl 0x82466e20
	ctx.lr = 0x826CF124;
	sub_82466E20(ctx, base);
	// 826CF124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF12C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF138 size=112
    let mut pc: u32 = 0x826CF138;
    'dispatch: loop {
        match pc {
            0x826CF138 => {
    //   block [0x826CF138..0x826CF1A8)
	// 826CF138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF144: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF148: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF14C: 38AA96EC  addi r5, r10, -0x6914
	ctx.r[5].s64 = ctx.r[10].s64 + -26900;
	// 826CF150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF154: 390BC978  addi r8, r11, -0x3688
	ctx.r[8].s64 = ctx.r[11].s64 + -13960;
	// 826CF158: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF15C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826CF160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF170: 386A995C  addi r3, r10, -0x66a4
	ctx.r[3].s64 = ctx.r[10].s64 + -26276;
	// 826CF174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF194: 4BD97C8D  bl 0x82466e20
	ctx.lr = 0x826CF198;
	sub_82466E20(ctx, base);
	// 826CF198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF1A8 size=112
    let mut pc: u32 = 0x826CF1A8;
    'dispatch: loop {
        match pc {
            0x826CF1A8 => {
    //   block [0x826CF1A8..0x826CF218)
	// 826CF1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF1B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF1B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF1BC: 38AA983C  addi r5, r10, -0x67c4
	ctx.r[5].s64 = ctx.r[10].s64 + -26564;
	// 826CF1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF1C4: 390BC990  addi r8, r11, -0x3670
	ctx.r[8].s64 = ctx.r[11].s64 + -13936;
	// 826CF1C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CF1CC: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826CF1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF1D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF1E0: 386A998C  addi r3, r10, -0x6674
	ctx.r[3].s64 = ctx.r[10].s64 + -26228;
	// 826CF1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF204: 4BD97C1D  bl 0x82466e20
	ctx.lr = 0x826CF208;
	sub_82466E20(ctx, base);
	// 826CF208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF218 size=112
    let mut pc: u32 = 0x826CF218;
    'dispatch: loop {
        match pc {
            0x826CF218 => {
    //   block [0x826CF218..0x826CF288)
	// 826CF218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF224: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF228: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF22C: 38AA92FC  addi r5, r10, -0x6d04
	ctx.r[5].s64 = ctx.r[10].s64 + -27908;
	// 826CF230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF234: 390BC9C0  addi r8, r11, -0x3640
	ctx.r[8].s64 = ctx.r[11].s64 + -13888;
	// 826CF238: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CF23C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826CF240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF250: 386A99BC  addi r3, r10, -0x6644
	ctx.r[3].s64 = ctx.r[10].s64 + -26180;
	// 826CF254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF274: 4BD97BAD  bl 0x82466e20
	ctx.lr = 0x826CF278;
	sub_82466E20(ctx, base);
	// 826CF278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF288 size=112
    let mut pc: u32 = 0x826CF288;
    'dispatch: loop {
        match pc {
            0x826CF288 => {
    //   block [0x826CF288..0x826CF2F8)
	// 826CF288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF298: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF29C: 38AA944C  addi r5, r10, -0x6bb4
	ctx.r[5].s64 = ctx.r[10].s64 + -27572;
	// 826CF2A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF2A4: 390BCA08  addi r8, r11, -0x35f8
	ctx.r[8].s64 = ctx.r[11].s64 + -13816;
	// 826CF2A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CF2AC: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826CF2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF2B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF2C0: 386A99EC  addi r3, r10, -0x6614
	ctx.r[3].s64 = ctx.r[10].s64 + -26132;
	// 826CF2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF2E4: 4BD97B3D  bl 0x82466e20
	ctx.lr = 0x826CF2E8;
	sub_82466E20(ctx, base);
	// 826CF2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF2F8 size=108
    let mut pc: u32 = 0x826CF2F8;
    'dispatch: loop {
        match pc {
            0x826CF2F8 => {
    //   block [0x826CF2F8..0x826CF364)
	// 826CF2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF304: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF308: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CF30C: 38EBCA50  addi r7, r11, -0x35b0
	ctx.r[7].s64 = ctx.r[11].s64 + -13744;
	// 826CF310: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CF314: 388A21D8  addi r4, r10, 0x21d8
	ctx.r[4].s64 = ctx.r[10].s64 + 8664;
	// 826CF318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF31C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF328: 386A9A1C  addi r3, r10, -0x65e4
	ctx.r[3].s64 = ctx.r[10].s64 + -26084;
	// 826CF32C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF350: 4BD97AD1  bl 0x82466e20
	ctx.lr = 0x826CF354;
	sub_82466E20(ctx, base);
	// 826CF354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF368 size=112
    let mut pc: u32 = 0x826CF368;
    'dispatch: loop {
        match pc {
            0x826CF368 => {
    //   block [0x826CF368..0x826CF3D8)
	// 826CF368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF374: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF378: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF37C: 38AA93BC  addi r5, r10, -0x6c44
	ctx.r[5].s64 = ctx.r[10].s64 + -27716;
	// 826CF380: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF384: 390BCA98  addi r8, r11, -0x3568
	ctx.r[8].s64 = ctx.r[11].s64 + -13672;
	// 826CF388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF38C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826CF390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF394: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF39C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF3A0: 386A9A4C  addi r3, r10, -0x65b4
	ctx.r[3].s64 = ctx.r[10].s64 + -26036;
	// 826CF3A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF3A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF3AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF3B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF3B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF3B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF3C4: 4BD97A5D  bl 0x82466e20
	ctx.lr = 0x826CF3C8;
	sub_82466E20(ctx, base);
	// 826CF3C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF3CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF3D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF3D8 size=112
    let mut pc: u32 = 0x826CF3D8;
    'dispatch: loop {
        match pc {
            0x826CF3D8 => {
    //   block [0x826CF3D8..0x826CF448)
	// 826CF3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF3E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF3E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF3EC: 38AA941C  addi r5, r10, -0x6be4
	ctx.r[5].s64 = ctx.r[10].s64 + -27620;
	// 826CF3F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF3F4: 390BCAB0  addi r8, r11, -0x3550
	ctx.r[8].s64 = ctx.r[11].s64 + -13648;
	// 826CF3F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CF3FC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826CF400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF404: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF410: 386A9A7C  addi r3, r10, -0x6584
	ctx.r[3].s64 = ctx.r[10].s64 + -25988;
	// 826CF414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF41C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF434: 4BD979ED  bl 0x82466e20
	ctx.lr = 0x826CF438;
	sub_82466E20(ctx, base);
	// 826CF438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF448 size=108
    let mut pc: u32 = 0x826CF448;
    'dispatch: loop {
        match pc {
            0x826CF448 => {
    //   block [0x826CF448..0x826CF4B4)
	// 826CF448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF454: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF45C: 38EBCAE0  addi r7, r11, -0x3520
	ctx.r[7].s64 = ctx.r[11].s64 + -13600;
	// 826CF460: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826CF464: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826CF468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF46C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF478: 386A9AAC  addi r3, r10, -0x6554
	ctx.r[3].s64 = ctx.r[10].s64 + -25940;
	// 826CF47C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF4A0: 4BD97981  bl 0x82466e20
	ctx.lr = 0x826CF4A4;
	sub_82466E20(ctx, base);
	// 826CF4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF4B8 size=108
    let mut pc: u32 = 0x826CF4B8;
    'dispatch: loop {
        match pc {
            0x826CF4B8 => {
    //   block [0x826CF4B8..0x826CF524)
	// 826CF4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF4C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF4C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF4CC: 38EBCBD0  addi r7, r11, -0x3430
	ctx.r[7].s64 = ctx.r[11].s64 + -13360;
	// 826CF4D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CF4D4: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826CF4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF4DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF4E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF4E8: 386A9ADC  addi r3, r10, -0x6524
	ctx.r[3].s64 = ctx.r[10].s64 + -25892;
	// 826CF4EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF50C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF510: 4BD97911  bl 0x82466e20
	ctx.lr = 0x826CF514;
	sub_82466E20(ctx, base);
	// 826CF514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF528 size=108
    let mut pc: u32 = 0x826CF528;
    'dispatch: loop {
        match pc {
            0x826CF528 => {
    //   block [0x826CF528..0x826CF594)
	// 826CF528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF534: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF538: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF53C: 38EBCC18  addi r7, r11, -0x33e8
	ctx.r[7].s64 = ctx.r[11].s64 + -13288;
	// 826CF540: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826CF544: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826CF548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF54C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF550: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF554: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF558: 386A9B0C  addi r3, r10, -0x64f4
	ctx.r[3].s64 = ctx.r[10].s64 + -25844;
	// 826CF55C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF560: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF568: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF56C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF570: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF574: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF578: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF57C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF580: 4BD978A1  bl 0x82466e20
	ctx.lr = 0x826CF584;
	sub_82466E20(ctx, base);
	// 826CF584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF598 size=108
    let mut pc: u32 = 0x826CF598;
    'dispatch: loop {
        match pc {
            0x826CF598 => {
    //   block [0x826CF598..0x826CF604)
	// 826CF598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF5A4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF5A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF5AC: 38EBCCF0  addi r7, r11, -0x3310
	ctx.r[7].s64 = ctx.r[11].s64 + -13072;
	// 826CF5B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CF5B4: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826CF5B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF5BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF5C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF5C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF5C8: 386A9B3C  addi r3, r10, -0x64c4
	ctx.r[3].s64 = ctx.r[10].s64 + -25796;
	// 826CF5CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF5D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF5D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF5D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF5E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF5E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF5EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF5F0: 4BD97831  bl 0x82466e20
	ctx.lr = 0x826CF5F4;
	sub_82466E20(ctx, base);
	// 826CF5F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF5F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF5FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF608 size=112
    let mut pc: u32 = 0x826CF608;
    'dispatch: loop {
        match pc {
            0x826CF608 => {
    //   block [0x826CF608..0x826CF678)
	// 826CF608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF614: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF618: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF61C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CF620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF624: 390BCD08  addi r8, r11, -0x32f8
	ctx.r[8].s64 = ctx.r[11].s64 + -13048;
	// 826CF628: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CF62C: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826CF630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF634: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF63C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF640: 386A9B6C  addi r3, r10, -0x6494
	ctx.r[3].s64 = ctx.r[10].s64 + -25748;
	// 826CF644: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF654: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF664: 4BD977BD  bl 0x82466e20
	ctx.lr = 0x826CF668;
	sub_82466E20(ctx, base);
	// 826CF668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF678 size=112
    let mut pc: u32 = 0x826CF678;
    'dispatch: loop {
        match pc {
            0x826CF678 => {
    //   block [0x826CF678..0x826CF6E8)
	// 826CF678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF684: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF688: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF68C: 38AA9B6C  addi r5, r10, -0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + -25748;
	// 826CF690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF694: 390BCD68  addi r8, r11, -0x3298
	ctx.r[8].s64 = ctx.r[11].s64 + -12952;
	// 826CF698: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF69C: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 826CF6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF6A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF6A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF6B0: 386A9B9C  addi r3, r10, -0x6464
	ctx.r[3].s64 = ctx.r[10].s64 + -25700;
	// 826CF6B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF6D4: 4BD9774D  bl 0x82466e20
	ctx.lr = 0x826CF6D8;
	sub_82466E20(ctx, base);
	// 826CF6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF6E8 size=112
    let mut pc: u32 = 0x826CF6E8;
    'dispatch: loop {
        match pc {
            0x826CF6E8 => {
    //   block [0x826CF6E8..0x826CF758)
	// 826CF6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF6F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF6F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF6FC: 38AA9B6C  addi r5, r10, -0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + -25748;
	// 826CF700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF704: 390BCD80  addi r8, r11, -0x3280
	ctx.r[8].s64 = ctx.r[11].s64 + -12928;
	// 826CF708: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CF70C: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 826CF710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF71C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF720: 386A9BCC  addi r3, r10, -0x6434
	ctx.r[3].s64 = ctx.r[10].s64 + -25652;
	// 826CF724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF72C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF73C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF744: 4BD976DD  bl 0x82466e20
	ctx.lr = 0x826CF748;
	sub_82466E20(ctx, base);
	// 826CF748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF758 size=112
    let mut pc: u32 = 0x826CF758;
    'dispatch: loop {
        match pc {
            0x826CF758 => {
    //   block [0x826CF758..0x826CF7C8)
	// 826CF758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF768: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF76C: 38AA9B6C  addi r5, r10, -0x6494
	ctx.r[5].s64 = ctx.r[10].s64 + -25748;
	// 826CF770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF774: 390BCDB0  addi r8, r11, -0x3250
	ctx.r[8].s64 = ctx.r[11].s64 + -12880;
	// 826CF778: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF77C: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 826CF780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF784: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF790: 386A9BFC  addi r3, r10, -0x6404
	ctx.r[3].s64 = ctx.r[10].s64 + -25604;
	// 826CF794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF7B4: 4BD9766D  bl 0x82466e20
	ctx.lr = 0x826CF7B8;
	sub_82466E20(ctx, base);
	// 826CF7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CF7C8 size=24
    let mut pc: u32 = 0x826CF7C8;
    'dispatch: loop {
        match pc {
            0x826CF7C8 => {
    //   block [0x826CF7C8..0x826CF7E0)
	// 826CF7C8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF7CC: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CF7D0: 394AF720  addi r10, r10, -0x8e0
	ctx.r[10].s64 = ctx.r[10].s64 + -2272;
	// 826CF7D4: 816BCDC8  lwz r11, -0x3238(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12856 as u32) ) } as u64;
	// 826CF7D8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826CF7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF7E0 size=112
    let mut pc: u32 = 0x826CF7E0;
    'dispatch: loop {
        match pc {
            0x826CF7E0 => {
    //   block [0x826CF7E0..0x826CF850)
	// 826CF7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF7EC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CF7F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF7F4: 392A3758  addi r9, r10, 0x3758
	ctx.r[9].s64 = ctx.r[10].s64 + 14168;
	// 826CF7F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF7FC: 390BF720  addi r8, r11, -0x8e0
	ctx.r[8].s64 = ctx.r[11].s64 + -2272;
	// 826CF800: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826CF804: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 826CF808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF80C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF814: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF818: 386A9C2C  addi r3, r10, -0x63d4
	ctx.r[3].s64 = ctx.r[10].s64 + -25556;
	// 826CF81C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CF820: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CF824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF82C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF83C: 4BD975E5  bl 0x82466e20
	ctx.lr = 0x826CF840;
	sub_82466E20(ctx, base);
	// 826CF840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF850 size=108
    let mut pc: u32 = 0x826CF850;
    'dispatch: loop {
        match pc {
            0x826CF850 => {
    //   block [0x826CF850..0x826CF8BC)
	// 826CF850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF85C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF864: 38EBCDCC  addi r7, r11, -0x3234
	ctx.r[7].s64 = ctx.r[11].s64 + -12852;
	// 826CF868: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826CF86C: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 826CF870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF874: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF880: 386A9C5C  addi r3, r10, -0x63a4
	ctx.r[3].s64 = ctx.r[10].s64 + -25508;
	// 826CF884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF8A8: 4BD97579  bl 0x82466e20
	ctx.lr = 0x826CF8AC;
	sub_82466E20(ctx, base);
	// 826CF8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF8C0 size=108
    let mut pc: u32 = 0x826CF8C0;
    'dispatch: loop {
        match pc {
            0x826CF8C0 => {
    //   block [0x826CF8C0..0x826CF92C)
	// 826CF8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF8CC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF8D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF8D4: 38EBCDE8  addi r7, r11, -0x3218
	ctx.r[7].s64 = ctx.r[11].s64 + -12824;
	// 826CF8D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CF8DC: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 826CF8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF8E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF8F0: 386A9C8C  addi r3, r10, -0x6374
	ctx.r[3].s64 = ctx.r[10].s64 + -25460;
	// 826CF8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF918: 4BD97509  bl 0x82466e20
	ctx.lr = 0x826CF91C;
	sub_82466E20(ctx, base);
	// 826CF91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF930 size=112
    let mut pc: u32 = 0x826CF930;
    'dispatch: loop {
        match pc {
            0x826CF930 => {
    //   block [0x826CF930..0x826CF9A0)
	// 826CF930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF93C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF940: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF944: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CF948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CF94C: 390BCE30  addi r8, r11, -0x31d0
	ctx.r[8].s64 = ctx.r[11].s64 + -12752;
	// 826CF950: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826CF954: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 826CF958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF95C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF960: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CF964: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF968: 386A9CBC  addi r3, r10, -0x6344
	ctx.r[3].s64 = ctx.r[10].s64 + -25412;
	// 826CF96C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CF970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF98C: 4BD97495  bl 0x82466e20
	ctx.lr = 0x826CF990;
	sub_82466E20(ctx, base);
	// 826CF990: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CF994: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CF998: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CF99C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CF9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CF9A0 size=108
    let mut pc: u32 = 0x826CF9A0;
    'dispatch: loop {
        match pc {
            0x826CF9A0 => {
    //   block [0x826CF9A0..0x826CFA0C)
	// 826CF9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CF9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CF9A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CF9AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CF9B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CF9B4: 38EBCE48  addi r7, r11, -0x31b8
	ctx.r[7].s64 = ctx.r[11].s64 + -12728;
	// 826CF9B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CF9BC: 388A21EC  addi r4, r10, 0x21ec
	ctx.r[4].s64 = ctx.r[10].s64 + 8684;
	// 826CF9C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CF9C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CF9C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CF9CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CF9D0: 386A9CEC  addi r3, r10, -0x6314
	ctx.r[3].s64 = ctx.r[10].s64 + -25364;
	// 826CF9D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CF9D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CF9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CF9E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CF9E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CF9E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CF9EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CF9F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CF9F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CF9F8: 4BD97429  bl 0x82466e20
	ctx.lr = 0x826CF9FC;
	sub_82466E20(ctx, base);
	// 826CF9FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFA00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFA04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFA08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CFA10 size=24
    let mut pc: u32 = 0x826CFA10;
    'dispatch: loop {
        match pc {
            0x826CFA10 => {
    //   block [0x826CFA10..0x826CFA28)
	// 826CFA10: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFA14: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CFA18: 394AF768  addi r10, r10, -0x898
	ctx.r[10].s64 = ctx.r[10].s64 + -2200;
	// 826CFA1C: 816BCEA8  lwz r11, -0x3158(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12632 as u32) ) } as u64;
	// 826CFA20: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826CFA24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFA28 size=116
    let mut pc: u32 = 0x826CFA28;
    'dispatch: loop {
        match pc {
            0x826CFA28 => {
    //   block [0x826CFA28..0x826CFA9C)
	// 826CFA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFA34: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFA38: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CFA3C: 390BF768  addi r8, r11, -0x898
	ctx.r[8].s64 = ctx.r[11].s64 + -2200;
	// 826CFA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFA44: 392A3820  addi r9, r10, 0x3820
	ctx.r[9].s64 = ctx.r[10].s64 + 14368;
	// 826CFA48: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFA4C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826CFA50: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CFA54: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFA58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFA5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFA60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFA68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFA6C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CFA70: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 826CFA74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CFA78: 386B9D1C  addi r3, r11, -0x62e4
	ctx.r[3].s64 = ctx.r[11].s64 + -25316;
	// 826CFA7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CFA80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFA88: 4BD97399  bl 0x82466e20
	ctx.lr = 0x826CFA8C;
	sub_82466E20(ctx, base);
	// 826CFA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFAA0 size=112
    let mut pc: u32 = 0x826CFAA0;
    'dispatch: loop {
        match pc {
            0x826CFAA0 => {
    //   block [0x826CFAA0..0x826CFB10)
	// 826CFAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFAAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFAB0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFAB4: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFAB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFABC: 390BCEAC  addi r8, r11, -0x3154
	ctx.r[8].s64 = ctx.r[11].s64 + -12628;
	// 826CFAC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CFAC4: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 826CFAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFACC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFAD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFAD8: 386A9D4C  addi r3, r10, -0x62b4
	ctx.r[3].s64 = ctx.r[10].s64 + -25268;
	// 826CFADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFAFC: 4BD97325  bl 0x82466e20
	ctx.lr = 0x826CFB00;
	sub_82466E20(ctx, base);
	// 826CFB00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFB10 size=108
    let mut pc: u32 = 0x826CFB10;
    'dispatch: loop {
        match pc {
            0x826CFB10 => {
    //   block [0x826CFB10..0x826CFB7C)
	// 826CFB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFB1C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFB20: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFB24: 38EBCEE0  addi r7, r11, -0x3120
	ctx.r[7].s64 = ctx.r[11].s64 + -12576;
	// 826CFB28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CFB2C: 388A2224  addi r4, r10, 0x2224
	ctx.r[4].s64 = ctx.r[10].s64 + 8740;
	// 826CFB30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFB34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFB38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CFB3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFB40: 386A9D7C  addi r3, r10, -0x6284
	ctx.r[3].s64 = ctx.r[10].s64 + -25220;
	// 826CFB44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CFB48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFB4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFB50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFB54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFB58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFB5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFB60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFB64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CFB68: 4BD972B9  bl 0x82466e20
	ctx.lr = 0x826CFB6C;
	sub_82466E20(ctx, base);
	// 826CFB6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFB70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFB74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFB78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFB80 size=108
    let mut pc: u32 = 0x826CFB80;
    'dispatch: loop {
        match pc {
            0x826CFB80 => {
    //   block [0x826CFB80..0x826CFBEC)
	// 826CFB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFB88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFB8C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFB90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFB94: 38EBCF28  addi r7, r11, -0x30d8
	ctx.r[7].s64 = ctx.r[11].s64 + -12504;
	// 826CFB98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826CFB9C: 388A224C  addi r4, r10, 0x224c
	ctx.r[4].s64 = ctx.r[10].s64 + 8780;
	// 826CFBA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFBA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFBA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CFBAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFBB0: 386A9DAC  addi r3, r10, -0x6254
	ctx.r[3].s64 = ctx.r[10].s64 + -25172;
	// 826CFBB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CFBB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFBBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFBC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFBC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFBC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFBCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFBD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFBD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CFBD8: 4BD97249  bl 0x82466e20
	ctx.lr = 0x826CFBDC;
	sub_82466E20(ctx, base);
	// 826CFBDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFBE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFBE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFBE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFBF0 size=112
    let mut pc: u32 = 0x826CFBF0;
    'dispatch: loop {
        match pc {
            0x826CFBF0 => {
    //   block [0x826CFBF0..0x826CFC60)
	// 826CFBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFBF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFBFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFC00: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFC04: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFC08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFC0C: 390BCF58  addi r8, r11, -0x30a8
	ctx.r[8].s64 = ctx.r[11].s64 + -12456;
	// 826CFC10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826CFC14: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 826CFC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFC1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFC28: 386A9DDC  addi r3, r10, -0x6224
	ctx.r[3].s64 = ctx.r[10].s64 + -25124;
	// 826CFC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFC4C: 4BD971D5  bl 0x82466e20
	ctx.lr = 0x826CFC50;
	sub_82466E20(ctx, base);
	// 826CFC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFC60 size=108
    let mut pc: u32 = 0x826CFC60;
    'dispatch: loop {
        match pc {
            0x826CFC60 => {
    //   block [0x826CFC60..0x826CFCCC)
	// 826CFC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFC6C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFC70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFC74: 38EBCF88  addi r7, r11, -0x3078
	ctx.r[7].s64 = ctx.r[11].s64 + -12408;
	// 826CFC78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826CFC7C: 388A2274  addi r4, r10, 0x2274
	ctx.r[4].s64 = ctx.r[10].s64 + 8820;
	// 826CFC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFC84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFC88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CFC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFC90: 386A9E0C  addi r3, r10, -0x61f4
	ctx.r[3].s64 = ctx.r[10].s64 + -25076;
	// 826CFC94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CFC98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFC9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFCA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFCA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFCA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFCAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFCB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFCB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CFCB8: 4BD97169  bl 0x82466e20
	ctx.lr = 0x826CFCBC;
	sub_82466E20(ctx, base);
	// 826CFCBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFCC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFCC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFCC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFCD0 size=108
    let mut pc: u32 = 0x826CFCD0;
    'dispatch: loop {
        match pc {
            0x826CFCD0 => {
    //   block [0x826CFCD0..0x826CFD3C)
	// 826CFCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFCD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFCDC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFCE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFCE4: 38EBCFE8  addi r7, r11, -0x3018
	ctx.r[7].s64 = ctx.r[11].s64 + -12312;
	// 826CFCE8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826CFCEC: 388A22A4  addi r4, r10, 0x22a4
	ctx.r[4].s64 = ctx.r[10].s64 + 8868;
	// 826CFCF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFCF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFCF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826CFCFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFD00: 386A9E3C  addi r3, r10, -0x61c4
	ctx.r[3].s64 = ctx.r[10].s64 + -25028;
	// 826CFD04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826CFD08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFD10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFD14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFD18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFD1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFD20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFD24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826CFD28: 4BD970F9  bl 0x82466e20
	ctx.lr = 0x826CFD2C;
	sub_82466E20(ctx, base);
	// 826CFD2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFD30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFD34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFD38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFD40 size=116
    let mut pc: u32 = 0x826CFD40;
    'dispatch: loop {
        match pc {
            0x826CFD40 => {
    //   block [0x826CFD40..0x826CFDB4)
	// 826CFD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFD48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFD4C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CFD50: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826CFD54: 390AD030  addi r8, r10, -0x2fd0
	ctx.r[8].s64 = ctx.r[10].s64 + -12240;
	// 826CFD58: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFD5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826CFD60: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFD64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFD68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CFD6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFD70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFD74: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 826CFD78: 396B3834  addi r11, r11, 0x3834
	ctx.r[11].s64 = ctx.r[11].s64 + 14388;
	// 826CFD7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFD80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFD84: 386A9E6C  addi r3, r10, -0x6194
	ctx.r[3].s64 = ctx.r[10].s64 + -24980;
	// 826CFD88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826CFD8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFD90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826CFD94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFDA0: 4BD97081  bl 0x82466e20
	ctx.lr = 0x826CFDA4;
	sub_82466E20(ctx, base);
	// 826CFDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFDB8 size=112
    let mut pc: u32 = 0x826CFDB8;
    'dispatch: loop {
        match pc {
            0x826CFDB8 => {
    //   block [0x826CFDB8..0x826CFE28)
	// 826CFDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFDC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFDC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFDCC: 38AA9ECC  addi r5, r10, -0x6134
	ctx.r[5].s64 = ctx.r[10].s64 + -24884;
	// 826CFDD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFDD4: 390BD0C0  addi r8, r11, -0x2f40
	ctx.r[8].s64 = ctx.r[11].s64 + -12096;
	// 826CFDD8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826CFDDC: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826CFDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFDE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFDE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFDF0: 386A9E9C  addi r3, r10, -0x6164
	ctx.r[3].s64 = ctx.r[10].s64 + -24932;
	// 826CFDF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFDF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFDFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFE0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFE14: 4BD9700D  bl 0x82466e20
	ctx.lr = 0x826CFE18;
	sub_82466E20(ctx, base);
	// 826CFE18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFE1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFE20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFE28 size=100
    let mut pc: u32 = 0x826CFE28;
    'dispatch: loop {
        match pc {
            0x826CFE28 => {
    //   block [0x826CFE28..0x826CFE8C)
	// 826CFE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFE30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFE34: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFE3C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826CFE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFE48: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 826CFE4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFE5C: 386A9ECC  addi r3, r10, -0x6134
	ctx.r[3].s64 = ctx.r[10].s64 + -24884;
	// 826CFE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFE64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFE68: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826CFE6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFE70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826CFE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFE78: 4BD96FA9  bl 0x82466e20
	ctx.lr = 0x826CFE7C;
	sub_82466E20(ctx, base);
	// 826CFE7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFE80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFE84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFE88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826CFE90 size=24
    let mut pc: u32 = 0x826CFE90;
    'dispatch: loop {
        match pc {
            0x826CFE90 => {
    //   block [0x826CFE90..0x826CFEA8)
	// 826CFE90: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFE94: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826CFE98: 394AF828  addi r10, r10, -0x7d8
	ctx.r[10].s64 = ctx.r[10].s64 + -2008;
	// 826CFE9C: 816BCEDC  lwz r11, -0x3124(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12580 as u32) ) } as u64;
	// 826CFEA0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826CFEA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFEA8 size=116
    let mut pc: u32 = 0x826CFEA8;
    'dispatch: loop {
        match pc {
            0x826CFEA8 => {
    //   block [0x826CFEA8..0x826CFF1C)
	// 826CFEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFEAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFEB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFEB4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFEB8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826CFEBC: 390BF828  addi r8, r11, -0x7d8
	ctx.r[8].s64 = ctx.r[11].s64 + -2008;
	// 826CFEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFEC4: 392A3878  addi r9, r10, 0x3878
	ctx.r[9].s64 = ctx.r[10].s64 + 14456;
	// 826CFEC8: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFECC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826CFED0: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFED4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFEDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFEE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFEEC: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826CFEF0: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826CFEF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826CFEF8: 386B9EFC  addi r3, r11, -0x6104
	ctx.r[3].s64 = ctx.r[11].s64 + -24836;
	// 826CFEFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826CFF00: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFF08: 4BD96F19  bl 0x82466e20
	ctx.lr = 0x826CFF0C;
	sub_82466E20(ctx, base);
	// 826CFF0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFF18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFF20 size=112
    let mut pc: u32 = 0x826CFF20;
    'dispatch: loop {
        match pc {
            0x826CFF20 => {
    //   block [0x826CFF20..0x826CFF90)
	// 826CFF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFF2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFF30: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFF34: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826CFF38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826CFF3C: 390BD138  addi r8, r11, -0x2ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -11976;
	// 826CFF40: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826CFF44: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826CFF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFF4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFF58: 386A9F2C  addi r3, r10, -0x60d4
	ctx.r[3].s64 = ctx.r[10].s64 + -24788;
	// 826CFF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFF7C: 4BD96EA5  bl 0x82466e20
	ctx.lr = 0x826CFF80;
	sub_82466E20(ctx, base);
	// 826CFF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826CFF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826CFF90 size=112
    let mut pc: u32 = 0x826CFF90;
    'dispatch: loop {
        match pc {
            0x826CFF90 => {
    //   block [0x826CFF90..0x826D0000)
	// 826CFF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826CFF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826CFF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826CFF9C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFFA0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826CFFA4: 38AA9E6C  addi r5, r10, -0x6194
	ctx.r[5].s64 = ctx.r[10].s64 + -24980;
	// 826CFFA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826CFFAC: 390BD180  addi r8, r11, -0x2e80
	ctx.r[8].s64 = ctx.r[11].s64 + -11904;
	// 826CFFB0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826CFFB4: 388A22D4  addi r4, r10, 0x22d4
	ctx.r[4].s64 = ctx.r[10].s64 + 8916;
	// 826CFFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826CFFBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826CFFC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826CFFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826CFFC8: 386A9F5C  addi r3, r10, -0x60a4
	ctx.r[3].s64 = ctx.r[10].s64 + -24740;
	// 826CFFCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826CFFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826CFFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826CFFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826CFFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826CFFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826CFFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826CFFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826CFFEC: 4BD96E35  bl 0x82466e20
	ctx.lr = 0x826CFFF0;
	sub_82466E20(ctx, base);
	// 826CFFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826CFFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826CFFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826CFFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0000 size=108
    let mut pc: u32 = 0x826D0000;
    'dispatch: loop {
        match pc {
            0x826D0000 => {
    //   block [0x826D0000..0x826D006C)
	// 826D0000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D000C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D0014: 38EBD1E0  addi r7, r11, -0x2e20
	ctx.r[7].s64 = ctx.r[11].s64 + -11808;
	// 826D0018: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D001C: 388A22F4  addi r4, r10, 0x22f4
	ctx.r[4].s64 = ctx.r[10].s64 + 8948;
	// 826D0020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0024: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0028: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D002C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0030: 386A9F8C  addi r3, r10, -0x6074
	ctx.r[3].s64 = ctx.r[10].s64 + -24692;
	// 826D0034: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D0038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D003C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D004C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0054: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0058: 4BD96DC9  bl 0x82466e20
	ctx.lr = 0x826D005C;
	sub_82466E20(ctx, base);
	// 826D005C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0070 size=108
    let mut pc: u32 = 0x826D0070;
    'dispatch: loop {
        match pc {
            0x826D0070 => {
    //   block [0x826D0070..0x826D00DC)
	// 826D0070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D007C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0080: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D0084: 38EBD228  addi r7, r11, -0x2dd8
	ctx.r[7].s64 = ctx.r[11].s64 + -11736;
	// 826D0088: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D008C: 388A2320  addi r4, r10, 0x2320
	ctx.r[4].s64 = ctx.r[10].s64 + 8992;
	// 826D0090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0094: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0098: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D009C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D00A0: 386A9FBC  addi r3, r10, -0x6044
	ctx.r[3].s64 = ctx.r[10].s64 + -24644;
	// 826D00A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D00A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D00AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D00B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D00B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D00B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D00BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D00C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D00C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D00C8: 4BD96D59  bl 0x82466e20
	ctx.lr = 0x826D00CC;
	sub_82466E20(ctx, base);
	// 826D00CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D00D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D00D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D00D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D00E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D00E0 size=112
    let mut pc: u32 = 0x826D00E0;
    'dispatch: loop {
        match pc {
            0x826D00E0 => {
    //   block [0x826D00E0..0x826D0150)
	// 826D00E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D00E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D00E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D00EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D00F0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D00F4: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D00F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D00FC: 390BD270  addi r8, r11, -0x2d90
	ctx.r[8].s64 = ctx.r[11].s64 + -11664;
	// 826D0100: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D0104: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 826D0108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D010C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0118: 386A9FEC  addi r3, r10, -0x6014
	ctx.r[3].s64 = ctx.r[10].s64 + -24596;
	// 826D011C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D012C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D013C: 4BD96CE5  bl 0x82466e20
	ctx.lr = 0x826D0140;
	sub_82466E20(ctx, base);
	// 826D0140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D014C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0150 size=112
    let mut pc: u32 = 0x826D0150;
    'dispatch: loop {
        match pc {
            0x826D0150 => {
    //   block [0x826D0150..0x826D01C0)
	// 826D0150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D015C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0160: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0164: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D0168: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D016C: 390BD318  addi r8, r11, -0x2ce8
	ctx.r[8].s64 = ctx.r[11].s64 + -11496;
	// 826D0170: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D0174: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 826D0178: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D017C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0180: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0188: 386AA01C  addi r3, r10, -0x5fe4
	ctx.r[3].s64 = ctx.r[10].s64 + -24548;
	// 826D018C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0190: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0198: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D019C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D01A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D01A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D01A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D01AC: 4BD96C75  bl 0x82466e20
	ctx.lr = 0x826D01B0;
	sub_82466E20(ctx, base);
	// 826D01B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D01B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D01B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D01BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D01C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D01C0 size=108
    let mut pc: u32 = 0x826D01C0;
    'dispatch: loop {
        match pc {
            0x826D01C0 => {
    //   block [0x826D01C0..0x826D022C)
	// 826D01C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D01C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D01C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D01CC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D01D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D01D4: 38EBD360  addi r7, r11, -0x2ca0
	ctx.r[7].s64 = ctx.r[11].s64 + -11424;
	// 826D01D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D01DC: 388A234C  addi r4, r10, 0x234c
	ctx.r[4].s64 = ctx.r[10].s64 + 9036;
	// 826D01E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D01E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D01E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D01EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D01F0: 386AA04C  addi r3, r10, -0x5fb4
	ctx.r[3].s64 = ctx.r[10].s64 + -24500;
	// 826D01F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D01F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D01FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D020C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0214: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0218: 4BD96C09  bl 0x82466e20
	ctx.lr = 0x826D021C;
	sub_82466E20(ctx, base);
	// 826D021C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0220: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0224: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0228: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0230 size=108
    let mut pc: u32 = 0x826D0230;
    'dispatch: loop {
        match pc {
            0x826D0230 => {
    //   block [0x826D0230..0x826D029C)
	// 826D0230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D023C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0240: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D0244: 38EBD390  addi r7, r11, -0x2c70
	ctx.r[7].s64 = ctx.r[11].s64 + -11376;
	// 826D0248: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D024C: 388A2374  addi r4, r10, 0x2374
	ctx.r[4].s64 = ctx.r[10].s64 + 9076;
	// 826D0250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0254: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0258: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D025C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0260: 386AA07C  addi r3, r10, -0x5f84
	ctx.r[3].s64 = ctx.r[10].s64 + -24452;
	// 826D0264: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D0268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D026C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0270: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0278: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D027C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0280: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0288: 4BD96B99  bl 0x82466e20
	ctx.lr = 0x826D028C;
	sub_82466E20(ctx, base);
	// 826D028C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D02A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D02A0 size=112
    let mut pc: u32 = 0x826D02A0;
    'dispatch: loop {
        match pc {
            0x826D02A0 => {
    //   block [0x826D02A0..0x826D0310)
	// 826D02A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D02A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D02A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D02AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D02B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D02B4: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D02B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D02BC: 390BD420  addi r8, r11, -0x2be0
	ctx.r[8].s64 = ctx.r[11].s64 + -11232;
	// 826D02C0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D02C4: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 826D02C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D02CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D02D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D02D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D02D8: 386AA0AC  addi r3, r10, -0x5f54
	ctx.r[3].s64 = ctx.r[10].s64 + -24404;
	// 826D02DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D02E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D02E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D02E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D02EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D02F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D02F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D02F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D02FC: 4BD96B25  bl 0x82466e20
	ctx.lr = 0x826D0300;
	sub_82466E20(ctx, base);
	// 826D0300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D030C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0310 size=112
    let mut pc: u32 = 0x826D0310;
    'dispatch: loop {
        match pc {
            0x826D0310 => {
    //   block [0x826D0310..0x826D0380)
	// 826D0310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D031C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0320: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0324: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D0328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D032C: 390BD4B0  addi r8, r11, -0x2b50
	ctx.r[8].s64 = ctx.r[11].s64 + -11088;
	// 826D0330: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826D0334: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826D0338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D033C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0340: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0344: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0348: 386AA0DC  addi r3, r10, -0x5f24
	ctx.r[3].s64 = ctx.r[10].s64 + -24356;
	// 826D034C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D035C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D036C: 4BD96AB5  bl 0x82466e20
	ctx.lr = 0x826D0370;
	sub_82466E20(ctx, base);
	// 826D0370: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D037C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0380 size=112
    let mut pc: u32 = 0x826D0380;
    'dispatch: loop {
        match pc {
            0x826D0380 => {
    //   block [0x826D0380..0x826D03F0)
	// 826D0380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D038C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0390: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0394: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D0398: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D039C: 390BD570  addi r8, r11, -0x2a90
	ctx.r[8].s64 = ctx.r[11].s64 + -10896;
	// 826D03A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D03A4: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 826D03A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D03AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D03B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D03B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D03B8: 386AA10C  addi r3, r10, -0x5ef4
	ctx.r[3].s64 = ctx.r[10].s64 + -24308;
	// 826D03BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D03C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D03C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D03C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D03CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D03D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D03D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D03D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D03DC: 4BD96A45  bl 0x82466e20
	ctx.lr = 0x826D03E0;
	sub_82466E20(ctx, base);
	// 826D03E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D03E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D03E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D03EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D03F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D03F0 size=108
    let mut pc: u32 = 0x826D03F0;
    'dispatch: loop {
        match pc {
            0x826D03F0 => {
    //   block [0x826D03F0..0x826D045C)
	// 826D03F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D03F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D03F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D03FC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0404: 38EBD588  addi r7, r11, -0x2a78
	ctx.r[7].s64 = ctx.r[11].s64 + -10872;
	// 826D0408: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D040C: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 826D0410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D041C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0420: 386AA13C  addi r3, r10, -0x5ec4
	ctx.r[3].s64 = ctx.r[10].s64 + -24260;
	// 826D0424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D0428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D042C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D043C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0448: 4BD969D9  bl 0x82466e20
	ctx.lr = 0x826D044C;
	sub_82466E20(ctx, base);
	// 826D044C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0460 size=112
    let mut pc: u32 = 0x826D0460;
    'dispatch: loop {
        match pc {
            0x826D0460 => {
    //   block [0x826D0460..0x826D04D0)
	// 826D0460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D046C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0470: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0474: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D0478: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D047C: 390BD600  addi r8, r11, -0x2a00
	ctx.r[8].s64 = ctx.r[11].s64 + -10752;
	// 826D0480: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D0484: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 826D0488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D048C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0490: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0494: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0498: 386AA16C  addi r3, r10, -0x5e94
	ctx.r[3].s64 = ctx.r[10].s64 + -24212;
	// 826D049C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D04A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D04A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D04A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D04AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D04B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D04B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D04B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D04BC: 4BD96965  bl 0x82466e20
	ctx.lr = 0x826D04C0;
	sub_82466E20(ctx, base);
	// 826D04C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D04C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D04C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D04CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D04D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D04D0 size=100
    let mut pc: u32 = 0x826D04D0;
    'dispatch: loop {
        match pc {
            0x826D04D0 => {
    //   block [0x826D04D0..0x826D0534)
	// 826D04D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D04D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D04D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D04DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D04E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D04E4: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D04E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D04EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D04F0: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 826D04F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D04F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D04FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0504: 386AA19C  addi r3, r10, -0x5e64
	ctx.r[3].s64 = ctx.r[10].s64 + -24164;
	// 826D0508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D050C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0510: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0518: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D051C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0520: 4BD96901  bl 0x82466e20
	ctx.lr = 0x826D0524;
	sub_82466E20(ctx, base);
	// 826D0524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D052C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0538 size=112
    let mut pc: u32 = 0x826D0538;
    'dispatch: loop {
        match pc {
            0x826D0538 => {
    //   block [0x826D0538..0x826D05A8)
	// 826D0538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D053C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0544: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0548: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D054C: 38AAA19C  addi r5, r10, -0x5e64
	ctx.r[5].s64 = ctx.r[10].s64 + -24164;
	// 826D0550: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0554: 390BD648  addi r8, r11, -0x29b8
	ctx.r[8].s64 = ctx.r[11].s64 + -10680;
	// 826D0558: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D055C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 826D0560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0564: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0568: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D056C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0570: 386AA1CC  addi r3, r10, -0x5e34
	ctx.r[3].s64 = ctx.r[10].s64 + -24116;
	// 826D0574: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0578: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D057C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D058C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0594: 4BD9688D  bl 0x82466e20
	ctx.lr = 0x826D0598;
	sub_82466E20(ctx, base);
	// 826D0598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D059C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D05A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D05A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D05A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D05A8 size=112
    let mut pc: u32 = 0x826D05A8;
    'dispatch: loop {
        match pc {
            0x826D05A8 => {
    //   block [0x826D05A8..0x826D0618)
	// 826D05A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D05AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D05B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D05B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D05B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D05BC: 38AAA19C  addi r5, r10, -0x5e64
	ctx.r[5].s64 = ctx.r[10].s64 + -24164;
	// 826D05C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D05C4: 390BD6C0  addi r8, r11, -0x2940
	ctx.r[8].s64 = ctx.r[11].s64 + -10560;
	// 826D05C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D05CC: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 826D05D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D05D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D05D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D05DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D05E0: 386AA1FC  addi r3, r10, -0x5e04
	ctx.r[3].s64 = ctx.r[10].s64 + -24068;
	// 826D05E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D05E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D05EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D05F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D05F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D05F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D05FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0604: 4BD9681D  bl 0x82466e20
	ctx.lr = 0x826D0608;
	sub_82466E20(ctx, base);
	// 826D0608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D060C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0618 size=112
    let mut pc: u32 = 0x826D0618;
    'dispatch: loop {
        match pc {
            0x826D0618 => {
    //   block [0x826D0618..0x826D0688)
	// 826D0618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D061C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0624: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0628: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D062C: 38AAA19C  addi r5, r10, -0x5e64
	ctx.r[5].s64 = ctx.r[10].s64 + -24164;
	// 826D0630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0634: 390BD720  addi r8, r11, -0x28e0
	ctx.r[8].s64 = ctx.r[11].s64 + -10464;
	// 826D0638: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D063C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 826D0640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0644: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0648: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D064C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0650: 386AA22C  addi r3, r10, -0x5dd4
	ctx.r[3].s64 = ctx.r[10].s64 + -24020;
	// 826D0654: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D065C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D066C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0674: 4BD967AD  bl 0x82466e20
	ctx.lr = 0x826D0678;
	sub_82466E20(ctx, base);
	// 826D0678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D067C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0688 size=116
    let mut pc: u32 = 0x826D0688;
    'dispatch: loop {
        match pc {
            0x826D0688 => {
    //   block [0x826D0688..0x826D06FC)
	// 826D0688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D068C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0694: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D0698: 38E00012  li r7, 0x12
	ctx.r[7].s64 = 18;
	// 826D069C: 390AD780  addi r8, r10, -0x2880
	ctx.r[8].s64 = ctx.r[10].s64 + -10368;
	// 826D06A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D06A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D06A8: 38AAA6DC  addi r5, r10, -0x5924
	ctx.r[5].s64 = ctx.r[10].s64 + -22820;
	// 826D06AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D06B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D06B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D06B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D06BC: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 826D06C0: 396B38A0  addi r11, r11, 0x38a0
	ctx.r[11].s64 = ctx.r[11].s64 + 14496;
	// 826D06C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D06C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D06CC: 386AA25C  addi r3, r10, -0x5da4
	ctx.r[3].s64 = ctx.r[10].s64 + -23972;
	// 826D06D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D06D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D06D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D06DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D06E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D06E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D06E8: 4BD96739  bl 0x82466e20
	ctx.lr = 0x826D06EC;
	sub_82466E20(ctx, base);
	// 826D06EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D06F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D06F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D06F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0700 size=100
    let mut pc: u32 = 0x826D0700;
    'dispatch: loop {
        match pc {
            0x826D0700 => {
    //   block [0x826D0700..0x826D0764)
	// 826D0700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D070C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0714: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D0718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D071C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0720: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826D0724: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D072C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0734: 386AA28C  addi r3, r10, -0x5d74
	ctx.r[3].s64 = ctx.r[10].s64 + -23924;
	// 826D0738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D073C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D074C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0750: 4BD966D1  bl 0x82466e20
	ctx.lr = 0x826D0754;
	sub_82466E20(ctx, base);
	// 826D0754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D075C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0768 size=100
    let mut pc: u32 = 0x826D0768;
    'dispatch: loop {
        match pc {
            0x826D0768 => {
    //   block [0x826D0768..0x826D07CC)
	// 826D0768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D076C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0774: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D077C: 38AAA31C  addi r5, r10, -0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + -23780;
	// 826D0780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0784: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0788: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 826D078C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0790: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0794: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0798: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D079C: 386AA2BC  addi r3, r10, -0x5d44
	ctx.r[3].s64 = ctx.r[10].s64 + -23876;
	// 826D07A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D07A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D07A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D07AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D07B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D07B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D07B8: 4BD96669  bl 0x82466e20
	ctx.lr = 0x826D07BC;
	sub_82466E20(ctx, base);
	// 826D07BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D07C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D07C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D07C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D07D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D07D0 size=100
    let mut pc: u32 = 0x826D07D0;
    'dispatch: loop {
        match pc {
            0x826D07D0 => {
    //   block [0x826D07D0..0x826D0834)
	// 826D07D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D07D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D07D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D07DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D07E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D07E4: 38AAA25C  addi r5, r10, -0x5da4
	ctx.r[5].s64 = ctx.r[10].s64 + -23972;
	// 826D07E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D07EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D07F0: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826D07F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D07F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D07FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0804: 386AA2EC  addi r3, r10, -0x5d14
	ctx.r[3].s64 = ctx.r[10].s64 + -23828;
	// 826D0808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D080C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0810: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0818: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D081C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0820: 4BD96601  bl 0x82466e20
	ctx.lr = 0x826D0824;
	sub_82466E20(ctx, base);
	// 826D0824: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D082C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0838 size=104
    let mut pc: u32 = 0x826D0838;
    'dispatch: loop {
        match pc {
            0x826D0838 => {
    //   block [0x826D0838..0x826D08A0)
	// 826D0838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D083C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0844: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D0848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D084C: 392A388C  addi r9, r10, 0x388c
	ctx.r[9].s64 = ctx.r[10].s64 + 14476;
	// 826D0850: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0858: 38AAA28C  addi r5, r10, -0x5d74
	ctx.r[5].s64 = ctx.r[10].s64 + -23924;
	// 826D085C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0864: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D086C: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 826D0870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0874: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0878: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D087C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0880: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0884: 386AA31C  addi r3, r10, -0x5ce4
	ctx.r[3].s64 = ctx.r[10].s64 + -23780;
	// 826D0888: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D088C: 4BD96595  bl 0x82466e20
	ctx.lr = 0x826D0890;
	sub_82466E20(ctx, base);
	// 826D0890: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D089C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D08A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D08A0 size=108
    let mut pc: u32 = 0x826D08A0;
    'dispatch: loop {
        match pc {
            0x826D08A0 => {
    //   block [0x826D08A0..0x826D090C)
	// 826D08A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D08A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D08A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D08AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D08B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D08B4: 38EBD934  addi r7, r11, -0x26cc
	ctx.r[7].s64 = ctx.r[11].s64 + -9932;
	// 826D08B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D08BC: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 826D08C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D08C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D08C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D08CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D08D0: 386AA34C  addi r3, r10, -0x5cb4
	ctx.r[3].s64 = ctx.r[10].s64 + -23732;
	// 826D08D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D08D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D08DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D08E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D08E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D08E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D08EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D08F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D08F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D08F8: 4BD96529  bl 0x82466e20
	ctx.lr = 0x826D08FC;
	sub_82466E20(ctx, base);
	// 826D08FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0910 size=112
    let mut pc: u32 = 0x826D0910;
    'dispatch: loop {
        match pc {
            0x826D0910 => {
    //   block [0x826D0910..0x826D0980)
	// 826D0910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D091C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0920: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0924: 38AAA31C  addi r5, r10, -0x5ce4
	ctx.r[5].s64 = ctx.r[10].s64 + -23780;
	// 826D0928: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D092C: 390BD968  addi r8, r11, -0x2698
	ctx.r[8].s64 = ctx.r[11].s64 + -9880;
	// 826D0930: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D0934: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826D0938: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D093C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0940: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0948: 386AA37C  addi r3, r10, -0x5c84
	ctx.r[3].s64 = ctx.r[10].s64 + -23684;
	// 826D094C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0950: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0954: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0958: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D095C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0960: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0968: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D096C: 4BD964B5  bl 0x82466e20
	ctx.lr = 0x826D0970;
	sub_82466E20(ctx, base);
	// 826D0970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D097C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0980 size=116
    let mut pc: u32 = 0x826D0980;
    'dispatch: loop {
        match pc {
            0x826D0980 => {
    //   block [0x826D0980..0x826D09F4)
	// 826D0980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D098C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0990: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D0994: 390BDA10  addi r8, r11, -0x25f0
	ctx.r[8].s64 = ctx.r[11].s64 + -9712;
	// 826D0998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D099C: 392A390C  addi r9, r10, 0x390c
	ctx.r[9].s64 = ctx.r[10].s64 + 14604;
	// 826D09A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D09A4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826D09A8: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D09AC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D09B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D09B4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D09B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D09BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D09C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D09C4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D09C8: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 826D09CC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D09D0: 386BA3AC  addi r3, r11, -0x5c54
	ctx.r[3].s64 = ctx.r[11].s64 + -23636;
	// 826D09D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D09D8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D09DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D09E0: 4BD96441  bl 0x82466e20
	ctx.lr = 0x826D09E4;
	sub_82466E20(ctx, base);
	// 826D09E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D09E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D09EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D09F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D09F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D09F8 size=112
    let mut pc: u32 = 0x826D09F8;
    'dispatch: loop {
        match pc {
            0x826D09F8 => {
    //   block [0x826D09F8..0x826D0A68)
	// 826D09F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D09FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0A04: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0A08: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0A0C: 38AAA46C  addi r5, r10, -0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + -23444;
	// 826D0A10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0A14: 390BDA28  addi r8, r11, -0x25d8
	ctx.r[8].s64 = ctx.r[11].s64 + -9688;
	// 826D0A18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D0A1C: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826D0A20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0A24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0A28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0A30: 386AA3DC  addi r3, r10, -0x5c24
	ctx.r[3].s64 = ctx.r[10].s64 + -23588;
	// 826D0A34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0A38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0A40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0A44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0A48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0A4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0A50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0A54: 4BD963CD  bl 0x82466e20
	ctx.lr = 0x826D0A58;
	sub_82466E20(ctx, base);
	// 826D0A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0A68 size=100
    let mut pc: u32 = 0x826D0A68;
    'dispatch: loop {
        match pc {
            0x826D0A68 => {
    //   block [0x826D0A68..0x826D0ACC)
	// 826D0A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0A74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0A78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0A7C: 38AAA43C  addi r5, r10, -0x5bc4
	ctx.r[5].s64 = ctx.r[10].s64 + -23492;
	// 826D0A80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0A88: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 826D0A8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0A94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0A9C: 386AA40C  addi r3, r10, -0x5bf4
	ctx.r[3].s64 = ctx.r[10].s64 + -23540;
	// 826D0AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0AA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0AA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0AB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0AB8: 4BD96369  bl 0x82466e20
	ctx.lr = 0x826D0ABC;
	sub_82466E20(ctx, base);
	// 826D0ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0AD0 size=112
    let mut pc: u32 = 0x826D0AD0;
    'dispatch: loop {
        match pc {
            0x826D0AD0 => {
    //   block [0x826D0AD0..0x826D0B40)
	// 826D0AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0ADC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0AE0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0AE4: 38AAA46C  addi r5, r10, -0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + -23444;
	// 826D0AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0AEC: 390BDA40  addi r8, r11, -0x25c0
	ctx.r[8].s64 = ctx.r[11].s64 + -9664;
	// 826D0AF0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D0AF4: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826D0AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0AFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0B00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0B08: 386AA43C  addi r3, r10, -0x5bc4
	ctx.r[3].s64 = ctx.r[10].s64 + -23492;
	// 826D0B0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0B1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0B2C: 4BD962F5  bl 0x82466e20
	ctx.lr = 0x826D0B30;
	sub_82466E20(ctx, base);
	// 826D0B30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0B40 size=112
    let mut pc: u32 = 0x826D0B40;
    'dispatch: loop {
        match pc {
            0x826D0B40 => {
    //   block [0x826D0B40..0x826D0BB0)
	// 826D0B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0B48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0B4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0B50: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0B54: 38AAA3AC  addi r5, r10, -0x5c54
	ctx.r[5].s64 = ctx.r[10].s64 + -23636;
	// 826D0B58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D0B5C: 390BDA70  addi r8, r11, -0x2590
	ctx.r[8].s64 = ctx.r[11].s64 + -9616;
	// 826D0B60: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D0B64: 388A2424  addi r4, r10, 0x2424
	ctx.r[4].s64 = ctx.r[10].s64 + 9252;
	// 826D0B68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0B6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0B70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0B74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0B78: 386AA46C  addi r3, r10, -0x5b94
	ctx.r[3].s64 = ctx.r[10].s64 + -23444;
	// 826D0B7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0B80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0B88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0B8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0B90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0B94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0B98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0B9C: 4BD96285  bl 0x82466e20
	ctx.lr = 0x826D0BA0;
	sub_82466E20(ctx, base);
	// 826D0BA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0BB0 size=100
    let mut pc: u32 = 0x826D0BB0;
    'dispatch: loop {
        match pc {
            0x826D0BB0 => {
    //   block [0x826D0BB0..0x826D0C14)
	// 826D0BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0BB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0BBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0BC4: 38AAA46C  addi r5, r10, -0x5b94
	ctx.r[5].s64 = ctx.r[10].s64 + -23444;
	// 826D0BC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0BD0: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 826D0BD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0BE4: 386AA49C  addi r3, r10, -0x5b64
	ctx.r[3].s64 = ctx.r[10].s64 + -23396;
	// 826D0BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0BEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0BF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0BF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0C00: 4BD96221  bl 0x82466e20
	ctx.lr = 0x826D0C04;
	sub_82466E20(ctx, base);
	// 826D0C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0C18 size=100
    let mut pc: u32 = 0x826D0C18;
    'dispatch: loop {
        match pc {
            0x826D0C18 => {
    //   block [0x826D0C18..0x826D0C7C)
	// 826D0C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0C24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0C2C: 38AAA3DC  addi r5, r10, -0x5c24
	ctx.r[5].s64 = ctx.r[10].s64 + -23588;
	// 826D0C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0C38: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826D0C3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0C4C: 386AA4CC  addi r3, r10, -0x5b34
	ctx.r[3].s64 = ctx.r[10].s64 + -23348;
	// 826D0C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0C54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0C58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0C60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0C68: 4BD961B9  bl 0x82466e20
	ctx.lr = 0x826D0C6C;
	sub_82466E20(ctx, base);
	// 826D0C6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0C70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0C74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0C78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0C80 size=100
    let mut pc: u32 = 0x826D0C80;
    'dispatch: loop {
        match pc {
            0x826D0C80 => {
    //   block [0x826D0C80..0x826D0CE4)
	// 826D0C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0C8C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0C94: 38AAA49C  addi r5, r10, -0x5b64
	ctx.r[5].s64 = ctx.r[10].s64 + -23396;
	// 826D0C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0C9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0CA0: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 826D0CA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0CA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0CAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0CB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0CB4: 386AA4FC  addi r3, r10, -0x5b04
	ctx.r[3].s64 = ctx.r[10].s64 + -23300;
	// 826D0CB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0CBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0CC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0CC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0CC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0CD0: 4BD96151  bl 0x82466e20
	ctx.lr = 0x826D0CD4;
	sub_82466E20(ctx, base);
	// 826D0CD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0CE8 size=100
    let mut pc: u32 = 0x826D0CE8;
    'dispatch: loop {
        match pc {
            0x826D0CE8 => {
    //   block [0x826D0CE8..0x826D0D4C)
	// 826D0CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0CF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0CFC: 38AAA3DC  addi r5, r10, -0x5c24
	ctx.r[5].s64 = ctx.r[10].s64 + -23588;
	// 826D0D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0D04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0D08: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826D0D0C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0D1C: 386AA52C  addi r3, r10, -0x5ad4
	ctx.r[3].s64 = ctx.r[10].s64 + -23252;
	// 826D0D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0D24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0D28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D0D2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0D30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D0D34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0D38: 4BD960E9  bl 0x82466e20
	ctx.lr = 0x826D0D3C;
	sub_82466E20(ctx, base);
	// 826D0D3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0D50 size=112
    let mut pc: u32 = 0x826D0D50;
    'dispatch: loop {
        match pc {
            0x826D0D50 => {
    //   block [0x826D0D50..0x826D0DC0)
	// 826D0D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0D58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0D5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0D60: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0D64: 38AAA5BC  addi r5, r10, -0x5a44
	ctx.r[5].s64 = ctx.r[10].s64 + -23108;
	// 826D0D68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0D6C: 390BDB18  addi r8, r11, -0x24e8
	ctx.r[8].s64 = ctx.r[11].s64 + -9448;
	// 826D0D70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D0D74: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 826D0D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0D7C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0D80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0D84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0D88: 386AA55C  addi r3, r10, -0x5aa4
	ctx.r[3].s64 = ctx.r[10].s64 + -23204;
	// 826D0D8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0D90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0D94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0D98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0D9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0DA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0DA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0DA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0DAC: 4BD96075  bl 0x82466e20
	ctx.lr = 0x826D0DB0;
	sub_82466E20(ctx, base);
	// 826D0DB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0DB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0DB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0DC0 size=112
    let mut pc: u32 = 0x826D0DC0;
    'dispatch: loop {
        match pc {
            0x826D0DC0 => {
    //   block [0x826D0DC0..0x826D0E30)
	// 826D0DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0DC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0DCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0DD0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0DD4: 38AAA5EC  addi r5, r10, -0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + -23060;
	// 826D0DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0DDC: 390BDB48  addi r8, r11, -0x24b8
	ctx.r[8].s64 = ctx.r[11].s64 + -9400;
	// 826D0DE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D0DE4: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826D0DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0DEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0DF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0DF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0DF8: 386AA58C  addi r3, r10, -0x5a74
	ctx.r[3].s64 = ctx.r[10].s64 + -23156;
	// 826D0DFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0E04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0E0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0E1C: 4BD96005  bl 0x82466e20
	ctx.lr = 0x826D0E20;
	sub_82466E20(ctx, base);
	// 826D0E20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0E30 size=112
    let mut pc: u32 = 0x826D0E30;
    'dispatch: loop {
        match pc {
            0x826D0E30 => {
    //   block [0x826D0E30..0x826D0EA0)
	// 826D0E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0E3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0E40: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0E44: 38AAA6DC  addi r5, r10, -0x5924
	ctx.r[5].s64 = ctx.r[10].s64 + -22820;
	// 826D0E48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0E4C: 390BDB60  addi r8, r11, -0x24a0
	ctx.r[8].s64 = ctx.r[11].s64 + -9376;
	// 826D0E50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D0E54: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 826D0E58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0E5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0E60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0E64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0E68: 386AA5BC  addi r3, r10, -0x5a44
	ctx.r[3].s64 = ctx.r[10].s64 + -23108;
	// 826D0E6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0E70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0E74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0E78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0E7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0E80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0E84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0E88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0E8C: 4BD95F95  bl 0x82466e20
	ctx.lr = 0x826D0E90;
	sub_82466E20(ctx, base);
	// 826D0E90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0E94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0E98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0EA0 size=112
    let mut pc: u32 = 0x826D0EA0;
    'dispatch: loop {
        match pc {
            0x826D0EA0 => {
    //   block [0x826D0EA0..0x826D0F10)
	// 826D0EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0EA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0EAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0EB0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0EB4: 38AAA5BC  addi r5, r10, -0x5a44
	ctx.r[5].s64 = ctx.r[10].s64 + -23108;
	// 826D0EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0EBC: 390BDB90  addi r8, r11, -0x2470
	ctx.r[8].s64 = ctx.r[11].s64 + -9328;
	// 826D0EC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D0EC4: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 826D0EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0ED0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0ED4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0ED8: 386AA5EC  addi r3, r10, -0x5a14
	ctx.r[3].s64 = ctx.r[10].s64 + -23060;
	// 826D0EDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0EE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0EEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0EFC: 4BD95F25  bl 0x82466e20
	ctx.lr = 0x826D0F00;
	sub_82466E20(ctx, base);
	// 826D0F00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0F10 size=112
    let mut pc: u32 = 0x826D0F10;
    'dispatch: loop {
        match pc {
            0x826D0F10 => {
    //   block [0x826D0F10..0x826D0F80)
	// 826D0F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0F18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0F1C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0F20: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0F24: 38AAA5EC  addi r5, r10, -0x5a14
	ctx.r[5].s64 = ctx.r[10].s64 + -23060;
	// 826D0F28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0F2C: 390BDBA8  addi r8, r11, -0x2458
	ctx.r[8].s64 = ctx.r[11].s64 + -9304;
	// 826D0F30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D0F34: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826D0F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0F3C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0F40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D0F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0F48: 386AA61C  addi r3, r10, -0x59e4
	ctx.r[3].s64 = ctx.r[10].s64 + -23012;
	// 826D0F4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D0F50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0F54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0F58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0F5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0F60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0F64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0F68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0F6C: 4BD95EB5  bl 0x82466e20
	ctx.lr = 0x826D0F70;
	sub_82466E20(ctx, base);
	// 826D0F70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0F74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0F78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0F80 size=108
    let mut pc: u32 = 0x826D0F80;
    'dispatch: loop {
        match pc {
            0x826D0F80 => {
    //   block [0x826D0F80..0x826D0FEC)
	// 826D0F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0F88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0F8C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D0F90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D0F94: 38EBDBC4  addi r7, r11, -0x243c
	ctx.r[7].s64 = ctx.r[11].s64 + -9276;
	// 826D0F98: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D0F9C: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 826D0FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D0FA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D0FA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D0FAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D0FB0: 386AA64C  addi r3, r10, -0x59b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22964;
	// 826D0FB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D0FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D0FBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D0FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D0FC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D0FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D0FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D0FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D0FD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D0FD8: 4BD95E49  bl 0x82466e20
	ctx.lr = 0x826D0FDC;
	sub_82466E20(ctx, base);
	// 826D0FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D0FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D0FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D0FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D0FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D0FF0 size=112
    let mut pc: u32 = 0x826D0FF0;
    'dispatch: loop {
        match pc {
            0x826D0FF0 => {
    //   block [0x826D0FF0..0x826D1060)
	// 826D0FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D0FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D0FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D0FFC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1000: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1004: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1008: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D100C: 390BDBF8  addi r8, r11, -0x2408
	ctx.r[8].s64 = ctx.r[11].s64 + -9224;
	// 826D1010: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D1014: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 826D1018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D101C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1020: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1028: 386AA67C  addi r3, r10, -0x5984
	ctx.r[3].s64 = ctx.r[10].s64 + -22916;
	// 826D102C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1034: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D103C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D104C: 4BD95DD5  bl 0x82466e20
	ctx.lr = 0x826D1050;
	sub_82466E20(ctx, base);
	// 826D1050: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D105C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D1060 size=48
    let mut pc: u32 = 0x826D1060;
    'dispatch: loop {
        match pc {
            0x826D1060 => {
    //   block [0x826D1060..0x826D1090)
	// 826D1060: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1064: 814BDC90  lwz r10, -0x2370(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9072 as u32) ) } as u64;
	// 826D1068: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D106C: 396BF888  addi r11, r11, -0x778
	ctx.r[11].s64 = ctx.r[11].s64 + -1912;
	// 826D1070: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 826D1074: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D1078: 814ADC8C  lwz r10, -0x2374(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-9076 as u32) ) } as u64;
	// 826D107C: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826D1080: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D1084: 814ADC88  lwz r10, -0x2378(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-9080 as u32) ) } as u64;
	// 826D1088: 914B0230  stw r10, 0x230(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(560 as u32), ctx.r[10].u32 ) };
	// 826D108C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1090 size=116
    let mut pc: u32 = 0x826D1090;
    'dispatch: loop {
        match pc {
            0x826D1090 => {
    //   block [0x826D1090..0x826D1104)
	// 826D1090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D109C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D10A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D10A4: 392B3A90  addi r9, r11, 0x3a90
	ctx.r[9].s64 = ctx.r[11].s64 + 14992;
	// 826D10A8: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D10AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D10B0: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 826D10B4: 38C0001B  li r6, 0x1b
	ctx.r[6].s64 = 27;
	// 826D10B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D10BC: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 826D10C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D10C4: 396BF888  addi r11, r11, -0x778
	ctx.r[11].s64 = ctx.r[11].s64 + -1912;
	// 826D10C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D10CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D10D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D10D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D10D8: 386AA6AC  addi r3, r10, -0x5954
	ctx.r[3].s64 = ctx.r[10].s64 + -22868;
	// 826D10DC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 826D10E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D10E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D10E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D10EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D10F0: 4BD95D31  bl 0x82466e20
	ctx.lr = 0x826D10F4;
	sub_82466E20(ctx, base);
	// 826D10F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D10F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D10FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1108 size=116
    let mut pc: u32 = 0x826D1108;
    'dispatch: loop {
        match pc {
            0x826D1108 => {
    //   block [0x826D1108..0x826D117C)
	// 826D1108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D110C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1114: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1118: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D111C: 390BDC98  addi r8, r11, -0x2368
	ctx.r[8].s64 = ctx.r[11].s64 + -9064;
	// 826D1120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1124: 392A399C  addi r9, r10, 0x399c
	ctx.r[9].s64 = ctx.r[10].s64 + 14748;
	// 826D1128: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D112C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D1130: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1134: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D113C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D114C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D1150: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 826D1154: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D1158: 386BA6DC  addi r3, r11, -0x5924
	ctx.r[3].s64 = ctx.r[11].s64 + -22820;
	// 826D115C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D1160: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1168: 4BD95CB9  bl 0x82466e20
	ctx.lr = 0x826D116C;
	sub_82466E20(ctx, base);
	// 826D116C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1180 size=112
    let mut pc: u32 = 0x826D1180;
    'dispatch: loop {
        match pc {
            0x826D1180 => {
    //   block [0x826D1180..0x826D11F0)
	// 826D1180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D118C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1190: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1194: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D119C: 390BDD10  addi r8, r11, -0x22f0
	ctx.r[8].s64 = ctx.r[11].s64 + -8944;
	// 826D11A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D11A4: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 826D11A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D11AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D11B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D11B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D11B8: 386AA70C  addi r3, r10, -0x58f4
	ctx.r[3].s64 = ctx.r[10].s64 + -22772;
	// 826D11BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D11C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D11C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D11C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D11CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D11D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D11D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D11D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D11DC: 4BD95C45  bl 0x82466e20
	ctx.lr = 0x826D11E0;
	sub_82466E20(ctx, base);
	// 826D11E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D11E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D11E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D11EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D11F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D11F0 size=112
    let mut pc: u32 = 0x826D11F0;
    'dispatch: loop {
        match pc {
            0x826D11F0 => {
    //   block [0x826D11F0..0x826D1260)
	// 826D11F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D11F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D11F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D11FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1200: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1204: 38AA911C  addi r5, r10, -0x6ee4
	ctx.r[5].s64 = ctx.r[10].s64 + -28388;
	// 826D1208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D120C: 390BDD28  addi r8, r11, -0x22d8
	ctx.r[8].s64 = ctx.r[11].s64 + -8920;
	// 826D1210: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1214: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 826D1218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D121C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1228: 386AA73C  addi r3, r10, -0x58c4
	ctx.r[3].s64 = ctx.r[10].s64 + -22724;
	// 826D122C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D123C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D124C: 4BD95BD5  bl 0x82466e20
	ctx.lr = 0x826D1250;
	sub_82466E20(ctx, base);
	// 826D1250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D125C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1260 size=108
    let mut pc: u32 = 0x826D1260;
    'dispatch: loop {
        match pc {
            0x826D1260 => {
    //   block [0x826D1260..0x826D12CC)
	// 826D1260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D126C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1270: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1274: 38EBDD40  addi r7, r11, -0x22c0
	ctx.r[7].s64 = ctx.r[11].s64 + -8896;
	// 826D1278: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D127C: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 826D1280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1284: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1288: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D128C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1290: 386AA76C  addi r3, r10, -0x5894
	ctx.r[3].s64 = ctx.r[10].s64 + -22676;
	// 826D1294: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D129C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D12A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D12A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D12A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D12AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D12B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D12B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D12B8: 4BD95B69  bl 0x82466e20
	ctx.lr = 0x826D12BC;
	sub_82466E20(ctx, base);
	// 826D12BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D12C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D12C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D12C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D12D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D12D0 size=112
    let mut pc: u32 = 0x826D12D0;
    'dispatch: loop {
        match pc {
            0x826D12D0 => {
    //   block [0x826D12D0..0x826D1340)
	// 826D12D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D12D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D12D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D12DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D12E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D12E4: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D12E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D12EC: 390BDD58  addi r8, r11, -0x22a8
	ctx.r[8].s64 = ctx.r[11].s64 + -8872;
	// 826D12F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D12F4: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 826D12F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D12FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1308: 386AA79C  addi r3, r10, -0x5864
	ctx.r[3].s64 = ctx.r[10].s64 + -22628;
	// 826D130C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D131C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D132C: 4BD95AF5  bl 0x82466e20
	ctx.lr = 0x826D1330;
	sub_82466E20(ctx, base);
	// 826D1330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D133C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1340 size=108
    let mut pc: u32 = 0x826D1340;
    'dispatch: loop {
        match pc {
            0x826D1340 => {
    //   block [0x826D1340..0x826D13AC)
	// 826D1340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D134C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1354: 38EBDDA0  addi r7, r11, -0x2260
	ctx.r[7].s64 = ctx.r[11].s64 + -8800;
	// 826D1358: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D135C: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 826D1360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1364: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1368: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D136C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1370: 386AA7CC  addi r3, r10, -0x5834
	ctx.r[3].s64 = ctx.r[10].s64 + -22580;
	// 826D1374: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D137C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D138C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1398: 4BD95A89  bl 0x82466e20
	ctx.lr = 0x826D139C;
	sub_82466E20(ctx, base);
	// 826D139C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D13A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D13A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D13A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D13B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D13B0 size=112
    let mut pc: u32 = 0x826D13B0;
    'dispatch: loop {
        match pc {
            0x826D13B0 => {
    //   block [0x826D13B0..0x826D1420)
	// 826D13B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D13B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D13B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D13BC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D13C0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D13C4: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D13C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D13CC: 390BDDB8  addi r8, r11, -0x2248
	ctx.r[8].s64 = ctx.r[11].s64 + -8776;
	// 826D13D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D13D4: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 826D13D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D13DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D13E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D13E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D13E8: 386AA7FC  addi r3, r10, -0x5804
	ctx.r[3].s64 = ctx.r[10].s64 + -22532;
	// 826D13EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D13F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D13F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D13F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D13FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D140C: 4BD95A15  bl 0x82466e20
	ctx.lr = 0x826D1410;
	sub_82466E20(ctx, base);
	// 826D1410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1420 size=112
    let mut pc: u32 = 0x826D1420;
    'dispatch: loop {
        match pc {
            0x826D1420 => {
    //   block [0x826D1420..0x826D1490)
	// 826D1420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D142C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1430: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1434: 38AAA8BC  addi r5, r10, -0x5744
	ctx.r[5].s64 = ctx.r[10].s64 + -22340;
	// 826D1438: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D143C: 390BDDE8  addi r8, r11, -0x2218
	ctx.r[8].s64 = ctx.r[11].s64 + -8728;
	// 826D1440: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D1444: 388A2464  addi r4, r10, 0x2464
	ctx.r[4].s64 = ctx.r[10].s64 + 9316;
	// 826D1448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D144C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1458: 386AA82C  addi r3, r10, -0x57d4
	ctx.r[3].s64 = ctx.r[10].s64 + -22484;
	// 826D145C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D146C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D147C: 4BD959A5  bl 0x82466e20
	ctx.lr = 0x826D1480;
	sub_82466E20(ctx, base);
	// 826D1480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D148C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1490 size=108
    let mut pc: u32 = 0x826D1490;
    'dispatch: loop {
        match pc {
            0x826D1490 => {
    //   block [0x826D1490..0x826D14FC)
	// 826D1490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D149C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D14A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D14A4: 38EBDE60  addi r7, r11, -0x21a0
	ctx.r[7].s64 = ctx.r[11].s64 + -8608;
	// 826D14A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D14AC: 388A2484  addi r4, r10, 0x2484
	ctx.r[4].s64 = ctx.r[10].s64 + 9348;
	// 826D14B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D14B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D14B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D14BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D14C0: 386AA85C  addi r3, r10, -0x57a4
	ctx.r[3].s64 = ctx.r[10].s64 + -22436;
	// 826D14C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D14C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D14CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D14D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D14D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D14D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D14DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D14E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D14E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D14E8: 4BD95939  bl 0x82466e20
	ctx.lr = 0x826D14EC;
	sub_82466E20(ctx, base);
	// 826D14EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D14F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D14F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D14F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1500 size=108
    let mut pc: u32 = 0x826D1500;
    'dispatch: loop {
        match pc {
            0x826D1500 => {
    //   block [0x826D1500..0x826D156C)
	// 826D1500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D150C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D1514: 38EBDEA8  addi r7, r11, -0x2158
	ctx.r[7].s64 = ctx.r[11].s64 + -8536;
	// 826D1518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D151C: 388A24AC  addi r4, r10, 0x24ac
	ctx.r[4].s64 = ctx.r[10].s64 + 9388;
	// 826D1520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1524: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D152C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1530: 386AA88C  addi r3, r10, -0x5774
	ctx.r[3].s64 = ctx.r[10].s64 + -22388;
	// 826D1534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D153C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D154C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1558: 4BD958C9  bl 0x82466e20
	ctx.lr = 0x826D155C;
	sub_82466E20(ctx, base);
	// 826D155C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1570 size=116
    let mut pc: u32 = 0x826D1570;
    'dispatch: loop {
        match pc {
            0x826D1570 => {
    //   block [0x826D1570..0x826D15E4)
	// 826D1570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D157C: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D1580: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 826D1584: 390ADEF0  addi r8, r10, -0x2110
	ctx.r[8].s64 = ctx.r[10].s64 + -8464;
	// 826D1588: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D158C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D1590: 38AA9CBC  addi r5, r10, -0x6344
	ctx.r[5].s64 = ctx.r[10].s64 + -25412;
	// 826D1594: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1598: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D159C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D15A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D15A4: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 826D15A8: 396B3B78  addi r11, r11, 0x3b78
	ctx.r[11].s64 = ctx.r[11].s64 + 15224;
	// 826D15AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D15B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D15B4: 386AA8BC  addi r3, r10, -0x5744
	ctx.r[3].s64 = ctx.r[10].s64 + -22340;
	// 826D15B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D15BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D15C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D15C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D15C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D15CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D15D0: 4BD95851  bl 0x82466e20
	ctx.lr = 0x826D15D4;
	sub_82466E20(ctx, base);
	// 826D15D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D15D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D15DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D15E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D15E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D15E8 size=112
    let mut pc: u32 = 0x826D15E8;
    'dispatch: loop {
        match pc {
            0x826D15E8 => {
    //   block [0x826D15E8..0x826D1658)
	// 826D15E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D15EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D15F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D15F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D15F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D15FC: 38AA9BCC  addi r5, r10, -0x6434
	ctx.r[5].s64 = ctx.r[10].s64 + -25652;
	// 826D1600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1604: 390BDFE0  addi r8, r11, -0x2020
	ctx.r[8].s64 = ctx.r[11].s64 + -8224;
	// 826D1608: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D160C: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 826D1610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1614: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D161C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1620: 386AA8EC  addi r3, r10, -0x5714
	ctx.r[3].s64 = ctx.r[10].s64 + -22292;
	// 826D1624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D162C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D163C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1644: 4BD957DD  bl 0x82466e20
	ctx.lr = 0x826D1648;
	sub_82466E20(ctx, base);
	// 826D1648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D164C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1658 size=112
    let mut pc: u32 = 0x826D1658;
    'dispatch: loop {
        match pc {
            0x826D1658 => {
    //   block [0x826D1658..0x826D16C8)
	// 826D1658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D165C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1668: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D166C: 38AA9BCC  addi r5, r10, -0x6434
	ctx.r[5].s64 = ctx.r[10].s64 + -25652;
	// 826D1670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1674: 390BE028  addi r8, r11, -0x1fd8
	ctx.r[8].s64 = ctx.r[11].s64 + -8152;
	// 826D1678: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D167C: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826D1680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1684: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D168C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1690: 386AA91C  addi r3, r10, -0x56e4
	ctx.r[3].s64 = ctx.r[10].s64 + -22244;
	// 826D1694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D169C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D16A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D16A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D16A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D16AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D16B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D16B4: 4BD9576D  bl 0x82466e20
	ctx.lr = 0x826D16B8;
	sub_82466E20(ctx, base);
	// 826D16B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D16BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D16C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D16C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D16C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D16C8 size=112
    let mut pc: u32 = 0x826D16C8;
    'dispatch: loop {
        match pc {
            0x826D16C8 => {
    //   block [0x826D16C8..0x826D1738)
	// 826D16C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D16CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D16D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D16D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D16D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D16DC: 38AA9BFC  addi r5, r10, -0x6404
	ctx.r[5].s64 = ctx.r[10].s64 + -25604;
	// 826D16E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D16E4: 390BE088  addi r8, r11, -0x1f78
	ctx.r[8].s64 = ctx.r[11].s64 + -8056;
	// 826D16E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D16EC: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 826D16F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D16F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D16F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D16FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1700: 386AA94C  addi r3, r10, -0x56b4
	ctx.r[3].s64 = ctx.r[10].s64 + -22196;
	// 826D1704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D170C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D171C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1724: 4BD956FD  bl 0x82466e20
	ctx.lr = 0x826D1728;
	sub_82466E20(ctx, base);
	// 826D1728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D172C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1738 size=112
    let mut pc: u32 = 0x826D1738;
    'dispatch: loop {
        match pc {
            0x826D1738 => {
    //   block [0x826D1738..0x826D17A8)
	// 826D1738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D173C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1748: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D174C: 38AA9BFC  addi r5, r10, -0x6404
	ctx.r[5].s64 = ctx.r[10].s64 + -25604;
	// 826D1750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1754: 390BE0E8  addi r8, r11, -0x1f18
	ctx.r[8].s64 = ctx.r[11].s64 + -7960;
	// 826D1758: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D175C: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 826D1760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D176C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1770: 386AA97C  addi r3, r10, -0x5684
	ctx.r[3].s64 = ctx.r[10].s64 + -22148;
	// 826D1774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D177C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D178C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1794: 4BD9568D  bl 0x82466e20
	ctx.lr = 0x826D1798;
	sub_82466E20(ctx, base);
	// 826D1798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D179C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D17A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D17A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D17A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D17A8 size=112
    let mut pc: u32 = 0x826D17A8;
    'dispatch: loop {
        match pc {
            0x826D17A8 => {
    //   block [0x826D17A8..0x826D1818)
	// 826D17A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D17AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D17B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D17B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D17B8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D17BC: 38AA9BCC  addi r5, r10, -0x6434
	ctx.r[5].s64 = ctx.r[10].s64 + -25652;
	// 826D17C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D17C4: 390BE148  addi r8, r11, -0x1eb8
	ctx.r[8].s64 = ctx.r[11].s64 + -7864;
	// 826D17C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826D17CC: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826D17D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D17D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D17D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D17DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D17E0: 386AA9AC  addi r3, r10, -0x5654
	ctx.r[3].s64 = ctx.r[10].s64 + -22100;
	// 826D17E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D17E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D17EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D17F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D17F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D17F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D17FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1804: 4BD9561D  bl 0x82466e20
	ctx.lr = 0x826D1808;
	sub_82466E20(ctx, base);
	// 826D1808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D180C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1818 size=108
    let mut pc: u32 = 0x826D1818;
    'dispatch: loop {
        match pc {
            0x826D1818 => {
    //   block [0x826D1818..0x826D1884)
	// 826D1818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D181C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1824: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D182C: 38EBE208  addi r7, r11, -0x1df8
	ctx.r[7].s64 = ctx.r[11].s64 + -7672;
	// 826D1830: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826D1834: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826D1838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D183C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1848: 386AA9DC  addi r3, r10, -0x5624
	ctx.r[3].s64 = ctx.r[10].s64 + -22052;
	// 826D184C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D185C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D186C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1870: 4BD955B1  bl 0x82466e20
	ctx.lr = 0x826D1874;
	sub_82466E20(ctx, base);
	// 826D1874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D187C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1888 size=112
    let mut pc: u32 = 0x826D1888;
    'dispatch: loop {
        match pc {
            0x826D1888 => {
    //   block [0x826D1888..0x826D18F8)
	// 826D1888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D188C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1894: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1898: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D189C: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826D18A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D18A4: 390BE3A0  addi r8, r11, -0x1c60
	ctx.r[8].s64 = ctx.r[11].s64 + -7264;
	// 826D18A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D18AC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826D18B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D18B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D18B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D18BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D18C0: 386AAA0C  addi r3, r10, -0x55f4
	ctx.r[3].s64 = ctx.r[10].s64 + -22004;
	// 826D18C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D18C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D18CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D18D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D18D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D18D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D18DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D18E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D18E4: 4BD9553D  bl 0x82466e20
	ctx.lr = 0x826D18E8;
	sub_82466E20(ctx, base);
	// 826D18E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D18EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D18F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D18F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D18F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D18F8 size=112
    let mut pc: u32 = 0x826D18F8;
    'dispatch: loop {
        match pc {
            0x826D18F8 => {
    //   block [0x826D18F8..0x826D1968)
	// 826D18F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D18FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1904: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1908: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D190C: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826D1910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1914: 390BE3B8  addi r8, r11, -0x1c48
	ctx.r[8].s64 = ctx.r[11].s64 + -7240;
	// 826D1918: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D191C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826D1920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1924: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1928: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D192C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1930: 386AAA3C  addi r3, r10, -0x55c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21956;
	// 826D1934: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D193C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1944: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D1948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D194C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1954: 4BD954CD  bl 0x82466e20
	ctx.lr = 0x826D1958;
	sub_82466E20(ctx, base);
	// 826D1958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D195C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1968 size=112
    let mut pc: u32 = 0x826D1968;
    'dispatch: loop {
        match pc {
            0x826D1968 => {
    //   block [0x826D1968..0x826D19D8)
	// 826D1968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D196C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1974: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1978: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D197C: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826D1980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1984: 390BE3D0  addi r8, r11, -0x1c30
	ctx.r[8].s64 = ctx.r[11].s64 + -7216;
	// 826D1988: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D198C: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826D1990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1994: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1998: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D199C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D19A0: 386AAA6C  addi r3, r10, -0x5594
	ctx.r[3].s64 = ctx.r[10].s64 + -21908;
	// 826D19A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D19A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D19AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D19B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D19B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D19B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D19BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D19C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D19C4: 4BD9545D  bl 0x82466e20
	ctx.lr = 0x826D19C8;
	sub_82466E20(ctx, base);
	// 826D19C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D19CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D19D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D19D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D19D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D19D8 size=108
    let mut pc: u32 = 0x826D19D8;
    'dispatch: loop {
        match pc {
            0x826D19D8 => {
    //   block [0x826D19D8..0x826D1A44)
	// 826D19D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D19DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D19E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D19E4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D19E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D19EC: 38EBE400  addi r7, r11, -0x1c00
	ctx.r[7].s64 = ctx.r[11].s64 + -7168;
	// 826D19F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D19F4: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826D19F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D19FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1A00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1A08: 386AAA9C  addi r3, r10, -0x5564
	ctx.r[3].s64 = ctx.r[10].s64 + -21860;
	// 826D1A0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1A30: 4BD953F1  bl 0x82466e20
	ctx.lr = 0x826D1A34;
	sub_82466E20(ctx, base);
	// 826D1A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1A48 size=112
    let mut pc: u32 = 0x826D1A48;
    'dispatch: loop {
        match pc {
            0x826D1A48 => {
    //   block [0x826D1A48..0x826D1AB8)
	// 826D1A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1A54: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1A58: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1A5C: 38AA91DC  addi r5, r10, -0x6e24
	ctx.r[5].s64 = ctx.r[10].s64 + -28196;
	// 826D1A60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1A64: 390BE430  addi r8, r11, -0x1bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -7120;
	// 826D1A68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1A6C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826D1A70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1A74: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1A78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1A7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1A80: 386AAACC  addi r3, r10, -0x5534
	ctx.r[3].s64 = ctx.r[10].s64 + -21812;
	// 826D1A84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1A8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1A90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1A94: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D1A98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1AA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1AA4: 4BD9537D  bl 0x82466e20
	ctx.lr = 0x826D1AA8;
	sub_82466E20(ctx, base);
	// 826D1AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1AAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1AB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1AB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1AB8 size=112
    let mut pc: u32 = 0x826D1AB8;
    'dispatch: loop {
        match pc {
            0x826D1AB8 => {
    //   block [0x826D1AB8..0x826D1B28)
	// 826D1AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1AC4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1AC8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1ACC: 38AA9BFC  addi r5, r10, -0x6404
	ctx.r[5].s64 = ctx.r[10].s64 + -25604;
	// 826D1AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1AD4: 390BE448  addi r8, r11, -0x1bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -7096;
	// 826D1AD8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D1ADC: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826D1AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1AE4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1AF0: 386AAAFC  addi r3, r10, -0x5504
	ctx.r[3].s64 = ctx.r[10].s64 + -21764;
	// 826D1AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1B14: 4BD9530D  bl 0x82466e20
	ctx.lr = 0x826D1B18;
	sub_82466E20(ctx, base);
	// 826D1B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1B28 size=108
    let mut pc: u32 = 0x826D1B28;
    'dispatch: loop {
        match pc {
            0x826D1B28 => {
    //   block [0x826D1B28..0x826D1B94)
	// 826D1B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1B34: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1B3C: 38EBE4D8  addi r7, r11, -0x1b28
	ctx.r[7].s64 = ctx.r[11].s64 + -6952;
	// 826D1B40: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D1B44: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826D1B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1B4C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1B50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1B58: 386AAB2C  addi r3, r10, -0x54d4
	ctx.r[3].s64 = ctx.r[10].s64 + -21716;
	// 826D1B5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1B6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1B7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1B80: 4BD952A1  bl 0x82466e20
	ctx.lr = 0x826D1B84;
	sub_82466E20(ctx, base);
	// 826D1B84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1B90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1B98 size=108
    let mut pc: u32 = 0x826D1B98;
    'dispatch: loop {
        match pc {
            0x826D1B98 => {
    //   block [0x826D1B98..0x826D1C04)
	// 826D1B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1BA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1BA4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1BAC: 38EBE520  addi r7, r11, -0x1ae0
	ctx.r[7].s64 = ctx.r[11].s64 + -6880;
	// 826D1BB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D1BB4: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826D1BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1BBC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1BC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1BC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1BC8: 386AAB5C  addi r3, r10, -0x54a4
	ctx.r[3].s64 = ctx.r[10].s64 + -21668;
	// 826D1BCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1BDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1BF0: 4BD95231  bl 0x82466e20
	ctx.lr = 0x826D1BF4;
	sub_82466E20(ctx, base);
	// 826D1BF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1C00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1C08 size=108
    let mut pc: u32 = 0x826D1C08;
    'dispatch: loop {
        match pc {
            0x826D1C08 => {
    //   block [0x826D1C08..0x826D1C74)
	// 826D1C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1C10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1C14: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1C18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1C1C: 38EBE550  addi r7, r11, -0x1ab0
	ctx.r[7].s64 = ctx.r[11].s64 + -6832;
	// 826D1C20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D1C24: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826D1C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1C2C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1C30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1C34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1C38: 386AAB8C  addi r3, r10, -0x5474
	ctx.r[3].s64 = ctx.r[10].s64 + -21620;
	// 826D1C3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1C40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1C48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1C50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1C54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1C58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1C5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1C60: 4BD951C1  bl 0x82466e20
	ctx.lr = 0x826D1C64;
	sub_82466E20(ctx, base);
	// 826D1C64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1C78 size=112
    let mut pc: u32 = 0x826D1C78;
    'dispatch: loop {
        match pc {
            0x826D1C78 => {
    //   block [0x826D1C78..0x826D1CE8)
	// 826D1C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1C80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1C84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1C88: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1C8C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1C90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1C94: 390BE580  addi r8, r11, -0x1a80
	ctx.r[8].s64 = ctx.r[11].s64 + -6784;
	// 826D1C98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D1C9C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826D1CA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1CA4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1CA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1CB0: 386AABBC  addi r3, r10, -0x5444
	ctx.r[3].s64 = ctx.r[10].s64 + -21572;
	// 826D1CB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1CB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1CBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1CC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1CC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1CCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1CD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1CD4: 4BD9514D  bl 0x82466e20
	ctx.lr = 0x826D1CD8;
	sub_82466E20(ctx, base);
	// 826D1CD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1CE8 size=112
    let mut pc: u32 = 0x826D1CE8;
    'dispatch: loop {
        match pc {
            0x826D1CE8 => {
    //   block [0x826D1CE8..0x826D1D58)
	// 826D1CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1CEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1CF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1CF8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1CFC: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1D00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1D04: 390BE5B0  addi r8, r11, -0x1a50
	ctx.r[8].s64 = ctx.r[11].s64 + -6736;
	// 826D1D08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1D0C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 826D1D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1D14: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1D18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1D1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1D20: 386AABEC  addi r3, r10, -0x5414
	ctx.r[3].s64 = ctx.r[10].s64 + -21524;
	// 826D1D24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1D28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1D2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1D30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1D38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1D3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1D40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1D44: 4BD950DD  bl 0x82466e20
	ctx.lr = 0x826D1D48;
	sub_82466E20(ctx, base);
	// 826D1D48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1D4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1D50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1D54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1D58 size=112
    let mut pc: u32 = 0x826D1D58;
    'dispatch: loop {
        match pc {
            0x826D1D58 => {
    //   block [0x826D1D58..0x826D1DC8)
	// 826D1D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1D60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1D64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1D68: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1D6C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1D70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1D74: 390BE5C8  addi r8, r11, -0x1a38
	ctx.r[8].s64 = ctx.r[11].s64 + -6712;
	// 826D1D78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1D7C: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826D1D80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1D84: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1D88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1D90: 386AAC1C  addi r3, r10, -0x53e4
	ctx.r[3].s64 = ctx.r[10].s64 + -21476;
	// 826D1D94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1D9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1DA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1DA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1DA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1DAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1DB4: 4BD9506D  bl 0x82466e20
	ctx.lr = 0x826D1DB8;
	sub_82466E20(ctx, base);
	// 826D1DB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1DC8 size=108
    let mut pc: u32 = 0x826D1DC8;
    'dispatch: loop {
        match pc {
            0x826D1DC8 => {
    //   block [0x826D1DC8..0x826D1E34)
	// 826D1DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1DD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1DD4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1DD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1DDC: 38EBE5E0  addi r7, r11, -0x1a20
	ctx.r[7].s64 = ctx.r[11].s64 + -6688;
	// 826D1DE0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D1DE4: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 826D1DE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1DEC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1DF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1DF8: 386AAC4C  addi r3, r10, -0x53b4
	ctx.r[3].s64 = ctx.r[10].s64 + -21428;
	// 826D1DFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1E00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1E04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1E08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1E0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1E10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1E14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1E18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1E1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1E20: 4BD95001  bl 0x82466e20
	ctx.lr = 0x826D1E24;
	sub_82466E20(ctx, base);
	// 826D1E24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1E38 size=112
    let mut pc: u32 = 0x826D1E38;
    'dispatch: loop {
        match pc {
            0x826D1E38 => {
    //   block [0x826D1E38..0x826D1EA8)
	// 826D1E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1E40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1E44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1E48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1E4C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1E50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1E54: 390BE610  addi r8, r11, -0x19f0
	ctx.r[8].s64 = ctx.r[11].s64 + -6640;
	// 826D1E58: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D1E5C: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826D1E60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1E64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1E68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1E70: 386AAC7C  addi r3, r10, -0x5384
	ctx.r[3].s64 = ctx.r[10].s64 + -21380;
	// 826D1E74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1E78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1E80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1E84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1E88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1E90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1E94: 4BD94F8D  bl 0x82466e20
	ctx.lr = 0x826D1E98;
	sub_82466E20(ctx, base);
	// 826D1E98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1EA8 size=108
    let mut pc: u32 = 0x826D1EA8;
    'dispatch: loop {
        match pc {
            0x826D1EA8 => {
    //   block [0x826D1EA8..0x826D1F14)
	// 826D1EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1EB4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1EB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1EBC: 38EBE628  addi r7, r11, -0x19d8
	ctx.r[7].s64 = ctx.r[11].s64 + -6616;
	// 826D1EC0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D1EC4: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826D1EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1ED8: 386AACAC  addi r3, r10, -0x5354
	ctx.r[3].s64 = ctx.r[10].s64 + -21332;
	// 826D1EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1F00: 4BD94F21  bl 0x82466e20
	ctx.lr = 0x826D1F04;
	sub_82466E20(ctx, base);
	// 826D1F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1F18 size=112
    let mut pc: u32 = 0x826D1F18;
    'dispatch: loop {
        match pc {
            0x826D1F18 => {
    //   block [0x826D1F18..0x826D1F88)
	// 826D1F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1F28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1F2C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D1F30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1F34: 390BE700  addi r8, r11, -0x1900
	ctx.r[8].s64 = ctx.r[11].s64 + -6400;
	// 826D1F38: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 826D1F3C: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826D1F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1F44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D1F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1F50: 386AACDC  addi r3, r10, -0x5324
	ctx.r[3].s64 = ctx.r[10].s64 + -21284;
	// 826D1F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D1F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1F74: 4BD94EAD  bl 0x82466e20
	ctx.lr = 0x826D1F78;
	sub_82466E20(ctx, base);
	// 826D1F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1F88 size=108
    let mut pc: u32 = 0x826D1F88;
    'dispatch: loop {
        match pc {
            0x826D1F88 => {
    //   block [0x826D1F88..0x826D1FF4)
	// 826D1F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D1F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D1F94: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D1F98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D1F9C: 38EBE8B0  addi r7, r11, -0x1750
	ctx.r[7].s64 = ctx.r[11].s64 + -5968;
	// 826D1FA0: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826D1FA4: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826D1FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D1FAC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D1FB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D1FB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D1FB8: 386AAD0C  addi r3, r10, -0x52f4
	ctx.r[3].s64 = ctx.r[10].s64 + -21236;
	// 826D1FBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D1FC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D1FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D1FC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D1FCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D1FD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D1FD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D1FD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D1FDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D1FE0: 4BD94E41  bl 0x82466e20
	ctx.lr = 0x826D1FE4;
	sub_82466E20(ctx, base);
	// 826D1FE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D1FE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D1FEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D1FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D1FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D1FF8 size=112
    let mut pc: u32 = 0x826D1FF8;
    'dispatch: loop {
        match pc {
            0x826D1FF8 => {
    //   block [0x826D1FF8..0x826D2068)
	// 826D1FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D1FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2004: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2008: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D200C: 38AA9BFC  addi r5, r10, -0x6404
	ctx.r[5].s64 = ctx.r[10].s64 + -25604;
	// 826D2010: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2014: 390BEA48  addi r8, r11, -0x15b8
	ctx.r[8].s64 = ctx.r[11].s64 + -5560;
	// 826D2018: 3920001A  li r9, 0x1a
	ctx.r[9].s64 = 26;
	// 826D201C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 826D2020: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2024: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2028: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D202C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2030: 386AAD3C  addi r3, r10, -0x52c4
	ctx.r[3].s64 = ctx.r[10].s64 + -21188;
	// 826D2034: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2038: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D203C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2040: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2044: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2048: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D204C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2050: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2054: 4BD94DCD  bl 0x82466e20
	ctx.lr = 0x826D2058;
	sub_82466E20(ctx, base);
	// 826D2058: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D205C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2068 size=100
    let mut pc: u32 = 0x826D2068;
    'dispatch: loop {
        match pc {
            0x826D2068 => {
    //   block [0x826D2068..0x826D20CC)
	// 826D2068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2074: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D207C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2080: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2088: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826D208C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2090: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2094: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2098: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D209C: 386AAD6C  addi r3, r10, -0x5294
	ctx.r[3].s64 = ctx.r[10].s64 + -21140;
	// 826D20A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D20A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D20A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D20AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D20B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D20B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D20B8: 4BD94D69  bl 0x82466e20
	ctx.lr = 0x826D20BC;
	sub_82466E20(ctx, base);
	// 826D20BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D20C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D20C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D20C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D20D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D20D0 size=112
    let mut pc: u32 = 0x826D20D0;
    'dispatch: loop {
        match pc {
            0x826D20D0 => {
    //   block [0x826D20D0..0x826D2140)
	// 826D20D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D20D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D20D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D20DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D20E0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D20E4: 38AAAD6C  addi r5, r10, -0x5294
	ctx.r[5].s64 = ctx.r[10].s64 + -21140;
	// 826D20E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D20EC: 390BECB8  addi r8, r11, -0x1348
	ctx.r[8].s64 = ctx.r[11].s64 + -4936;
	// 826D20F0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826D20F4: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 826D20F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D20FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2100: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2104: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2108: 386AAD9C  addi r3, r10, -0x5264
	ctx.r[3].s64 = ctx.r[10].s64 + -21092;
	// 826D210C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2110: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2114: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2118: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D211C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2120: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2124: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2128: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D212C: 4BD94CF5  bl 0x82466e20
	ctx.lr = 0x826D2130;
	sub_82466E20(ctx, base);
	// 826D2130: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D213C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2140 size=100
    let mut pc: u32 = 0x826D2140;
    'dispatch: loop {
        match pc {
            0x826D2140 => {
    //   block [0x826D2140..0x826D21A4)
	// 826D2140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D214C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2150: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2154: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2158: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D215C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2160: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826D2164: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D216C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2174: 386AADCC  addi r3, r10, -0x5234
	ctx.r[3].s64 = ctx.r[10].s64 + -21044;
	// 826D2178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D217C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2180: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D2184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2188: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D218C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2190: 4BD94C91  bl 0x82466e20
	ctx.lr = 0x826D2194;
	sub_82466E20(ctx, base);
	// 826D2194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D219C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D21A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D21A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D21A8 size=108
    let mut pc: u32 = 0x826D21A8;
    'dispatch: loop {
        match pc {
            0x826D21A8 => {
    //   block [0x826D21A8..0x826D2214)
	// 826D21A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D21AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D21B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D21B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D21B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D21BC: 38EBED30  addi r7, r11, -0x12d0
	ctx.r[7].s64 = ctx.r[11].s64 + -4816;
	// 826D21C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D21C4: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826D21C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D21CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D21D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D21D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D21D8: 386AADFC  addi r3, r10, -0x5204
	ctx.r[3].s64 = ctx.r[10].s64 + -20996;
	// 826D21DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D21E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D21E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D21E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D21EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D21F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D21F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D21F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D21FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2200: 4BD94C21  bl 0x82466e20
	ctx.lr = 0x826D2204;
	sub_82466E20(ctx, base);
	// 826D2204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D220C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2218 size=112
    let mut pc: u32 = 0x826D2218;
    'dispatch: loop {
        match pc {
            0x826D2218 => {
    //   block [0x826D2218..0x826D2288)
	// 826D2218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2224: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2228: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D222C: 38AAADCC  addi r5, r10, -0x5234
	ctx.r[5].s64 = ctx.r[10].s64 + -21044;
	// 826D2230: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2234: 390BED78  addi r8, r11, -0x1288
	ctx.r[8].s64 = ctx.r[11].s64 + -4744;
	// 826D2238: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826D223C: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826D2240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2244: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D224C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2250: 386AAE2C  addi r3, r10, -0x51d4
	ctx.r[3].s64 = ctx.r[10].s64 + -20948;
	// 826D2254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D225C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D226C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2274: 4BD94BAD  bl 0x82466e20
	ctx.lr = 0x826D2278;
	sub_82466E20(ctx, base);
	// 826D2278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D227C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2288 size=100
    let mut pc: u32 = 0x826D2288;
    'dispatch: loop {
        match pc {
            0x826D2288 => {
    //   block [0x826D2288..0x826D22EC)
	// 826D2288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D228C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2294: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D229C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D22A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D22A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D22A8: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826D22AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D22B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D22B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D22B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D22BC: 386AAE5C  addi r3, r10, -0x51a4
	ctx.r[3].s64 = ctx.r[10].s64 + -20900;
	// 826D22C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D22C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D22C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D22CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D22D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D22D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D22D8: 4BD94B49  bl 0x82466e20
	ctx.lr = 0x826D22DC;
	sub_82466E20(ctx, base);
	// 826D22DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D22E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D22E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D22E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D22F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D22F0 size=100
    let mut pc: u32 = 0x826D22F0;
    'dispatch: loop {
        match pc {
            0x826D22F0 => {
    //   block [0x826D22F0..0x826D2354)
	// 826D22F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D22F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D22F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D22FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2304: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2308: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D230C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2310: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826D2314: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D231C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2324: 386AAE8C  addi r3, r10, -0x5174
	ctx.r[3].s64 = ctx.r[10].s64 + -20852;
	// 826D2328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D232C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2330: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D2334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2338: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D233C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2340: 4BD94AE1  bl 0x82466e20
	ctx.lr = 0x826D2344;
	sub_82466E20(ctx, base);
	// 826D2344: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D234C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2358 size=112
    let mut pc: u32 = 0x826D2358;
    'dispatch: loop {
        match pc {
            0x826D2358 => {
    //   block [0x826D2358..0x826D23C8)
	// 826D2358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D235C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2364: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2368: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D236C: 38AAAE5C  addi r5, r10, -0x51a4
	ctx.r[5].s64 = ctx.r[10].s64 + -20900;
	// 826D2370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2374: 390BEDA8  addi r8, r11, -0x1258
	ctx.r[8].s64 = ctx.r[11].s64 + -4696;
	// 826D2378: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D237C: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826D2380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2384: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D238C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2390: 386AAEBC  addi r3, r10, -0x5144
	ctx.r[3].s64 = ctx.r[10].s64 + -20804;
	// 826D2394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D239C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D23A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D23A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D23A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D23AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D23B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D23B4: 4BD94A6D  bl 0x82466e20
	ctx.lr = 0x826D23B8;
	sub_82466E20(ctx, base);
	// 826D23B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D23BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D23C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D23C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D23C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D23C8 size=112
    let mut pc: u32 = 0x826D23C8;
    'dispatch: loop {
        match pc {
            0x826D23C8 => {
    //   block [0x826D23C8..0x826D2438)
	// 826D23C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D23CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D23D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D23D4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D23D8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D23DC: 38AAAE8C  addi r5, r10, -0x5174
	ctx.r[5].s64 = ctx.r[10].s64 + -20852;
	// 826D23E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D23E4: 390BEE08  addi r8, r11, -0x11f8
	ctx.r[8].s64 = ctx.r[11].s64 + -4600;
	// 826D23E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D23EC: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 826D23F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D23F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D23F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D23FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2400: 386AAEEC  addi r3, r10, -0x5114
	ctx.r[3].s64 = ctx.r[10].s64 + -20756;
	// 826D2404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D240C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D241C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2424: 4BD949FD  bl 0x82466e20
	ctx.lr = 0x826D2428;
	sub_82466E20(ctx, base);
	// 826D2428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D242C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2438 size=100
    let mut pc: u32 = 0x826D2438;
    'dispatch: loop {
        match pc {
            0x826D2438 => {
    //   block [0x826D2438..0x826D249C)
	// 826D2438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D243C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2444: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D244C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2458: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826D245C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D246C: 386AAF1C  addi r3, r10, -0x50e4
	ctx.r[3].s64 = ctx.r[10].s64 + -20708;
	// 826D2470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2474: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2478: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D247C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2480: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D2484: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2488: 4BD94999  bl 0x82466e20
	ctx.lr = 0x826D248C;
	sub_82466E20(ctx, base);
	// 826D248C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D24A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D24A0 size=112
    let mut pc: u32 = 0x826D24A0;
    'dispatch: loop {
        match pc {
            0x826D24A0 => {
    //   block [0x826D24A0..0x826D2510)
	// 826D24A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D24A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D24A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D24AC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D24B0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D24B4: 38AAAF1C  addi r5, r10, -0x50e4
	ctx.r[5].s64 = ctx.r[10].s64 + -20708;
	// 826D24B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D24BC: 390BEE68  addi r8, r11, -0x1198
	ctx.r[8].s64 = ctx.r[11].s64 + -4504;
	// 826D24C0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826D24C4: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826D24C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D24CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D24D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D24D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D24D8: 386AAF4C  addi r3, r10, -0x50b4
	ctx.r[3].s64 = ctx.r[10].s64 + -20660;
	// 826D24DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D24E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D24E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D24E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D24EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D24F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D24F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D24F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D24FC: 4BD94925  bl 0x82466e20
	ctx.lr = 0x826D2500;
	sub_82466E20(ctx, base);
	// 826D2500: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D250C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2510 size=100
    let mut pc: u32 = 0x826D2510;
    'dispatch: loop {
        match pc {
            0x826D2510 => {
    //   block [0x826D2510..0x826D2574)
	// 826D2510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D251C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2524: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2528: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D252C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2530: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 826D2534: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D253C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2544: 386AAF7C  addi r3, r10, -0x5084
	ctx.r[3].s64 = ctx.r[10].s64 + -20612;
	// 826D2548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D254C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2550: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D2554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2558: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D255C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2560: 4BD948C1  bl 0x82466e20
	ctx.lr = 0x826D2564;
	sub_82466E20(ctx, base);
	// 826D2564: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D256C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2578 size=112
    let mut pc: u32 = 0x826D2578;
    'dispatch: loop {
        match pc {
            0x826D2578 => {
    //   block [0x826D2578..0x826D25E8)
	// 826D2578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D257C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2584: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2588: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D258C: 38AAAF7C  addi r5, r10, -0x5084
	ctx.r[5].s64 = ctx.r[10].s64 + -20612;
	// 826D2590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2594: 390BEF58  addi r8, r11, -0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + -4264;
	// 826D2598: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D259C: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826D25A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D25A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D25A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D25AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D25B0: 386AAFAC  addi r3, r10, -0x5054
	ctx.r[3].s64 = ctx.r[10].s64 + -20564;
	// 826D25B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D25B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D25BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D25C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D25C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D25C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D25CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D25D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D25D4: 4BD9484D  bl 0x82466e20
	ctx.lr = 0x826D25D8;
	sub_82466E20(ctx, base);
	// 826D25D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D25DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D25E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D25E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D25E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D25E8 size=108
    let mut pc: u32 = 0x826D25E8;
    'dispatch: loop {
        match pc {
            0x826D25E8 => {
    //   block [0x826D25E8..0x826D2654)
	// 826D25E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D25EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D25F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D25F4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D25F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D25FC: 38EBEFA0  addi r7, r11, -0x1060
	ctx.r[7].s64 = ctx.r[11].s64 + -4192;
	// 826D2600: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D2604: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 826D2608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D260C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2610: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2618: 386AAFDC  addi r3, r10, -0x5024
	ctx.r[3].s64 = ctx.r[10].s64 + -20516;
	// 826D261C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2628: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D262C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2630: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2634: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D263C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2640: 4BD947E1  bl 0x82466e20
	ctx.lr = 0x826D2644;
	sub_82466E20(ctx, base);
	// 826D2644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D264C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2658 size=112
    let mut pc: u32 = 0x826D2658;
    'dispatch: loop {
        match pc {
            0x826D2658 => {
    //   block [0x826D2658..0x826D26C8)
	// 826D2658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D265C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2664: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2668: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D266C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2674: 390BEFE8  addi r8, r11, -0x1018
	ctx.r[8].s64 = ctx.r[11].s64 + -4120;
	// 826D2678: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D267C: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826D2680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2684: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D268C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2690: 386AB00C  addi r3, r10, -0x4ff4
	ctx.r[3].s64 = ctx.r[10].s64 + -20468;
	// 826D2694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D269C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D26A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D26A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D26A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D26AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D26B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D26B4: 4BD9476D  bl 0x82466e20
	ctx.lr = 0x826D26B8;
	sub_82466E20(ctx, base);
	// 826D26B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D26BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D26C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D26C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D26C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D26C8 size=108
    let mut pc: u32 = 0x826D26C8;
    'dispatch: loop {
        match pc {
            0x826D26C8 => {
    //   block [0x826D26C8..0x826D2734)
	// 826D26C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D26CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D26D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D26D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D26D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D26DC: 38EBF000  addi r7, r11, -0x1000
	ctx.r[7].s64 = ctx.r[11].s64 + -4096;
	// 826D26E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D26E4: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826D26E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D26EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D26F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D26F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D26F8: 386AB03C  addi r3, r10, -0x4fc4
	ctx.r[3].s64 = ctx.r[10].s64 + -20420;
	// 826D26FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D270C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D271C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2720: 4BD94701  bl 0x82466e20
	ctx.lr = 0x826D2724;
	sub_82466E20(ctx, base);
	// 826D2724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D272C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2738 size=112
    let mut pc: u32 = 0x826D2738;
    'dispatch: loop {
        match pc {
            0x826D2738 => {
    //   block [0x826D2738..0x826D27A8)
	// 826D2738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D273C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2744: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2748: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D274C: 38AAB00C  addi r5, r10, -0x4ff4
	ctx.r[5].s64 = ctx.r[10].s64 + -20468;
	// 826D2750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2754: 390BF048  addi r8, r11, -0xfb8
	ctx.r[8].s64 = ctx.r[11].s64 + -4024;
	// 826D2758: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D275C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826D2760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2764: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D276C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2770: 386AB06C  addi r3, r10, -0x4f94
	ctx.r[3].s64 = ctx.r[10].s64 + -20372;
	// 826D2774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D277C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2794: 4BD9468D  bl 0x82466e20
	ctx.lr = 0x826D2798;
	sub_82466E20(ctx, base);
	// 826D2798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D279C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D27A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D27A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D27A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D27A8 size=100
    let mut pc: u32 = 0x826D27A8;
    'dispatch: loop {
        match pc {
            0x826D27A8 => {
    //   block [0x826D27A8..0x826D280C)
	// 826D27A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D27AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D27B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D27B4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D27B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D27BC: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D27C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D27C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D27C8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826D27CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D27D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D27D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D27D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D27DC: 386AB09C  addi r3, r10, -0x4f64
	ctx.r[3].s64 = ctx.r[10].s64 + -20324;
	// 826D27E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D27E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D27E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D27EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D27F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D27F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D27F8: 4BD94629  bl 0x82466e20
	ctx.lr = 0x826D27FC;
	sub_82466E20(ctx, base);
	// 826D27FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2810 size=112
    let mut pc: u32 = 0x826D2810;
    'dispatch: loop {
        match pc {
            0x826D2810 => {
    //   block [0x826D2810..0x826D2880)
	// 826D2810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D281C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2820: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2824: 38AAB09C  addi r5, r10, -0x4f64
	ctx.r[5].s64 = ctx.r[10].s64 + -20324;
	// 826D2828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D282C: 390BF060  addi r8, r11, -0xfa0
	ctx.r[8].s64 = ctx.r[11].s64 + -4000;
	// 826D2830: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D2834: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 826D2838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D283C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2848: 386AB0CC  addi r3, r10, -0x4f34
	ctx.r[3].s64 = ctx.r[10].s64 + -20276;
	// 826D284C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D285C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D286C: 4BD945B5  bl 0x82466e20
	ctx.lr = 0x826D2870;
	sub_82466E20(ctx, base);
	// 826D2870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D287C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2880 size=108
    let mut pc: u32 = 0x826D2880;
    'dispatch: loop {
        match pc {
            0x826D2880 => {
    //   block [0x826D2880..0x826D28EC)
	// 826D2880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D288C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2894: 38EBF108  addi r7, r11, -0xef8
	ctx.r[7].s64 = ctx.r[11].s64 + -3832;
	// 826D2898: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D289C: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826D28A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D28A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D28A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D28AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D28B0: 386AB0FC  addi r3, r10, -0x4f04
	ctx.r[3].s64 = ctx.r[10].s64 + -20228;
	// 826D28B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D28B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D28BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D28C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D28C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D28C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D28CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D28D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D28D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D28D8: 4BD94549  bl 0x82466e20
	ctx.lr = 0x826D28DC;
	sub_82466E20(ctx, base);
	// 826D28DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D28E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D28E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D28E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D28F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D28F0 size=112
    let mut pc: u32 = 0x826D28F0;
    'dispatch: loop {
        match pc {
            0x826D28F0 => {
    //   block [0x826D28F0..0x826D2960)
	// 826D28F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D28F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D28F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D28FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2900: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2904: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D290C: 390BF138  addi r8, r11, -0xec8
	ctx.r[8].s64 = ctx.r[11].s64 + -3784;
	// 826D2910: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D2914: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 826D2918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D291C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2928: 386AB12C  addi r3, r10, -0x4ed4
	ctx.r[3].s64 = ctx.r[10].s64 + -20180;
	// 826D292C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D293C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D294C: 4BD944D5  bl 0x82466e20
	ctx.lr = 0x826D2950;
	sub_82466E20(ctx, base);
	// 826D2950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D295C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2960 size=112
    let mut pc: u32 = 0x826D2960;
    'dispatch: loop {
        match pc {
            0x826D2960 => {
    //   block [0x826D2960..0x826D29D0)
	// 826D2960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D296C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2970: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2974: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D297C: 390BF180  addi r8, r11, -0xe80
	ctx.r[8].s64 = ctx.r[11].s64 + -3712;
	// 826D2980: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D2984: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826D2988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D298C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2998: 386AB15C  addi r3, r10, -0x4ea4
	ctx.r[3].s64 = ctx.r[10].s64 + -20132;
	// 826D299C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D29A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D29A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D29A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D29AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D29B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D29B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D29B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D29BC: 4BD94465  bl 0x82466e20
	ctx.lr = 0x826D29C0;
	sub_82466E20(ctx, base);
	// 826D29C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D29C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D29C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D29CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D29D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D29D0 size=100
    let mut pc: u32 = 0x826D29D0;
    'dispatch: loop {
        match pc {
            0x826D29D0 => {
    //   block [0x826D29D0..0x826D2A34)
	// 826D29D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D29D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D29D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D29DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D29E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D29E4: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D29E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D29EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D29F0: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 826D29F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D29F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D29FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2A00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2A04: 386AB18C  addi r3, r10, -0x4e74
	ctx.r[3].s64 = ctx.r[10].s64 + -20084;
	// 826D2A08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2A0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2A10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D2A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2A18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D2A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2A20: 4BD94401  bl 0x82466e20
	ctx.lr = 0x826D2A24;
	sub_82466E20(ctx, base);
	// 826D2A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2A38 size=112
    let mut pc: u32 = 0x826D2A38;
    'dispatch: loop {
        match pc {
            0x826D2A38 => {
    //   block [0x826D2A38..0x826D2AA8)
	// 826D2A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2A44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2A48: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2A4C: 38AAB18C  addi r5, r10, -0x4e74
	ctx.r[5].s64 = ctx.r[10].s64 + -20084;
	// 826D2A50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2A54: 390BF1C8  addi r8, r11, -0xe38
	ctx.r[8].s64 = ctx.r[11].s64 + -3640;
	// 826D2A58: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D2A5C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826D2A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2A64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2A68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2A6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2A70: 386AB1BC  addi r3, r10, -0x4e44
	ctx.r[3].s64 = ctx.r[10].s64 + -20036;
	// 826D2A74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2A7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2A94: 4BD9438D  bl 0x82466e20
	ctx.lr = 0x826D2A98;
	sub_82466E20(ctx, base);
	// 826D2A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2AA8 size=112
    let mut pc: u32 = 0x826D2AA8;
    'dispatch: loop {
        match pc {
            0x826D2AA8 => {
    //   block [0x826D2AA8..0x826D2B18)
	// 826D2AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2AB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2AB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2ABC: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2AC4: 390BF210  addi r8, r11, -0xdf0
	ctx.r[8].s64 = ctx.r[11].s64 + -3568;
	// 826D2AC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D2ACC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826D2AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2AD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2AE0: 386AB1EC  addi r3, r10, -0x4e14
	ctx.r[3].s64 = ctx.r[10].s64 + -19988;
	// 826D2AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2B04: 4BD9431D  bl 0x82466e20
	ctx.lr = 0x826D2B08;
	sub_82466E20(ctx, base);
	// 826D2B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2B18 size=112
    let mut pc: u32 = 0x826D2B18;
    'dispatch: loop {
        match pc {
            0x826D2B18 => {
    //   block [0x826D2B18..0x826D2B88)
	// 826D2B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2B24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2B28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2B2C: 38AA86FC  addi r5, r10, -0x7904
	ctx.r[5].s64 = ctx.r[10].s64 + -30980;
	// 826D2B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2B34: 390BF228  addi r8, r11, -0xdd8
	ctx.r[8].s64 = ctx.r[11].s64 + -3544;
	// 826D2B38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D2B3C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826D2B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2B44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2B50: 386AB21C  addi r3, r10, -0x4de4
	ctx.r[3].s64 = ctx.r[10].s64 + -19940;
	// 826D2B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2B64: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D2B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2B74: 4BD942AD  bl 0x82466e20
	ctx.lr = 0x826D2B78;
	sub_82466E20(ctx, base);
	// 826D2B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2B88 size=112
    let mut pc: u32 = 0x826D2B88;
    'dispatch: loop {
        match pc {
            0x826D2B88 => {
    //   block [0x826D2B88..0x826D2BF8)
	// 826D2B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2B94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2B98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2B9C: 38AAB1EC  addi r5, r10, -0x4e14
	ctx.r[5].s64 = ctx.r[10].s64 + -19988;
	// 826D2BA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D2BA4: 390BF240  addi r8, r11, -0xdc0
	ctx.r[8].s64 = ctx.r[11].s64 + -3520;
	// 826D2BA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D2BAC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826D2BB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2BB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2BB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2BBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2BC0: 386AB24C  addi r3, r10, -0x4db4
	ctx.r[3].s64 = ctx.r[10].s64 + -19892;
	// 826D2BC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2BCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2BD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2BD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2BD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2BE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2BE4: 4BD9423D  bl 0x82466e20
	ctx.lr = 0x826D2BE8;
	sub_82466E20(ctx, base);
	// 826D2BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2BEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2BF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2BF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2BF8 size=72
    let mut pc: u32 = 0x826D2BF8;
    'dispatch: loop {
        match pc {
            0x826D2BF8 => {
    //   block [0x826D2BF8..0x826D2C40)
	// 826D2BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2C00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2C04: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D2C08: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 826D2C0C: 38CB32B0  addi r6, r11, 0x32b0
	ctx.r[6].s64 = ctx.r[11].s64 + 12976;
	// 826D2C10: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D2C14: 388B3BA8  addi r4, r11, 0x3ba8
	ctx.r[4].s64 = ctx.r[11].s64 + 15272;
	// 826D2C18: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D2C1C: 386BB27C  addi r3, r11, -0x4d84
	ctx.r[3].s64 = ctx.r[11].s64 + -19844;
	// 826D2C20: 4BDA8E69  bl 0x8247ba88
	ctx.lr = 0x826D2C24;
	sub_8247BA88(ctx, base);
	// 826D2C24: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826D2C28: 386BCF00  addi r3, r11, -0x3100
	ctx.r[3].s64 = ctx.r[11].s64 + -12544;
	// 826D2C2C: 4BE5FF0D  bl 0x82532b38
	ctx.lr = 0x826D2C30;
	sub_82532B38(ctx, base);
	// 826D2C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826D2C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2C40 size=108
    let mut pc: u32 = 0x826D2C40;
    'dispatch: loop {
        match pc {
            0x826D2C40 => {
    //   block [0x826D2C40..0x826D2CAC)
	// 826D2C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2C4C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2C50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2C54: 38EBFB10  addi r7, r11, -0x4f0
	ctx.r[7].s64 = ctx.r[11].s64 + -1264;
	// 826D2C58: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826D2C5C: 388AA590  addi r4, r10, -0x5a70
	ctx.r[4].s64 = ctx.r[10].s64 + -23152;
	// 826D2C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2C64: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2C68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2C70: 386AB294  addi r3, r10, -0x4d6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19820;
	// 826D2C74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2C94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2C98: 4BD94189  bl 0x82466e20
	ctx.lr = 0x826D2C9C;
	sub_82466E20(ctx, base);
	// 826D2C9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D2CB0 size=24
    let mut pc: u32 = 0x826D2CB0;
    'dispatch: loop {
        match pc {
            0x826D2CB0 => {
    //   block [0x826D2CB0..0x826D2CC8)
	// 826D2CB0: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2CB4: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D2CB8: 394A91B8  addi r10, r10, -0x6e48
	ctx.r[10].s64 = ctx.r[10].s64 + -28232;
	// 826D2CBC: 816BFB88  lwz r11, -0x478(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1144 as u32) ) } as u64;
	// 826D2CC0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826D2CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2CC8 size=112
    let mut pc: u32 = 0x826D2CC8;
    'dispatch: loop {
        match pc {
            0x826D2CC8 => {
    //   block [0x826D2CC8..0x826D2D38)
	// 826D2CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2CD4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D2CD8: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D2CDC: 392A40DC  addi r9, r10, 0x40dc
	ctx.r[9].s64 = ctx.r[10].s64 + 16604;
	// 826D2CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2CE4: 390B91B8  addi r8, r11, -0x6e48
	ctx.r[8].s64 = ctx.r[11].s64 + -28232;
	// 826D2CE8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826D2CEC: 388AA5A8  addi r4, r10, -0x5a58
	ctx.r[4].s64 = ctx.r[10].s64 + -23128;
	// 826D2CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2CF4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2D00: 386AB2C4  addi r3, r10, -0x4d3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19772;
	// 826D2D04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D2D08: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D2D0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2D1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2D24: 4BD940FD  bl 0x82466e20
	ctx.lr = 0x826D2D28;
	sub_82466E20(ctx, base);
	// 826D2D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2D38 size=108
    let mut pc: u32 = 0x826D2D38;
    'dispatch: loop {
        match pc {
            0x826D2D38 => {
    //   block [0x826D2D38..0x826D2DA4)
	// 826D2D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2D44: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2D48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2D4C: 38EBFB8C  addi r7, r11, -0x474
	ctx.r[7].s64 = ctx.r[11].s64 + -1140;
	// 826D2D50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D2D54: 388AA5BC  addi r4, r10, -0x5a44
	ctx.r[4].s64 = ctx.r[10].s64 + -23108;
	// 826D2D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2D5C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2D60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2D68: 386AB2F4  addi r3, r10, -0x4d0c
	ctx.r[3].s64 = ctx.r[10].s64 + -19724;
	// 826D2D6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2D70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2D78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2D7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2D80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2D84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2D88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2D8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2D90: 4BD94091  bl 0x82466e20
	ctx.lr = 0x826D2D94;
	sub_82466E20(ctx, base);
	// 826D2D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2D98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2D9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2DA8 size=108
    let mut pc: u32 = 0x826D2DA8;
    'dispatch: loop {
        match pc {
            0x826D2DA8 => {
    //   block [0x826D2DA8..0x826D2E14)
	// 826D2DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2DB4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2DB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2DBC: 38EBFBBC  addi r7, r11, -0x444
	ctx.r[7].s64 = ctx.r[11].s64 + -1092;
	// 826D2DC0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D2DC4: 388AA5DC  addi r4, r10, -0x5a24
	ctx.r[4].s64 = ctx.r[10].s64 + -23076;
	// 826D2DC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2DCC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2DD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2DD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2DD8: 386AB324  addi r3, r10, -0x4cdc
	ctx.r[3].s64 = ctx.r[10].s64 + -19676;
	// 826D2DDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2DE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2DE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2DE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2DEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2DF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2DF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2DF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2E00: 4BD94021  bl 0x82466e20
	ctx.lr = 0x826D2E04;
	sub_82466E20(ctx, base);
	// 826D2E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D2E18 size=24
    let mut pc: u32 = 0x826D2E18;
    'dispatch: loop {
        match pc {
            0x826D2E18 => {
    //   block [0x826D2E18..0x826D2E30)
	// 826D2E18: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2E1C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D2E20: 394A9200  addi r10, r10, -0x6e00
	ctx.r[10].s64 = ctx.r[10].s64 + -28160;
	// 826D2E24: 816BFBEC  lwz r11, -0x414(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-1044 as u32) ) } as u64;
	// 826D2E28: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D2E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2E30 size=116
    let mut pc: u32 = 0x826D2E30;
    'dispatch: loop {
        match pc {
            0x826D2E30 => {
    //   block [0x826D2E30..0x826D2EA4)
	// 826D2E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2E34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2E3C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D2E40: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D2E44: 390B9200  addi r8, r11, -0x6e00
	ctx.r[8].s64 = ctx.r[11].s64 + -28160;
	// 826D2E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2E4C: 392A4118  addi r9, r10, 0x4118
	ctx.r[9].s64 = ctx.r[10].s64 + 16664;
	// 826D2E50: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2E54: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826D2E58: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D2E5C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2E64: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2E6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2E74: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D2E78: 388AA5F0  addi r4, r10, -0x5a10
	ctx.r[4].s64 = ctx.r[10].s64 + -23056;
	// 826D2E7C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D2E80: 386BB354  addi r3, r11, -0x4cac
	ctx.r[3].s64 = ctx.r[11].s64 + -19628;
	// 826D2E84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D2E88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2E8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2E90: 4BD93F91  bl 0x82466e20
	ctx.lr = 0x826D2E94;
	sub_82466E20(ctx, base);
	// 826D2E94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2E98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2E9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2EA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2EA8 size=108
    let mut pc: u32 = 0x826D2EA8;
    'dispatch: loop {
        match pc {
            0x826D2EA8 => {
    //   block [0x826D2EA8..0x826D2F14)
	// 826D2EA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2EAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2EB4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2EB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2EBC: 38EBFBF0  addi r7, r11, -0x410
	ctx.r[7].s64 = ctx.r[11].s64 + -1040;
	// 826D2EC0: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D2EC4: 388AA608  addi r4, r10, -0x59f8
	ctx.r[4].s64 = ctx.r[10].s64 + -23032;
	// 826D2EC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2ECC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2ED0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D2ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2ED8: 386AB384  addi r3, r10, -0x4c7c
	ctx.r[3].s64 = ctx.r[10].s64 + -19580;
	// 826D2EDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D2EE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2EE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2EE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2EEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2EF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2EF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2EF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2EFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D2F00: 4BD93F21  bl 0x82466e20
	ctx.lr = 0x826D2F04;
	sub_82466E20(ctx, base);
	// 826D2F04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2F08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2F0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2F10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2F18 size=112
    let mut pc: u32 = 0x826D2F18;
    'dispatch: loop {
        match pc {
            0x826D2F18 => {
    //   block [0x826D2F18..0x826D2F88)
	// 826D2F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2F20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2F24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2F28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2F2C: 38AAB354  addi r5, r10, -0x4cac
	ctx.r[5].s64 = ctx.r[10].s64 + -19628;
	// 826D2F30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2F34: 390BFC80  addi r8, r11, -0x380
	ctx.r[8].s64 = ctx.r[11].s64 + -896;
	// 826D2F38: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826D2F3C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826D2F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2F44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2F48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2F50: 386AB3B4  addi r3, r10, -0x4c4c
	ctx.r[3].s64 = ctx.r[10].s64 + -19532;
	// 826D2F54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2F58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2F5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2F60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2F64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2F68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2F6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2F70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2F74: 4BD93EAD  bl 0x82466e20
	ctx.lr = 0x826D2F78;
	sub_82466E20(ctx, base);
	// 826D2F78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2F7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2F80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2F88 size=112
    let mut pc: u32 = 0x826D2F88;
    'dispatch: loop {
        match pc {
            0x826D2F88 => {
    //   block [0x826D2F88..0x826D2FF8)
	// 826D2F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D2F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D2F94: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2F98: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D2F9C: 38AAB354  addi r5, r10, -0x4cac
	ctx.r[5].s64 = ctx.r[10].s64 + -19628;
	// 826D2FA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D2FA4: 390BFDA0  addi r8, r11, -0x260
	ctx.r[8].s64 = ctx.r[11].s64 + -608;
	// 826D2FA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826D2FAC: 388AA664  addi r4, r10, -0x599c
	ctx.r[4].s64 = ctx.r[10].s64 + -22940;
	// 826D2FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D2FB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D2FB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D2FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D2FC0: 386AB3E4  addi r3, r10, -0x4c1c
	ctx.r[3].s64 = ctx.r[10].s64 + -19484;
	// 826D2FC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D2FC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D2FCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D2FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D2FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D2FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D2FDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D2FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D2FE4: 4BD93E3D  bl 0x82466e20
	ctx.lr = 0x826D2FE8;
	sub_82466E20(ctx, base);
	// 826D2FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D2FEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D2FF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D2FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D2FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D2FF8 size=108
    let mut pc: u32 = 0x826D2FF8;
    'dispatch: loop {
        match pc {
            0x826D2FF8 => {
    //   block [0x826D2FF8..0x826D3064)
	// 826D2FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D2FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3004: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D300C: 38EBFDB8  addi r7, r11, -0x248
	ctx.r[7].s64 = ctx.r[11].s64 + -584;
	// 826D3010: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826D3014: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826D3018: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D301C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3020: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3028: 386AB414  addi r3, r10, -0x4bec
	ctx.r[3].s64 = ctx.r[10].s64 + -19436;
	// 826D302C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3030: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3038: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D303C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3040: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3048: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D304C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3050: 4BD93DD1  bl 0x82466e20
	ctx.lr = 0x826D3054;
	sub_82466E20(ctx, base);
	// 826D3054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3058: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D305C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3068 size=108
    let mut pc: u32 = 0x826D3068;
    'dispatch: loop {
        match pc {
            0x826D3068 => {
    //   block [0x826D3068..0x826D30D4)
	// 826D3068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D306C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3074: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D307C: 38EBFE90  addi r7, r11, -0x170
	ctx.r[7].s64 = ctx.r[11].s64 + -368;
	// 826D3080: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D3084: 388AA6B4  addi r4, r10, -0x594c
	ctx.r[4].s64 = ctx.r[10].s64 + -22860;
	// 826D3088: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D308C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3090: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3094: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3098: 386AB444  addi r3, r10, -0x4bbc
	ctx.r[3].s64 = ctx.r[10].s64 + -19388;
	// 826D309C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D30A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D30A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D30A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D30AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D30B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D30B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D30B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D30BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D30C0: 4BD93D61  bl 0x82466e20
	ctx.lr = 0x826D30C4;
	sub_82466E20(ctx, base);
	// 826D30C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D30C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D30CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D30D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D30D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D30D8 size=112
    let mut pc: u32 = 0x826D30D8;
    'dispatch: loop {
        match pc {
            0x826D30D8 => {
    //   block [0x826D30D8..0x826D3148)
	// 826D30D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D30DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D30E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D30E4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D30E8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D30EC: 38AAB354  addi r5, r10, -0x4cac
	ctx.r[5].s64 = ctx.r[10].s64 + -19628;
	// 826D30F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D30F4: 390BFF20  addi r8, r11, -0xe0
	ctx.r[8].s64 = ctx.r[11].s64 + -224;
	// 826D30F8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826D30FC: 388AA6E4  addi r4, r10, -0x591c
	ctx.r[4].s64 = ctx.r[10].s64 + -22812;
	// 826D3100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3104: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3108: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D310C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3110: 386AB474  addi r3, r10, -0x4b8c
	ctx.r[3].s64 = ctx.r[10].s64 + -19340;
	// 826D3114: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3118: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D311C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3120: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3124: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3128: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D312C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3130: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3134: 4BD93CED  bl 0x82466e20
	ctx.lr = 0x826D3138;
	sub_82466E20(ctx, base);
	// 826D3138: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D313C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3144: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3148 size=108
    let mut pc: u32 = 0x826D3148;
    'dispatch: loop {
        match pc {
            0x826D3148 => {
    //   block [0x826D3148..0x826D31B4)
	// 826D3148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D314C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3150: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3154: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3158: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D315C: 38EB0010  addi r7, r11, 0x10
	ctx.r[7].s64 = ctx.r[11].s64 + 16;
	// 826D3160: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826D3164: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 826D3168: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D316C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3170: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3174: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3178: 386AB4A4  addi r3, r10, -0x4b5c
	ctx.r[3].s64 = ctx.r[10].s64 + -19292;
	// 826D317C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3180: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3188: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D318C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3190: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3198: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D319C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D31A0: 4BD93C81  bl 0x82466e20
	ctx.lr = 0x826D31A4;
	sub_82466E20(ctx, base);
	// 826D31A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D31A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D31AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D31B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D31B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D31B8 size=108
    let mut pc: u32 = 0x826D31B8;
    'dispatch: loop {
        match pc {
            0x826D31B8 => {
    //   block [0x826D31B8..0x826D3224)
	// 826D31B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D31BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D31C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D31C4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D31C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D31CC: 38EB0028  addi r7, r11, 0x28
	ctx.r[7].s64 = ctx.r[11].s64 + 40;
	// 826D31D0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D31D4: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 826D31D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D31DC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D31E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D31E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D31E8: 386AB4D4  addi r3, r10, -0x4b2c
	ctx.r[3].s64 = ctx.r[10].s64 + -19244;
	// 826D31EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D31F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D31F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D31F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D31FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3200: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3208: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D320C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3210: 4BD93C11  bl 0x82466e20
	ctx.lr = 0x826D3214;
	sub_82466E20(ctx, base);
	// 826D3214: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D321C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3228 size=116
    let mut pc: u32 = 0x826D3228;
    'dispatch: loop {
        match pc {
            0x826D3228 => {
    //   block [0x826D3228..0x826D329C)
	// 826D3228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D322C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3230: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3234: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3238: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D323C: 390B008C  addi r8, r11, 0x8c
	ctx.r[8].s64 = ctx.r[11].s64 + 140;
	// 826D3240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3244: 392A4144  addi r9, r10, 0x4144
	ctx.r[9].s64 = ctx.r[10].s64 + 16708;
	// 826D3248: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D324C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826D3250: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3254: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D325C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D326C: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D3270: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 826D3274: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3278: 386BB504  addi r3, r11, -0x4afc
	ctx.r[3].s64 = ctx.r[11].s64 + -19196;
	// 826D327C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3280: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3284: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3288: 4BD93B99  bl 0x82466e20
	ctx.lr = 0x826D328C;
	sub_82466E20(ctx, base);
	// 826D328C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D32A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D32A0 size=108
    let mut pc: u32 = 0x826D32A0;
    'dispatch: loop {
        match pc {
            0x826D32A0 => {
    //   block [0x826D32A0..0x826D330C)
	// 826D32A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D32A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D32A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D32AC: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D32B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D32B4: 38EB00A8  addi r7, r11, 0xa8
	ctx.r[7].s64 = ctx.r[11].s64 + 168;
	// 826D32B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826D32BC: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 826D32C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D32C4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D32C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D32CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D32D0: 386AB534  addi r3, r10, -0x4acc
	ctx.r[3].s64 = ctx.r[10].s64 + -19148;
	// 826D32D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D32D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D32DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D32E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D32E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D32E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D32EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D32F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D32F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D32F8: 4BD93B29  bl 0x82466e20
	ctx.lr = 0x826D32FC;
	sub_82466E20(ctx, base);
	// 826D32FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3300: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3304: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3310 size=108
    let mut pc: u32 = 0x826D3310;
    'dispatch: loop {
        match pc {
            0x826D3310 => {
    //   block [0x826D3310..0x826D337C)
	// 826D3310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3318: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D331C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3320: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3324: 38EB00F0  addi r7, r11, 0xf0
	ctx.r[7].s64 = ctx.r[11].s64 + 240;
	// 826D3328: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D332C: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 826D3330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3334: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3338: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D333C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3340: 386AB564  addi r3, r10, -0x4a9c
	ctx.r[3].s64 = ctx.r[10].s64 + -19100;
	// 826D3344: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3348: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D334C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3350: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3358: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D335C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3360: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3364: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3368: 4BD93AB9  bl 0x82466e20
	ctx.lr = 0x826D336C;
	sub_82466E20(ctx, base);
	// 826D336C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3370: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3374: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3378: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3380 size=108
    let mut pc: u32 = 0x826D3380;
    'dispatch: loop {
        match pc {
            0x826D3380 => {
    //   block [0x826D3380..0x826D33EC)
	// 826D3380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3388: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D338C: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3390: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3394: 38EB0180  addi r7, r11, 0x180
	ctx.r[7].s64 = ctx.r[11].s64 + 384;
	// 826D3398: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826D339C: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 826D33A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D33A4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D33A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D33AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D33B0: 386AB594  addi r3, r10, -0x4a6c
	ctx.r[3].s64 = ctx.r[10].s64 + -19052;
	// 826D33B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D33B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D33BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D33C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D33C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D33C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D33CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D33D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D33D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D33D8: 4BD93A49  bl 0x82466e20
	ctx.lr = 0x826D33DC;
	sub_82466E20(ctx, base);
	// 826D33DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D33E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D33E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D33E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D33F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D33F0 size=100
    let mut pc: u32 = 0x826D33F0;
    'dispatch: loop {
        match pc {
            0x826D33F0 => {
    //   block [0x826D33F0..0x826D3454)
	// 826D33F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D33F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D33F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D33FC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3404: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3408: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D340C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3410: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 826D3414: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D341C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3424: 386AB5C4  addi r3, r10, -0x4a3c
	ctx.r[3].s64 = ctx.r[10].s64 + -19004;
	// 826D3428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D342C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3430: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D3434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3438: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D343C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3440: 4BD939E1  bl 0x82466e20
	ctx.lr = 0x826D3444;
	sub_82466E20(ctx, base);
	// 826D3444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D344C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3450: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3458 size=112
    let mut pc: u32 = 0x826D3458;
    'dispatch: loop {
        match pc {
            0x826D3458 => {
    //   block [0x826D3458..0x826D34C8)
	// 826D3458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D345C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3464: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3468: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D346C: 38AAB5C4  addi r5, r10, -0x4a3c
	ctx.r[5].s64 = ctx.r[10].s64 + -19004;
	// 826D3470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3474: 390B0210  addi r8, r11, 0x210
	ctx.r[8].s64 = ctx.r[11].s64 + 528;
	// 826D3478: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D347C: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 826D3480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3484: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D348C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3490: 386AB5F4  addi r3, r10, -0x4a0c
	ctx.r[3].s64 = ctx.r[10].s64 + -18956;
	// 826D3494: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3498: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D349C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D34A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D34A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D34A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D34AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D34B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D34B4: 4BD9396D  bl 0x82466e20
	ctx.lr = 0x826D34B8;
	sub_82466E20(ctx, base);
	// 826D34B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D34BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D34C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D34C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D34C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D34C8 size=108
    let mut pc: u32 = 0x826D34C8;
    'dispatch: loop {
        match pc {
            0x826D34C8 => {
    //   block [0x826D34C8..0x826D3534)
	// 826D34C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D34CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D34D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D34D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D34D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D34DC: 38EB0270  addi r7, r11, 0x270
	ctx.r[7].s64 = ctx.r[11].s64 + 624;
	// 826D34E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826D34E4: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 826D34E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D34EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D34F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D34F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D34F8: 386AB624  addi r3, r10, -0x49dc
	ctx.r[3].s64 = ctx.r[10].s64 + -18908;
	// 826D34FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3500: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D350C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D351C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3520: 4BD93901  bl 0x82466e20
	ctx.lr = 0x826D3524;
	sub_82466E20(ctx, base);
	// 826D3524: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D352C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3538 size=108
    let mut pc: u32 = 0x826D3538;
    'dispatch: loop {
        match pc {
            0x826D3538 => {
    //   block [0x826D3538..0x826D35A4)
	// 826D3538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3544: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D354C: 38EB02A0  addi r7, r11, 0x2a0
	ctx.r[7].s64 = ctx.r[11].s64 + 672;
	// 826D3550: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D3554: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 826D3558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D355C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3568: 386AB654  addi r3, r10, -0x49ac
	ctx.r[3].s64 = ctx.r[10].s64 + -18860;
	// 826D356C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D357C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D358C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3590: 4BD93891  bl 0x82466e20
	ctx.lr = 0x826D3594;
	sub_82466E20(ctx, base);
	// 826D3594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D359C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D35A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D35A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D35A8 size=108
    let mut pc: u32 = 0x826D35A8;
    'dispatch: loop {
        match pc {
            0x826D35A8 => {
    //   block [0x826D35A8..0x826D3614)
	// 826D35A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D35AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D35B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D35B4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D35B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D35BC: 38EB0300  addi r7, r11, 0x300
	ctx.r[7].s64 = ctx.r[11].s64 + 768;
	// 826D35C0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D35C4: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 826D35C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D35CC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D35D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D35D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D35D8: 386AB684  addi r3, r10, -0x497c
	ctx.r[3].s64 = ctx.r[10].s64 + -18812;
	// 826D35DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D35E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D35E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D35E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D35EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D35F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D35F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D35F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D35FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3600: 4BD93821  bl 0x82466e20
	ctx.lr = 0x826D3604;
	sub_82466E20(ctx, base);
	// 826D3604: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D360C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3618 size=112
    let mut pc: u32 = 0x826D3618;
    'dispatch: loop {
        match pc {
            0x826D3618 => {
    //   block [0x826D3618..0x826D3688)
	// 826D3618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D361C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3624: 3D40827F  lis r10, -0x7d81
	ctx.r[10].s64 = -2105606144;
	// 826D3628: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826D362C: 38EA0360  addi r7, r10, 0x360
	ctx.r[7].s64 = ctx.r[10].s64 + 864;
	// 826D3630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3634: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D3638: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 826D363C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3640: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3644: 396B4158  addi r11, r11, 0x4158
	ctx.r[11].s64 = ctx.r[11].s64 + 16728;
	// 826D3648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D364C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3650: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3654: 386AB6B4  addi r3, r10, -0x494c
	ctx.r[3].s64 = ctx.r[10].s64 + -18764;
	// 826D3658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D365C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826D3660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3664: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826D3668: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D366C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3670: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3674: 4BD937AD  bl 0x82466e20
	ctx.lr = 0x826D3678;
	sub_82466E20(ctx, base);
	// 826D3678: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D367C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3688 size=96
    let mut pc: u32 = 0x826D3688;
    'dispatch: loop {
        match pc {
            0x826D3688 => {
    //   block [0x826D3688..0x826D36E8)
	// 826D3688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D368C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3690: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3694: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D369C: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 826D36A0: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D36A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D36A8: 386AB6E4  addi r3, r10, -0x491c
	ctx.r[3].s64 = ctx.r[10].s64 + -18716;
	// 826D36AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D36B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D36B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D36B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D36BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D36C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D36C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D36C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826D36CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D36D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D36D4: 4BD9374D  bl 0x82466e20
	ctx.lr = 0x826D36D8;
	sub_82466E20(ctx, base);
	// 826D36D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D36DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D36E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D36E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D36E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D36E8 size=112
    let mut pc: u32 = 0x826D36E8;
    'dispatch: loop {
        match pc {
            0x826D36E8 => {
    //   block [0x826D36E8..0x826D3758)
	// 826D36E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D36EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D36F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D36F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D36F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D36FC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3700: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826D3704: 390B0480  addi r8, r11, 0x480
	ctx.r[8].s64 = ctx.r[11].s64 + 1152;
	// 826D3708: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826D370C: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 826D3710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3714: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D371C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3720: 386AB714  addi r3, r10, -0x48ec
	ctx.r[3].s64 = ctx.r[10].s64 + -18668;
	// 826D3724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D372C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D373C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3744: 4BD936DD  bl 0x82466e20
	ctx.lr = 0x826D3748;
	sub_82466E20(ctx, base);
	// 826D3748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D374C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D3758 size=24
    let mut pc: u32 = 0x826D3758;
    'dispatch: loop {
        match pc {
            0x826D3758 => {
    //   block [0x826D3758..0x826D3770)
	// 826D3758: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D375C: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D3760: 394A9278  addi r10, r10, -0x6d88
	ctx.r[10].s64 = ctx.r[10].s64 + -28040;
	// 826D3764: 816B00A4  lwz r11, 0xa4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 826D3768: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826D376C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3770 size=116
    let mut pc: u32 = 0x826D3770;
    'dispatch: loop {
        match pc {
            0x826D3770 => {
    //   block [0x826D3770..0x826D37E4)
	// 826D3770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D377C: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D3780: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D3784: 390B9278  addi r8, r11, -0x6d88
	ctx.r[8].s64 = ctx.r[11].s64 + -28040;
	// 826D3788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D378C: 392A41D4  addi r9, r10, 0x41d4
	ctx.r[9].s64 = ctx.r[10].s64 + 16852;
	// 826D3790: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3794: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826D3798: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D379C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D37A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D37A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D37A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D37AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D37B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D37B4: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D37B8: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 826D37BC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D37C0: 386BB744  addi r3, r11, -0x48bc
	ctx.r[3].s64 = ctx.r[11].s64 + -18620;
	// 826D37C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D37C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D37CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D37D0: 4BD93651  bl 0x82466e20
	ctx.lr = 0x826D37D4;
	sub_82466E20(ctx, base);
	// 826D37D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D37D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D37DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D37E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D37E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D37E8 size=112
    let mut pc: u32 = 0x826D37E8;
    'dispatch: loop {
        match pc {
            0x826D37E8 => {
    //   block [0x826D37E8..0x826D3858)
	// 826D37E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D37EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D37F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D37F4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D37F8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D37FC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3804: 390B04E0  addi r8, r11, 0x4e0
	ctx.r[8].s64 = ctx.r[11].s64 + 1248;
	// 826D3808: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D380C: 388A2DA4  addi r4, r10, 0x2da4
	ctx.r[4].s64 = ctx.r[10].s64 + 11684;
	// 826D3810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3814: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D381C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3820: 386AB774  addi r3, r10, -0x488c
	ctx.r[3].s64 = ctx.r[10].s64 + -18572;
	// 826D3824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D382C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D383C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3844: 4BD935DD  bl 0x82466e20
	ctx.lr = 0x826D3848;
	sub_82466E20(ctx, base);
	// 826D3848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D384C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3858 size=112
    let mut pc: u32 = 0x826D3858;
    'dispatch: loop {
        match pc {
            0x826D3858 => {
    //   block [0x826D3858..0x826D38C8)
	// 826D3858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D385C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3864: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3868: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D386C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3874: 390B0528  addi r8, r11, 0x528
	ctx.r[8].s64 = ctx.r[11].s64 + 1320;
	// 826D3878: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826D387C: 388A2DBC  addi r4, r10, 0x2dbc
	ctx.r[4].s64 = ctx.r[10].s64 + 11708;
	// 826D3880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3884: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D388C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3890: 386AB7A4  addi r3, r10, -0x485c
	ctx.r[3].s64 = ctx.r[10].s64 + -18524;
	// 826D3894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D389C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D38A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D38A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D38A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D38AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D38B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D38B4: 4BD9356D  bl 0x82466e20
	ctx.lr = 0x826D38B8;
	sub_82466E20(ctx, base);
	// 826D38B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D38BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D38C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D38C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D38C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826D38C8 size=24
    let mut pc: u32 = 0x826D38C8;
    'dispatch: loop {
        match pc {
            0x826D38C8 => {
    //   block [0x826D38C8..0x826D38E0)
	// 826D38C8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D38CC: 3D408280  lis r10, -0x7d80
	ctx.r[10].s64 = -2105540608;
	// 826D38D0: 394A9350  addi r10, r10, -0x6cb0
	ctx.r[10].s64 = ctx.r[10].s64 + -27824;
	// 826D38D4: 816B0570  lwz r11, 0x570(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1392 as u32) ) } as u64;
	// 826D38D8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826D38DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D38E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D38E0 size=116
    let mut pc: u32 = 0x826D38E0;
    'dispatch: loop {
        match pc {
            0x826D38E0 => {
    //   block [0x826D38E0..0x826D3954)
	// 826D38E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D38E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D38E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D38EC: 3D608280  lis r11, -0x7d80
	ctx.r[11].s64 = -2105540608;
	// 826D38F0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826D38F4: 390B9350  addi r8, r11, -0x6cb0
	ctx.r[8].s64 = ctx.r[11].s64 + -27824;
	// 826D38F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D38FC: 392A4200  addi r9, r10, 0x4200
	ctx.r[9].s64 = ctx.r[10].s64 + 16896;
	// 826D3900: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3904: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826D3908: 38AAB984  addi r5, r10, -0x467c
	ctx.r[5].s64 = ctx.r[10].s64 + -18044;
	// 826D390C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3914: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D391C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3924: 3D608299  lis r11, -0x7d67
	ctx.r[11].s64 = -2103902208;
	// 826D3928: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 826D392C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826D3930: 386BB7D4  addi r3, r11, -0x482c
	ctx.r[3].s64 = ctx.r[11].s64 + -18476;
	// 826D3934: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3938: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D393C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3940: 4BD934E1  bl 0x82466e20
	ctx.lr = 0x826D3944;
	sub_82466E20(ctx, base);
	// 826D3944: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D394C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3958 size=108
    let mut pc: u32 = 0x826D3958;
    'dispatch: loop {
        match pc {
            0x826D3958 => {
    //   block [0x826D3958..0x826D39C4)
	// 826D3958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D395C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3960: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3964: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D396C: 38EB0578  addi r7, r11, 0x578
	ctx.r[7].s64 = ctx.r[11].s64 + 1400;
	// 826D3970: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826D3974: 388A2DD8  addi r4, r10, 0x2dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 11736;
	// 826D3978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D397C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3980: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D3984: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3988: 386AB804  addi r3, r10, -0x47fc
	ctx.r[3].s64 = ctx.r[10].s64 + -18428;
	// 826D398C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3994: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D399C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D39A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D39A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D39A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D39AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D39B0: 4BD93471  bl 0x82466e20
	ctx.lr = 0x826D39B4;
	sub_82466E20(ctx, base);
	// 826D39B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D39B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D39BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D39C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D39C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D39C8 size=108
    let mut pc: u32 = 0x826D39C8;
    'dispatch: loop {
        match pc {
            0x826D39C8 => {
    //   block [0x826D39C8..0x826D3A34)
	// 826D39C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D39CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D39D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D39D4: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D39D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D39DC: 38EB05D8  addi r7, r11, 0x5d8
	ctx.r[7].s64 = ctx.r[11].s64 + 1496;
	// 826D39E0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826D39E4: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 826D39E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D39EC: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D39F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826D39F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D39F8: 386AB834  addi r3, r10, -0x47cc
	ctx.r[3].s64 = ctx.r[10].s64 + -18380;
	// 826D39FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826D3A00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3A04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3A08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3A14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3A18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3A1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3A20: 4BD93401  bl 0x82466e20
	ctx.lr = 0x826D3A24;
	sub_82466E20(ctx, base);
	// 826D3A24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3A38 size=112
    let mut pc: u32 = 0x826D3A38;
    'dispatch: loop {
        match pc {
            0x826D3A38 => {
    //   block [0x826D3A38..0x826D3AA8)
	// 826D3A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3A44: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826D3A48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3A4C: 392B4234  addi r9, r11, 0x4234
	ctx.r[9].s64 = ctx.r[11].s64 + 16948;
	// 826D3A50: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826D3A54: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826D3A58: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3A5C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826D3A60: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3A64: 396B0680  addi r11, r11, 0x680
	ctx.r[11].s64 = ctx.r[11].s64 + 1664;
	// 826D3A68: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826D3A6C: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3A70: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826D3A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3A78: 386AB864  addi r3, r10, -0x479c
	ctx.r[3].s64 = ctx.r[10].s64 + -18332;
	// 826D3A7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826D3A80: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826D3A84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3A88: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826D3A8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826D3A90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826D3A94: 4BD9338D  bl 0x82466e20
	ctx.lr = 0x826D3A98;
	sub_82466E20(ctx, base);
	// 826D3A98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3A9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3AA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3AA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3AA8 size=112
    let mut pc: u32 = 0x826D3AA8;
    'dispatch: loop {
        match pc {
            0x826D3AA8 => {
    //   block [0x826D3AA8..0x826D3B18)
	// 826D3AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3AB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3AB4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3AB8: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3ABC: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3AC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3AC4: 390B07D0  addi r8, r11, 0x7d0
	ctx.r[8].s64 = ctx.r[11].s64 + 2000;
	// 826D3AC8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826D3ACC: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 826D3AD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3AD4: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3AD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3ADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3AE0: 386AB894  addi r3, r10, -0x476c
	ctx.r[3].s64 = ctx.r[10].s64 + -18284;
	// 826D3AE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3AE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3AF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3AF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3AF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3B00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3B04: 4BD9331D  bl 0x82466e20
	ctx.lr = 0x826D3B08;
	sub_82466E20(ctx, base);
	// 826D3B08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3B14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826D3B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826D3B18 size=112
    let mut pc: u32 = 0x826D3B18;
    'dispatch: loop {
        match pc {
            0x826D3B18 => {
    //   block [0x826D3B18..0x826D3B88)
	// 826D3B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826D3B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826D3B20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826D3B24: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3B28: 3D60827F  lis r11, -0x7d81
	ctx.r[11].s64 = -2105606144;
	// 826D3B2C: 38AACFD4  addi r5, r10, -0x302c
	ctx.r[5].s64 = ctx.r[10].s64 + -12332;
	// 826D3B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826D3B34: 390B0878  addi r8, r11, 0x878
	ctx.r[8].s64 = ctx.r[11].s64 + 2168;
	// 826D3B38: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826D3B3C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826D3B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826D3B44: 3D408299  lis r10, -0x7d67
	ctx.r[10].s64 = -2103902208;
	// 826D3B48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826D3B4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826D3B50: 386AB8C4  addi r3, r10, -0x473c
	ctx.r[3].s64 = ctx.r[10].s64 + -18236;
	// 826D3B54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826D3B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826D3B5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826D3B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826D3B64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826D3B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826D3B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826D3B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826D3B74: 4BD932AD  bl 0x82466e20
	ctx.lr = 0x826D3B78;
	sub_82466E20(ctx, base);
	// 826D3B78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826D3B7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826D3B80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826D3B84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


