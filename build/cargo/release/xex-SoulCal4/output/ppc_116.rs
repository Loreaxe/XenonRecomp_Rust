pub fn sub_8269D860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D860 size=112
    let mut pc: u32 = 0x8269D860;
    'dispatch: loop {
        match pc {
            0x8269D860 => {
    //   block [0x8269D860..0x8269D8D0)
	// 8269D860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D86C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D870: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D874: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269D878: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D87C: 390B1908  addi r8, r11, 0x1908
	ctx.r[8].s64 = ctx.r[11].s64 + 6408;
	// 8269D880: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D884: 388ABF68  addi r4, r10, -0x4098
	ctx.r[4].s64 = ctx.r[10].s64 + -16536;
	// 8269D888: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D88C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D890: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D898: 386A4B50  addi r3, r10, 0x4b50
	ctx.r[3].s64 = ctx.r[10].s64 + 19280;
	// 8269D89C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D8A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D8A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D8AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D8B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D8BC: 4BDC9565  bl 0x82466e20
	ctx.lr = 0x8269D8C0;
	sub_82466E20(ctx, base);
	// 8269D8C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D8C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D8C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D8CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D8D0 size=112
    let mut pc: u32 = 0x8269D8D0;
    'dispatch: loop {
        match pc {
            0x8269D8D0 => {
    //   block [0x8269D8D0..0x8269D940)
	// 8269D8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D8DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D8E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D8E4: 38AA4BB0  addi r5, r10, 0x4bb0
	ctx.r[5].s64 = ctx.r[10].s64 + 19376;
	// 8269D8E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D8EC: 390B1920  addi r8, r11, 0x1920
	ctx.r[8].s64 = ctx.r[11].s64 + 6432;
	// 8269D8F0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269D8F4: 388ABF88  addi r4, r10, -0x4078
	ctx.r[4].s64 = ctx.r[10].s64 + -16504;
	// 8269D8F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D8FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D900: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D908: 386A4B80  addi r3, r10, 0x4b80
	ctx.r[3].s64 = ctx.r[10].s64 + 19328;
	// 8269D90C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D92C: 4BDC94F5  bl 0x82466e20
	ctx.lr = 0x8269D930;
	sub_82466E20(ctx, base);
	// 8269D930: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D940 size=100
    let mut pc: u32 = 0x8269D940;
    'dispatch: loop {
        match pc {
            0x8269D940 => {
    //   block [0x8269D940..0x8269D9A4)
	// 8269D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D94C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D954: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269D958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D960: 388ABFA4  addi r4, r10, -0x405c
	ctx.r[4].s64 = ctx.r[10].s64 + -16476;
	// 8269D964: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D974: 386A4BB0  addi r3, r10, 0x4bb0
	ctx.r[3].s64 = ctx.r[10].s64 + 19376;
	// 8269D978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D97C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D980: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269D984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D988: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269D98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D990: 4BDC9491  bl 0x82466e20
	ctx.lr = 0x8269D994;
	sub_82466E20(ctx, base);
	// 8269D994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269D9A8 size=24
    let mut pc: u32 = 0x8269D9A8;
    'dispatch: loop {
        match pc {
            0x8269D9A8 => {
    //   block [0x8269D9A8..0x8269D9C0)
	// 8269D9A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D9AC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269D9B0: 394A7368  addi r10, r10, 0x7368
	ctx.r[10].s64 = ctx.r[10].s64 + 29544;
	// 8269D9B4: 816B1998  lwz r11, 0x1998(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6552 as u32) ) } as u64;
	// 8269D9B8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8269D9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D9C0 size=116
    let mut pc: u32 = 0x8269D9C0;
    'dispatch: loop {
        match pc {
            0x8269D9C0 => {
    //   block [0x8269D9C0..0x8269DA34)
	// 8269D9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D9CC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D9D0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D9D4: 390B7368  addi r8, r11, 0x7368
	ctx.r[8].s64 = ctx.r[11].s64 + 29544;
	// 8269D9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D9DC: 392AA0F8  addi r9, r10, -0x5f08
	ctx.r[9].s64 = ctx.r[10].s64 + -24328;
	// 8269D9E0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D9E4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269D9E8: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269D9EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D9F4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D9FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DA04: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269DA08: 388ABFB8  addi r4, r10, -0x4048
	ctx.r[4].s64 = ctx.r[10].s64 + -16456;
	// 8269DA0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DA10: 386B4BE0  addi r3, r11, 0x4be0
	ctx.r[3].s64 = ctx.r[11].s64 + 19424;
	// 8269DA14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DA18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DA20: 4BDC9401  bl 0x82466e20
	ctx.lr = 0x8269DA24;
	sub_82466E20(ctx, base);
	// 8269DA24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DA28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DA2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DA30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DA38 size=108
    let mut pc: u32 = 0x8269DA38;
    'dispatch: loop {
        match pc {
            0x8269DA38 => {
    //   block [0x8269DA38..0x8269DAA4)
	// 8269DA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DA3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DA40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DA44: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DA48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DA4C: 38EB199C  addi r7, r11, 0x199c
	ctx.r[7].s64 = ctx.r[11].s64 + 6556;
	// 8269DA50: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269DA54: 388ABFD8  addi r4, r10, -0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + -16424;
	// 8269DA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DA5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DA60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269DA64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DA68: 386A4C10  addi r3, r10, 0x4c10
	ctx.r[3].s64 = ctx.r[10].s64 + 19472;
	// 8269DA6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269DA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DA7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DA8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DA90: 4BDC9391  bl 0x82466e20
	ctx.lr = 0x8269DA94;
	sub_82466E20(ctx, base);
	// 8269DA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DAA8 size=112
    let mut pc: u32 = 0x8269DAA8;
    'dispatch: loop {
        match pc {
            0x8269DAA8 => {
    //   block [0x8269DAA8..0x8269DB18)
	// 8269DAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DAB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DAB8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DABC: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DAC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DAC4: 390B19CC  addi r8, r11, 0x19cc
	ctx.r[8].s64 = ctx.r[11].s64 + 6604;
	// 8269DAC8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269DACC: 388ABFFC  addi r4, r10, -0x4004
	ctx.r[4].s64 = ctx.r[10].s64 + -16388;
	// 8269DAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DAD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DAE0: 386A4C40  addi r3, r10, 0x4c40
	ctx.r[3].s64 = ctx.r[10].s64 + 19520;
	// 8269DAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DB04: 4BDC931D  bl 0x82466e20
	ctx.lr = 0x8269DB08;
	sub_82466E20(ctx, base);
	// 8269DB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DB18 size=112
    let mut pc: u32 = 0x8269DB18;
    'dispatch: loop {
        match pc {
            0x8269DB18 => {
    //   block [0x8269DB18..0x8269DB88)
	// 8269DB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DB24: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269DB28: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DB2C: 392AA11C  addi r9, r10, -0x5ee4
	ctx.r[9].s64 = ctx.r[10].s64 + -24292;
	// 8269DB30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DB34: 390B19E8  addi r8, r11, 0x19e8
	ctx.r[8].s64 = ctx.r[11].s64 + 6632;
	// 8269DB38: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269DB3C: 388AC01C  addi r4, r10, -0x3fe4
	ctx.r[4].s64 = ctx.r[10].s64 + -16356;
	// 8269DB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DB44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DB50: 386A4C70  addi r3, r10, 0x4c70
	ctx.r[3].s64 = ctx.r[10].s64 + 19568;
	// 8269DB54: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DB58: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DB6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DB74: 4BDC92AD  bl 0x82466e20
	ctx.lr = 0x8269DB78;
	sub_82466E20(ctx, base);
	// 8269DB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DB88 size=112
    let mut pc: u32 = 0x8269DB88;
    'dispatch: loop {
        match pc {
            0x8269DB88 => {
    //   block [0x8269DB88..0x8269DBF8)
	// 8269DB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DB94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DB98: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DB9C: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DBA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DBA4: 390B1A90  addi r8, r11, 0x1a90
	ctx.r[8].s64 = ctx.r[11].s64 + 6800;
	// 8269DBA8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269DBAC: 388AC03C  addi r4, r10, -0x3fc4
	ctx.r[4].s64 = ctx.r[10].s64 + -16324;
	// 8269DBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DBB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DBB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DBC0: 386A4CA0  addi r3, r10, 0x4ca0
	ctx.r[3].s64 = ctx.r[10].s64 + 19616;
	// 8269DBC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DBCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DBD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DBE4: 4BDC923D  bl 0x82466e20
	ctx.lr = 0x8269DBE8;
	sub_82466E20(ctx, base);
	// 8269DBE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DBEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DBF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DBF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DBF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DBF8 size=112
    let mut pc: u32 = 0x8269DBF8;
    'dispatch: loop {
        match pc {
            0x8269DBF8 => {
    //   block [0x8269DBF8..0x8269DC68)
	// 8269DBF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DBFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DC00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DC04: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269DC08: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DC0C: 392AA174  addi r9, r10, -0x5e8c
	ctx.r[9].s64 = ctx.r[10].s64 + -24204;
	// 8269DC10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DC14: 390B1AB0  addi r8, r11, 0x1ab0
	ctx.r[8].s64 = ctx.r[11].s64 + 6832;
	// 8269DC18: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269DC1C: 388AC058  addi r4, r10, -0x3fa8
	ctx.r[4].s64 = ctx.r[10].s64 + -16296;
	// 8269DC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DC24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DC28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DC30: 386A4CD0  addi r3, r10, 0x4cd0
	ctx.r[3].s64 = ctx.r[10].s64 + 19664;
	// 8269DC34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DC38: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DC4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DC54: 4BDC91CD  bl 0x82466e20
	ctx.lr = 0x8269DC58;
	sub_82466E20(ctx, base);
	// 8269DC58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DC5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DC60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DC64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DC68 size=116
    let mut pc: u32 = 0x8269DC68;
    'dispatch: loop {
        match pc {
            0x8269DC68 => {
    //   block [0x8269DC68..0x8269DCDC)
	// 8269DC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DC70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DC74: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DC78: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269DC7C: 390B1B58  addi r8, r11, 0x1b58
	ctx.r[8].s64 = ctx.r[11].s64 + 7000;
	// 8269DC80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DC84: 392AA148  addi r9, r10, -0x5eb8
	ctx.r[9].s64 = ctx.r[10].s64 + -24248;
	// 8269DC88: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DC8C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269DC90: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DC94: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DC9C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DCA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DCA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DCA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DCAC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269DCB0: 388AC078  addi r4, r10, -0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + -16264;
	// 8269DCB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DCB8: 386B4D00  addi r3, r11, 0x4d00
	ctx.r[3].s64 = ctx.r[11].s64 + 19712;
	// 8269DCBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DCC0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DCC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DCC8: 4BDC9159  bl 0x82466e20
	ctx.lr = 0x8269DCCC;
	sub_82466E20(ctx, base);
	// 8269DCCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DCD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DCD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DCD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DCE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DCE0 size=108
    let mut pc: u32 = 0x8269DCE0;
    'dispatch: loop {
        match pc {
            0x8269DCE0 => {
    //   block [0x8269DCE0..0x8269DD4C)
	// 8269DCE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DCE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DCE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DCEC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DCF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DCF4: 38EB1B70  addi r7, r11, 0x1b70
	ctx.r[7].s64 = ctx.r[11].s64 + 7024;
	// 8269DCF8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269DCFC: 388AC094  addi r4, r10, -0x3f6c
	ctx.r[4].s64 = ctx.r[10].s64 + -16236;
	// 8269DD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DD04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DD08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269DD0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DD10: 386A4D30  addi r3, r10, 0x4d30
	ctx.r[3].s64 = ctx.r[10].s64 + 19760;
	// 8269DD14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269DD18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DD1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DD20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DD24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DD28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DD30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DD34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DD38: 4BDC90E9  bl 0x82466e20
	ctx.lr = 0x8269DD3C;
	sub_82466E20(ctx, base);
	// 8269DD3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DD40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DD44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DD48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DD50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DD50 size=112
    let mut pc: u32 = 0x8269DD50;
    'dispatch: loop {
        match pc {
            0x8269DD50 => {
    //   block [0x8269DD50..0x8269DDC0)
	// 8269DD50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DD54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DD58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DD5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DD60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DD64: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DD68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DD6C: 390B1BA0  addi r8, r11, 0x1ba0
	ctx.r[8].s64 = ctx.r[11].s64 + 7072;
	// 8269DD70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269DD74: 388AC0B8  addi r4, r10, -0x3f48
	ctx.r[4].s64 = ctx.r[10].s64 + -16200;
	// 8269DD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DD7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DD80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DD84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DD88: 386A4D60  addi r3, r10, 0x4d60
	ctx.r[3].s64 = ctx.r[10].s64 + 19808;
	// 8269DD8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DD94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DDA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DDAC: 4BDC9075  bl 0x82466e20
	ctx.lr = 0x8269DDB0;
	sub_82466E20(ctx, base);
	// 8269DDB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DDB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DDB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DDBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DDC0 size=112
    let mut pc: u32 = 0x8269DDC0;
    'dispatch: loop {
        match pc {
            0x8269DDC0 => {
    //   block [0x8269DDC0..0x8269DE30)
	// 8269DDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DDC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DDCC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269DDD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DDD4: 392AA1A8  addi r9, r10, -0x5e58
	ctx.r[9].s64 = ctx.r[10].s64 + -24152;
	// 8269DDD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DDDC: 390B1BC0  addi r8, r11, 0x1bc0
	ctx.r[8].s64 = ctx.r[11].s64 + 7104;
	// 8269DDE0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269DDE4: 388AC0D8  addi r4, r10, -0x3f28
	ctx.r[4].s64 = ctx.r[10].s64 + -16168;
	// 8269DDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DDEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DDF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DDF8: 386A4D90  addi r3, r10, 0x4d90
	ctx.r[3].s64 = ctx.r[10].s64 + 19856;
	// 8269DDFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269DE00: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269DE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DE14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DE1C: 4BDC9005  bl 0x82466e20
	ctx.lr = 0x8269DE20;
	sub_82466E20(ctx, base);
	// 8269DE20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DE24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DE28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DE2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DE30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DE30 size=112
    let mut pc: u32 = 0x8269DE30;
    'dispatch: loop {
        match pc {
            0x8269DE30 => {
    //   block [0x8269DE30..0x8269DEA0)
	// 8269DE30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DE34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DE38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DE3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DE40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DE44: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DE48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DE4C: 390B1C68  addi r8, r11, 0x1c68
	ctx.r[8].s64 = ctx.r[11].s64 + 7272;
	// 8269DE50: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269DE54: 388AC0F4  addi r4, r10, -0x3f0c
	ctx.r[4].s64 = ctx.r[10].s64 + -16140;
	// 8269DE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DE60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DE64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DE68: 386A4DC0  addi r3, r10, 0x4dc0
	ctx.r[3].s64 = ctx.r[10].s64 + 19904;
	// 8269DE6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DE74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DE7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DE8C: 4BDC8F95  bl 0x82466e20
	ctx.lr = 0x8269DE90;
	sub_82466E20(ctx, base);
	// 8269DE90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DE94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DE98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DE9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DEA0 size=108
    let mut pc: u32 = 0x8269DEA0;
    'dispatch: loop {
        match pc {
            0x8269DEA0 => {
    //   block [0x8269DEA0..0x8269DF0C)
	// 8269DEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DEA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DEAC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DEB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DEB4: 38EB1CB0  addi r7, r11, 0x1cb0
	ctx.r[7].s64 = ctx.r[11].s64 + 7344;
	// 8269DEB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269DEBC: 388AC10C  addi r4, r10, -0x3ef4
	ctx.r[4].s64 = ctx.r[10].s64 + -16116;
	// 8269DEC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DEC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DEC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269DECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DED0: 386A4DF0  addi r3, r10, 0x4df0
	ctx.r[3].s64 = ctx.r[10].s64 + 19952;
	// 8269DED4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269DED8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DEE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DEE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DEEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DEF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DEF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269DEF8: 4BDC8F29  bl 0x82466e20
	ctx.lr = 0x8269DEFC;
	sub_82466E20(ctx, base);
	// 8269DEFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DF00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DF04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DF08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DF10 size=112
    let mut pc: u32 = 0x8269DF10;
    'dispatch: loop {
        match pc {
            0x8269DF10 => {
    //   block [0x8269DF10..0x8269DF80)
	// 8269DF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DF18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DF1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DF20: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DF24: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DF28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DF2C: 390B1CE0  addi r8, r11, 0x1ce0
	ctx.r[8].s64 = ctx.r[11].s64 + 7392;
	// 8269DF30: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269DF34: 388AC130  addi r4, r10, -0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -16080;
	// 8269DF38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DF3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DF40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DF44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DF48: 386A4E20  addi r3, r10, 0x4e20
	ctx.r[3].s64 = ctx.r[10].s64 + 20000;
	// 8269DF4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DF50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DF58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DF5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DF60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DF64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DF68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DF6C: 4BDC8EB5  bl 0x82466e20
	ctx.lr = 0x8269DF70;
	sub_82466E20(ctx, base);
	// 8269DF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DF74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DF78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DF80 size=112
    let mut pc: u32 = 0x8269DF80;
    'dispatch: loop {
        match pc {
            0x8269DF80 => {
    //   block [0x8269DF80..0x8269DFF0)
	// 8269DF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DF8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DF90: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269DF94: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269DF98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269DF9C: 390B1CF8  addi r8, r11, 0x1cf8
	ctx.r[8].s64 = ctx.r[11].s64 + 7416;
	// 8269DFA0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269DFA4: 388AC14C  addi r4, r10, -0x3eb4
	ctx.r[4].s64 = ctx.r[10].s64 + -16052;
	// 8269DFA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269DFAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269DFB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269DFB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269DFB8: 386A4E50  addi r3, r10, 0x4e50
	ctx.r[3].s64 = ctx.r[10].s64 + 20048;
	// 8269DFBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269DFC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269DFC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269DFC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269DFCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269DFD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269DFD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269DFD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269DFDC: 4BDC8E45  bl 0x82466e20
	ctx.lr = 0x8269DFE0;
	sub_82466E20(ctx, base);
	// 8269DFE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269DFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269DFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269DFEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269DFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269DFF0 size=100
    let mut pc: u32 = 0x8269DFF0;
    'dispatch: loop {
        match pc {
            0x8269DFF0 => {
    //   block [0x8269DFF0..0x8269E054)
	// 8269DFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269DFF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269DFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269DFFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E004: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E008: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E010: 388AC168  addi r4, r10, -0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + -16024;
	// 8269E014: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E01C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E024: 386A4E80  addi r3, r10, 0x4e80
	ctx.r[3].s64 = ctx.r[10].s64 + 20096;
	// 8269E028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E02C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E030: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269E034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E038: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269E03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E040: 4BDC8DE1  bl 0x82466e20
	ctx.lr = 0x8269E044;
	sub_82466E20(ctx, base);
	// 8269E044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E04C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E058 size=112
    let mut pc: u32 = 0x8269E058;
    'dispatch: loop {
        match pc {
            0x8269E058 => {
    //   block [0x8269E058..0x8269E0C8)
	// 8269E058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E068: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E06C: 38AA4A30  addi r5, r10, 0x4a30
	ctx.r[5].s64 = ctx.r[10].s64 + 18992;
	// 8269E070: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E074: 390B1DB8  addi r8, r11, 0x1db8
	ctx.r[8].s64 = ctx.r[11].s64 + 7608;
	// 8269E078: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269E07C: 388AC180  addi r4, r10, -0x3e80
	ctx.r[4].s64 = ctx.r[10].s64 + -16000;
	// 8269E080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E090: 386A4EB0  addi r3, r10, 0x4eb0
	ctx.r[3].s64 = ctx.r[10].s64 + 20144;
	// 8269E094: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E098: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E09C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E0A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E0A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E0A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E0B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E0B4: 4BDC8D6D  bl 0x82466e20
	ctx.lr = 0x8269E0B8;
	sub_82466E20(ctx, base);
	// 8269E0B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E0BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E0C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E0C8 size=112
    let mut pc: u32 = 0x8269E0C8;
    'dispatch: loop {
        match pc {
            0x8269E0C8 => {
    //   block [0x8269E0C8..0x8269E138)
	// 8269E0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E0D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E0D8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E0DC: 38AA48B0  addi r5, r10, 0x48b0
	ctx.r[5].s64 = ctx.r[10].s64 + 18608;
	// 8269E0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E0E4: 390B1DE8  addi r8, r11, 0x1de8
	ctx.r[8].s64 = ctx.r[11].s64 + 7656;
	// 8269E0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269E0EC: 388AC19C  addi r4, r10, -0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + -15972;
	// 8269E0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E0F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E100: 386A4EE0  addi r3, r10, 0x4ee0
	ctx.r[3].s64 = ctx.r[10].s64 + 20192;
	// 8269E104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E124: 4BDC8CFD  bl 0x82466e20
	ctx.lr = 0x8269E128;
	sub_82466E20(ctx, base);
	// 8269E128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E138 size=108
    let mut pc: u32 = 0x8269E138;
    'dispatch: loop {
        match pc {
            0x8269E138 => {
    //   block [0x8269E138..0x8269E1A4)
	// 8269E138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E144: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E148: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E14C: 38EB1E00  addi r7, r11, 0x1e00
	ctx.r[7].s64 = ctx.r[11].s64 + 7680;
	// 8269E150: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269E154: 388AC1C0  addi r4, r10, -0x3e40
	ctx.r[4].s64 = ctx.r[10].s64 + -15936;
	// 8269E158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E15C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E160: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E168: 386A4F10  addi r3, r10, 0x4f10
	ctx.r[3].s64 = ctx.r[10].s64 + 20240;
	// 8269E16C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E170: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E174: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E178: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E17C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E180: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E184: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E188: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E18C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E190: 4BDC8C91  bl 0x82466e20
	ctx.lr = 0x8269E194;
	sub_82466E20(ctx, base);
	// 8269E194: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E1A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E1A8 size=112
    let mut pc: u32 = 0x8269E1A8;
    'dispatch: loop {
        match pc {
            0x8269E1A8 => {
    //   block [0x8269E1A8..0x8269E218)
	// 8269E1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E1B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E1B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E1BC: 38AA4E80  addi r5, r10, 0x4e80
	ctx.r[5].s64 = ctx.r[10].s64 + 20096;
	// 8269E1C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E1C4: 390B1E30  addi r8, r11, 0x1e30
	ctx.r[8].s64 = ctx.r[11].s64 + 7728;
	// 8269E1C8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269E1CC: 388AC1E8  addi r4, r10, -0x3e18
	ctx.r[4].s64 = ctx.r[10].s64 + -15896;
	// 8269E1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E1D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E1E0: 386A4F40  addi r3, r10, 0x4f40
	ctx.r[3].s64 = ctx.r[10].s64 + 20288;
	// 8269E1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E204: 4BDC8C1D  bl 0x82466e20
	ctx.lr = 0x8269E208;
	sub_82466E20(ctx, base);
	// 8269E208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E218 size=112
    let mut pc: u32 = 0x8269E218;
    'dispatch: loop {
        match pc {
            0x8269E218 => {
    //   block [0x8269E218..0x8269E288)
	// 8269E218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E224: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269E228: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E22C: 392AA1D4  addi r9, r10, -0x5e2c
	ctx.r[9].s64 = ctx.r[10].s64 + -24108;
	// 8269E230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E234: 390B1EC0  addi r8, r11, 0x1ec0
	ctx.r[8].s64 = ctx.r[11].s64 + 7872;
	// 8269E238: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269E23C: 388AC200  addi r4, r10, -0x3e00
	ctx.r[4].s64 = ctx.r[10].s64 + -15872;
	// 8269E240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E250: 386A4F70  addi r3, r10, 0x4f70
	ctx.r[3].s64 = ctx.r[10].s64 + 20336;
	// 8269E254: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E258: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269E25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E26C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E274: 4BDC8BAD  bl 0x82466e20
	ctx.lr = 0x8269E278;
	sub_82466E20(ctx, base);
	// 8269E278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E288 size=112
    let mut pc: u32 = 0x8269E288;
    'dispatch: loop {
        match pc {
            0x8269E288 => {
    //   block [0x8269E288..0x8269E2F8)
	// 8269E288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E298: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E29C: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E2A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E2A4: 390B1F08  addi r8, r11, 0x1f08
	ctx.r[8].s64 = ctx.r[11].s64 + 7944;
	// 8269E2A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269E2AC: 388AC218  addi r4, r10, -0x3de8
	ctx.r[4].s64 = ctx.r[10].s64 + -15848;
	// 8269E2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E2B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E2C0: 386A4FA0  addi r3, r10, 0x4fa0
	ctx.r[3].s64 = ctx.r[10].s64 + 20384;
	// 8269E2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E2E4: 4BDC8B3D  bl 0x82466e20
	ctx.lr = 0x8269E2E8;
	sub_82466E20(ctx, base);
	// 8269E2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E2F8 size=108
    let mut pc: u32 = 0x8269E2F8;
    'dispatch: loop {
        match pc {
            0x8269E2F8 => {
    //   block [0x8269E2F8..0x8269E364)
	// 8269E2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E304: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E308: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E30C: 38EB1F20  addi r7, r11, 0x1f20
	ctx.r[7].s64 = ctx.r[11].s64 + 7968;
	// 8269E310: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8269E314: 388AC22C  addi r4, r10, -0x3dd4
	ctx.r[4].s64 = ctx.r[10].s64 + -15828;
	// 8269E318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E31C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E320: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E328: 386A4FD0  addi r3, r10, 0x4fd0
	ctx.r[3].s64 = ctx.r[10].s64 + 20432;
	// 8269E32C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E330: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E334: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E338: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E340: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E348: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E350: 4BDC8AD1  bl 0x82466e20
	ctx.lr = 0x8269E354;
	sub_82466E20(ctx, base);
	// 8269E354: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E358: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E35C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E368 size=116
    let mut pc: u32 = 0x8269E368;
    'dispatch: loop {
        match pc {
            0x8269E368 => {
    //   block [0x8269E368..0x8269E3DC)
	// 8269E368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E374: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269E378: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8269E37C: 390A1FB0  addi r8, r10, 0x1fb0
	ctx.r[8].s64 = ctx.r[10].s64 + 8112;
	// 8269E380: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E384: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269E388: 38AA4E80  addi r5, r10, 0x4e80
	ctx.r[5].s64 = ctx.r[10].s64 + 20096;
	// 8269E38C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E390: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E398: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E39C: 388AC250  addi r4, r10, -0x3db0
	ctx.r[4].s64 = ctx.r[10].s64 + -15792;
	// 8269E3A0: 396BA1E8  addi r11, r11, -0x5e18
	ctx.r[11].s64 = ctx.r[11].s64 + -24088;
	// 8269E3A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E3A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E3AC: 386A5000  addi r3, r10, 0x5000
	ctx.r[3].s64 = ctx.r[10].s64 + 20480;
	// 8269E3B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269E3B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E3B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269E3BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E3C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E3C8: 4BDC8A59  bl 0x82466e20
	ctx.lr = 0x8269E3CC;
	sub_82466E20(ctx, base);
	// 8269E3CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E3D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E3D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E3D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E3E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E3E0 size=112
    let mut pc: u32 = 0x8269E3E0;
    'dispatch: loop {
        match pc {
            0x8269E3E0 => {
    //   block [0x8269E3E0..0x8269E450)
	// 8269E3E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E3E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E3E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E3EC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269E3F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E3F4: 392AA234  addi r9, r10, -0x5dcc
	ctx.r[9].s64 = ctx.r[10].s64 + -24012;
	// 8269E3F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E3FC: 390B2090  addi r8, r11, 0x2090
	ctx.r[8].s64 = ctx.r[11].s64 + 8336;
	// 8269E400: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8269E404: 388AC264  addi r4, r10, -0x3d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15772;
	// 8269E408: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E40C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E410: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E418: 386A5030  addi r3, r10, 0x5030
	ctx.r[3].s64 = ctx.r[10].s64 + 20528;
	// 8269E41C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E420: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269E424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E428: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E42C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E430: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E434: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E438: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E43C: 4BDC89E5  bl 0x82466e20
	ctx.lr = 0x8269E440;
	sub_82466E20(ctx, base);
	// 8269E440: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E444: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E448: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E450 size=112
    let mut pc: u32 = 0x8269E450;
    'dispatch: loop {
        match pc {
            0x8269E450 => {
    //   block [0x8269E450..0x8269E4C0)
	// 8269E450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E45C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E460: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E464: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E468: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E46C: 390B20F0  addi r8, r11, 0x20f0
	ctx.r[8].s64 = ctx.r[11].s64 + 8432;
	// 8269E470: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269E474: 388AC280  addi r4, r10, -0x3d80
	ctx.r[4].s64 = ctx.r[10].s64 + -15744;
	// 8269E478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E47C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E480: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E484: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E488: 386A5060  addi r3, r10, 0x5060
	ctx.r[3].s64 = ctx.r[10].s64 + 20576;
	// 8269E48C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E498: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E4A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E4A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E4AC: 4BDC8975  bl 0x82466e20
	ctx.lr = 0x8269E4B0;
	sub_82466E20(ctx, base);
	// 8269E4B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E4B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E4B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E4BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E4C0 size=108
    let mut pc: u32 = 0x8269E4C0;
    'dispatch: loop {
        match pc {
            0x8269E4C0 => {
    //   block [0x8269E4C0..0x8269E52C)
	// 8269E4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E4C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E4C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E4CC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E4D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E4D4: 38EB2108  addi r7, r11, 0x2108
	ctx.r[7].s64 = ctx.r[11].s64 + 8456;
	// 8269E4D8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269E4DC: 388AC298  addi r4, r10, -0x3d68
	ctx.r[4].s64 = ctx.r[10].s64 + -15720;
	// 8269E4E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E4E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E4E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E4EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E4F0: 386A5090  addi r3, r10, 0x5090
	ctx.r[3].s64 = ctx.r[10].s64 + 20624;
	// 8269E4F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E4F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E4FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E504: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E50C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E514: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E518: 4BDC8909  bl 0x82466e20
	ctx.lr = 0x8269E51C;
	sub_82466E20(ctx, base);
	// 8269E51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E530 size=112
    let mut pc: u32 = 0x8269E530;
    'dispatch: loop {
        match pc {
            0x8269E530 => {
    //   block [0x8269E530..0x8269E5A0)
	// 8269E530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E53C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E540: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E544: 38AA4E80  addi r5, r10, 0x4e80
	ctx.r[5].s64 = ctx.r[10].s64 + 20096;
	// 8269E548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E54C: 390B2150  addi r8, r11, 0x2150
	ctx.r[8].s64 = ctx.r[11].s64 + 8528;
	// 8269E550: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269E554: 388AC2C0  addi r4, r10, -0x3d40
	ctx.r[4].s64 = ctx.r[10].s64 + -15680;
	// 8269E558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E55C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E560: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E564: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E568: 386A50C0  addi r3, r10, 0x50c0
	ctx.r[3].s64 = ctx.r[10].s64 + 20672;
	// 8269E56C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E57C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E58C: 4BDC8895  bl 0x82466e20
	ctx.lr = 0x8269E590;
	sub_82466E20(ctx, base);
	// 8269E590: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E59C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E5A0 size=112
    let mut pc: u32 = 0x8269E5A0;
    'dispatch: loop {
        match pc {
            0x8269E5A0 => {
    //   block [0x8269E5A0..0x8269E610)
	// 8269E5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E5A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E5AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E5B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E5B4: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E5B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E5BC: 390B21C8  addi r8, r11, 0x21c8
	ctx.r[8].s64 = ctx.r[11].s64 + 8648;
	// 8269E5C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269E5C4: 388AC2D8  addi r4, r10, -0x3d28
	ctx.r[4].s64 = ctx.r[10].s64 + -15656;
	// 8269E5C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E5CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E5D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E5D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E5D8: 386A50F0  addi r3, r10, 0x50f0
	ctx.r[3].s64 = ctx.r[10].s64 + 20720;
	// 8269E5DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E5E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E5E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E5EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E5F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E5FC: 4BDC8825  bl 0x82466e20
	ctx.lr = 0x8269E600;
	sub_82466E20(ctx, base);
	// 8269E600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E610 size=108
    let mut pc: u32 = 0x8269E610;
    'dispatch: loop {
        match pc {
            0x8269E610 => {
    //   block [0x8269E610..0x8269E67C)
	// 8269E610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E618: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E61C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E620: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E624: 38EB21F8  addi r7, r11, 0x21f8
	ctx.r[7].s64 = ctx.r[11].s64 + 8696;
	// 8269E628: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269E62C: 388AC2F4  addi r4, r10, -0x3d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -15628;
	// 8269E630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E634: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E638: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E63C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E640: 386A5120  addi r3, r10, 0x5120
	ctx.r[3].s64 = ctx.r[10].s64 + 20768;
	// 8269E644: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E648: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E664: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E668: 4BDC87B9  bl 0x82466e20
	ctx.lr = 0x8269E66C;
	sub_82466E20(ctx, base);
	// 8269E66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E680 size=108
    let mut pc: u32 = 0x8269E680;
    'dispatch: loop {
        match pc {
            0x8269E680 => {
    //   block [0x8269E680..0x8269E6EC)
	// 8269E680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E68C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E690: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E694: 38EB2258  addi r7, r11, 0x2258
	ctx.r[7].s64 = ctx.r[11].s64 + 8792;
	// 8269E698: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8269E69C: 388AC324  addi r4, r10, -0x3cdc
	ctx.r[4].s64 = ctx.r[10].s64 + -15580;
	// 8269E6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E6A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E6A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269E6AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E6B0: 386A5150  addi r3, r10, 0x5150
	ctx.r[3].s64 = ctx.r[10].s64 + 20816;
	// 8269E6B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269E6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E6BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E6D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269E6D8: 4BDC8749  bl 0x82466e20
	ctx.lr = 0x8269E6DC;
	sub_82466E20(ctx, base);
	// 8269E6DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E6E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E6E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E6E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E6F0 size=112
    let mut pc: u32 = 0x8269E6F0;
    'dispatch: loop {
        match pc {
            0x8269E6F0 => {
    //   block [0x8269E6F0..0x8269E760)
	// 8269E6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E6FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E700: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E704: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269E708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E70C: 390B22D0  addi r8, r11, 0x22d0
	ctx.r[8].s64 = ctx.r[11].s64 + 8912;
	// 8269E710: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269E714: 388AC344  addi r4, r10, -0x3cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -15548;
	// 8269E718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E71C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E728: 386A5180  addi r3, r10, 0x5180
	ctx.r[3].s64 = ctx.r[10].s64 + 20864;
	// 8269E72C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E74C: 4BDC86D5  bl 0x82466e20
	ctx.lr = 0x8269E750;
	sub_82466E20(ctx, base);
	// 8269E750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269E760 size=24
    let mut pc: u32 = 0x8269E760;
    'dispatch: loop {
        match pc {
            0x8269E760 => {
    //   block [0x8269E760..0x8269E778)
	// 8269E760: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E764: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269E768: 394A73E0  addi r10, r10, 0x73e0
	ctx.r[10].s64 = ctx.r[10].s64 + 29664;
	// 8269E76C: 816B208C  lwz r11, 0x208c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8332 as u32) ) } as u64;
	// 8269E770: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269E774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E778 size=116
    let mut pc: u32 = 0x8269E778;
    'dispatch: loop {
        match pc {
            0x8269E778 => {
    //   block [0x8269E778..0x8269E7EC)
	// 8269E778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E784: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E788: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269E78C: 390B73E0  addi r8, r11, 0x73e0
	ctx.r[8].s64 = ctx.r[11].s64 + 29664;
	// 8269E790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E794: 392AA278  addi r9, r10, -0x5d88
	ctx.r[9].s64 = ctx.r[10].s64 + -23944;
	// 8269E798: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E79C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269E7A0: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269E7A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E7AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E7BC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269E7C0: 388AC360  addi r4, r10, -0x3ca0
	ctx.r[4].s64 = ctx.r[10].s64 + -15520;
	// 8269E7C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E7C8: 386B51B0  addi r3, r11, 0x51b0
	ctx.r[3].s64 = ctx.r[11].s64 + 20912;
	// 8269E7CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269E7D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E7D8: 4BDC8649  bl 0x82466e20
	ctx.lr = 0x8269E7DC;
	sub_82466E20(ctx, base);
	// 8269E7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E7F0 size=112
    let mut pc: u32 = 0x8269E7F0;
    'dispatch: loop {
        match pc {
            0x8269E7F0 => {
    //   block [0x8269E7F0..0x8269E860)
	// 8269E7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E7FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E800: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E804: 38AA51B0  addi r5, r10, 0x51b0
	ctx.r[5].s64 = ctx.r[10].s64 + 20912;
	// 8269E808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E80C: 390B2318  addi r8, r11, 0x2318
	ctx.r[8].s64 = ctx.r[11].s64 + 8984;
	// 8269E810: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269E814: 388AC374  addi r4, r10, -0x3c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -15500;
	// 8269E818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E81C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E828: 386A51E0  addi r3, r10, 0x51e0
	ctx.r[3].s64 = ctx.r[10].s64 + 20960;
	// 8269E82C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E830: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E834: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E83C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E84C: 4BDC85D5  bl 0x82466e20
	ctx.lr = 0x8269E850;
	sub_82466E20(ctx, base);
	// 8269E850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269E860 size=24
    let mut pc: u32 = 0x8269E860;
    'dispatch: loop {
        match pc {
            0x8269E860 => {
    //   block [0x8269E860..0x8269E878)
	// 8269E860: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E864: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269E868: 394A73F8  addi r10, r10, 0x73f8
	ctx.r[10].s64 = ctx.r[10].s64 + 29688;
	// 8269E86C: 816B2348  lwz r11, 0x2348(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9032 as u32) ) } as u64;
	// 8269E870: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269E874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E878 size=116
    let mut pc: u32 = 0x8269E878;
    'dispatch: loop {
        match pc {
            0x8269E878 => {
    //   block [0x8269E878..0x8269E8EC)
	// 8269E878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E880: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E884: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E888: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269E88C: 390B73F8  addi r8, r11, 0x73f8
	ctx.r[8].s64 = ctx.r[11].s64 + 29688;
	// 8269E890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E894: 392AA2B4  addi r9, r10, -0x5d4c
	ctx.r[9].s64 = ctx.r[10].s64 + -23884;
	// 8269E898: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E89C: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269E8A0: 38AA51E0  addi r5, r10, 0x51e0
	ctx.r[5].s64 = ctx.r[10].s64 + 20960;
	// 8269E8A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E8A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E8AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E8B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E8B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E8B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E8BC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269E8C0: 388AC394  addi r4, r10, -0x3c6c
	ctx.r[4].s64 = ctx.r[10].s64 + -15468;
	// 8269E8C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269E8C8: 386B5210  addi r3, r11, 0x5210
	ctx.r[3].s64 = ctx.r[11].s64 + 21008;
	// 8269E8CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269E8D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E8D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E8D8: 4BDC8549  bl 0x82466e20
	ctx.lr = 0x8269E8DC;
	sub_82466E20(ctx, base);
	// 8269E8DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E8E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E8E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E8F0 size=112
    let mut pc: u32 = 0x8269E8F0;
    'dispatch: loop {
        match pc {
            0x8269E8F0 => {
    //   block [0x8269E8F0..0x8269E960)
	// 8269E8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E8FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E900: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E904: 38AA51E0  addi r5, r10, 0x51e0
	ctx.r[5].s64 = ctx.r[10].s64 + 20960;
	// 8269E908: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E90C: 390B2350  addi r8, r11, 0x2350
	ctx.r[8].s64 = ctx.r[11].s64 + 9040;
	// 8269E910: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269E914: 388AC3B0  addi r4, r10, -0x3c50
	ctx.r[4].s64 = ctx.r[10].s64 + -15440;
	// 8269E918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E91C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E928: 386A5240  addi r3, r10, 0x5240
	ctx.r[3].s64 = ctx.r[10].s64 + 21056;
	// 8269E92C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E94C: 4BDC84D5  bl 0x82466e20
	ctx.lr = 0x8269E950;
	sub_82466E20(ctx, base);
	// 8269E950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E95C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E960 size=112
    let mut pc: u32 = 0x8269E960;
    'dispatch: loop {
        match pc {
            0x8269E960 => {
    //   block [0x8269E960..0x8269E9D0)
	// 8269E960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E96C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E970: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E974: 38AA51E0  addi r5, r10, 0x51e0
	ctx.r[5].s64 = ctx.r[10].s64 + 20960;
	// 8269E978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E97C: 390B23B0  addi r8, r11, 0x23b0
	ctx.r[8].s64 = ctx.r[11].s64 + 9136;
	// 8269E980: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269E984: 388AC3CC  addi r4, r10, -0x3c34
	ctx.r[4].s64 = ctx.r[10].s64 + -15412;
	// 8269E988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E98C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269E994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269E998: 386A5270  addi r3, r10, 0x5270
	ctx.r[3].s64 = ctx.r[10].s64 + 21104;
	// 8269E99C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269E9A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269E9A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269E9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269E9AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269E9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269E9B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269E9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269E9BC: 4BDC8465  bl 0x82466e20
	ctx.lr = 0x8269E9C0;
	sub_82466E20(ctx, base);
	// 8269E9C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269E9C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269E9C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269E9CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269E9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269E9D0 size=112
    let mut pc: u32 = 0x8269E9D0;
    'dispatch: loop {
        match pc {
            0x8269E9D0 => {
    //   block [0x8269E9D0..0x8269EA40)
	// 8269E9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269E9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269E9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269E9DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269E9E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269E9E4: 38AA51E0  addi r5, r10, 0x51e0
	ctx.r[5].s64 = ctx.r[10].s64 + 20960;
	// 8269E9E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269E9EC: 390B23E0  addi r8, r11, 0x23e0
	ctx.r[8].s64 = ctx.r[11].s64 + 9184;
	// 8269E9F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269E9F4: 388AC3EC  addi r4, r10, -0x3c14
	ctx.r[4].s64 = ctx.r[10].s64 + -15380;
	// 8269E9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269E9FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EA00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269EA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EA08: 386A52A0  addi r3, r10, 0x52a0
	ctx.r[3].s64 = ctx.r[10].s64 + 21152;
	// 8269EA0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269EA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EA2C: 4BDC83F5  bl 0x82466e20
	ctx.lr = 0x8269EA30;
	sub_82466E20(ctx, base);
	// 8269EA30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EA40 size=108
    let mut pc: u32 = 0x8269EA40;
    'dispatch: loop {
        match pc {
            0x8269EA40 => {
    //   block [0x8269EA40..0x8269EAAC)
	// 8269EA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EA4C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EA50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EA54: 38EB2428  addi r7, r11, 0x2428
	ctx.r[7].s64 = ctx.r[11].s64 + 9256;
	// 8269EA58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269EA5C: 388AC408  addi r4, r10, -0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + -15352;
	// 8269EA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EA64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EA68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EA70: 386A52D0  addi r3, r10, 0x52d0
	ctx.r[3].s64 = ctx.r[10].s64 + 21200;
	// 8269EA74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EA78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EA80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EA90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EA94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EA98: 4BDC8389  bl 0x82466e20
	ctx.lr = 0x8269EA9C;
	sub_82466E20(ctx, base);
	// 8269EA9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EAB0 size=112
    let mut pc: u32 = 0x8269EAB0;
    'dispatch: loop {
        match pc {
            0x8269EAB0 => {
    //   block [0x8269EAB0..0x8269EB20)
	// 8269EAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EAC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EAC4: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269EAC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EACC: 390B2458  addi r8, r11, 0x2458
	ctx.r[8].s64 = ctx.r[11].s64 + 9304;
	// 8269EAD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269EAD4: 388AC428  addi r4, r10, -0x3bd8
	ctx.r[4].s64 = ctx.r[10].s64 + -15320;
	// 8269EAD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EADC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EAE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269EAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EAE8: 386A5300  addi r3, r10, 0x5300
	ctx.r[3].s64 = ctx.r[10].s64 + 21248;
	// 8269EAEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269EAF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EAF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EAF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EB00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EB08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EB0C: 4BDC8315  bl 0x82466e20
	ctx.lr = 0x8269EB10;
	sub_82466E20(ctx, base);
	// 8269EB10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EB20 size=108
    let mut pc: u32 = 0x8269EB20;
    'dispatch: loop {
        match pc {
            0x8269EB20 => {
    //   block [0x8269EB20..0x8269EB8C)
	// 8269EB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EB2C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EB30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EB34: 38EB2470  addi r7, r11, 0x2470
	ctx.r[7].s64 = ctx.r[11].s64 + 9328;
	// 8269EB38: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269EB3C: 388AC440  addi r4, r10, -0x3bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -15296;
	// 8269EB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EB44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EB48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EB50: 386A5330  addi r3, r10, 0x5330
	ctx.r[3].s64 = ctx.r[10].s64 + 21296;
	// 8269EB54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EB78: 4BDC82A9  bl 0x82466e20
	ctx.lr = 0x8269EB7C;
	sub_82466E20(ctx, base);
	// 8269EB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EB90 size=108
    let mut pc: u32 = 0x8269EB90;
    'dispatch: loop {
        match pc {
            0x8269EB90 => {
    //   block [0x8269EB90..0x8269EBFC)
	// 8269EB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EB9C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EBA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EBA4: 38EB24B8  addi r7, r11, 0x24b8
	ctx.r[7].s64 = ctx.r[11].s64 + 9400;
	// 8269EBA8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269EBAC: 388AC46C  addi r4, r10, -0x3b94
	ctx.r[4].s64 = ctx.r[10].s64 + -15252;
	// 8269EBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EBB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EBC0: 386A5360  addi r3, r10, 0x5360
	ctx.r[3].s64 = ctx.r[10].s64 + 21344;
	// 8269EBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EBE8: 4BDC8239  bl 0x82466e20
	ctx.lr = 0x8269EBEC;
	sub_82466E20(ctx, base);
	// 8269EBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EC00 size=108
    let mut pc: u32 = 0x8269EC00;
    'dispatch: loop {
        match pc {
            0x8269EC00 => {
    //   block [0x8269EC00..0x8269EC6C)
	// 8269EC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EC04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EC08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EC0C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EC10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EC14: 38EB2518  addi r7, r11, 0x2518
	ctx.r[7].s64 = ctx.r[11].s64 + 9496;
	// 8269EC18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269EC1C: 388AC48C  addi r4, r10, -0x3b74
	ctx.r[4].s64 = ctx.r[10].s64 + -15220;
	// 8269EC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EC24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EC28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EC2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EC30: 386A5390  addi r3, r10, 0x5390
	ctx.r[3].s64 = ctx.r[10].s64 + 21392;
	// 8269EC34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EC38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EC40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EC48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EC50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EC54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EC58: 4BDC81C9  bl 0x82466e20
	ctx.lr = 0x8269EC5C;
	sub_82466E20(ctx, base);
	// 8269EC5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EC60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EC64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EC68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EC70 size=116
    let mut pc: u32 = 0x8269EC70;
    'dispatch: loop {
        match pc {
            0x8269EC70 => {
    //   block [0x8269EC70..0x8269ECE4)
	// 8269EC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EC74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EC78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EC7C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269EC80: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EC84: 392BA2F0  addi r9, r11, -0x5d10
	ctx.r[9].s64 = ctx.r[11].s64 + -23824;
	// 8269EC88: 38AA58A0  addi r5, r10, 0x58a0
	ctx.r[5].s64 = ctx.r[10].s64 + 22688;
	// 8269EC8C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EC90: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 8269EC94: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 8269EC98: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EC9C: 388AC4A8  addi r4, r10, -0x3b58
	ctx.r[4].s64 = ctx.r[10].s64 + -15192;
	// 8269ECA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269ECA4: 396B2548  addi r11, r11, 0x2548
	ctx.r[11].s64 = ctx.r[11].s64 + 9544;
	// 8269ECA8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269ECAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ECB0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269ECB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ECB8: 386A53C0  addi r3, r10, 0x53c0
	ctx.r[3].s64 = ctx.r[10].s64 + 21440;
	// 8269ECBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269ECC0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269ECC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269ECC8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269ECCC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ECD0: 4BDC8151  bl 0x82466e20
	ctx.lr = 0x8269ECD4;
	sub_82466E20(ctx, base);
	// 8269ECD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269ECD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269ECDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269ECE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ECE8 size=100
    let mut pc: u32 = 0x8269ECE8;
    'dispatch: loop {
        match pc {
            0x8269ECE8 => {
    //   block [0x8269ECE8..0x8269ED4C)
	// 8269ECE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ECEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ECF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ECF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ECF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269ECFC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269ED00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ED04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269ED08: 388AC4B4  addi r4, r10, -0x3b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -15180;
	// 8269ED0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ED10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269ED14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269ED18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269ED1C: 386A53F0  addi r3, r10, 0x53f0
	ctx.r[3].s64 = ctx.r[10].s64 + 21488;
	// 8269ED20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269ED24: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269ED28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269ED2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ED30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ED34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269ED38: 4BDC80E9  bl 0x82466e20
	ctx.lr = 0x8269ED3C;
	sub_82466E20(ctx, base);
	// 8269ED3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269ED40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269ED44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269ED48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ED50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ED50 size=100
    let mut pc: u32 = 0x8269ED50;
    'dispatch: loop {
        match pc {
            0x8269ED50 => {
    //   block [0x8269ED50..0x8269EDB4)
	// 8269ED50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ED54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ED58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ED5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ED60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269ED64: 38AA5480  addi r5, r10, 0x5480
	ctx.r[5].s64 = ctx.r[10].s64 + 21632;
	// 8269ED68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ED6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269ED70: 388AC4CC  addi r4, r10, -0x3b34
	ctx.r[4].s64 = ctx.r[10].s64 + -15156;
	// 8269ED74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ED78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269ED7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269ED80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269ED84: 386A5420  addi r3, r10, 0x5420
	ctx.r[3].s64 = ctx.r[10].s64 + 21536;
	// 8269ED88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269ED8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269ED90: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269ED94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ED98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ED9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EDA0: 4BDC8081  bl 0x82466e20
	ctx.lr = 0x8269EDA4;
	sub_82466E20(ctx, base);
	// 8269EDA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EDB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EDB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EDB8 size=100
    let mut pc: u32 = 0x8269EDB8;
    'dispatch: loop {
        match pc {
            0x8269EDB8 => {
    //   block [0x8269EDB8..0x8269EE1C)
	// 8269EDB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EDBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EDC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EDC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EDC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EDCC: 38AA53C0  addi r5, r10, 0x53c0
	ctx.r[5].s64 = ctx.r[10].s64 + 21440;
	// 8269EDD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EDD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EDD8: 388AC4E8  addi r4, r10, -0x3b18
	ctx.r[4].s64 = ctx.r[10].s64 + -15128;
	// 8269EDDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EDEC: 386A5450  addi r3, r10, 0x5450
	ctx.r[3].s64 = ctx.r[10].s64 + 21584;
	// 8269EDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EDF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EDF8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269EDFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EE00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269EE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EE08: 4BDC8019  bl 0x82466e20
	ctx.lr = 0x8269EE0C;
	sub_82466E20(ctx, base);
	// 8269EE0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EE10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EE14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EE18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EE20 size=104
    let mut pc: u32 = 0x8269EE20;
    'dispatch: loop {
        match pc {
            0x8269EE20 => {
    //   block [0x8269EE20..0x8269EE88)
	// 8269EE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EE2C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269EE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EE34: 392AA370  addi r9, r10, -0x5c90
	ctx.r[9].s64 = ctx.r[10].s64 + -23696;
	// 8269EE38: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EE3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EE40: 38AA53F0  addi r5, r10, 0x53f0
	ctx.r[5].s64 = ctx.r[10].s64 + 21488;
	// 8269EE44: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EE48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EE4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EE50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EE54: 388AC4F8  addi r4, r10, -0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + -15112;
	// 8269EE58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EE60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269EE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EE68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269EE6C: 386A5480  addi r3, r10, 0x5480
	ctx.r[3].s64 = ctx.r[10].s64 + 21632;
	// 8269EE70: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269EE74: 4BDC7FAD  bl 0x82466e20
	ctx.lr = 0x8269EE78;
	sub_82466E20(ctx, base);
	// 8269EE78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EE84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EE88 size=108
    let mut pc: u32 = 0x8269EE88;
    'dispatch: loop {
        match pc {
            0x8269EE88 => {
    //   block [0x8269EE88..0x8269EEF4)
	// 8269EE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EE94: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EE98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EE9C: 38EB26E4  addi r7, r11, 0x26e4
	ctx.r[7].s64 = ctx.r[11].s64 + 9956;
	// 8269EEA0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269EEA4: 388AC510  addi r4, r10, -0x3af0
	ctx.r[4].s64 = ctx.r[10].s64 + -15088;
	// 8269EEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EEAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EEB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269EEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EEB8: 386A54B0  addi r3, r10, 0x54b0
	ctx.r[3].s64 = ctx.r[10].s64 + 21680;
	// 8269EEBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269EEC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269EEE0: 4BDC7F41  bl 0x82466e20
	ctx.lr = 0x8269EEE4;
	sub_82466E20(ctx, base);
	// 8269EEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EEF8 size=112
    let mut pc: u32 = 0x8269EEF8;
    'dispatch: loop {
        match pc {
            0x8269EEF8 => {
    //   block [0x8269EEF8..0x8269EF68)
	// 8269EEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EF04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EF08: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EF0C: 38AA5480  addi r5, r10, 0x5480
	ctx.r[5].s64 = ctx.r[10].s64 + 21632;
	// 8269EF10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EF14: 390B2718  addi r8, r11, 0x2718
	ctx.r[8].s64 = ctx.r[11].s64 + 10008;
	// 8269EF18: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8269EF1C: 388AC538  addi r4, r10, -0x3ac8
	ctx.r[4].s64 = ctx.r[10].s64 + -15048;
	// 8269EF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EF24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EF28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269EF2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EF30: 386A54E0  addi r3, r10, 0x54e0
	ctx.r[3].s64 = ctx.r[10].s64 + 21728;
	// 8269EF34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269EF38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269EF3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269EF40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EF44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EF48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EF4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EF50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EF54: 4BDC7ECD  bl 0x82466e20
	ctx.lr = 0x8269EF58;
	sub_82466E20(ctx, base);
	// 8269EF58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EF64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269EF68 size=24
    let mut pc: u32 = 0x8269EF68;
    'dispatch: loop {
        match pc {
            0x8269EF68 => {
    //   block [0x8269EF68..0x8269EF80)
	// 8269EF68: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EF6C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269EF70: 394A7470  addi r10, r10, 0x7470
	ctx.r[10].s64 = ctx.r[10].s64 + 29808;
	// 8269EF74: 816B2714  lwz r11, 0x2714(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10004 as u32) ) } as u64;
	// 8269EF78: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269EF7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EF80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EF80 size=116
    let mut pc: u32 = 0x8269EF80;
    'dispatch: loop {
        match pc {
            0x8269EF80 => {
    //   block [0x8269EF80..0x8269EFF4)
	// 8269EF80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EF84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269EF88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269EF8C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269EF90: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269EF94: 390B7470  addi r8, r11, 0x7470
	ctx.r[8].s64 = ctx.r[11].s64 + 29808;
	// 8269EF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269EF9C: 392AA3E0  addi r9, r10, -0x5c20
	ctx.r[9].s64 = ctx.r[10].s64 + -23584;
	// 8269EFA0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269EFA4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 8269EFA8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269EFAC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269EFB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269EFB4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269EFB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269EFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269EFC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269EFC4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269EFC8: 388AC558  addi r4, r10, -0x3aa8
	ctx.r[4].s64 = ctx.r[10].s64 + -15016;
	// 8269EFCC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269EFD0: 386B5510  addi r3, r11, 0x5510
	ctx.r[3].s64 = ctx.r[11].s64 + 21776;
	// 8269EFD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269EFD8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269EFDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269EFE0: 4BDC7E41  bl 0x82466e20
	ctx.lr = 0x8269EFE4;
	sub_82466E20(ctx, base);
	// 8269EFE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269EFE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269EFEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269EFF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269EFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269EFF8 size=100
    let mut pc: u32 = 0x8269EFF8;
    'dispatch: loop {
        match pc {
            0x8269EFF8 => {
    //   block [0x8269EFF8..0x8269F05C)
	// 8269EFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269EFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F000: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F004: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F00C: 38AA5510  addi r5, r10, 0x5510
	ctx.r[5].s64 = ctx.r[10].s64 + 21776;
	// 8269F010: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F018: 388AC564  addi r4, r10, -0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + -15004;
	// 8269F01C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F02C: 386A5540  addi r3, r10, 0x5540
	ctx.r[3].s64 = ctx.r[10].s64 + 21824;
	// 8269F030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F034: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F038: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F03C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F040: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F044: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F048: 4BDC7DD9  bl 0x82466e20
	ctx.lr = 0x8269F04C;
	sub_82466E20(ctx, base);
	// 8269F04C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F058: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F060 size=100
    let mut pc: u32 = 0x8269F060;
    'dispatch: loop {
        match pc {
            0x8269F060 => {
    //   block [0x8269F060..0x8269F0C4)
	// 8269F060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F068: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F06C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F074: 38AA5510  addi r5, r10, 0x5510
	ctx.r[5].s64 = ctx.r[10].s64 + 21776;
	// 8269F078: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F080: 388AC574  addi r4, r10, -0x3a8c
	ctx.r[4].s64 = ctx.r[10].s64 + -14988;
	// 8269F084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F094: 386A5570  addi r3, r10, 0x5570
	ctx.r[3].s64 = ctx.r[10].s64 + 21872;
	// 8269F098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F0A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F0A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F0B0: 4BDC7D71  bl 0x82466e20
	ctx.lr = 0x8269F0B4;
	sub_82466E20(ctx, base);
	// 8269F0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F0C8 size=100
    let mut pc: u32 = 0x8269F0C8;
    'dispatch: loop {
        match pc {
            0x8269F0C8 => {
    //   block [0x8269F0C8..0x8269F12C)
	// 8269F0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F0D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F0DC: 38AA55D0  addi r5, r10, 0x55d0
	ctx.r[5].s64 = ctx.r[10].s64 + 21968;
	// 8269F0E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F0E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F0E8: 388AC588  addi r4, r10, -0x3a78
	ctx.r[4].s64 = ctx.r[10].s64 + -14968;
	// 8269F0EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F0FC: 386A55A0  addi r3, r10, 0x55a0
	ctx.r[3].s64 = ctx.r[10].s64 + 21920;
	// 8269F100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F104: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F108: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F10C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F110: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F118: 4BDC7D09  bl 0x82466e20
	ctx.lr = 0x8269F11C;
	sub_82466E20(ctx, base);
	// 8269F11C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F130 size=100
    let mut pc: u32 = 0x8269F130;
    'dispatch: loop {
        match pc {
            0x8269F130 => {
    //   block [0x8269F130..0x8269F194)
	// 8269F130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F13C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F144: 38AA5510  addi r5, r10, 0x5510
	ctx.r[5].s64 = ctx.r[10].s64 + 21776;
	// 8269F148: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F150: 388AC59C  addi r4, r10, -0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + -14948;
	// 8269F154: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F164: 386A55D0  addi r3, r10, 0x55d0
	ctx.r[3].s64 = ctx.r[10].s64 + 21968;
	// 8269F168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F16C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F170: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F174: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F178: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F17C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F180: 4BDC7CA1  bl 0x82466e20
	ctx.lr = 0x8269F184;
	sub_82466E20(ctx, base);
	// 8269F184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F198 size=100
    let mut pc: u32 = 0x8269F198;
    'dispatch: loop {
        match pc {
            0x8269F198 => {
    //   block [0x8269F198..0x8269F1FC)
	// 8269F198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F1A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F1AC: 38AA55D0  addi r5, r10, 0x55d0
	ctx.r[5].s64 = ctx.r[10].s64 + 21968;
	// 8269F1B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F1B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F1B8: 388AC5B4  addi r4, r10, -0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + -14924;
	// 8269F1BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F1C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F1CC: 386A5600  addi r3, r10, 0x5600
	ctx.r[3].s64 = ctx.r[10].s64 + 22016;
	// 8269F1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F1D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F1D8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F1E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F1E8: 4BDC7C39  bl 0x82466e20
	ctx.lr = 0x8269F1EC;
	sub_82466E20(ctx, base);
	// 8269F1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F200 size=100
    let mut pc: u32 = 0x8269F200;
    'dispatch: loop {
        match pc {
            0x8269F200 => {
    //   block [0x8269F200..0x8269F264)
	// 8269F200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F20C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F214: 38AA5510  addi r5, r10, 0x5510
	ctx.r[5].s64 = ctx.r[10].s64 + 21776;
	// 8269F218: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F220: 388AC5C8  addi r4, r10, -0x3a38
	ctx.r[4].s64 = ctx.r[10].s64 + -14904;
	// 8269F224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F234: 386A5630  addi r3, r10, 0x5630
	ctx.r[3].s64 = ctx.r[10].s64 + 22064;
	// 8269F238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F23C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F240: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F248: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F250: 4BDC7BD1  bl 0x82466e20
	ctx.lr = 0x8269F254;
	sub_82466E20(ctx, base);
	// 8269F254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F268 size=100
    let mut pc: u32 = 0x8269F268;
    'dispatch: loop {
        match pc {
            0x8269F268 => {
    //   block [0x8269F268..0x8269F2CC)
	// 8269F268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F274: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F27C: 38AA5540  addi r5, r10, 0x5540
	ctx.r[5].s64 = ctx.r[10].s64 + 21824;
	// 8269F280: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F288: 388AC5D8  addi r4, r10, -0x3a28
	ctx.r[4].s64 = ctx.r[10].s64 + -14888;
	// 8269F28C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F290: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F298: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F29C: 386A5660  addi r3, r10, 0x5660
	ctx.r[3].s64 = ctx.r[10].s64 + 22112;
	// 8269F2A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F2A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F2A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F2AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F2B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F2B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F2B8: 4BDC7B69  bl 0x82466e20
	ctx.lr = 0x8269F2BC;
	sub_82466E20(ctx, base);
	// 8269F2BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F2C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F2C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F2C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F2D0 size=100
    let mut pc: u32 = 0x8269F2D0;
    'dispatch: loop {
        match pc {
            0x8269F2D0 => {
    //   block [0x8269F2D0..0x8269F334)
	// 8269F2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F2DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F2E4: 38AA5630  addi r5, r10, 0x5630
	ctx.r[5].s64 = ctx.r[10].s64 + 22064;
	// 8269F2E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F2F0: 388AC5F0  addi r4, r10, -0x3a10
	ctx.r[4].s64 = ctx.r[10].s64 + -14864;
	// 8269F2F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F304: 386A5690  addi r3, r10, 0x5690
	ctx.r[3].s64 = ctx.r[10].s64 + 22160;
	// 8269F308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F30C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F320: 4BDC7B01  bl 0x82466e20
	ctx.lr = 0x8269F324;
	sub_82466E20(ctx, base);
	// 8269F324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F338 size=100
    let mut pc: u32 = 0x8269F338;
    'dispatch: loop {
        match pc {
            0x8269F338 => {
    //   block [0x8269F338..0x8269F39C)
	// 8269F338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F344: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F34C: 38AA5540  addi r5, r10, 0x5540
	ctx.r[5].s64 = ctx.r[10].s64 + 21824;
	// 8269F350: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F358: 388AC60C  addi r4, r10, -0x39f4
	ctx.r[4].s64 = ctx.r[10].s64 + -14836;
	// 8269F35C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F36C: 386A56C0  addi r3, r10, 0x56c0
	ctx.r[3].s64 = ctx.r[10].s64 + 22208;
	// 8269F370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F374: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F378: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269F37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F388: 4BDC7A99  bl 0x82466e20
	ctx.lr = 0x8269F38C;
	sub_82466E20(ctx, base);
	// 8269F38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F3A0 size=112
    let mut pc: u32 = 0x8269F3A0;
    'dispatch: loop {
        match pc {
            0x8269F3A0 => {
    //   block [0x8269F3A0..0x8269F410)
	// 8269F3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F3AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F3B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F3B4: 38AA5750  addi r5, r10, 0x5750
	ctx.r[5].s64 = ctx.r[10].s64 + 22352;
	// 8269F3B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F3BC: 390B27C0  addi r8, r11, 0x27c0
	ctx.r[8].s64 = ctx.r[11].s64 + 10176;
	// 8269F3C0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269F3C4: 388AC620  addi r4, r10, -0x39e0
	ctx.r[4].s64 = ctx.r[10].s64 + -14816;
	// 8269F3C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F3CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F3D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F3D8: 386A56F0  addi r3, r10, 0x56f0
	ctx.r[3].s64 = ctx.r[10].s64 + 22256;
	// 8269F3DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F3E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F3E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F3E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F3EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F3F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F3F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F3F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F3FC: 4BDC7A25  bl 0x82466e20
	ctx.lr = 0x8269F400;
	sub_82466E20(ctx, base);
	// 8269F400: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F410 size=112
    let mut pc: u32 = 0x8269F410;
    'dispatch: loop {
        match pc {
            0x8269F410 => {
    //   block [0x8269F410..0x8269F480)
	// 8269F410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F418: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F41C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F420: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F424: 38AA5780  addi r5, r10, 0x5780
	ctx.r[5].s64 = ctx.r[10].s64 + 22400;
	// 8269F428: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F42C: 390B27F0  addi r8, r11, 0x27f0
	ctx.r[8].s64 = ctx.r[11].s64 + 10224;
	// 8269F430: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F434: 388AC630  addi r4, r10, -0x39d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14800;
	// 8269F438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F43C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F440: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F444: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F448: 386A5720  addi r3, r10, 0x5720
	ctx.r[3].s64 = ctx.r[10].s64 + 22304;
	// 8269F44C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F450: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F454: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F458: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F45C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F460: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F468: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F46C: 4BDC79B5  bl 0x82466e20
	ctx.lr = 0x8269F470;
	sub_82466E20(ctx, base);
	// 8269F470: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F480 size=112
    let mut pc: u32 = 0x8269F480;
    'dispatch: loop {
        match pc {
            0x8269F480 => {
    //   block [0x8269F480..0x8269F4F0)
	// 8269F480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F48C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F490: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F494: 38AA58A0  addi r5, r10, 0x58a0
	ctx.r[5].s64 = ctx.r[10].s64 + 22688;
	// 8269F498: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F49C: 390B2808  addi r8, r11, 0x2808
	ctx.r[8].s64 = ctx.r[11].s64 + 10248;
	// 8269F4A0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269F4A4: 388AC648  addi r4, r10, -0x39b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14776;
	// 8269F4A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F4AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F4B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F4B8: 386A5750  addi r3, r10, 0x5750
	ctx.r[3].s64 = ctx.r[10].s64 + 22352;
	// 8269F4BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F4C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F4C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F4C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F4CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F4D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F4D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F4D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F4DC: 4BDC7945  bl 0x82466e20
	ctx.lr = 0x8269F4E0;
	sub_82466E20(ctx, base);
	// 8269F4E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F4E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F4E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F4F0 size=112
    let mut pc: u32 = 0x8269F4F0;
    'dispatch: loop {
        match pc {
            0x8269F4F0 => {
    //   block [0x8269F4F0..0x8269F560)
	// 8269F4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F4F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F4FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F500: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F504: 38AA5750  addi r5, r10, 0x5750
	ctx.r[5].s64 = ctx.r[10].s64 + 22352;
	// 8269F508: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F50C: 390B2838  addi r8, r11, 0x2838
	ctx.r[8].s64 = ctx.r[11].s64 + 10296;
	// 8269F510: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F514: 388AC654  addi r4, r10, -0x39ac
	ctx.r[4].s64 = ctx.r[10].s64 + -14764;
	// 8269F518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F51C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F528: 386A5780  addi r3, r10, 0x5780
	ctx.r[3].s64 = ctx.r[10].s64 + 22400;
	// 8269F52C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F530: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F534: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F538: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F53C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F540: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F544: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F54C: 4BDC78D5  bl 0x82466e20
	ctx.lr = 0x8269F550;
	sub_82466E20(ctx, base);
	// 8269F550: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F554: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F558: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F560 size=108
    let mut pc: u32 = 0x8269F560;
    'dispatch: loop {
        match pc {
            0x8269F560 => {
    //   block [0x8269F560..0x8269F5CC)
	// 8269F560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F56C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F570: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F574: 38EB2850  addi r7, r11, 0x2850
	ctx.r[7].s64 = ctx.r[11].s64 + 10320;
	// 8269F578: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269F57C: 388AC664  addi r4, r10, -0x399c
	ctx.r[4].s64 = ctx.r[10].s64 + -14748;
	// 8269F580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F584: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F588: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269F58C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F590: 386A57B0  addi r3, r10, 0x57b0
	ctx.r[3].s64 = ctx.r[10].s64 + 22448;
	// 8269F594: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269F598: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F59C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F5A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F5A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F5A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F5AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F5B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F5B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269F5B8: 4BDC7869  bl 0x82466e20
	ctx.lr = 0x8269F5BC;
	sub_82466E20(ctx, base);
	// 8269F5BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F5C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F5C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F5C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F5D0 size=112
    let mut pc: u32 = 0x8269F5D0;
    'dispatch: loop {
        match pc {
            0x8269F5D0 => {
    //   block [0x8269F5D0..0x8269F640)
	// 8269F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F5D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F5DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F5E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F5E4: 38AA5780  addi r5, r10, 0x5780
	ctx.r[5].s64 = ctx.r[10].s64 + 22400;
	// 8269F5E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F5EC: 390B2868  addi r8, r11, 0x2868
	ctx.r[8].s64 = ctx.r[11].s64 + 10344;
	// 8269F5F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F5F4: 388AC68C  addi r4, r10, -0x3974
	ctx.r[4].s64 = ctx.r[10].s64 + -14708;
	// 8269F5F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F5FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F600: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F604: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F608: 386A57E0  addi r3, r10, 0x57e0
	ctx.r[3].s64 = ctx.r[10].s64 + 22496;
	// 8269F60C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F610: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F618: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F61C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F620: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F628: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F62C: 4BDC77F5  bl 0x82466e20
	ctx.lr = 0x8269F630;
	sub_82466E20(ctx, base);
	// 8269F630: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F640 size=116
    let mut pc: u32 = 0x8269F640;
    'dispatch: loop {
        match pc {
            0x8269F640 => {
    //   block [0x8269F640..0x8269F6B4)
	// 8269F640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F64C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269F650: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269F654: 390A2880  addi r8, r10, 0x2880
	ctx.r[8].s64 = ctx.r[10].s64 + 10368;
	// 8269F658: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F65C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269F660: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F664: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F668: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269F66C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F670: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F674: 388AC6A4  addi r4, r10, -0x395c
	ctx.r[4].s64 = ctx.r[10].s64 + -14684;
	// 8269F678: 396BA3F4  addi r11, r11, -0x5c0c
	ctx.r[11].s64 = ctx.r[11].s64 + -23564;
	// 8269F67C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F680: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F684: 386A5810  addi r3, r10, 0x5810
	ctx.r[3].s64 = ctx.r[10].s64 + 22544;
	// 8269F688: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269F68C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F690: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269F694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F6A0: 4BDC7781  bl 0x82466e20
	ctx.lr = 0x8269F6A4;
	sub_82466E20(ctx, base);
	// 8269F6A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F6B8 size=116
    let mut pc: u32 = 0x8269F6B8;
    'dispatch: loop {
        match pc {
            0x8269F6B8 => {
    //   block [0x8269F6B8..0x8269F72C)
	// 8269F6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F6C4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269F6C8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269F6CC: 392AA51C  addi r9, r10, -0x5ae4
	ctx.r[9].s64 = ctx.r[10].s64 + -23268;
	// 8269F6D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F6D4: 38C00044  li r6, 0x44
	ctx.r[6].s64 = 68;
	// 8269F6D8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F6DC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F6E0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 8269F6E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F6E8: 388AC6B8  addi r4, r10, -0x3948
	ctx.r[4].s64 = ctx.r[10].s64 + -14664;
	// 8269F6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F6F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269F6F4: 396B2948  addi r11, r11, 0x2948
	ctx.r[11].s64 = ctx.r[11].s64 + 10568;
	// 8269F6F8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F6FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F700: 386A5840  addi r3, r10, 0x5840
	ctx.r[3].s64 = ctx.r[10].s64 + 22592;
	// 8269F704: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8269F708: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269F70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F710: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8269F714: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F718: 4BDC7709  bl 0x82466e20
	ctx.lr = 0x8269F71C;
	sub_82466E20(ctx, base);
	// 8269F71C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269F730 size=48
    let mut pc: u32 = 0x8269F730;
    'dispatch: loop {
        match pc {
            0x8269F730 => {
    //   block [0x8269F730..0x8269F760)
	// 8269F730: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F734: 814B2FB0  lwz r10, 0x2fb0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12208 as u32) ) } as u64;
	// 8269F738: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F73C: 396B7578  addi r11, r11, 0x7578
	ctx.r[11].s64 = ctx.r[11].s64 + 30072;
	// 8269F740: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8269F744: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269F748: 814A2FAC  lwz r10, 0x2fac(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12204 as u32) ) } as u64;
	// 8269F74C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 8269F750: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269F754: 814A2FA8  lwz r10, 0x2fa8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12200 as u32) ) } as u64;
	// 8269F758: 914B0380  stw r10, 0x380(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(896 as u32), ctx.r[10].u32 ) };
	// 8269F75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F760 size=116
    let mut pc: u32 = 0x8269F760;
    'dispatch: loop {
        match pc {
            0x8269F760 => {
    //   block [0x8269F760..0x8269F7D4)
	// 8269F760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F76C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269F770: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F774: 392BA5F8  addi r9, r11, -0x5a08
	ctx.r[9].s64 = ctx.r[11].s64 + -23048;
	// 8269F778: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F77C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F780: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 8269F784: 38C0002A  li r6, 0x2a
	ctx.r[6].s64 = 42;
	// 8269F788: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F78C: 388AC6C4  addi r4, r10, -0x393c
	ctx.r[4].s64 = ctx.r[10].s64 + -14652;
	// 8269F790: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F794: 396B7578  addi r11, r11, 0x7578
	ctx.r[11].s64 = ctx.r[11].s64 + 30072;
	// 8269F798: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269F79C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F7A0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269F7A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F7A8: 386A5870  addi r3, r10, 0x5870
	ctx.r[3].s64 = ctx.r[10].s64 + 22640;
	// 8269F7AC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8269F7B0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269F7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F7B8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269F7BC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F7C0: 4BDC7661  bl 0x82466e20
	ctx.lr = 0x8269F7C4;
	sub_82466E20(ctx, base);
	// 8269F7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F7D8 size=116
    let mut pc: u32 = 0x8269F7D8;
    'dispatch: loop {
        match pc {
            0x8269F7D8 => {
    //   block [0x8269F7D8..0x8269F84C)
	// 8269F7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F7E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F7E8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269F7EC: 390B2FC0  addi r8, r11, 0x2fc0
	ctx.r[8].s64 = ctx.r[11].s64 + 12224;
	// 8269F7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F7F4: 392AA7A8  addi r9, r10, -0x5858
	ctx.r[9].s64 = ctx.r[10].s64 + -22616;
	// 8269F7F8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F7FC: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8269F800: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F804: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F80C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F81C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269F820: 388AC6D4  addi r4, r10, -0x392c
	ctx.r[4].s64 = ctx.r[10].s64 + -14636;
	// 8269F824: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269F828: 386B58A0  addi r3, r11, 0x58a0
	ctx.r[3].s64 = ctx.r[11].s64 + 22688;
	// 8269F82C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269F830: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F838: 4BDC75E9  bl 0x82466e20
	ctx.lr = 0x8269F83C;
	sub_82466E20(ctx, base);
	// 8269F83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F850 size=112
    let mut pc: u32 = 0x8269F850;
    'dispatch: loop {
        match pc {
            0x8269F850 => {
    //   block [0x8269F850..0x8269F8C0)
	// 8269F850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F85C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F860: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F864: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F868: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F86C: 390B3050  addi r8, r11, 0x3050
	ctx.r[8].s64 = ctx.r[11].s64 + 12368;
	// 8269F870: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F874: 388AC6E4  addi r4, r10, -0x391c
	ctx.r[4].s64 = ctx.r[10].s64 + -14620;
	// 8269F878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F87C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269F884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F888: 386A58D0  addi r3, r10, 0x58d0
	ctx.r[3].s64 = ctx.r[10].s64 + 22736;
	// 8269F88C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269F890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F8AC: 4BDC7575  bl 0x82466e20
	ctx.lr = 0x8269F8B0;
	sub_82466E20(ctx, base);
	// 8269F8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269F8C0 size=36
    let mut pc: u32 = 0x8269F8C0;
    'dispatch: loop {
        match pc {
            0x8269F8C0 => {
    //   block [0x8269F8C0..0x8269F8E4)
	// 8269F8C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F8C4: 814B306C  lwz r10, 0x306c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12396 as u32) ) } as u64;
	// 8269F8C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F8CC: 396B7968  addi r11, r11, 0x7968
	ctx.r[11].s64 = ctx.r[11].s64 + 31080;
	// 8269F8D0: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8269F8D4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269F8D8: 814A2FBC  lwz r10, 0x2fbc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12220 as u32) ) } as u64;
	// 8269F8DC: 914B00B0  stw r10, 0xb0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 8269F8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F8E8 size=116
    let mut pc: u32 = 0x8269F8E8;
    'dispatch: loop {
        match pc {
            0x8269F8E8 => {
    //   block [0x8269F8E8..0x8269F95C)
	// 8269F8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F8EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F8F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F8F4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269F8F8: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269F8FC: 392AA810  addi r9, r10, -0x57f0
	ctx.r[9].s64 = ctx.r[10].s64 + -22512;
	// 8269F900: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F904: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8269F908: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269F90C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F910: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 8269F914: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F918: 388AC75C  addi r4, r10, -0x38a4
	ctx.r[4].s64 = ctx.r[10].s64 + -14500;
	// 8269F91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F920: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269F924: 396B7968  addi r11, r11, 0x7968
	ctx.r[11].s64 = ctx.r[11].s64 + 31080;
	// 8269F928: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F92C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F930: 386A5900  addi r3, r10, 0x5900
	ctx.r[3].s64 = ctx.r[10].s64 + 22784;
	// 8269F934: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8269F938: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269F93C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F940: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 8269F944: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269F948: 4BDC74D9  bl 0x82466e20
	ctx.lr = 0x8269F94C;
	sub_82466E20(ctx, base);
	// 8269F94C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F960 size=108
    let mut pc: u32 = 0x8269F960;
    'dispatch: loop {
        match pc {
            0x8269F960 => {
    //   block [0x8269F960..0x8269F9CC)
	// 8269F960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F96C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F970: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F974: 38EB3070  addi r7, r11, 0x3070
	ctx.r[7].s64 = ctx.r[11].s64 + 12400;
	// 8269F978: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 8269F97C: 388AC76C  addi r4, r10, -0x3894
	ctx.r[4].s64 = ctx.r[10].s64 + -14484;
	// 8269F980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F984: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269F98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269F990: 386A5930  addi r3, r10, 0x5930
	ctx.r[3].s64 = ctx.r[10].s64 + 22832;
	// 8269F994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269F998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269F99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269F9A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269F9A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269F9A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269F9AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269F9B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269F9B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269F9B8: 4BDC7469  bl 0x82466e20
	ctx.lr = 0x8269F9BC;
	sub_82466E20(ctx, base);
	// 8269F9BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269F9C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269F9C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269F9C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269F9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269F9D0 size=112
    let mut pc: u32 = 0x8269F9D0;
    'dispatch: loop {
        match pc {
            0x8269F9D0 => {
    //   block [0x8269F9D0..0x8269FA40)
	// 8269F9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269F9D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269F9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269F9DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269F9E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269F9E4: 38AA36B0  addi r5, r10, 0x36b0
	ctx.r[5].s64 = ctx.r[10].s64 + 14000;
	// 8269F9E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269F9EC: 390B30E8  addi r8, r11, 0x30e8
	ctx.r[8].s64 = ctx.r[11].s64 + 12520;
	// 8269F9F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269F9F4: 388AC780  addi r4, r10, -0x3880
	ctx.r[4].s64 = ctx.r[10].s64 + -14464;
	// 8269F9F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269F9FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FA00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FA08: 386A5960  addi r3, r10, 0x5960
	ctx.r[3].s64 = ctx.r[10].s64 + 22880;
	// 8269FA0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FA10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FA14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FA18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FA1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FA24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FA28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FA2C: 4BDC73F5  bl 0x82466e20
	ctx.lr = 0x8269FA30;
	sub_82466E20(ctx, base);
	// 8269FA30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FA40 size=108
    let mut pc: u32 = 0x8269FA40;
    'dispatch: loop {
        match pc {
            0x8269FA40 => {
    //   block [0x8269FA40..0x8269FAAC)
	// 8269FA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FA4C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FA50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FA54: 38EB3100  addi r7, r11, 0x3100
	ctx.r[7].s64 = ctx.r[11].s64 + 12544;
	// 8269FA58: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269FA5C: 388AC794  addi r4, r10, -0x386c
	ctx.r[4].s64 = ctx.r[10].s64 + -14444;
	// 8269FA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FA64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FA68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269FA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FA70: 386A5990  addi r3, r10, 0x5990
	ctx.r[3].s64 = ctx.r[10].s64 + 22928;
	// 8269FA74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269FA78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FA80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FA88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FA90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FA94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269FA98: 4BDC7389  bl 0x82466e20
	ctx.lr = 0x8269FA9C;
	sub_82466E20(ctx, base);
	// 8269FA9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FAA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FAA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FAA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FAB0 size=112
    let mut pc: u32 = 0x8269FAB0;
    'dispatch: loop {
        match pc {
            0x8269FAB0 => {
    //   block [0x8269FAB0..0x8269FB20)
	// 8269FAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FAB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FAC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FAC4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269FAC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FACC: 390B3118  addi r8, r11, 0x3118
	ctx.r[8].s64 = ctx.r[11].s64 + 12568;
	// 8269FAD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269FAD4: 388AC7A8  addi r4, r10, -0x3858
	ctx.r[4].s64 = ctx.r[10].s64 + -14424;
	// 8269FAD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FADC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FAE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FAE8: 386A59C0  addi r3, r10, 0x59c0
	ctx.r[3].s64 = ctx.r[10].s64 + 22976;
	// 8269FAEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FAF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FAF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FAF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FAFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FB00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FB04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FB08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FB0C: 4BDC7315  bl 0x82466e20
	ctx.lr = 0x8269FB10;
	sub_82466E20(ctx, base);
	// 8269FB10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FB14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FB18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FB1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FB20 size=108
    let mut pc: u32 = 0x8269FB20;
    'dispatch: loop {
        match pc {
            0x8269FB20 => {
    //   block [0x8269FB20..0x8269FB8C)
	// 8269FB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FB2C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FB30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FB34: 38EB3160  addi r7, r11, 0x3160
	ctx.r[7].s64 = ctx.r[11].s64 + 12640;
	// 8269FB38: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269FB3C: 388AC7C4  addi r4, r10, -0x383c
	ctx.r[4].s64 = ctx.r[10].s64 + -14396;
	// 8269FB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FB44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FB48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269FB4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FB50: 386A59F0  addi r3, r10, 0x59f0
	ctx.r[3].s64 = ctx.r[10].s64 + 23024;
	// 8269FB54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269FB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FB64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FB74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269FB78: 4BDC72A9  bl 0x82466e20
	ctx.lr = 0x8269FB7C;
	sub_82466E20(ctx, base);
	// 8269FB7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FB90 size=108
    let mut pc: u32 = 0x8269FB90;
    'dispatch: loop {
        match pc {
            0x8269FB90 => {
    //   block [0x8269FB90..0x8269FBFC)
	// 8269FB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FB98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FB9C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FBA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FBA4: 38EB3194  addi r7, r11, 0x3194
	ctx.r[7].s64 = ctx.r[11].s64 + 12692;
	// 8269FBA8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269FBAC: 388AC7E4  addi r4, r10, -0x381c
	ctx.r[4].s64 = ctx.r[10].s64 + -14364;
	// 8269FBB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FBB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FBB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269FBBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FBC0: 386A5A20  addi r3, r10, 0x5a20
	ctx.r[3].s64 = ctx.r[10].s64 + 23072;
	// 8269FBC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269FBC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FBCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FBD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FBD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FBD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FBE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FBE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269FBE8: 4BDC7239  bl 0x82466e20
	ctx.lr = 0x8269FBEC;
	sub_82466E20(ctx, base);
	// 8269FBEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FBF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FBF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FBF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269FC00 size=24
    let mut pc: u32 = 0x8269FC00;
    'dispatch: loop {
        match pc {
            0x8269FC00 => {
    //   block [0x8269FC00..0x8269FC18)
	// 8269FC00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FC04: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269FC08: 394A7A28  addi r10, r10, 0x7a28
	ctx.r[10].s64 = ctx.r[10].s64 + 31272;
	// 8269FC0C: 816B3190  lwz r11, 0x3190(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12688 as u32) ) } as u64;
	// 8269FC10: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 8269FC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FC18 size=116
    let mut pc: u32 = 0x8269FC18;
    'dispatch: loop {
        match pc {
            0x8269FC18 => {
    //   block [0x8269FC18..0x8269FC8C)
	// 8269FC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FC20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FC24: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FC28: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269FC2C: 390B7A28  addi r8, r11, 0x7a28
	ctx.r[8].s64 = ctx.r[11].s64 + 31272;
	// 8269FC30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FC34: 392AA848  addi r9, r10, -0x57b8
	ctx.r[9].s64 = ctx.r[10].s64 + -22456;
	// 8269FC38: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FC3C: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269FC40: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269FC44: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FC4C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FC54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FC5C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269FC60: 388AC7F8  addi r4, r10, -0x3808
	ctx.r[4].s64 = ctx.r[10].s64 + -14344;
	// 8269FC64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269FC68: 386B5A50  addi r3, r11, 0x5a50
	ctx.r[3].s64 = ctx.r[11].s64 + 23120;
	// 8269FC6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269FC70: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FC78: 4BDC71A9  bl 0x82466e20
	ctx.lr = 0x8269FC7C;
	sub_82466E20(ctx, base);
	// 8269FC7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FC80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FC84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FC90 size=112
    let mut pc: u32 = 0x8269FC90;
    'dispatch: loop {
        match pc {
            0x8269FC90 => {
    //   block [0x8269FC90..0x8269FD00)
	// 8269FC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FC9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FCA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FCA4: 38AA4910  addi r5, r10, 0x4910
	ctx.r[5].s64 = ctx.r[10].s64 + 18704;
	// 8269FCA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FCAC: 390B31B0  addi r8, r11, 0x31b0
	ctx.r[8].s64 = ctx.r[11].s64 + 12720;
	// 8269FCB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269FCB4: 388AC804  addi r4, r10, -0x37fc
	ctx.r[4].s64 = ctx.r[10].s64 + -14332;
	// 8269FCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FCBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FCC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FCC8: 386A5A80  addi r3, r10, 0x5a80
	ctx.r[3].s64 = ctx.r[10].s64 + 23168;
	// 8269FCCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FCE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FCEC: 4BDC7135  bl 0x82466e20
	ctx.lr = 0x8269FCF0;
	sub_82466E20(ctx, base);
	// 8269FCF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FD00 size=112
    let mut pc: u32 = 0x8269FD00;
    'dispatch: loop {
        match pc {
            0x8269FD00 => {
    //   block [0x8269FD00..0x8269FD70)
	// 8269FD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FD0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FD10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FD14: 38AA4910  addi r5, r10, 0x4910
	ctx.r[5].s64 = ctx.r[10].s64 + 18704;
	// 8269FD18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FD1C: 390B31F8  addi r8, r11, 0x31f8
	ctx.r[8].s64 = ctx.r[11].s64 + 12792;
	// 8269FD20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269FD24: 388AC81C  addi r4, r10, -0x37e4
	ctx.r[4].s64 = ctx.r[10].s64 + -14308;
	// 8269FD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FD2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FD30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FD38: 386A5AB0  addi r3, r10, 0x5ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 23216;
	// 8269FD3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FD40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FD48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FD50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FD54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FD58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FD5C: 4BDC70C5  bl 0x82466e20
	ctx.lr = 0x8269FD60;
	sub_82466E20(ctx, base);
	// 8269FD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FD70 size=112
    let mut pc: u32 = 0x8269FD70;
    'dispatch: loop {
        match pc {
            0x8269FD70 => {
    //   block [0x8269FD70..0x8269FDE0)
	// 8269FD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FD7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FD80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FD84: 38AA4940  addi r5, r10, 0x4940
	ctx.r[5].s64 = ctx.r[10].s64 + 18752;
	// 8269FD88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FD8C: 390B3258  addi r8, r11, 0x3258
	ctx.r[8].s64 = ctx.r[11].s64 + 12888;
	// 8269FD90: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269FD94: 388AC830  addi r4, r10, -0x37d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14288;
	// 8269FD98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FD9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FDA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FDA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FDA8: 386A5AE0  addi r3, r10, 0x5ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 23264;
	// 8269FDAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FDB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FDB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FDBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FDC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FDCC: 4BDC7055  bl 0x82466e20
	ctx.lr = 0x8269FDD0;
	sub_82466E20(ctx, base);
	// 8269FDD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FDE0 size=112
    let mut pc: u32 = 0x8269FDE0;
    'dispatch: loop {
        match pc {
            0x8269FDE0 => {
    //   block [0x8269FDE0..0x8269FE50)
	// 8269FDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FDEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FDF0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FDF4: 38AA4940  addi r5, r10, 0x4940
	ctx.r[5].s64 = ctx.r[10].s64 + 18752;
	// 8269FDF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FDFC: 390B32B8  addi r8, r11, 0x32b8
	ctx.r[8].s64 = ctx.r[11].s64 + 12984;
	// 8269FE00: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269FE04: 388AC840  addi r4, r10, -0x37c0
	ctx.r[4].s64 = ctx.r[10].s64 + -14272;
	// 8269FE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FE0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FE10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FE18: 386A5B10  addi r3, r10, 0x5b10
	ctx.r[3].s64 = ctx.r[10].s64 + 23312;
	// 8269FE1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FE20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FE28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FE30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FE38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FE3C: 4BDC6FE5  bl 0x82466e20
	ctx.lr = 0x8269FE40;
	sub_82466E20(ctx, base);
	// 8269FE40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FE50 size=112
    let mut pc: u32 = 0x8269FE50;
    'dispatch: loop {
        match pc {
            0x8269FE50 => {
    //   block [0x8269FE50..0x8269FEC0)
	// 8269FE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FE5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FE60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FE64: 38AA4940  addi r5, r10, 0x4940
	ctx.r[5].s64 = ctx.r[10].s64 + 18752;
	// 8269FE68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FE6C: 390B3378  addi r8, r11, 0x3378
	ctx.r[8].s64 = ctx.r[11].s64 + 13176;
	// 8269FE70: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269FE74: 388AC858  addi r4, r10, -0x37a8
	ctx.r[4].s64 = ctx.r[10].s64 + -14248;
	// 8269FE78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FE7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FE80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FE84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FE88: 386A5B40  addi r3, r10, 0x5b40
	ctx.r[3].s64 = ctx.r[10].s64 + 23360;
	// 8269FE8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FE90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FE94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FE98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FE9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FEA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FEA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FEA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FEAC: 4BDC6F75  bl 0x82466e20
	ctx.lr = 0x8269FEB0;
	sub_82466E20(ctx, base);
	// 8269FEB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FEB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FEB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FEC0 size=112
    let mut pc: u32 = 0x8269FEC0;
    'dispatch: loop {
        match pc {
            0x8269FEC0 => {
    //   block [0x8269FEC0..0x8269FF30)
	// 8269FEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FECC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FED0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FED4: 38AA4910  addi r5, r10, 0x4910
	ctx.r[5].s64 = ctx.r[10].s64 + 18704;
	// 8269FED8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FEDC: 390B33D8  addi r8, r11, 0x33d8
	ctx.r[8].s64 = ctx.r[11].s64 + 13272;
	// 8269FEE0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269FEE4: 388AC86C  addi r4, r10, -0x3794
	ctx.r[4].s64 = ctx.r[10].s64 + -14228;
	// 8269FEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FEEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FEF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FEF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FEF8: 386A5B70  addi r3, r10, 0x5b70
	ctx.r[3].s64 = ctx.r[10].s64 + 23408;
	// 8269FEFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FF00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FF04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FF08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FF0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FF10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FF14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FF18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FF1C: 4BDC6F05  bl 0x82466e20
	ctx.lr = 0x8269FF20;
	sub_82466E20(ctx, base);
	// 8269FF20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FF24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FF28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FF2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FF30 size=112
    let mut pc: u32 = 0x8269FF30;
    'dispatch: loop {
        match pc {
            0x8269FF30 => {
    //   block [0x8269FF30..0x8269FFA0)
	// 8269FF30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FF34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FF38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FF3C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269FF40: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 8269FF44: 38EA3498  addi r7, r10, 0x3498
	ctx.r[7].s64 = ctx.r[10].s64 + 13464;
	// 8269FF48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FF4C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269FF50: 388AC87C  addi r4, r10, -0x3784
	ctx.r[4].s64 = ctx.r[10].s64 + -14212;
	// 8269FF54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FF58: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269FF5C: 396BA860  addi r11, r11, -0x57a0
	ctx.r[11].s64 = ctx.r[11].s64 + -22432;
	// 8269FF60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269FF64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FF68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269FF6C: 386A5BA0  addi r3, r10, 0x5ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 23456;
	// 8269FF70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FF74: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269FF78: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FF7C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269FF80: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FF84: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FF88: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269FF8C: 4BDC6E95  bl 0x82466e20
	ctx.lr = 0x8269FF90;
	sub_82466E20(ctx, base);
	// 8269FF90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269FF94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269FF98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269FF9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269FFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269FFA0 size=112
    let mut pc: u32 = 0x8269FFA0;
    'dispatch: loop {
        match pc {
            0x8269FFA0 => {
    //   block [0x8269FFA0..0x826A0010)
	// 8269FFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269FFA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269FFA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269FFAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FFB0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269FFB4: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 8269FFB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269FFBC: 390B3660  addi r8, r11, 0x3660
	ctx.r[8].s64 = ctx.r[11].s64 + 13920;
	// 8269FFC0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269FFC4: 388AC894  addi r4, r10, -0x376c
	ctx.r[4].s64 = ctx.r[10].s64 + -14188;
	// 8269FFC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269FFCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269FFD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269FFD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269FFD8: 386A5BD0  addi r3, r10, 0x5bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 23504;
	// 8269FFDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269FFE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269FFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269FFE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269FFEC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269FFF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269FFF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269FFF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269FFFC: 4BDC6E25  bl 0x82466e20
	ctx.lr = 0x826A0000;
	sub_82466E20(ctx, base);
	// 826A0000: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0004: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0008: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A000C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0010 size=108
    let mut pc: u32 = 0x826A0010;
    'dispatch: loop {
        match pc {
            0x826A0010 => {
    //   block [0x826A0010..0x826A007C)
	// 826A0010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A001C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0020: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0024: 38EB3678  addi r7, r11, 0x3678
	ctx.r[7].s64 = ctx.r[11].s64 + 13944;
	// 826A0028: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A002C: 388AC8B0  addi r4, r10, -0x3750
	ctx.r[4].s64 = ctx.r[10].s64 + -14160;
	// 826A0030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0034: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0038: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A003C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0040: 386A5C00  addi r3, r10, 0x5c00
	ctx.r[3].s64 = ctx.r[10].s64 + 23552;
	// 826A0044: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A004C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0054: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A005C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0064: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0068: 4BDC6DB9  bl 0x82466e20
	ctx.lr = 0x826A006C;
	sub_82466E20(ctx, base);
	// 826A006C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0080 size=112
    let mut pc: u32 = 0x826A0080;
    'dispatch: loop {
        match pc {
            0x826A0080 => {
    //   block [0x826A0080..0x826A00F0)
	// 826A0080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A008C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0090: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0094: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 826A0098: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A009C: 390B36A8  addi r8, r11, 0x36a8
	ctx.r[8].s64 = ctx.r[11].s64 + 13992;
	// 826A00A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A00A4: 388AC8D8  addi r4, r10, -0x3728
	ctx.r[4].s64 = ctx.r[10].s64 + -14120;
	// 826A00A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A00AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A00B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A00B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A00B8: 386A5C30  addi r3, r10, 0x5c30
	ctx.r[3].s64 = ctx.r[10].s64 + 23600;
	// 826A00BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A00C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A00C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A00C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A00CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A00D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A00D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A00D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A00DC: 4BDC6D45  bl 0x82466e20
	ctx.lr = 0x826A00E0;
	sub_82466E20(ctx, base);
	// 826A00E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A00E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A00E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A00EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A00F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A00F0 size=108
    let mut pc: u32 = 0x826A00F0;
    'dispatch: loop {
        match pc {
            0x826A00F0 => {
    //   block [0x826A00F0..0x826A015C)
	// 826A00F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A00F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A00F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A00FC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0100: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0104: 38EB36C0  addi r7, r11, 0x36c0
	ctx.r[7].s64 = ctx.r[11].s64 + 14016;
	// 826A0108: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A010C: 388AC8F4  addi r4, r10, -0x370c
	ctx.r[4].s64 = ctx.r[10].s64 + -14092;
	// 826A0110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0114: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A011C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0120: 386A5C60  addi r3, r10, 0x5c60
	ctx.r[3].s64 = ctx.r[10].s64 + 23648;
	// 826A0124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A012C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A013C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0148: 4BDC6CD9  bl 0x82466e20
	ctx.lr = 0x826A014C;
	sub_82466E20(ctx, base);
	// 826A014C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0160 size=108
    let mut pc: u32 = 0x826A0160;
    'dispatch: loop {
        match pc {
            0x826A0160 => {
    //   block [0x826A0160..0x826A01CC)
	// 826A0160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A016C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0170: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0174: 38EB36F0  addi r7, r11, 0x36f0
	ctx.r[7].s64 = ctx.r[11].s64 + 14064;
	// 826A0178: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A017C: 388AC910  addi r4, r10, -0x36f0
	ctx.r[4].s64 = ctx.r[10].s64 + -14064;
	// 826A0180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0184: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A018C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0190: 386A5C90  addi r3, r10, 0x5c90
	ctx.r[3].s64 = ctx.r[10].s64 + 23696;
	// 826A0194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A019C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A01A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A01A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A01A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A01AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A01B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A01B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A01B8: 4BDC6C69  bl 0x82466e20
	ctx.lr = 0x826A01BC;
	sub_82466E20(ctx, base);
	// 826A01BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A01C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A01C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A01C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A01D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A01D0 size=112
    let mut pc: u32 = 0x826A01D0;
    'dispatch: loop {
        match pc {
            0x826A01D0 => {
    //   block [0x826A01D0..0x826A0240)
	// 826A01D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A01D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A01D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A01DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A01E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A01E4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A01E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A01EC: 390B3738  addi r8, r11, 0x3738
	ctx.r[8].s64 = ctx.r[11].s64 + 14136;
	// 826A01F0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A01F4: 388AC930  addi r4, r10, -0x36d0
	ctx.r[4].s64 = ctx.r[10].s64 + -14032;
	// 826A01F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A01FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0200: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0204: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0208: 386A5CC0  addi r3, r10, 0x5cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 23744;
	// 826A020C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A021C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A022C: 4BDC6BF5  bl 0x82466e20
	ctx.lr = 0x826A0230;
	sub_82466E20(ctx, base);
	// 826A0230: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A023C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0240 size=112
    let mut pc: u32 = 0x826A0240;
    'dispatch: loop {
        match pc {
            0x826A0240 => {
    //   block [0x826A0240..0x826A02B0)
	// 826A0240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A024C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0250: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0254: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 826A0258: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A025C: 390B3780  addi r8, r11, 0x3780
	ctx.r[8].s64 = ctx.r[11].s64 + 14208;
	// 826A0260: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A0264: 388AC948  addi r4, r10, -0x36b8
	ctx.r[4].s64 = ctx.r[10].s64 + -14008;
	// 826A0268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A026C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0278: 386A5CF0  addi r3, r10, 0x5cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 23792;
	// 826A027C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A028C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A0290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A029C: 4BDC6B85  bl 0x82466e20
	ctx.lr = 0x826A02A0;
	sub_82466E20(ctx, base);
	// 826A02A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A02A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A02A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A02AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A02B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A02B0 size=112
    let mut pc: u32 = 0x826A02B0;
    'dispatch: loop {
        match pc {
            0x826A02B0 => {
    //   block [0x826A02B0..0x826A0320)
	// 826A02B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A02B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A02B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A02BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A02C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A02C4: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 826A02C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A02CC: 390B3798  addi r8, r11, 0x3798
	ctx.r[8].s64 = ctx.r[11].s64 + 14232;
	// 826A02D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A02D4: 388AC968  addi r4, r10, -0x3698
	ctx.r[4].s64 = ctx.r[10].s64 + -13976;
	// 826A02D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A02DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A02E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A02E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A02E8: 386A5D20  addi r3, r10, 0x5d20
	ctx.r[3].s64 = ctx.r[10].s64 + 23840;
	// 826A02EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A02F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A02F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A02F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A02FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A030C: 4BDC6B15  bl 0x82466e20
	ctx.lr = 0x826A0310;
	sub_82466E20(ctx, base);
	// 826A0310: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A031C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0320 size=112
    let mut pc: u32 = 0x826A0320;
    'dispatch: loop {
        match pc {
            0x826A0320 => {
    //   block [0x826A0320..0x826A0390)
	// 826A0320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0324: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A032C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0330: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0334: 38AA5810  addi r5, r10, 0x5810
	ctx.r[5].s64 = ctx.r[10].s64 + 22544;
	// 826A0338: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A033C: 390B37C8  addi r8, r11, 0x37c8
	ctx.r[8].s64 = ctx.r[11].s64 + 14280;
	// 826A0340: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A0344: 388AC980  addi r4, r10, -0x3680
	ctx.r[4].s64 = ctx.r[10].s64 + -13952;
	// 826A0348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A034C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0350: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0354: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0358: 386A5D50  addi r3, r10, 0x5d50
	ctx.r[3].s64 = ctx.r[10].s64 + 23888;
	// 826A035C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A036C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A037C: 4BDC6AA5  bl 0x82466e20
	ctx.lr = 0x826A0380;
	sub_82466E20(ctx, base);
	// 826A0380: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0384: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0388: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A038C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0390 size=108
    let mut pc: u32 = 0x826A0390;
    'dispatch: loop {
        match pc {
            0x826A0390 => {
    //   block [0x826A0390..0x826A03FC)
	// 826A0390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A039C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A03A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A03A4: 38EB37E0  addi r7, r11, 0x37e0
	ctx.r[7].s64 = ctx.r[11].s64 + 14304;
	// 826A03A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A03AC: 388AC9A0  addi r4, r10, -0x3660
	ctx.r[4].s64 = ctx.r[10].s64 + -13920;
	// 826A03B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A03B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A03B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A03BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A03C0: 386A5D80  addi r3, r10, 0x5d80
	ctx.r[3].s64 = ctx.r[10].s64 + 23936;
	// 826A03C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A03C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A03CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A03D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A03D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A03D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A03DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A03E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A03E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A03E8: 4BDC6A39  bl 0x82466e20
	ctx.lr = 0x826A03EC;
	sub_82466E20(ctx, base);
	// 826A03EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A03F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A03F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A03F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0400 size=112
    let mut pc: u32 = 0x826A0400;
    'dispatch: loop {
        match pc {
            0x826A0400 => {
    //   block [0x826A0400..0x826A0470)
	// 826A0400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A040C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0410: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0414: 38AA5D80  addi r5, r10, 0x5d80
	ctx.r[5].s64 = ctx.r[10].s64 + 23936;
	// 826A0418: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A041C: 390B3810  addi r8, r11, 0x3810
	ctx.r[8].s64 = ctx.r[11].s64 + 14352;
	// 826A0420: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A0424: 388AC9BC  addi r4, r10, -0x3644
	ctx.r[4].s64 = ctx.r[10].s64 + -13892;
	// 826A0428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A042C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0438: 386A5DB0  addi r3, r10, 0x5db0
	ctx.r[3].s64 = ctx.r[10].s64 + 23984;
	// 826A043C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A044C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A045C: 4BDC69C5  bl 0x82466e20
	ctx.lr = 0x826A0460;
	sub_82466E20(ctx, base);
	// 826A0460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A046C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A0470 size=24
    let mut pc: u32 = 0x826A0470;
    'dispatch: loop {
        match pc {
            0x826A0470 => {
    //   block [0x826A0470..0x826A0488)
	// 826A0470: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0474: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826A0478: 394A7A70  addi r10, r10, 0x7a70
	ctx.r[10].s64 = ctx.r[10].s64 + 31344;
	// 826A047C: 816B31AC  lwz r11, 0x31ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12716 as u32) ) } as u64;
	// 826A0480: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826A0484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0488 size=116
    let mut pc: u32 = 0x826A0488;
    'dispatch: loop {
        match pc {
            0x826A0488 => {
    //   block [0x826A0488..0x826A04FC)
	// 826A0488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A048C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0494: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0498: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A049C: 390B7A70  addi r8, r11, 0x7a70
	ctx.r[8].s64 = ctx.r[11].s64 + 31344;
	// 826A04A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A04A4: 392AA900  addi r9, r10, -0x5700
	ctx.r[9].s64 = ctx.r[10].s64 + -22272;
	// 826A04A8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A04AC: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 826A04B0: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A04B4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A04B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A04BC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A04C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A04C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A04C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A04CC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A04D0: 388ACA40  addi r4, r10, -0x35c0
	ctx.r[4].s64 = ctx.r[10].s64 + -13760;
	// 826A04D4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A04D8: 386B5DE0  addi r3, r11, 0x5de0
	ctx.r[3].s64 = ctx.r[11].s64 + 24032;
	// 826A04DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A04E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A04E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A04E8: 4BDC6939  bl 0x82466e20
	ctx.lr = 0x826A04EC;
	sub_82466E20(ctx, base);
	// 826A04EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A04F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A04F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A04F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0500 size=108
    let mut pc: u32 = 0x826A0500;
    'dispatch: loop {
        match pc {
            0x826A0500 => {
    //   block [0x826A0500..0x826A056C)
	// 826A0500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A050C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0510: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0514: 38EB3840  addi r7, r11, 0x3840
	ctx.r[7].s64 = ctx.r[11].s64 + 14400;
	// 826A0518: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A051C: 388ACA5C  addi r4, r10, -0x35a4
	ctx.r[4].s64 = ctx.r[10].s64 + -13732;
	// 826A0520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0524: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A052C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0530: 386A5E10  addi r3, r10, 0x5e10
	ctx.r[3].s64 = ctx.r[10].s64 + 24080;
	// 826A0534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A053C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A054C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0558: 4BDC68C9  bl 0x82466e20
	ctx.lr = 0x826A055C;
	sub_82466E20(ctx, base);
	// 826A055C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0570 size=108
    let mut pc: u32 = 0x826A0570;
    'dispatch: loop {
        match pc {
            0x826A0570 => {
    //   block [0x826A0570..0x826A05DC)
	// 826A0570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A057C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0580: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0584: 38EB3888  addi r7, r11, 0x3888
	ctx.r[7].s64 = ctx.r[11].s64 + 14472;
	// 826A0588: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A058C: 388ACA78  addi r4, r10, -0x3588
	ctx.r[4].s64 = ctx.r[10].s64 + -13704;
	// 826A0590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A059C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A05A0: 386A5E40  addi r3, r10, 0x5e40
	ctx.r[3].s64 = ctx.r[10].s64 + 24128;
	// 826A05A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A05A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A05AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A05B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A05B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A05B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A05BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A05C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A05C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A05C8: 4BDC6859  bl 0x82466e20
	ctx.lr = 0x826A05CC;
	sub_82466E20(ctx, base);
	// 826A05CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A05D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A05D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A05D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A05E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A05E0 size=108
    let mut pc: u32 = 0x826A05E0;
    'dispatch: loop {
        match pc {
            0x826A05E0 => {
    //   block [0x826A05E0..0x826A064C)
	// 826A05E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A05E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A05E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A05EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A05F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A05F4: 38EB38B8  addi r7, r11, 0x38b8
	ctx.r[7].s64 = ctx.r[11].s64 + 14520;
	// 826A05F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A05FC: 388ACA98  addi r4, r10, -0x3568
	ctx.r[4].s64 = ctx.r[10].s64 + -13672;
	// 826A0600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A060C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0610: 386A5E70  addi r3, r10, 0x5e70
	ctx.r[3].s64 = ctx.r[10].s64 + 24176;
	// 826A0614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A061C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A062C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0638: 4BDC67E9  bl 0x82466e20
	ctx.lr = 0x826A063C;
	sub_82466E20(ctx, base);
	// 826A063C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0650 size=112
    let mut pc: u32 = 0x826A0650;
    'dispatch: loop {
        match pc {
            0x826A0650 => {
    //   block [0x826A0650..0x826A06C0)
	// 826A0650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A065C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0660: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0664: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A066C: 390B38E8  addi r8, r11, 0x38e8
	ctx.r[8].s64 = ctx.r[11].s64 + 14568;
	// 826A0670: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A0674: 388ACAB0  addi r4, r10, -0x3550
	ctx.r[4].s64 = ctx.r[10].s64 + -13648;
	// 826A0678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A067C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0680: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0688: 386A5EA0  addi r3, r10, 0x5ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 24224;
	// 826A068C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0694: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A069C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A06A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A06A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A06A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A06AC: 4BDC6775  bl 0x82466e20
	ctx.lr = 0x826A06B0;
	sub_82466E20(ctx, base);
	// 826A06B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A06B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A06B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A06BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A06C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A06C0 size=112
    let mut pc: u32 = 0x826A06C0;
    'dispatch: loop {
        match pc {
            0x826A06C0 => {
    //   block [0x826A06C0..0x826A0730)
	// 826A06C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A06C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A06C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A06CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A06D0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A06D4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A06D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A06DC: 390B3918  addi r8, r11, 0x3918
	ctx.r[8].s64 = ctx.r[11].s64 + 14616;
	// 826A06E0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A06E4: 388ACAC0  addi r4, r10, -0x3540
	ctx.r[4].s64 = ctx.r[10].s64 + -13632;
	// 826A06E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A06EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A06F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A06F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A06F8: 386A5ED0  addi r3, r10, 0x5ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 24272;
	// 826A06FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A070C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A071C: 4BDC6705  bl 0x82466e20
	ctx.lr = 0x826A0720;
	sub_82466E20(ctx, base);
	// 826A0720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A072C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0730 size=112
    let mut pc: u32 = 0x826A0730;
    'dispatch: loop {
        match pc {
            0x826A0730 => {
    //   block [0x826A0730..0x826A07A0)
	// 826A0730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A073C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0740: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0744: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0748: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A074C: 390B3930  addi r8, r11, 0x3930
	ctx.r[8].s64 = ctx.r[11].s64 + 14640;
	// 826A0750: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A0754: 388ACADC  addi r4, r10, -0x3524
	ctx.r[4].s64 = ctx.r[10].s64 + -13604;
	// 826A0758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A075C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0768: 386A5F00  addi r3, r10, 0x5f00
	ctx.r[3].s64 = ctx.r[10].s64 + 24320;
	// 826A076C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A077C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A078C: 4BDC6695  bl 0x82466e20
	ctx.lr = 0x826A0790;
	sub_82466E20(ctx, base);
	// 826A0790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A079C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A07A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A07A0 size=108
    let mut pc: u32 = 0x826A07A0;
    'dispatch: loop {
        match pc {
            0x826A07A0 => {
    //   block [0x826A07A0..0x826A080C)
	// 826A07A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A07A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A07A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A07AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A07B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A07B4: 38EB3948  addi r7, r11, 0x3948
	ctx.r[7].s64 = ctx.r[11].s64 + 14664;
	// 826A07B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A07BC: 388ACAFC  addi r4, r10, -0x3504
	ctx.r[4].s64 = ctx.r[10].s64 + -13572;
	// 826A07C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A07C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A07C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A07CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A07D0: 386A5F30  addi r3, r10, 0x5f30
	ctx.r[3].s64 = ctx.r[10].s64 + 24368;
	// 826A07D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A07D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A07DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A07E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A07E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A07E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A07EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A07F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A07F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A07F8: 4BDC6629  bl 0x82466e20
	ctx.lr = 0x826A07FC;
	sub_82466E20(ctx, base);
	// 826A07FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0810 size=112
    let mut pc: u32 = 0x826A0810;
    'dispatch: loop {
        match pc {
            0x826A0810 => {
    //   block [0x826A0810..0x826A0880)
	// 826A0810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A081C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0820: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0824: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0828: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A082C: 390B3978  addi r8, r11, 0x3978
	ctx.r[8].s64 = ctx.r[11].s64 + 14712;
	// 826A0830: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A0834: 388ACB34  addi r4, r10, -0x34cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13516;
	// 826A0838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A083C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0848: 386A5F60  addi r3, r10, 0x5f60
	ctx.r[3].s64 = ctx.r[10].s64 + 24416;
	// 826A084C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A085C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A086C: 4BDC65B5  bl 0x82466e20
	ctx.lr = 0x826A0870;
	sub_82466E20(ctx, base);
	// 826A0870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A087C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0880 size=108
    let mut pc: u32 = 0x826A0880;
    'dispatch: loop {
        match pc {
            0x826A0880 => {
    //   block [0x826A0880..0x826A08EC)
	// 826A0880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A088C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0890: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0894: 38EB3990  addi r7, r11, 0x3990
	ctx.r[7].s64 = ctx.r[11].s64 + 14736;
	// 826A0898: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826A089C: 388ACB58  addi r4, r10, -0x34a8
	ctx.r[4].s64 = ctx.r[10].s64 + -13480;
	// 826A08A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A08A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A08A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A08AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A08B0: 386A5F90  addi r3, r10, 0x5f90
	ctx.r[3].s64 = ctx.r[10].s64 + 24464;
	// 826A08B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A08B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A08BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A08C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A08C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A08C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A08CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A08D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A08D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A08D8: 4BDC6549  bl 0x82466e20
	ctx.lr = 0x826A08DC;
	sub_82466E20(ctx, base);
	// 826A08DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A08E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A08E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A08E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A08F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A08F0 size=116
    let mut pc: u32 = 0x826A08F0;
    'dispatch: loop {
        match pc {
            0x826A08F0 => {
    //   block [0x826A08F0..0x826A0964)
	// 826A08F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A08F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A08F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A08FC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826A0900: 38E00013  li r7, 0x13
	ctx.r[7].s64 = 19;
	// 826A0904: 390A3A80  addi r8, r10, 0x3a80
	ctx.r[8].s64 = ctx.r[10].s64 + 14976;
	// 826A0908: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A090C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A0910: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0914: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0918: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A091C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0924: 388ACB7C  addi r4, r10, -0x3484
	ctx.r[4].s64 = ctx.r[10].s64 + -13444;
	// 826A0928: 396BA918  addi r11, r11, -0x56e8
	ctx.r[11].s64 = ctx.r[11].s64 + -22248;
	// 826A092C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0930: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0934: 386A5FC0  addi r3, r10, 0x5fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 24512;
	// 826A0938: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A093C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0940: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A0944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A094C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0950: 4BDC64D1  bl 0x82466e20
	ctx.lr = 0x826A0954;
	sub_82466E20(ctx, base);
	// 826A0954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A095C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0968 size=108
    let mut pc: u32 = 0x826A0968;
    'dispatch: loop {
        match pc {
            0x826A0968 => {
    //   block [0x826A0968..0x826A09D4)
	// 826A0968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A096C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0974: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0978: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A097C: 38EB3C48  addi r7, r11, 0x3c48
	ctx.r[7].s64 = ctx.r[11].s64 + 15432;
	// 826A0980: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826A0984: 388ACB8C  addi r4, r10, -0x3474
	ctx.r[4].s64 = ctx.r[10].s64 + -13428;
	// 826A0988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A098C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0998: 386A5FF0  addi r3, r10, 0x5ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 24560;
	// 826A099C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A09A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A09A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A09A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A09AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A09B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A09B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A09B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A09BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A09C0: 4BDC6461  bl 0x82466e20
	ctx.lr = 0x826A09C4;
	sub_82466E20(ctx, base);
	// 826A09C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A09C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A09CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A09D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A09D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A09D8 size=112
    let mut pc: u32 = 0x826A09D8;
    'dispatch: loop {
        match pc {
            0x826A09D8 => {
    //   block [0x826A09D8..0x826A0A48)
	// 826A09D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A09DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A09E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A09E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A09E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A09EC: 38AA4940  addi r5, r10, 0x4940
	ctx.r[5].s64 = ctx.r[10].s64 + 18752;
	// 826A09F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A09F4: 390B3DE0  addi r8, r11, 0x3de0
	ctx.r[8].s64 = ctx.r[11].s64 + 15840;
	// 826A09F8: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826A09FC: 388ACBA8  addi r4, r10, -0x3458
	ctx.r[4].s64 = ctx.r[10].s64 + -13400;
	// 826A0A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0A04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0A08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0A0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0A10: 386A6020  addi r3, r10, 0x6020
	ctx.r[3].s64 = ctx.r[10].s64 + 24608;
	// 826A0A14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0A18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0A1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0A20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0A28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0A30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0A34: 4BDC63ED  bl 0x82466e20
	ctx.lr = 0x826A0A38;
	sub_82466E20(ctx, base);
	// 826A0A38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0A3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0A40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0A44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0A48 size=100
    let mut pc: u32 = 0x826A0A48;
    'dispatch: loop {
        match pc {
            0x826A0A48 => {
    //   block [0x826A0A48..0x826A0AAC)
	// 826A0A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0A54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0A5C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0A60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0A64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0A68: 388ACBBC  addi r4, r10, -0x3444
	ctx.r[4].s64 = ctx.r[10].s64 + -13380;
	// 826A0A6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0A70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0A74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0A78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0A7C: 386A6050  addi r3, r10, 0x6050
	ctx.r[3].s64 = ctx.r[10].s64 + 24656;
	// 826A0A80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0A84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0A88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0A90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0A98: 4BDC6389  bl 0x82466e20
	ctx.lr = 0x826A0A9C;
	sub_82466E20(ctx, base);
	// 826A0A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0AB0 size=112
    let mut pc: u32 = 0x826A0AB0;
    'dispatch: loop {
        match pc {
            0x826A0AB0 => {
    //   block [0x826A0AB0..0x826A0B20)
	// 826A0AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0ABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0AC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0AC4: 38AA6050  addi r5, r10, 0x6050
	ctx.r[5].s64 = ctx.r[10].s64 + 24656;
	// 826A0AC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0ACC: 390B4038  addi r8, r11, 0x4038
	ctx.r[8].s64 = ctx.r[11].s64 + 16440;
	// 826A0AD0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826A0AD4: 388ACBD4  addi r4, r10, -0x342c
	ctx.r[4].s64 = ctx.r[10].s64 + -13356;
	// 826A0AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0ADC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0AE8: 386A6080  addi r3, r10, 0x6080
	ctx.r[3].s64 = ctx.r[10].s64 + 24704;
	// 826A0AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0B0C: 4BDC6315  bl 0x82466e20
	ctx.lr = 0x826A0B10;
	sub_82466E20(ctx, base);
	// 826A0B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0B20 size=100
    let mut pc: u32 = 0x826A0B20;
    'dispatch: loop {
        match pc {
            0x826A0B20 => {
    //   block [0x826A0B20..0x826A0B84)
	// 826A0B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0B2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0B30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0B34: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0B38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0B40: 388ACBF4  addi r4, r10, -0x340c
	ctx.r[4].s64 = ctx.r[10].s64 + -13324;
	// 826A0B44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0B48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0B4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0B50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0B54: 386A60B0  addi r3, r10, 0x60b0
	ctx.r[3].s64 = ctx.r[10].s64 + 24752;
	// 826A0B58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0B5C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0B60: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0B68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0B70: 4BDC62B1  bl 0x82466e20
	ctx.lr = 0x826A0B74;
	sub_82466E20(ctx, base);
	// 826A0B74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0B78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0B7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0B88 size=108
    let mut pc: u32 = 0x826A0B88;
    'dispatch: loop {
        match pc {
            0x826A0B88 => {
    //   block [0x826A0B88..0x826A0BF4)
	// 826A0B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0B90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0B94: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0B98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0B9C: 38EB40B0  addi r7, r11, 0x40b0
	ctx.r[7].s64 = ctx.r[11].s64 + 16560;
	// 826A0BA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A0BA4: 388ACC04  addi r4, r10, -0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + -13308;
	// 826A0BA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0BAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0BB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0BB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0BB8: 386A60E0  addi r3, r10, 0x60e0
	ctx.r[3].s64 = ctx.r[10].s64 + 24800;
	// 826A0BBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0BC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0BC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0BC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0BCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0BD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0BD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0BD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0BDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0BE0: 4BDC6241  bl 0x82466e20
	ctx.lr = 0x826A0BE4;
	sub_82466E20(ctx, base);
	// 826A0BE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0BF8 size=112
    let mut pc: u32 = 0x826A0BF8;
    'dispatch: loop {
        match pc {
            0x826A0BF8 => {
    //   block [0x826A0BF8..0x826A0C68)
	// 826A0BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0C00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0C04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0C08: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0C0C: 38AA60B0  addi r5, r10, 0x60b0
	ctx.r[5].s64 = ctx.r[10].s64 + 24752;
	// 826A0C10: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0C14: 390B40F8  addi r8, r11, 0x40f8
	ctx.r[8].s64 = ctx.r[11].s64 + 16632;
	// 826A0C18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A0C1C: 388ACC34  addi r4, r10, -0x33cc
	ctx.r[4].s64 = ctx.r[10].s64 + -13260;
	// 826A0C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0C24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0C28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0C30: 386A6110  addi r3, r10, 0x6110
	ctx.r[3].s64 = ctx.r[10].s64 + 24848;
	// 826A0C34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0C3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0C44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0C54: 4BDC61CD  bl 0x82466e20
	ctx.lr = 0x826A0C58;
	sub_82466E20(ctx, base);
	// 826A0C58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0C64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0C68 size=100
    let mut pc: u32 = 0x826A0C68;
    'dispatch: loop {
        match pc {
            0x826A0C68 => {
    //   block [0x826A0C68..0x826A0CCC)
	// 826A0C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0C70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0C74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0C7C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0C80: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0C88: 388ACC4C  addi r4, r10, -0x33b4
	ctx.r[4].s64 = ctx.r[10].s64 + -13236;
	// 826A0C8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0C9C: 386A6140  addi r3, r10, 0x6140
	ctx.r[3].s64 = ctx.r[10].s64 + 24896;
	// 826A0CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0CA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0CA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0CAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0CB0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0CB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0CB8: 4BDC6169  bl 0x82466e20
	ctx.lr = 0x826A0CBC;
	sub_82466E20(ctx, base);
	// 826A0CBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0CD0 size=100
    let mut pc: u32 = 0x826A0CD0;
    'dispatch: loop {
        match pc {
            0x826A0CD0 => {
    //   block [0x826A0CD0..0x826A0D34)
	// 826A0CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0CD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0CDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0CE4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0CE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0CF0: 388ACC68  addi r4, r10, -0x3398
	ctx.r[4].s64 = ctx.r[10].s64 + -13208;
	// 826A0CF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0D04: 386A6170  addi r3, r10, 0x6170
	ctx.r[3].s64 = ctx.r[10].s64 + 24944;
	// 826A0D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0D0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0D10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0D14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0D18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0D20: 4BDC6101  bl 0x82466e20
	ctx.lr = 0x826A0D24;
	sub_82466E20(ctx, base);
	// 826A0D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0D38 size=112
    let mut pc: u32 = 0x826A0D38;
    'dispatch: loop {
        match pc {
            0x826A0D38 => {
    //   block [0x826A0D38..0x826A0DA8)
	// 826A0D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0D44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0D48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0D4C: 38AA6140  addi r5, r10, 0x6140
	ctx.r[5].s64 = ctx.r[10].s64 + 24896;
	// 826A0D50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0D54: 390B4128  addi r8, r11, 0x4128
	ctx.r[8].s64 = ctx.r[11].s64 + 16680;
	// 826A0D58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A0D5C: 388ACC80  addi r4, r10, -0x3380
	ctx.r[4].s64 = ctx.r[10].s64 + -13184;
	// 826A0D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0D64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0D70: 386A61A0  addi r3, r10, 0x61a0
	ctx.r[3].s64 = ctx.r[10].s64 + 24992;
	// 826A0D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0D94: 4BDC608D  bl 0x82466e20
	ctx.lr = 0x826A0D98;
	sub_82466E20(ctx, base);
	// 826A0D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0DA8 size=112
    let mut pc: u32 = 0x826A0DA8;
    'dispatch: loop {
        match pc {
            0x826A0DA8 => {
    //   block [0x826A0DA8..0x826A0E18)
	// 826A0DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0DB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0DB8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0DBC: 38AA6170  addi r5, r10, 0x6170
	ctx.r[5].s64 = ctx.r[10].s64 + 24944;
	// 826A0DC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0DC4: 390B4188  addi r8, r11, 0x4188
	ctx.r[8].s64 = ctx.r[11].s64 + 16776;
	// 826A0DC8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A0DCC: 388ACCA4  addi r4, r10, -0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + -13148;
	// 826A0DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0DD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0DD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0DE0: 386A61D0  addi r3, r10, 0x61d0
	ctx.r[3].s64 = ctx.r[10].s64 + 25040;
	// 826A0DE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0DEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0DF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0DF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0E00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0E04: 4BDC601D  bl 0x82466e20
	ctx.lr = 0x826A0E08;
	sub_82466E20(ctx, base);
	// 826A0E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0E18 size=100
    let mut pc: u32 = 0x826A0E18;
    'dispatch: loop {
        match pc {
            0x826A0E18 => {
    //   block [0x826A0E18..0x826A0E7C)
	// 826A0E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0E24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0E2C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A0E30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0E38: 388ACCC8  addi r4, r10, -0x3338
	ctx.r[4].s64 = ctx.r[10].s64 + -13112;
	// 826A0E3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0E44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0E4C: 386A6200  addi r3, r10, 0x6200
	ctx.r[3].s64 = ctx.r[10].s64 + 25088;
	// 826A0E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0E54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0E58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A0E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0E60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A0E64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0E68: 4BDC5FB9  bl 0x82466e20
	ctx.lr = 0x826A0E6C;
	sub_82466E20(ctx, base);
	// 826A0E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0E80 size=112
    let mut pc: u32 = 0x826A0E80;
    'dispatch: loop {
        match pc {
            0x826A0E80 => {
    //   block [0x826A0E80..0x826A0EF0)
	// 826A0E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0E8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0E90: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0E94: 38AA6200  addi r5, r10, 0x6200
	ctx.r[5].s64 = ctx.r[10].s64 + 25088;
	// 826A0E98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0E9C: 390B41E8  addi r8, r11, 0x41e8
	ctx.r[8].s64 = ctx.r[11].s64 + 16872;
	// 826A0EA0: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826A0EA4: 388ACCDC  addi r4, r10, -0x3324
	ctx.r[4].s64 = ctx.r[10].s64 + -13092;
	// 826A0EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0EAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0EB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A0EB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0EB8: 386A6230  addi r3, r10, 0x6230
	ctx.r[3].s64 = ctx.r[10].s64 + 25136;
	// 826A0EBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A0EC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0EC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0ECC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0ED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0ED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0EDC: 4BDC5F45  bl 0x82466e20
	ctx.lr = 0x826A0EE0;
	sub_82466E20(ctx, base);
	// 826A0EE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0EF0 size=108
    let mut pc: u32 = 0x826A0EF0;
    'dispatch: loop {
        match pc {
            0x826A0EF0 => {
    //   block [0x826A0EF0..0x826A0F5C)
	// 826A0EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0EFC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0F00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0F04: 38EB42D8  addi r7, r11, 0x42d8
	ctx.r[7].s64 = ctx.r[11].s64 + 17112;
	// 826A0F08: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826A0F0C: 388ACCF4  addi r4, r10, -0x330c
	ctx.r[4].s64 = ctx.r[10].s64 + -13068;
	// 826A0F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0F14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0F20: 386A6260  addi r3, r10, 0x6260
	ctx.r[3].s64 = ctx.r[10].s64 + 25184;
	// 826A0F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0F48: 4BDC5ED9  bl 0x82466e20
	ctx.lr = 0x826A0F4C;
	sub_82466E20(ctx, base);
	// 826A0F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0F60 size=108
    let mut pc: u32 = 0x826A0F60;
    'dispatch: loop {
        match pc {
            0x826A0F60 => {
    //   block [0x826A0F60..0x826A0FCC)
	// 826A0F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0F6C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0F70: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0F74: 38EB43C8  addi r7, r11, 0x43c8
	ctx.r[7].s64 = ctx.r[11].s64 + 17352;
	// 826A0F78: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A0F7C: 388ACD24  addi r4, r10, -0x32dc
	ctx.r[4].s64 = ctx.r[10].s64 + -13020;
	// 826A0F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A0F90: 386A6290  addi r3, r10, 0x6290
	ctx.r[3].s64 = ctx.r[10].s64 + 25232;
	// 826A0F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A0F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A0F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A0FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A0FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A0FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A0FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A0FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A0FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A0FB8: 4BDC5E69  bl 0x82466e20
	ctx.lr = 0x826A0FBC;
	sub_82466E20(ctx, base);
	// 826A0FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A0FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A0FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A0FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A0FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A0FD0 size=108
    let mut pc: u32 = 0x826A0FD0;
    'dispatch: loop {
        match pc {
            0x826A0FD0 => {
    //   block [0x826A0FD0..0x826A103C)
	// 826A0FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A0FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A0FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A0FDC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A0FE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A0FE4: 38EB4410  addi r7, r11, 0x4410
	ctx.r[7].s64 = ctx.r[11].s64 + 17424;
	// 826A0FE8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A0FEC: 388ACD44  addi r4, r10, -0x32bc
	ctx.r[4].s64 = ctx.r[10].s64 + -12988;
	// 826A0FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A0FF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A0FF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A0FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1000: 386A62C0  addi r3, r10, 0x62c0
	ctx.r[3].s64 = ctx.r[10].s64 + 25280;
	// 826A1004: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A100C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A101C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1028: 4BDC5DF9  bl 0x82466e20
	ctx.lr = 0x826A102C;
	sub_82466E20(ctx, base);
	// 826A102C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1040 size=108
    let mut pc: u32 = 0x826A1040;
    'dispatch: loop {
        match pc {
            0x826A1040 => {
    //   block [0x826A1040..0x826A10AC)
	// 826A1040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A104C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1050: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1054: 38EB44E8  addi r7, r11, 0x44e8
	ctx.r[7].s64 = ctx.r[11].s64 + 17640;
	// 826A1058: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A105C: 388ACD68  addi r4, r10, -0x3298
	ctx.r[4].s64 = ctx.r[10].s64 + -12952;
	// 826A1060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A106C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1070: 386A62F0  addi r3, r10, 0x62f0
	ctx.r[3].s64 = ctx.r[10].s64 + 25328;
	// 826A1074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A107C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A108C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1098: 4BDC5D89  bl 0x82466e20
	ctx.lr = 0x826A109C;
	sub_82466E20(ctx, base);
	// 826A109C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A10A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A10A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A10A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A10B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A10B0 size=100
    let mut pc: u32 = 0x826A10B0;
    'dispatch: loop {
        match pc {
            0x826A10B0 => {
    //   block [0x826A10B0..0x826A1114)
	// 826A10B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A10B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A10B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A10BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A10C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A10C4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A10C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A10CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A10D0: 388ACD84  addi r4, r10, -0x327c
	ctx.r[4].s64 = ctx.r[10].s64 + -12924;
	// 826A10D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A10D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A10DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A10E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A10E4: 386A6320  addi r3, r10, 0x6320
	ctx.r[3].s64 = ctx.r[10].s64 + 25376;
	// 826A10E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A10EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A10F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A10F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A10F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A10FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1100: 4BDC5D21  bl 0x82466e20
	ctx.lr = 0x826A1104;
	sub_82466E20(ctx, base);
	// 826A1104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A110C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1118 size=112
    let mut pc: u32 = 0x826A1118;
    'dispatch: loop {
        match pc {
            0x826A1118 => {
    //   block [0x826A1118..0x826A1188)
	// 826A1118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A111C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1124: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1128: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A112C: 38AA6320  addi r5, r10, 0x6320
	ctx.r[5].s64 = ctx.r[10].s64 + 25376;
	// 826A1130: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1134: 390B4500  addi r8, r11, 0x4500
	ctx.r[8].s64 = ctx.r[11].s64 + 17664;
	// 826A1138: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A113C: 388ACD98  addi r4, r10, -0x3268
	ctx.r[4].s64 = ctx.r[10].s64 + -12904;
	// 826A1140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1144: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1148: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A114C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1150: 386A6350  addi r3, r10, 0x6350
	ctx.r[3].s64 = ctx.r[10].s64 + 25424;
	// 826A1154: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A115C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A116C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1174: 4BDC5CAD  bl 0x82466e20
	ctx.lr = 0x826A1178;
	sub_82466E20(ctx, base);
	// 826A1178: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A117C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1188 size=108
    let mut pc: u32 = 0x826A1188;
    'dispatch: loop {
        match pc {
            0x826A1188 => {
    //   block [0x826A1188..0x826A11F4)
	// 826A1188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A118C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1190: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1194: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1198: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A119C: 38EB4548  addi r7, r11, 0x4548
	ctx.r[7].s64 = ctx.r[11].s64 + 17736;
	// 826A11A0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A11A4: 388ACDB4  addi r4, r10, -0x324c
	ctx.r[4].s64 = ctx.r[10].s64 + -12876;
	// 826A11A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A11AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A11B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A11B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A11B8: 386A6380  addi r3, r10, 0x6380
	ctx.r[3].s64 = ctx.r[10].s64 + 25472;
	// 826A11BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A11C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A11C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A11C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A11CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A11D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A11D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A11D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A11DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A11E0: 4BDC5C41  bl 0x82466e20
	ctx.lr = 0x826A11E4;
	sub_82466E20(ctx, base);
	// 826A11E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A11E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A11EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A11F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A11F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A11F8 size=112
    let mut pc: u32 = 0x826A11F8;
    'dispatch: loop {
        match pc {
            0x826A11F8 => {
    //   block [0x826A11F8..0x826A1268)
	// 826A11F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A11FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1204: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1208: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A120C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1210: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1214: 390B4590  addi r8, r11, 0x4590
	ctx.r[8].s64 = ctx.r[11].s64 + 17808;
	// 826A1218: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A121C: 388ACDE4  addi r4, r10, -0x321c
	ctx.r[4].s64 = ctx.r[10].s64 + -12828;
	// 826A1220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1228: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A122C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1230: 386A63B0  addi r3, r10, 0x63b0
	ctx.r[3].s64 = ctx.r[10].s64 + 25520;
	// 826A1234: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A123C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A124C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1254: 4BDC5BCD  bl 0x82466e20
	ctx.lr = 0x826A1258;
	sub_82466E20(ctx, base);
	// 826A1258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A125C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1268 size=108
    let mut pc: u32 = 0x826A1268;
    'dispatch: loop {
        match pc {
            0x826A1268 => {
    //   block [0x826A1268..0x826A12D4)
	// 826A1268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A126C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1274: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A127C: 38EB45A8  addi r7, r11, 0x45a8
	ctx.r[7].s64 = ctx.r[11].s64 + 17832;
	// 826A1280: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A1284: 388ACDFC  addi r4, r10, -0x3204
	ctx.r[4].s64 = ctx.r[10].s64 + -12804;
	// 826A1288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A128C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1290: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1294: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1298: 386A63E0  addi r3, r10, 0x63e0
	ctx.r[3].s64 = ctx.r[10].s64 + 25568;
	// 826A129C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A12A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A12A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A12A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A12AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A12B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A12B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A12B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A12BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A12C0: 4BDC5B61  bl 0x82466e20
	ctx.lr = 0x826A12C4;
	sub_82466E20(ctx, base);
	// 826A12C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A12C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A12CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A12D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A12D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A12D8 size=112
    let mut pc: u32 = 0x826A12D8;
    'dispatch: loop {
        match pc {
            0x826A12D8 => {
    //   block [0x826A12D8..0x826A1348)
	// 826A12D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A12DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A12E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A12E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A12E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A12EC: 38AA63B0  addi r5, r10, 0x63b0
	ctx.r[5].s64 = ctx.r[10].s64 + 25520;
	// 826A12F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A12F4: 390B45F0  addi r8, r11, 0x45f0
	ctx.r[8].s64 = ctx.r[11].s64 + 17904;
	// 826A12F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A12FC: 388ACE38  addi r4, r10, -0x31c8
	ctx.r[4].s64 = ctx.r[10].s64 + -12744;
	// 826A1300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1304: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A130C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1310: 386A6410  addi r3, r10, 0x6410
	ctx.r[3].s64 = ctx.r[10].s64 + 25616;
	// 826A1314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A131C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A132C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1334: 4BDC5AED  bl 0x82466e20
	ctx.lr = 0x826A1338;
	sub_82466E20(ctx, base);
	// 826A1338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A133C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1348 size=100
    let mut pc: u32 = 0x826A1348;
    'dispatch: loop {
        match pc {
            0x826A1348 => {
    //   block [0x826A1348..0x826A13AC)
	// 826A1348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A134C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1354: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1358: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A135C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1360: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1368: 388ACE54  addi r4, r10, -0x31ac
	ctx.r[4].s64 = ctx.r[10].s64 + -12716;
	// 826A136C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1374: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A137C: 386A6440  addi r3, r10, 0x6440
	ctx.r[3].s64 = ctx.r[10].s64 + 25664;
	// 826A1380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1384: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1388: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A138C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A1394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1398: 4BDC5A89  bl 0x82466e20
	ctx.lr = 0x826A139C;
	sub_82466E20(ctx, base);
	// 826A139C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A13A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A13A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A13A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A13B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A13B0 size=112
    let mut pc: u32 = 0x826A13B0;
    'dispatch: loop {
        match pc {
            0x826A13B0 => {
    //   block [0x826A13B0..0x826A1420)
	// 826A13B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A13B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A13B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A13BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A13C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A13C4: 38AA6440  addi r5, r10, 0x6440
	ctx.r[5].s64 = ctx.r[10].s64 + 25664;
	// 826A13C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A13CC: 390B4608  addi r8, r11, 0x4608
	ctx.r[8].s64 = ctx.r[11].s64 + 17928;
	// 826A13D0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826A13D4: 388ACE6C  addi r4, r10, -0x3194
	ctx.r[4].s64 = ctx.r[10].s64 + -12692;
	// 826A13D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A13DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A13E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A13E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A13E8: 386A6470  addi r3, r10, 0x6470
	ctx.r[3].s64 = ctx.r[10].s64 + 25712;
	// 826A13EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A13F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A13F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A13F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A13FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A140C: 4BDC5A15  bl 0x82466e20
	ctx.lr = 0x826A1410;
	sub_82466E20(ctx, base);
	// 826A1410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A141C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1420 size=108
    let mut pc: u32 = 0x826A1420;
    'dispatch: loop {
        match pc {
            0x826A1420 => {
    //   block [0x826A1420..0x826A148C)
	// 826A1420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A142C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1430: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1434: 38EB46B0  addi r7, r11, 0x46b0
	ctx.r[7].s64 = ctx.r[11].s64 + 18096;
	// 826A1438: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A143C: 388ACE8C  addi r4, r10, -0x3174
	ctx.r[4].s64 = ctx.r[10].s64 + -12660;
	// 826A1440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A144C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1450: 386A64A0  addi r3, r10, 0x64a0
	ctx.r[3].s64 = ctx.r[10].s64 + 25760;
	// 826A1454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A145C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A146C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1478: 4BDC59A9  bl 0x82466e20
	ctx.lr = 0x826A147C;
	sub_82466E20(ctx, base);
	// 826A147C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1490 size=112
    let mut pc: u32 = 0x826A1490;
    'dispatch: loop {
        match pc {
            0x826A1490 => {
    //   block [0x826A1490..0x826A1500)
	// 826A1490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A149C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A14A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A14A4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A14A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A14AC: 390B46E0  addi r8, r11, 0x46e0
	ctx.r[8].s64 = ctx.r[11].s64 + 18144;
	// 826A14B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A14B4: 388ACEA0  addi r4, r10, -0x3160
	ctx.r[4].s64 = ctx.r[10].s64 + -12640;
	// 826A14B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A14BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A14C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A14C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A14C8: 386A64D0  addi r3, r10, 0x64d0
	ctx.r[3].s64 = ctx.r[10].s64 + 25808;
	// 826A14CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A14D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A14D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A14D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A14DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A14E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A14E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A14E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A14EC: 4BDC5935  bl 0x82466e20
	ctx.lr = 0x826A14F0;
	sub_82466E20(ctx, base);
	// 826A14F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A14F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A14F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A14FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1500 size=112
    let mut pc: u32 = 0x826A1500;
    'dispatch: loop {
        match pc {
            0x826A1500 => {
    //   block [0x826A1500..0x826A1570)
	// 826A1500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A150C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1510: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1514: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A151C: 390B4728  addi r8, r11, 0x4728
	ctx.r[8].s64 = ctx.r[11].s64 + 18216;
	// 826A1520: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A1524: 388A88D0  addi r4, r10, -0x7730
	ctx.r[4].s64 = ctx.r[10].s64 + -30512;
	// 826A1528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A152C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1538: 386A6500  addi r3, r10, 0x6500
	ctx.r[3].s64 = ctx.r[10].s64 + 25856;
	// 826A153C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A154C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A155C: 4BDC58C5  bl 0x82466e20
	ctx.lr = 0x826A1560;
	sub_82466E20(ctx, base);
	// 826A1560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A156C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1570 size=100
    let mut pc: u32 = 0x826A1570;
    'dispatch: loop {
        match pc {
            0x826A1570 => {
    //   block [0x826A1570..0x826A15D4)
	// 826A1570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A157C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1584: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1588: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A158C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1590: 388ACEB4  addi r4, r10, -0x314c
	ctx.r[4].s64 = ctx.r[10].s64 + -12620;
	// 826A1594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1598: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A159C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A15A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A15A4: 386A6530  addi r3, r10, 0x6530
	ctx.r[3].s64 = ctx.r[10].s64 + 25904;
	// 826A15A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A15AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A15B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A15B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A15B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A15BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A15C0: 4BDC5861  bl 0x82466e20
	ctx.lr = 0x826A15C4;
	sub_82466E20(ctx, base);
	// 826A15C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A15C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A15CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A15D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A15D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A15D8 size=112
    let mut pc: u32 = 0x826A15D8;
    'dispatch: loop {
        match pc {
            0x826A15D8 => {
    //   block [0x826A15D8..0x826A1648)
	// 826A15D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A15DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A15E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A15E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A15E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A15EC: 38AA6530  addi r5, r10, 0x6530
	ctx.r[5].s64 = ctx.r[10].s64 + 25904;
	// 826A15F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A15F4: 390B4770  addi r8, r11, 0x4770
	ctx.r[8].s64 = ctx.r[11].s64 + 18288;
	// 826A15F8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A15FC: 388ACED0  addi r4, r10, -0x3130
	ctx.r[4].s64 = ctx.r[10].s64 + -12592;
	// 826A1600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A160C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1610: 386A6560  addi r3, r10, 0x6560
	ctx.r[3].s64 = ctx.r[10].s64 + 25952;
	// 826A1614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A161C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A162C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1634: 4BDC57ED  bl 0x82466e20
	ctx.lr = 0x826A1638;
	sub_82466E20(ctx, base);
	// 826A1638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A163C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1648 size=112
    let mut pc: u32 = 0x826A1648;
    'dispatch: loop {
        match pc {
            0x826A1648 => {
    //   block [0x826A1648..0x826A16B8)
	// 826A1648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A164C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1654: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1658: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A165C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A1660: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1664: 390B47B8  addi r8, r11, 0x47b8
	ctx.r[8].s64 = ctx.r[11].s64 + 18360;
	// 826A1668: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A166C: 388ACEF0  addi r4, r10, -0x3110
	ctx.r[4].s64 = ctx.r[10].s64 + -12560;
	// 826A1670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1674: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A167C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1680: 386A6590  addi r3, r10, 0x6590
	ctx.r[3].s64 = ctx.r[10].s64 + 26000;
	// 826A1684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A168C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A169C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A16A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A16A4: 4BDC577D  bl 0x82466e20
	ctx.lr = 0x826A16A8;
	sub_82466E20(ctx, base);
	// 826A16A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A16AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A16B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A16B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A16B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A16B8 size=112
    let mut pc: u32 = 0x826A16B8;
    'dispatch: loop {
        match pc {
            0x826A16B8 => {
    //   block [0x826A16B8..0x826A1728)
	// 826A16B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A16BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A16C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A16C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A16C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A16CC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826A16D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A16D4: 390B47D0  addi r8, r11, 0x47d0
	ctx.r[8].s64 = ctx.r[11].s64 + 18384;
	// 826A16D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A16DC: 388ACF08  addi r4, r10, -0x30f8
	ctx.r[4].s64 = ctx.r[10].s64 + -12536;
	// 826A16E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A16E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A16E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A16EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A16F0: 386A65C0  addi r3, r10, 0x65c0
	ctx.r[3].s64 = ctx.r[10].s64 + 26048;
	// 826A16F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A16F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A16FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1704: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A1708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A170C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1714: 4BDC570D  bl 0x82466e20
	ctx.lr = 0x826A1718;
	sub_82466E20(ctx, base);
	// 826A1718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A171C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1728 size=112
    let mut pc: u32 = 0x826A1728;
    'dispatch: loop {
        match pc {
            0x826A1728 => {
    //   block [0x826A1728..0x826A1798)
	// 826A1728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A172C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1734: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1738: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A173C: 38AA6590  addi r5, r10, 0x6590
	ctx.r[5].s64 = ctx.r[10].s64 + 26000;
	// 826A1740: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1744: 390B47E8  addi r8, r11, 0x47e8
	ctx.r[8].s64 = ctx.r[11].s64 + 18408;
	// 826A1748: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A174C: 388ACF24  addi r4, r10, -0x30dc
	ctx.r[4].s64 = ctx.r[10].s64 + -12508;
	// 826A1750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1754: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A175C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1760: 386A65F0  addi r3, r10, 0x65f0
	ctx.r[3].s64 = ctx.r[10].s64 + 26096;
	// 826A1764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A176C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A177C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1784: 4BDC569D  bl 0x82466e20
	ctx.lr = 0x826A1788;
	sub_82466E20(ctx, base);
	// 826A1788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A178C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1798 size=72
    let mut pc: u32 = 0x826A1798;
    'dispatch: loop {
        match pc {
            0x826A1798 => {
    //   block [0x826A1798..0x826A17E0)
	// 826A1798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A179C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A17A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A17A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A17A8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 826A17AC: 38CB8180  addi r6, r11, -0x7e80
	ctx.r[6].s64 = ctx.r[11].s64 + -32384;
	// 826A17B0: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A17B4: 388BA968  addi r4, r11, -0x5698
	ctx.r[4].s64 = ctx.r[11].s64 + -22168;
	// 826A17B8: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A17BC: 386B6620  addi r3, r11, 0x6620
	ctx.r[3].s64 = ctx.r[11].s64 + 26144;
	// 826A17C0: 4BDDA2C9  bl 0x8247ba88
	ctx.lr = 0x826A17C4;
	sub_8247BA88(ctx, base);
	// 826A17C4: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826A17C8: 386BCE88  addi r3, r11, -0x3178
	ctx.r[3].s64 = ctx.r[11].s64 + -12664;
	// 826A17CC: 4BE9136D  bl 0x82532b38
	ctx.lr = 0x826A17D0;
	sub_82532B38(ctx, base);
	// 826A17D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826A17D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A17D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A17DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A17E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A17E0 size=108
    let mut pc: u32 = 0x826A17E0;
    'dispatch: loop {
        match pc {
            0x826A17E0 => {
    //   block [0x826A17E0..0x826A184C)
	// 826A17E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A17E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A17E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A17EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A17F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A17F4: 38EB7BC0  addi r7, r11, 0x7bc0
	ctx.r[7].s64 = ctx.r[11].s64 + 31680;
	// 826A17F8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A17FC: 388AA590  addi r4, r10, -0x5a70
	ctx.r[4].s64 = ctx.r[10].s64 + -23152;
	// 826A1800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1804: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A180C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1810: 386A6638  addi r3, r10, 0x6638
	ctx.r[3].s64 = ctx.r[10].s64 + 26168;
	// 826A1814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A181C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A182C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1838: 4BDC55E9  bl 0x82466e20
	ctx.lr = 0x826A183C;
	sub_82466E20(ctx, base);
	// 826A183C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A1850 size=24
    let mut pc: u32 = 0x826A1850;
    'dispatch: loop {
        match pc {
            0x826A1850 => {
    //   block [0x826A1850..0x826A1868)
	// 826A1850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1854: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A1858: 394A17E0  addi r10, r10, 0x17e0
	ctx.r[10].s64 = ctx.r[10].s64 + 6112;
	// 826A185C: 816B7C38  lwz r11, 0x7c38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31800 as u32) ) } as u64;
	// 826A1860: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826A1864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1868 size=112
    let mut pc: u32 = 0x826A1868;
    'dispatch: loop {
        match pc {
            0x826A1868 => {
    //   block [0x826A1868..0x826A18D8)
	// 826A1868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A186C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1870: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1874: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A1878: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A187C: 392AB434  addi r9, r10, -0x4bcc
	ctx.r[9].s64 = ctx.r[10].s64 + -19404;
	// 826A1880: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1884: 390B17E0  addi r8, r11, 0x17e0
	ctx.r[8].s64 = ctx.r[11].s64 + 6112;
	// 826A1888: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826A188C: 388AA5A8  addi r4, r10, -0x5a58
	ctx.r[4].s64 = ctx.r[10].s64 + -23128;
	// 826A1890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1894: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1898: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A189C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A18A0: 386A6668  addi r3, r10, 0x6668
	ctx.r[3].s64 = ctx.r[10].s64 + 26216;
	// 826A18A4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A18A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A18AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A18B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A18B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A18B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A18BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A18C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A18C4: 4BDC555D  bl 0x82466e20
	ctx.lr = 0x826A18C8;
	sub_82466E20(ctx, base);
	// 826A18C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A18CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A18D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A18D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A18D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A18D8 size=108
    let mut pc: u32 = 0x826A18D8;
    'dispatch: loop {
        match pc {
            0x826A18D8 => {
    //   block [0x826A18D8..0x826A1944)
	// 826A18D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A18DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A18E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A18E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A18E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A18EC: 38EB7C3C  addi r7, r11, 0x7c3c
	ctx.r[7].s64 = ctx.r[11].s64 + 31804;
	// 826A18F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A18F4: 388AA5BC  addi r4, r10, -0x5a44
	ctx.r[4].s64 = ctx.r[10].s64 + -23108;
	// 826A18F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A18FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1900: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1904: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1908: 386A6698  addi r3, r10, 0x6698
	ctx.r[3].s64 = ctx.r[10].s64 + 26264;
	// 826A190C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1910: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1918: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A191C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1920: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1924: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1928: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A192C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1930: 4BDC54F1  bl 0x82466e20
	ctx.lr = 0x826A1934;
	sub_82466E20(ctx, base);
	// 826A1934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1938: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A193C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1948 size=108
    let mut pc: u32 = 0x826A1948;
    'dispatch: loop {
        match pc {
            0x826A1948 => {
    //   block [0x826A1948..0x826A19B4)
	// 826A1948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A194C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1954: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1958: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A195C: 38EB7C6C  addi r7, r11, 0x7c6c
	ctx.r[7].s64 = ctx.r[11].s64 + 31852;
	// 826A1960: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A1964: 388AA5DC  addi r4, r10, -0x5a24
	ctx.r[4].s64 = ctx.r[10].s64 + -23076;
	// 826A1968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A196C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1978: 386A66C8  addi r3, r10, 0x66c8
	ctx.r[3].s64 = ctx.r[10].s64 + 26312;
	// 826A197C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A198C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A199C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A19A0: 4BDC5481  bl 0x82466e20
	ctx.lr = 0x826A19A4;
	sub_82466E20(ctx, base);
	// 826A19A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A19A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A19AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A19B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A19B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A19B8 size=24
    let mut pc: u32 = 0x826A19B8;
    'dispatch: loop {
        match pc {
            0x826A19B8 => {
    //   block [0x826A19B8..0x826A19D0)
	// 826A19B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A19BC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A19C0: 394A1840  addi r10, r10, 0x1840
	ctx.r[10].s64 = ctx.r[10].s64 + 6208;
	// 826A19C4: 816B7C9C  lwz r11, 0x7c9c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31900 as u32) ) } as u64;
	// 826A19C8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A19CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A19D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A19D0 size=116
    let mut pc: u32 = 0x826A19D0;
    'dispatch: loop {
        match pc {
            0x826A19D0 => {
    //   block [0x826A19D0..0x826A1A44)
	// 826A19D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A19D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A19D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A19DC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A19E0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A19E4: 390B1840  addi r8, r11, 0x1840
	ctx.r[8].s64 = ctx.r[11].s64 + 6208;
	// 826A19E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A19EC: 392AB478  addi r9, r10, -0x4b88
	ctx.r[9].s64 = ctx.r[10].s64 + -19336;
	// 826A19F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A19F4: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826A19F8: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A19FC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1A00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1A04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1A08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1A14: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A1A18: 388AA5F0  addi r4, r10, -0x5a10
	ctx.r[4].s64 = ctx.r[10].s64 + -23056;
	// 826A1A1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A1A20: 386B66F8  addi r3, r11, 0x66f8
	ctx.r[3].s64 = ctx.r[11].s64 + 26360;
	// 826A1A24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A1A28: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1A2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1A30: 4BDC53F1  bl 0x82466e20
	ctx.lr = 0x826A1A34;
	sub_82466E20(ctx, base);
	// 826A1A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1A48 size=108
    let mut pc: u32 = 0x826A1A48;
    'dispatch: loop {
        match pc {
            0x826A1A48 => {
    //   block [0x826A1A48..0x826A1AB4)
	// 826A1A48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1A4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1A50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1A54: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1A58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1A5C: 38EB7CA0  addi r7, r11, 0x7ca0
	ctx.r[7].s64 = ctx.r[11].s64 + 31904;
	// 826A1A60: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A1A64: 388AA608  addi r4, r10, -0x59f8
	ctx.r[4].s64 = ctx.r[10].s64 + -23032;
	// 826A1A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1A6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1A70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1A74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1A78: 386A6728  addi r3, r10, 0x6728
	ctx.r[3].s64 = ctx.r[10].s64 + 26408;
	// 826A1A7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1A84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1A9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1AA0: 4BDC5381  bl 0x82466e20
	ctx.lr = 0x826A1AA4;
	sub_82466E20(ctx, base);
	// 826A1AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1AB8 size=112
    let mut pc: u32 = 0x826A1AB8;
    'dispatch: loop {
        match pc {
            0x826A1AB8 => {
    //   block [0x826A1AB8..0x826A1B28)
	// 826A1AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1AC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1AC8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1ACC: 38AA66F8  addi r5, r10, 0x66f8
	ctx.r[5].s64 = ctx.r[10].s64 + 26360;
	// 826A1AD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1AD4: 390B7D30  addi r8, r11, 0x7d30
	ctx.r[8].s64 = ctx.r[11].s64 + 32048;
	// 826A1AD8: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 826A1ADC: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826A1AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1AE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1AF0: 386A6758  addi r3, r10, 0x6758
	ctx.r[3].s64 = ctx.r[10].s64 + 26456;
	// 826A1AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1B14: 4BDC530D  bl 0x82466e20
	ctx.lr = 0x826A1B18;
	sub_82466E20(ctx, base);
	// 826A1B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1B28 size=112
    let mut pc: u32 = 0x826A1B28;
    'dispatch: loop {
        match pc {
            0x826A1B28 => {
    //   block [0x826A1B28..0x826A1B98)
	// 826A1B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1B34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1B38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1B3C: 38AA66F8  addi r5, r10, 0x66f8
	ctx.r[5].s64 = ctx.r[10].s64 + 26360;
	// 826A1B40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1B44: 390B7E50  addi r8, r11, 0x7e50
	ctx.r[8].s64 = ctx.r[11].s64 + 32336;
	// 826A1B48: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A1B4C: 388AA664  addi r4, r10, -0x599c
	ctx.r[4].s64 = ctx.r[10].s64 + -22940;
	// 826A1B50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1B54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1B58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1B5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1B60: 386A6788  addi r3, r10, 0x6788
	ctx.r[3].s64 = ctx.r[10].s64 + 26504;
	// 826A1B64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1B68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1B6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1B70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1B78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1B7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1B84: 4BDC529D  bl 0x82466e20
	ctx.lr = 0x826A1B88;
	sub_82466E20(ctx, base);
	// 826A1B88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1B8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1B90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1B94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A1B98 size=44
    let mut pc: u32 = 0x826A1B98;
    'dispatch: loop {
        match pc {
            0x826A1B98 => {
    //   block [0x826A1B98..0x826A1BC4)
	// 826A1B98: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1B9C: 814B7E80  lwz r10, 0x7e80(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32384 as u32) ) } as u64;
	// 826A1BA0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1BA4: 396B18D0  addi r11, r11, 0x18d0
	ctx.r[11].s64 = ctx.r[11].s64 + 6352;
	// 826A1BA8: 914B00C8  stw r10, 0xc8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 826A1BAC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826A1BB0: 814A7E84  lwz r10, 0x7e84(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32388 as u32) ) } as u64;
	// 826A1BB4: 914B00E0  stw r10, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[10].u32 ) };
	// 826A1BB8: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 826A1BBC: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 826A1BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1BC8 size=112
    let mut pc: u32 = 0x826A1BC8;
    'dispatch: loop {
        match pc {
            0x826A1BC8 => {
    //   block [0x826A1BC8..0x826A1C38)
	// 826A1BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1BD4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A1BD8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1BDC: 392AB4CC  addi r9, r10, -0x4b34
	ctx.r[9].s64 = ctx.r[10].s64 + -19252;
	// 826A1BE0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826A1BE4: 390B18D0  addi r8, r11, 0x18d0
	ctx.r[8].s64 = ctx.r[11].s64 + 6352;
	// 826A1BE8: 38E0000E  li r7, 0xe
	ctx.r[7].s64 = 14;
	// 826A1BEC: 388A8F0C  addi r4, r10, -0x70f4
	ctx.r[4].s64 = ctx.r[10].s64 + -28916;
	// 826A1BF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1BF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1BF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1BFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1C00: 386A67B8  addi r3, r10, 0x67b8
	ctx.r[3].s64 = ctx.r[10].s64 + 26552;
	// 826A1C04: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A1C08: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A1C0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1C10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1C18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1C1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1C20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1C24: 4BDC51FD  bl 0x82466e20
	ctx.lr = 0x826A1C28;
	sub_82466E20(ctx, base);
	// 826A1C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1C38 size=112
    let mut pc: u32 = 0x826A1C38;
    'dispatch: loop {
        match pc {
            0x826A1C38 => {
    //   block [0x826A1C38..0x826A1CA8)
	// 826A1C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1C44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1C48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1C4C: 38AA66F8  addi r5, r10, 0x66f8
	ctx.r[5].s64 = ctx.r[10].s64 + 26360;
	// 826A1C50: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A1C54: 390B7E88  addi r8, r11, 0x7e88
	ctx.r[8].s64 = ctx.r[11].s64 + 32392;
	// 826A1C58: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 826A1C5C: 388AB1CC  addi r4, r10, -0x4e34
	ctx.r[4].s64 = ctx.r[10].s64 + -20020;
	// 826A1C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1C64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1C70: 386A67E8  addi r3, r10, 0x67e8
	ctx.r[3].s64 = ctx.r[10].s64 + 26600;
	// 826A1C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1C94: 4BDC518D  bl 0x82466e20
	ctx.lr = 0x826A1C98;
	sub_82466E20(ctx, base);
	// 826A1C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1CA8 size=108
    let mut pc: u32 = 0x826A1CA8;
    'dispatch: loop {
        match pc {
            0x826A1CA8 => {
    //   block [0x826A1CA8..0x826A1D14)
	// 826A1CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1CB4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826A1CB8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1CBC: 38EB7FC0  addi r7, r11, 0x7fc0
	ctx.r[7].s64 = ctx.r[11].s64 + 32704;
	// 826A1CC0: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826A1CC4: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 826A1CC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1CCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1CD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1CD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1CD8: 386A6818  addi r3, r10, 0x6818
	ctx.r[3].s64 = ctx.r[10].s64 + 26648;
	// 826A1CDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1CE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1CE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1CEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1CF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1CF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1CFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1D00: 4BDC5121  bl 0x82466e20
	ctx.lr = 0x826A1D04;
	sub_82466E20(ctx, base);
	// 826A1D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1D18 size=108
    let mut pc: u32 = 0x826A1D18;
    'dispatch: loop {
        match pc {
            0x826A1D18 => {
    //   block [0x826A1D18..0x826A1D84)
	// 826A1D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1D24: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1D28: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1D2C: 38EB80B0  addi r7, r11, -0x7f50
	ctx.r[7].s64 = ctx.r[11].s64 + -32592;
	// 826A1D30: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A1D34: 388AA6B4  addi r4, r10, -0x594c
	ctx.r[4].s64 = ctx.r[10].s64 + -22860;
	// 826A1D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1D3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1D48: 386A6848  addi r3, r10, 0x6848
	ctx.r[3].s64 = ctx.r[10].s64 + 26696;
	// 826A1D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1D70: 4BDC50B1  bl 0x82466e20
	ctx.lr = 0x826A1D74;
	sub_82466E20(ctx, base);
	// 826A1D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1D88 size=112
    let mut pc: u32 = 0x826A1D88;
    'dispatch: loop {
        match pc {
            0x826A1D88 => {
    //   block [0x826A1D88..0x826A1DF8)
	// 826A1D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1D94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1D98: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1D9C: 38AA66F8  addi r5, r10, 0x66f8
	ctx.r[5].s64 = ctx.r[10].s64 + 26360;
	// 826A1DA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1DA4: 390B8140  addi r8, r11, -0x7ec0
	ctx.r[8].s64 = ctx.r[11].s64 + -32448;
	// 826A1DA8: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826A1DAC: 388AA6E4  addi r4, r10, -0x591c
	ctx.r[4].s64 = ctx.r[10].s64 + -22812;
	// 826A1DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1DB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1DC0: 386A6878  addi r3, r10, 0x6878
	ctx.r[3].s64 = ctx.r[10].s64 + 26744;
	// 826A1DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A1DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1DE4: 4BDC503D  bl 0x82466e20
	ctx.lr = 0x826A1DE8;
	sub_82466E20(ctx, base);
	// 826A1DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1DF8 size=108
    let mut pc: u32 = 0x826A1DF8;
    'dispatch: loop {
        match pc {
            0x826A1DF8 => {
    //   block [0x826A1DF8..0x826A1E64)
	// 826A1DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1E04: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1E08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1E0C: 38EB8230  addi r7, r11, -0x7dd0
	ctx.r[7].s64 = ctx.r[11].s64 + -32208;
	// 826A1E10: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A1E14: 388AA700  addi r4, r10, -0x5900
	ctx.r[4].s64 = ctx.r[10].s64 + -22784;
	// 826A1E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1E1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1E28: 386A68A8  addi r3, r10, 0x68a8
	ctx.r[3].s64 = ctx.r[10].s64 + 26792;
	// 826A1E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1E50: 4BDC4FD1  bl 0x82466e20
	ctx.lr = 0x826A1E54;
	sub_82466E20(ctx, base);
	// 826A1E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1E68 size=108
    let mut pc: u32 = 0x826A1E68;
    'dispatch: loop {
        match pc {
            0x826A1E68 => {
    //   block [0x826A1E68..0x826A1ED4)
	// 826A1E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1E70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1E74: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1E78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1E7C: 38EB8248  addi r7, r11, -0x7db8
	ctx.r[7].s64 = ctx.r[11].s64 + -32184;
	// 826A1E80: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A1E84: 388AA718  addi r4, r10, -0x58e8
	ctx.r[4].s64 = ctx.r[10].s64 + -22760;
	// 826A1E88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1E8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1E90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1E94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1E98: 386A68D8  addi r3, r10, 0x68d8
	ctx.r[3].s64 = ctx.r[10].s64 + 26840;
	// 826A1E9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1EA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1EA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1EA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1EB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1EB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1EB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1EBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1EC0: 4BDC4F61  bl 0x82466e20
	ctx.lr = 0x826A1EC4;
	sub_82466E20(ctx, base);
	// 826A1EC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1EC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1ECC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1ED8 size=116
    let mut pc: u32 = 0x826A1ED8;
    'dispatch: loop {
        match pc {
            0x826A1ED8 => {
    //   block [0x826A1ED8..0x826A1F4C)
	// 826A1ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1EE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1EE4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1EE8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A1EEC: 390B82AC  addi r8, r11, -0x7d54
	ctx.r[8].s64 = ctx.r[11].s64 + -32084;
	// 826A1EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1EF4: 392AB50C  addi r9, r10, -0x4af4
	ctx.r[9].s64 = ctx.r[10].s64 + -19188;
	// 826A1EF8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A1EFC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826A1F00: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A1F04: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A1F08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1F0C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1F10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1F18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1F1C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A1F20: 388AA728  addi r4, r10, -0x58d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22744;
	// 826A1F24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A1F28: 386B6908  addi r3, r11, 0x6908
	ctx.r[3].s64 = ctx.r[11].s64 + 26888;
	// 826A1F2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A1F30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1F34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1F38: 4BDC4EE9  bl 0x82466e20
	ctx.lr = 0x826A1F3C;
	sub_82466E20(ctx, base);
	// 826A1F3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1F50 size=108
    let mut pc: u32 = 0x826A1F50;
    'dispatch: loop {
        match pc {
            0x826A1F50 => {
    //   block [0x826A1F50..0x826A1FBC)
	// 826A1F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1F5C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1F60: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1F64: 38EB82C8  addi r7, r11, -0x7d38
	ctx.r[7].s64 = ctx.r[11].s64 + -32056;
	// 826A1F68: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A1F6C: 388AA73C  addi r4, r10, -0x58c4
	ctx.r[4].s64 = ctx.r[10].s64 + -22724;
	// 826A1F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1F74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1F78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1F80: 386A6938  addi r3, r10, 0x6938
	ctx.r[3].s64 = ctx.r[10].s64 + 26936;
	// 826A1F84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1F88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A1F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A1F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A1F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A1F9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A1FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A1FA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A1FA8: 4BDC4E79  bl 0x82466e20
	ctx.lr = 0x826A1FAC;
	sub_82466E20(ctx, base);
	// 826A1FAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A1FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A1FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A1FB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A1FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A1FC0 size=108
    let mut pc: u32 = 0x826A1FC0;
    'dispatch: loop {
        match pc {
            0x826A1FC0 => {
    //   block [0x826A1FC0..0x826A202C)
	// 826A1FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A1FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A1FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A1FCC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A1FD0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A1FD4: 38EB8310  addi r7, r11, -0x7cf0
	ctx.r[7].s64 = ctx.r[11].s64 + -31984;
	// 826A1FD8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A1FDC: 388AA760  addi r4, r10, -0x58a0
	ctx.r[4].s64 = ctx.r[10].s64 + -22688;
	// 826A1FE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A1FE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A1FE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A1FEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A1FF0: 386A6968  addi r3, r10, 0x6968
	ctx.r[3].s64 = ctx.r[10].s64 + 26984;
	// 826A1FF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A1FF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A1FFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A200C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2018: 4BDC4E09  bl 0x82466e20
	ctx.lr = 0x826A201C;
	sub_82466E20(ctx, base);
	// 826A201C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2030 size=108
    let mut pc: u32 = 0x826A2030;
    'dispatch: loop {
        match pc {
            0x826A2030 => {
    //   block [0x826A2030..0x826A209C)
	// 826A2030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A203C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2040: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2044: 38EB83A0  addi r7, r11, -0x7c60
	ctx.r[7].s64 = ctx.r[11].s64 + -31840;
	// 826A2048: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A204C: 388AA784  addi r4, r10, -0x587c
	ctx.r[4].s64 = ctx.r[10].s64 + -22652;
	// 826A2050: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2054: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2058: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A205C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2060: 386A6998  addi r3, r10, 0x6998
	ctx.r[3].s64 = ctx.r[10].s64 + 27032;
	// 826A2064: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2068: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A206C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A207C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2088: 4BDC4D99  bl 0x82466e20
	ctx.lr = 0x826A208C;
	sub_82466E20(ctx, base);
	// 826A208C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2090: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2094: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2098: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A20A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A20A0 size=100
    let mut pc: u32 = 0x826A20A0;
    'dispatch: loop {
        match pc {
            0x826A20A0 => {
    //   block [0x826A20A0..0x826A2104)
	// 826A20A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A20A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A20A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A20AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A20B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A20B4: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A20B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A20BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A20C0: 388AA79C  addi r4, r10, -0x5864
	ctx.r[4].s64 = ctx.r[10].s64 + -22628;
	// 826A20C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A20C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A20CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A20D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A20D4: 386A69C8  addi r3, r10, 0x69c8
	ctx.r[3].s64 = ctx.r[10].s64 + 27080;
	// 826A20D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A20DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A20E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A20E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A20E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A20EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A20F0: 4BDC4D31  bl 0x82466e20
	ctx.lr = 0x826A20F4;
	sub_82466E20(ctx, base);
	// 826A20F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A20F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A20FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2100: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2108 size=112
    let mut pc: u32 = 0x826A2108;
    'dispatch: loop {
        match pc {
            0x826A2108 => {
    //   block [0x826A2108..0x826A2178)
	// 826A2108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A210C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2114: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2118: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A211C: 38AA69C8  addi r5, r10, 0x69c8
	ctx.r[5].s64 = ctx.r[10].s64 + 27080;
	// 826A2120: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2124: 390B8430  addi r8, r11, -0x7bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -31696;
	// 826A2128: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A212C: 388AA7B8  addi r4, r10, -0x5848
	ctx.r[4].s64 = ctx.r[10].s64 + -22600;
	// 826A2130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2134: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2138: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A213C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2140: 386A69F8  addi r3, r10, 0x69f8
	ctx.r[3].s64 = ctx.r[10].s64 + 27128;
	// 826A2144: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A214C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A215C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2164: 4BDC4CBD  bl 0x82466e20
	ctx.lr = 0x826A2168;
	sub_82466E20(ctx, base);
	// 826A2168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A216C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2178 size=108
    let mut pc: u32 = 0x826A2178;
    'dispatch: loop {
        match pc {
            0x826A2178 => {
    //   block [0x826A2178..0x826A21E4)
	// 826A2178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A217C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2184: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2188: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A218C: 38EB8490  addi r7, r11, -0x7b70
	ctx.r[7].s64 = ctx.r[11].s64 + -31600;
	// 826A2190: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A2194: 388AA7DC  addi r4, r10, -0x5824
	ctx.r[4].s64 = ctx.r[10].s64 + -22564;
	// 826A2198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A219C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A21A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A21A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A21A8: 386A6A28  addi r3, r10, 0x6a28
	ctx.r[3].s64 = ctx.r[10].s64 + 27176;
	// 826A21AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A21B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A21B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A21B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A21BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A21C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A21C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A21C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A21CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A21D0: 4BDC4C51  bl 0x82466e20
	ctx.lr = 0x826A21D4;
	sub_82466E20(ctx, base);
	// 826A21D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A21D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A21DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A21E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A21E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A21E8 size=108
    let mut pc: u32 = 0x826A21E8;
    'dispatch: loop {
        match pc {
            0x826A21E8 => {
    //   block [0x826A21E8..0x826A2254)
	// 826A21E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A21EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A21F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A21F4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A21F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A21FC: 38EB84C0  addi r7, r11, -0x7b40
	ctx.r[7].s64 = ctx.r[11].s64 + -31552;
	// 826A2200: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A2204: 388AA7E4  addi r4, r10, -0x581c
	ctx.r[4].s64 = ctx.r[10].s64 + -22556;
	// 826A2208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A220C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2210: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2214: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2218: 386A6A58  addi r3, r10, 0x6a58
	ctx.r[3].s64 = ctx.r[10].s64 + 27224;
	// 826A221C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2220: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A222C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2234: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A223C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2240: 4BDC4BE1  bl 0x82466e20
	ctx.lr = 0x826A2244;
	sub_82466E20(ctx, base);
	// 826A2244: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A224C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2258 size=108
    let mut pc: u32 = 0x826A2258;
    'dispatch: loop {
        match pc {
            0x826A2258 => {
    //   block [0x826A2258..0x826A22C4)
	// 826A2258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2264: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2268: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A226C: 38EB8520  addi r7, r11, -0x7ae0
	ctx.r[7].s64 = ctx.r[11].s64 + -31456;
	// 826A2270: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A2274: 388AA7F8  addi r4, r10, -0x5808
	ctx.r[4].s64 = ctx.r[10].s64 + -22536;
	// 826A2278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A227C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2280: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2288: 386A6A88  addi r3, r10, 0x6a88
	ctx.r[3].s64 = ctx.r[10].s64 + 27272;
	// 826A228C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2290: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2294: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A229C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A22A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A22A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A22A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A22AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A22B0: 4BDC4B71  bl 0x82466e20
	ctx.lr = 0x826A22B4;
	sub_82466E20(ctx, base);
	// 826A22B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A22B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A22BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A22C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A22C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A22C8 size=112
    let mut pc: u32 = 0x826A22C8;
    'dispatch: loop {
        match pc {
            0x826A22C8 => {
    //   block [0x826A22C8..0x826A2338)
	// 826A22C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A22CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A22D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A22D4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A22D8: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826A22DC: 38EA8598  addi r7, r10, -0x7a68
	ctx.r[7].s64 = ctx.r[10].s64 + -31336;
	// 826A22E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A22E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A22E8: 388AA804  addi r4, r10, -0x57fc
	ctx.r[4].s64 = ctx.r[10].s64 + -22524;
	// 826A22EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A22F0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A22F4: 396BB520  addi r11, r11, -0x4ae0
	ctx.r[11].s64 = ctx.r[11].s64 + -19168;
	// 826A22F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A22FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2300: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2304: 386A6AB8  addi r3, r10, 0x6ab8
	ctx.r[3].s64 = ctx.r[10].s64 + 27320;
	// 826A2308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A230C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A2310: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2314: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A2318: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A231C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2320: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2324: 4BDC4AFD  bl 0x82466e20
	ctx.lr = 0x826A2328;
	sub_82466E20(ctx, base);
	// 826A2328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A232C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2338 size=96
    let mut pc: u32 = 0x826A2338;
    'dispatch: loop {
        match pc {
            0x826A2338 => {
    //   block [0x826A2338..0x826A2398)
	// 826A2338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A233C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2344: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A234C: 388AA82C  addi r4, r10, -0x57d4
	ctx.r[4].s64 = ctx.r[10].s64 + -22484;
	// 826A2350: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2358: 386A6AE8  addi r3, r10, 0x6ae8
	ctx.r[3].s64 = ctx.r[10].s64 + 27368;
	// 826A235C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A236C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2378: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A237C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2384: 4BDC4A9D  bl 0x82466e20
	ctx.lr = 0x826A2388;
	sub_82466E20(ctx, base);
	// 826A2388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A238C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2398 size=112
    let mut pc: u32 = 0x826A2398;
    'dispatch: loop {
        match pc {
            0x826A2398 => {
    //   block [0x826A2398..0x826A2408)
	// 826A2398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A239C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A23A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A23A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A23A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A23AC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A23B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A23B4: 390B86B8  addi r8, r11, -0x7948
	ctx.r[8].s64 = ctx.r[11].s64 + -31048;
	// 826A23B8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826A23BC: 388AA848  addi r4, r10, -0x57b8
	ctx.r[4].s64 = ctx.r[10].s64 + -22456;
	// 826A23C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A23C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A23C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A23CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A23D0: 386A6B18  addi r3, r10, 0x6b18
	ctx.r[3].s64 = ctx.r[10].s64 + 27416;
	// 826A23D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A23D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A23DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A23E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A23E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A23E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A23EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A23F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A23F4: 4BDC4A2D  bl 0x82466e20
	ctx.lr = 0x826A23F8;
	sub_82466E20(ctx, base);
	// 826A23F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A23FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A2408 size=24
    let mut pc: u32 = 0x826A2408;
    'dispatch: loop {
        match pc {
            0x826A2408 => {
    //   block [0x826A2408..0x826A2420)
	// 826A2408: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A240C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2410: 394A1A20  addi r10, r10, 0x1a20
	ctx.r[10].s64 = ctx.r[10].s64 + 6688;
	// 826A2414: 816B82C4  lwz r11, -0x7d3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-32060 as u32) ) } as u64;
	// 826A2418: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826A241C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2420 size=116
    let mut pc: u32 = 0x826A2420;
    'dispatch: loop {
        match pc {
            0x826A2420 => {
    //   block [0x826A2420..0x826A2494)
	// 826A2420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A242C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2430: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2434: 390B1A20  addi r8, r11, 0x1a20
	ctx.r[8].s64 = ctx.r[11].s64 + 6688;
	// 826A2438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A243C: 392AB59C  addi r9, r10, -0x4a64
	ctx.r[9].s64 = ctx.r[10].s64 + -19044;
	// 826A2440: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2444: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 826A2448: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A244C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2454: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A245C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2464: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A2468: 388A2D90  addi r4, r10, 0x2d90
	ctx.r[4].s64 = ctx.r[10].s64 + 11664;
	// 826A246C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2470: 386B6B48  addi r3, r11, 0x6b48
	ctx.r[3].s64 = ctx.r[11].s64 + 27464;
	// 826A2474: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2478: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A247C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2480: 4BDC49A1  bl 0x82466e20
	ctx.lr = 0x826A2484;
	sub_82466E20(ctx, base);
	// 826A2484: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2488: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A248C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A2498 size=24
    let mut pc: u32 = 0x826A2498;
    'dispatch: loop {
        match pc {
            0x826A2498 => {
    //   block [0x826A2498..0x826A24B0)
	// 826A2498: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A249C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A24A0: 394A1AF8  addi r10, r10, 0x1af8
	ctx.r[10].s64 = ctx.r[10].s64 + 6904;
	// 826A24A4: 816B8720  lwz r11, -0x78e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30944 as u32) ) } as u64;
	// 826A24A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A24AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A24B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A24B0 size=116
    let mut pc: u32 = 0x826A24B0;
    'dispatch: loop {
        match pc {
            0x826A24B0 => {
    //   block [0x826A24B0..0x826A2524)
	// 826A24B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A24B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A24B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A24BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A24C0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A24C4: 390B1AF8  addi r8, r11, 0x1af8
	ctx.r[8].s64 = ctx.r[11].s64 + 6904;
	// 826A24C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A24CC: 392AB650  addi r9, r10, -0x49b0
	ctx.r[9].s64 = ctx.r[10].s64 + -18864;
	// 826A24D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A24D4: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826A24D8: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A24DC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A24E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A24E4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A24E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A24EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A24F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A24F4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A24F8: 388AA870  addi r4, r10, -0x5790
	ctx.r[4].s64 = ctx.r[10].s64 + -22416;
	// 826A24FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2500: 386B6B78  addi r3, r11, 0x6b78
	ctx.r[3].s64 = ctx.r[11].s64 + 27512;
	// 826A2504: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2508: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A250C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2510: 4BDC4911  bl 0x82466e20
	ctx.lr = 0x826A2514;
	sub_82466E20(ctx, base);
	// 826A2514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A251C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2528 size=112
    let mut pc: u32 = 0x826A2528;
    'dispatch: loop {
        match pc {
            0x826A2528 => {
    //   block [0x826A2528..0x826A2598)
	// 826A2528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A252C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2534: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2538: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A253C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A2540: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2544: 390B8728  addi r8, r11, -0x78d8
	ctx.r[8].s64 = ctx.r[11].s64 + -30936;
	// 826A2548: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A254C: 388AA884  addi r4, r10, -0x577c
	ctx.r[4].s64 = ctx.r[10].s64 + -22396;
	// 826A2550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2554: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2558: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A255C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2560: 386A6BA8  addi r3, r10, 0x6ba8
	ctx.r[3].s64 = ctx.r[10].s64 + 27560;
	// 826A2564: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2568: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A256C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A257C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2584: 4BDC489D  bl 0x82466e20
	ctx.lr = 0x826A2588;
	sub_82466E20(ctx, base);
	// 826A2588: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A258C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2590: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2598 size=112
    let mut pc: u32 = 0x826A2598;
    'dispatch: loop {
        match pc {
            0x826A2598 => {
    //   block [0x826A2598..0x826A2608)
	// 826A2598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A259C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A25A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A25A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A25A8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A25AC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A25B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A25B4: 390B8770  addi r8, r11, -0x7890
	ctx.r[8].s64 = ctx.r[11].s64 + -30864;
	// 826A25B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A25BC: 388AA89C  addi r4, r10, -0x5764
	ctx.r[4].s64 = ctx.r[10].s64 + -22372;
	// 826A25C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A25C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A25C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A25CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A25D0: 386A6BD8  addi r3, r10, 0x6bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 27608;
	// 826A25D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A25D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A25DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A25E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A25E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A25E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A25EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A25F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A25F4: 4BDC482D  bl 0x82466e20
	ctx.lr = 0x826A25F8;
	sub_82466E20(ctx, base);
	// 826A25F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A25FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2608 size=108
    let mut pc: u32 = 0x826A2608;
    'dispatch: loop {
        match pc {
            0x826A2608 => {
    //   block [0x826A2608..0x826A2674)
	// 826A2608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A260C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2614: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2618: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A261C: 38EB87B8  addi r7, r11, -0x7848
	ctx.r[7].s64 = ctx.r[11].s64 + -30792;
	// 826A2620: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A2624: 388A2DF0  addi r4, r10, 0x2df0
	ctx.r[4].s64 = ctx.r[10].s64 + 11760;
	// 826A2628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A262C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2630: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2638: 386A6C08  addi r3, r10, 0x6c08
	ctx.r[3].s64 = ctx.r[10].s64 + 27656;
	// 826A263C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2640: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2648: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A264C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2650: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2658: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A265C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2660: 4BDC47C1  bl 0x82466e20
	ctx.lr = 0x826A2664;
	sub_82466E20(ctx, base);
	// 826A2664: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2668: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A266C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2678 size=112
    let mut pc: u32 = 0x826A2678;
    'dispatch: loop {
        match pc {
            0x826A2678 => {
    //   block [0x826A2678..0x826A26E8)
	// 826A2678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A267C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2684: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A268C: 392BB684  addi r9, r11, -0x497c
	ctx.r[9].s64 = ctx.r[11].s64 + -18812;
	// 826A2690: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826A2694: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A2698: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A269C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826A26A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A26A4: 396B8860  addi r11, r11, -0x77a0
	ctx.r[11].s64 = ctx.r[11].s64 + -30624;
	// 826A26A8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A26AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A26B0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A26B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A26B8: 386A6C38  addi r3, r10, 0x6c38
	ctx.r[3].s64 = ctx.r[10].s64 + 27704;
	// 826A26BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A26C0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A26C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A26C8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A26CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A26D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A26D4: 4BDC474D  bl 0x82466e20
	ctx.lr = 0x826A26D8;
	sub_82466E20(ctx, base);
	// 826A26D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A26DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A26E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A26E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A26E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A26E8 size=112
    let mut pc: u32 = 0x826A26E8;
    'dispatch: loop {
        match pc {
            0x826A26E8 => {
    //   block [0x826A26E8..0x826A2758)
	// 826A26E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A26EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A26F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A26F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A26F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A26FC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A2700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2704: 390B89B0  addi r8, r11, -0x7650
	ctx.r[8].s64 = ctx.r[11].s64 + -30288;
	// 826A2708: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826A270C: 388A2E30  addi r4, r10, 0x2e30
	ctx.r[4].s64 = ctx.r[10].s64 + 11824;
	// 826A2710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2714: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A271C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2720: 386A6C68  addi r3, r10, 0x6c68
	ctx.r[3].s64 = ctx.r[10].s64 + 27752;
	// 826A2724: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2728: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A272C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2734: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A273C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2744: 4BDC46DD  bl 0x82466e20
	ctx.lr = 0x826A2748;
	sub_82466E20(ctx, base);
	// 826A2748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A274C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2758 size=112
    let mut pc: u32 = 0x826A2758;
    'dispatch: loop {
        match pc {
            0x826A2758 => {
    //   block [0x826A2758..0x826A27C8)
	// 826A2758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A275C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2768: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A276C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A2770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2774: 390B8A58  addi r8, r11, -0x75a8
	ctx.r[8].s64 = ctx.r[11].s64 + -30120;
	// 826A2778: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826A277C: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826A2780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2784: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A278C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2790: 386A6C98  addi r3, r10, 0x6c98
	ctx.r[3].s64 = ctx.r[10].s64 + 27800;
	// 826A2794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A279C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A27A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A27A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A27A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A27AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A27B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A27B4: 4BDC466D  bl 0x82466e20
	ctx.lr = 0x826A27B8;
	sub_82466E20(ctx, base);
	// 826A27B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A27BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A27C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A27C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A27C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A27C8 size=112
    let mut pc: u32 = 0x826A27C8;
    'dispatch: loop {
        match pc {
            0x826A27C8 => {
    //   block [0x826A27C8..0x826A2838)
	// 826A27C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A27CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A27D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A27D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A27D8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A27DC: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A27E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A27E4: 390B8AE8  addi r8, r11, -0x7518
	ctx.r[8].s64 = ctx.r[11].s64 + -29976;
	// 826A27E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826A27EC: 388A2E18  addi r4, r10, 0x2e18
	ctx.r[4].s64 = ctx.r[10].s64 + 11800;
	// 826A27F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A27F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A27F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A27FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2800: 386A6CC8  addi r3, r10, 0x6cc8
	ctx.r[3].s64 = ctx.r[10].s64 + 27848;
	// 826A2804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A280C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A281C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2824: 4BDC45FD  bl 0x82466e20
	ctx.lr = 0x826A2828;
	sub_82466E20(ctx, base);
	// 826A2828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A282C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2838 size=116
    let mut pc: u32 = 0x826A2838;
    'dispatch: loop {
        match pc {
            0x826A2838 => {
    //   block [0x826A2838..0x826A28AC)
	// 826A2838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A283C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2844: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2848: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826A284C: 390A8B90  addi r8, r10, -0x7470
	ctx.r[8].s64 = ctx.r[10].s64 + -29808;
	// 826A2850: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2854: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2858: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A285C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2860: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2864: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A286C: 388AA8B8  addi r4, r10, -0x5748
	ctx.r[4].s64 = ctx.r[10].s64 + -22344;
	// 826A2870: 396BB6D8  addi r11, r11, -0x4928
	ctx.r[11].s64 = ctx.r[11].s64 + -18728;
	// 826A2874: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2878: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A287C: 386A6CF8  addi r3, r10, 0x6cf8
	ctx.r[3].s64 = ctx.r[10].s64 + 27896;
	// 826A2880: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A2884: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2888: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A288C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2898: 4BDC4589  bl 0x82466e20
	ctx.lr = 0x826A289C;
	sub_82466E20(ctx, base);
	// 826A289C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A28A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A28A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A28A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A28B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A28B0 size=108
    let mut pc: u32 = 0x826A28B0;
    'dispatch: loop {
        match pc {
            0x826A28B0 => {
    //   block [0x826A28B0..0x826A291C)
	// 826A28B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A28B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A28B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A28BC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A28C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A28C4: 38EB8C20  addi r7, r11, -0x73e0
	ctx.r[7].s64 = ctx.r[11].s64 + -29664;
	// 826A28C8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826A28CC: 388A2E58  addi r4, r10, 0x2e58
	ctx.r[4].s64 = ctx.r[10].s64 + 11864;
	// 826A28D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A28D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A28D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A28DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A28E0: 386A6D28  addi r3, r10, 0x6d28
	ctx.r[3].s64 = ctx.r[10].s64 + 27944;
	// 826A28E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A28E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A28EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A28F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A28F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A28F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A28FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2904: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2908: 4BDC4519  bl 0x82466e20
	ctx.lr = 0x826A290C;
	sub_82466E20(ctx, base);
	// 826A290C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2920 size=112
    let mut pc: u32 = 0x826A2920;
    'dispatch: loop {
        match pc {
            0x826A2920 => {
    //   block [0x826A2920..0x826A2990)
	// 826A2920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A292C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2930: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2934: 392BB798  addi r9, r11, -0x4868
	ctx.r[9].s64 = ctx.r[11].s64 + -18536;
	// 826A2938: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 826A293C: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A2940: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2944: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826A2948: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A294C: 396B8CE4  addi r11, r11, -0x731c
	ctx.r[11].s64 = ctx.r[11].s64 + -29468;
	// 826A2950: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A2954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2958: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A295C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2960: 386A6D58  addi r3, r10, 0x6d58
	ctx.r[3].s64 = ctx.r[10].s64 + 27992;
	// 826A2964: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2968: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A296C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2970: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A2974: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2978: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A297C: 4BDC44A5  bl 0x82466e20
	ctx.lr = 0x826A2980;
	sub_82466E20(ctx, base);
	// 826A2980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A298C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2990 size=100
    let mut pc: u32 = 0x826A2990;
    'dispatch: loop {
        match pc {
            0x826A2990 => {
    //   block [0x826A2990..0x826A29F4)
	// 826A2990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A299C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A29A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A29A4: 38AA7748  addi r5, r10, 0x7748
	ctx.r[5].s64 = ctx.r[10].s64 + 30536;
	// 826A29A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A29AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A29B0: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826A29B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A29B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A29BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A29C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A29C4: 386A6D88  addi r3, r10, 0x6d88
	ctx.r[3].s64 = ctx.r[10].s64 + 28040;
	// 826A29C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A29CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A29D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A29D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A29D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A29DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A29E0: 4BDC4441  bl 0x82466e20
	ctx.lr = 0x826A29E4;
	sub_82466E20(ctx, base);
	// 826A29E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A29E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A29EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A29F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A29F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A29F8 size=24
    let mut pc: u32 = 0x826A29F8;
    'dispatch: loop {
        match pc {
            0x826A29F8 => {
    //   block [0x826A29F8..0x826A2A10)
	// 826A29F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A29FC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2A00: 394A1C30  addi r10, r10, 0x1c30
	ctx.r[10].s64 = ctx.r[10].s64 + 7216;
	// 826A2A04: 816B8D18  lwz r11, -0x72e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29416 as u32) ) } as u64;
	// 826A2A08: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826A2A0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2A10 size=108
    let mut pc: u32 = 0x826A2A10;
    'dispatch: loop {
        match pc {
            0x826A2A10 => {
    //   block [0x826A2A10..0x826A2A7C)
	// 826A2A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2A18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2A1C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2A20: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2A24: 38EB1C30  addi r7, r11, 0x1c30
	ctx.r[7].s64 = ctx.r[11].s64 + 7216;
	// 826A2A28: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826A2A2C: 388AB204  addi r4, r10, -0x4dfc
	ctx.r[4].s64 = ctx.r[10].s64 + -19964;
	// 826A2A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2A34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2A38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2A3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2A40: 386A6DB8  addi r3, r10, 0x6db8
	ctx.r[3].s64 = ctx.r[10].s64 + 28088;
	// 826A2A44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2A48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2A4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2A50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2A58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2A60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2A64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2A68: 4BDC43B9  bl 0x82466e20
	ctx.lr = 0x826A2A6C;
	sub_82466E20(ctx, base);
	// 826A2A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2A78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2A80 size=108
    let mut pc: u32 = 0x826A2A80;
    'dispatch: loop {
        match pc {
            0x826A2A80 => {
    //   block [0x826A2A80..0x826A2AEC)
	// 826A2A80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2A84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2A88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2A8C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2A90: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2A94: 38EB8D20  addi r7, r11, -0x72e0
	ctx.r[7].s64 = ctx.r[11].s64 + -29408;
	// 826A2A98: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826A2A9C: 388AB224  addi r4, r10, -0x4ddc
	ctx.r[4].s64 = ctx.r[10].s64 + -19932;
	// 826A2AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2AA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2AAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2AB0: 386A6DE8  addi r3, r10, 0x6de8
	ctx.r[3].s64 = ctx.r[10].s64 + 28136;
	// 826A2AB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2AD4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2AD8: 4BDC4349  bl 0x82466e20
	ctx.lr = 0x826A2ADC;
	sub_82466E20(ctx, base);
	// 826A2ADC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2AF0 size=108
    let mut pc: u32 = 0x826A2AF0;
    'dispatch: loop {
        match pc {
            0x826A2AF0 => {
    //   block [0x826A2AF0..0x826A2B5C)
	// 826A2AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2AFC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2B00: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2B04: 38EB8D68  addi r7, r11, -0x7298
	ctx.r[7].s64 = ctx.r[11].s64 + -29336;
	// 826A2B08: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A2B0C: 388AB248  addi r4, r10, -0x4db8
	ctx.r[4].s64 = ctx.r[10].s64 + -19896;
	// 826A2B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2B14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2B18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2B1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2B20: 386A6E18  addi r3, r10, 0x6e18
	ctx.r[3].s64 = ctx.r[10].s64 + 28184;
	// 826A2B24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2B2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2B34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2B48: 4BDC42D9  bl 0x82466e20
	ctx.lr = 0x826A2B4C;
	sub_82466E20(ctx, base);
	// 826A2B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2B60 size=108
    let mut pc: u32 = 0x826A2B60;
    'dispatch: loop {
        match pc {
            0x826A2B60 => {
    //   block [0x826A2B60..0x826A2BCC)
	// 826A2B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2B6C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2B70: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2B74: 38EB8D98  addi r7, r11, -0x7268
	ctx.r[7].s64 = ctx.r[11].s64 + -29288;
	// 826A2B78: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A2B7C: 388AB264  addi r4, r10, -0x4d9c
	ctx.r[4].s64 = ctx.r[10].s64 + -19868;
	// 826A2B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2B84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2B88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2B90: 386A6E48  addi r3, r10, 0x6e48
	ctx.r[3].s64 = ctx.r[10].s64 + 28232;
	// 826A2B94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2BA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2BB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2BB8: 4BDC4269  bl 0x82466e20
	ctx.lr = 0x826A2BBC;
	sub_82466E20(ctx, base);
	// 826A2BBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2BC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2BC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2BD0 size=100
    let mut pc: u32 = 0x826A2BD0;
    'dispatch: loop {
        match pc {
            0x826A2BD0 => {
    //   block [0x826A2BD0..0x826A2C34)
	// 826A2BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2BD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2BDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2BE4: 38AA6E48  addi r5, r10, 0x6e48
	ctx.r[5].s64 = ctx.r[10].s64 + 28232;
	// 826A2BE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A2BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2BF0: 388AA8D0  addi r4, r10, -0x5730
	ctx.r[4].s64 = ctx.r[10].s64 + -22320;
	// 826A2BF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2C04: 386A6E78  addi r3, r10, 0x6e78
	ctx.r[3].s64 = ctx.r[10].s64 + 28280;
	// 826A2C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2C0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2C10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A2C14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2C18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2C1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2C20: 4BDC4201  bl 0x82466e20
	ctx.lr = 0x826A2C24;
	sub_82466E20(ctx, base);
	// 826A2C24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2C38 size=112
    let mut pc: u32 = 0x826A2C38;
    'dispatch: loop {
        match pc {
            0x826A2C38 => {
    //   block [0x826A2C38..0x826A2CA8)
	// 826A2C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2C44: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2C48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2C4C: 392AB820  addi r9, r10, -0x47e0
	ctx.r[9].s64 = ctx.r[10].s64 + -18400;
	// 826A2C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2C54: 390B8DC8  addi r8, r11, -0x7238
	ctx.r[8].s64 = ctx.r[11].s64 + -29240;
	// 826A2C58: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A2C5C: 388A2EA8  addi r4, r10, 0x2ea8
	ctx.r[4].s64 = ctx.r[10].s64 + 11944;
	// 826A2C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2C64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2C70: 386A6EA8  addi r3, r10, 0x6ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 28328;
	// 826A2C74: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2C78: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A2C7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2C8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2C94: 4BDC418D  bl 0x82466e20
	ctx.lr = 0x826A2C98;
	sub_82466E20(ctx, base);
	// 826A2C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2CA8 size=112
    let mut pc: u32 = 0x826A2CA8;
    'dispatch: loop {
        match pc {
            0x826A2CA8 => {
    //   block [0x826A2CA8..0x826A2D18)
	// 826A2CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2CAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2CB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2CB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2CB8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2CBC: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A2CC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2CC4: 390B8E10  addi r8, r11, -0x71f0
	ctx.r[8].s64 = ctx.r[11].s64 + -29168;
	// 826A2CC8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A2CCC: 388A2EBC  addi r4, r10, 0x2ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 11964;
	// 826A2CD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2CD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2CD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2CDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2CE0: 386A6ED8  addi r3, r10, 0x6ed8
	ctx.r[3].s64 = ctx.r[10].s64 + 28376;
	// 826A2CE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A2CE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2D04: 4BDC411D  bl 0x82466e20
	ctx.lr = 0x826A2D08;
	sub_82466E20(ctx, base);
	// 826A2D08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2D14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2D18 size=116
    let mut pc: u32 = 0x826A2D18;
    'dispatch: loop {
        match pc {
            0x826A2D18 => {
    //   block [0x826A2D18..0x826A2D8C)
	// 826A2D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2D24: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2D28: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826A2D2C: 390A8E40  addi r8, r10, -0x71c0
	ctx.r[8].s64 = ctx.r[10].s64 + -29120;
	// 826A2D30: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2D34: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2D38: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A2D3C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2D40: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2D4C: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826A2D50: 396BB848  addi r11, r11, -0x47b8
	ctx.r[11].s64 = ctx.r[11].s64 + -18360;
	// 826A2D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2D58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2D5C: 386A6F08  addi r3, r10, 0x6f08
	ctx.r[3].s64 = ctx.r[10].s64 + 28424;
	// 826A2D60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A2D64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2D68: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A2D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2D74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2D78: 4BDC40A9  bl 0x82466e20
	ctx.lr = 0x826A2D7C;
	sub_82466E20(ctx, base);
	// 826A2D7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2D90 size=100
    let mut pc: u32 = 0x826A2D90;
    'dispatch: loop {
        match pc {
            0x826A2D90 => {
    //   block [0x826A2D90..0x826A2DF4)
	// 826A2D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2D9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2DA4: 38AA6F08  addi r5, r10, 0x6f08
	ctx.r[5].s64 = ctx.r[10].s64 + 28424;
	// 826A2DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2DB0: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826A2DB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2DBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2DC4: 386A6F38  addi r3, r10, 0x6f38
	ctx.r[3].s64 = ctx.r[10].s64 + 28472;
	// 826A2DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2DCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2DD0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A2DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2DD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2DE0: 4BDC4041  bl 0x82466e20
	ctx.lr = 0x826A2DE4;
	sub_82466E20(ctx, base);
	// 826A2DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2DF8 size=116
    let mut pc: u32 = 0x826A2DF8;
    'dispatch: loop {
        match pc {
            0x826A2DF8 => {
    //   block [0x826A2DF8..0x826A2E6C)
	// 826A2DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2E04: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2E08: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2E0C: 392BB894  addi r9, r11, -0x476c
	ctx.r[9].s64 = ctx.r[11].s64 + -18284;
	// 826A2E10: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A2E14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2E18: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A2E1C: 38C0000E  li r6, 0xe
	ctx.r[6].s64 = 14;
	// 826A2E20: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2E24: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826A2E28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2E2C: 396B8EE8  addi r11, r11, -0x7118
	ctx.r[11].s64 = ctx.r[11].s64 + -28952;
	// 826A2E30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A2E34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2E38: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A2E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2E40: 386A6F68  addi r3, r10, 0x6f68
	ctx.r[3].s64 = ctx.r[10].s64 + 28520;
	// 826A2E44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2E48: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A2E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2E50: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A2E54: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2E58: 4BDC3FC9  bl 0x82466e20
	ctx.lr = 0x826A2E5C;
	sub_82466E20(ctx, base);
	// 826A2E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A2E70 size=24
    let mut pc: u32 = 0x826A2E70;
    'dispatch: loop {
        match pc {
            0x826A2E70 => {
    //   block [0x826A2E70..0x826A2E88)
	// 826A2E70: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2E74: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2E78: 394A1CC0  addi r10, r10, 0x1cc0
	ctx.r[10].s64 = ctx.r[10].s64 + 7360;
	// 826A2E7C: 816B9038  lwz r11, -0x6fc8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28616 as u32) ) } as u64;
	// 826A2E80: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826A2E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2E88 size=116
    let mut pc: u32 = 0x826A2E88;
    'dispatch: loop {
        match pc {
            0x826A2E88 => {
    //   block [0x826A2E88..0x826A2EFC)
	// 826A2E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2E94: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A2E98: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A2E9C: 392BB900  addi r9, r11, -0x4700
	ctx.r[9].s64 = ctx.r[11].s64 + -18176;
	// 826A2EA0: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A2EA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2EA8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826A2EAC: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826A2EB0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2EB4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826A2EB8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2EBC: 396B1CC0  addi r11, r11, 0x1cc0
	ctx.r[11].s64 = ctx.r[11].s64 + 7360;
	// 826A2EC0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A2EC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2EC8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A2ECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2ED0: 386A6F98  addi r3, r10, 0x6f98
	ctx.r[3].s64 = ctx.r[10].s64 + 28568;
	// 826A2ED4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A2ED8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A2EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2EE0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A2EE4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A2EE8: 4BDC3F39  bl 0x82466e20
	ctx.lr = 0x826A2EEC;
	sub_82466E20(ctx, base);
	// 826A2EEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2EF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2EF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2EF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2F00 size=108
    let mut pc: u32 = 0x826A2F00;
    'dispatch: loop {
        match pc {
            0x826A2F00 => {
    //   block [0x826A2F00..0x826A2F6C)
	// 826A2F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2F0C: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2F14: 38EB9040  addi r7, r11, -0x6fc0
	ctx.r[7].s64 = ctx.r[11].s64 + -28608;
	// 826A2F18: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A2F1C: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826A2F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2F24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2F28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A2F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A2F30: 386A6FC8  addi r3, r10, 0x6fc8
	ctx.r[3].s64 = ctx.r[10].s64 + 28616;
	// 826A2F34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A2F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A2F3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2F44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2F54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A2F58: 4BDC3EC9  bl 0x82466e20
	ctx.lr = 0x826A2F5C;
	sub_82466E20(ctx, base);
	// 826A2F5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2F60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2F64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2F68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A2F70 size=24
    let mut pc: u32 = 0x826A2F70;
    'dispatch: loop {
        match pc {
            0x826A2F70 => {
    //   block [0x826A2F70..0x826A2F88)
	// 826A2F70: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2F74: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A2F78: 394A1D80  addi r10, r10, 0x1d80
	ctx.r[10].s64 = ctx.r[10].s64 + 7552;
	// 826A2F7C: 816B903C  lwz r11, -0x6fc4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28612 as u32) ) } as u64;
	// 826A2F80: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826A2F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A2F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A2F88 size=116
    let mut pc: u32 = 0x826A2F88;
    'dispatch: loop {
        match pc {
            0x826A2F88 => {
    //   block [0x826A2F88..0x826A2FFC)
	// 826A2F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A2F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A2F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A2F94: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A2F98: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A2F9C: 390B1D80  addi r8, r11, 0x1d80
	ctx.r[8].s64 = ctx.r[11].s64 + 7552;
	// 826A2FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A2FA4: 392AB978  addi r9, r10, -0x4688
	ctx.r[9].s64 = ctx.r[10].s64 + -18056;
	// 826A2FA8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A2FAC: 38E00011  li r7, 0x11
	ctx.r[7].s64 = 17;
	// 826A2FB0: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A2FB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A2FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A2FBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A2FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A2FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A2FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A2FCC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A2FD0: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826A2FD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A2FD8: 386B6FF8  addi r3, r11, 0x6ff8
	ctx.r[3].s64 = ctx.r[11].s64 + 28664;
	// 826A2FDC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826A2FE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A2FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A2FE8: 4BDC3E39  bl 0x82466e20
	ctx.lr = 0x826A2FEC;
	sub_82466E20(ctx, base);
	// 826A2FEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A2FF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A2FF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A2FF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3000 size=112
    let mut pc: u32 = 0x826A3000;
    'dispatch: loop {
        match pc {
            0x826A3000 => {
    //   block [0x826A3000..0x826A3070)
	// 826A3000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A300C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3010: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3014: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A3018: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A301C: 390B90BC  addi r8, r11, -0x6f44
	ctx.r[8].s64 = ctx.r[11].s64 + -28484;
	// 826A3020: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A3024: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826A3028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A302C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3038: 386A7028  addi r3, r10, 0x7028
	ctx.r[3].s64 = ctx.r[10].s64 + 28712;
	// 826A303C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A304C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A305C: 4BDC3DC5  bl 0x82466e20
	ctx.lr = 0x826A3060;
	sub_82466E20(ctx, base);
	// 826A3060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A306C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A3070 size=24
    let mut pc: u32 = 0x826A3070;
    'dispatch: loop {
        match pc {
            0x826A3070 => {
    //   block [0x826A3070..0x826A3088)
	// 826A3070: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3074: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3078: 394A1F18  addi r10, r10, 0x1f18
	ctx.r[10].s64 = ctx.r[10].s64 + 7960;
	// 826A307C: 816B90EC  lwz r11, -0x6f14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28436 as u32) ) } as u64;
	// 826A3080: 916A00E0  stw r11, 0xe0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(224 as u32), ctx.r[11].u32 ) };
	// 826A3084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3088 size=116
    let mut pc: u32 = 0x826A3088;
    'dispatch: loop {
        match pc {
            0x826A3088 => {
    //   block [0x826A3088..0x826A30FC)
	// 826A3088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A308C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3090: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3094: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3098: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A309C: 392BB9B0  addi r9, r11, -0x4650
	ctx.r[9].s64 = ctx.r[11].s64 + -18000;
	// 826A30A0: 38AA6F68  addi r5, r10, 0x6f68
	ctx.r[5].s64 = ctx.r[10].s64 + 28520;
	// 826A30A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A30A8: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826A30AC: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 826A30B0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A30B4: 388A30BC  addi r4, r10, 0x30bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12476;
	// 826A30B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A30BC: 396B1F18  addi r11, r11, 0x1f18
	ctx.r[11].s64 = ctx.r[11].s64 + 7960;
	// 826A30C0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A30C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A30C8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A30CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A30D0: 386A7058  addi r3, r10, 0x7058
	ctx.r[3].s64 = ctx.r[10].s64 + 28760;
	// 826A30D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A30D8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A30DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A30E0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A30E4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A30E8: 4BDC3D39  bl 0x82466e20
	ctx.lr = 0x826A30EC;
	sub_82466E20(ctx, base);
	// 826A30EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A30F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A30F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A30F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3100 size=116
    let mut pc: u32 = 0x826A3100;
    'dispatch: loop {
        match pc {
            0x826A3100 => {
    //   block [0x826A3100..0x826A3174)
	// 826A3100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3108: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A310C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3110: 38E0000F  li r7, 0xf
	ctx.r[7].s64 = 15;
	// 826A3114: 390A90F0  addi r8, r10, -0x6f10
	ctx.r[8].s64 = ctx.r[10].s64 + -28432;
	// 826A3118: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A311C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3120: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A3124: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A3128: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A312C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3130: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3134: 388AB294  addi r4, r10, -0x4d6c
	ctx.r[4].s64 = ctx.r[10].s64 + -19820;
	// 826A3138: 396BBA20  addi r11, r11, -0x45e0
	ctx.r[11].s64 = ctx.r[11].s64 + -17888;
	// 826A313C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3140: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3144: 386A7088  addi r3, r10, 0x7088
	ctx.r[3].s64 = ctx.r[10].s64 + 28808;
	// 826A3148: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A314C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3150: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3158: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A315C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3160: 4BDC3CC1  bl 0x82466e20
	ctx.lr = 0x826A3164;
	sub_82466E20(ctx, base);
	// 826A3164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A316C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3178 size=116
    let mut pc: u32 = 0x826A3178;
    'dispatch: loop {
        match pc {
            0x826A3178 => {
    //   block [0x826A3178..0x826A31EC)
	// 826A3178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A317C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3184: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3188: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A318C: 390A9258  addi r8, r10, -0x6da8
	ctx.r[8].s64 = ctx.r[10].s64 + -28072;
	// 826A3190: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A3194: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3198: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A319C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A31A0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A31A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A31A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A31AC: 388AB2BC  addi r4, r10, -0x4d44
	ctx.r[4].s64 = ctx.r[10].s64 + -19780;
	// 826A31B0: 396BBA9C  addi r11, r11, -0x4564
	ctx.r[11].s64 = ctx.r[11].s64 + -17764;
	// 826A31B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A31B8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A31BC: 386A70B8  addi r3, r10, 0x70b8
	ctx.r[3].s64 = ctx.r[10].s64 + 28856;
	// 826A31C0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A31C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A31C8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A31CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A31D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A31D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A31D8: 4BDC3C49  bl 0x82466e20
	ctx.lr = 0x826A31DC;
	sub_82466E20(ctx, base);
	// 826A31DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A31E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A31E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A31E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A31F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A31F0 size=112
    let mut pc: u32 = 0x826A31F0;
    'dispatch: loop {
        match pc {
            0x826A31F0 => {
    //   block [0x826A31F0..0x826A3260)
	// 826A31F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A31F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A31F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A31FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3200: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3204: 38AA6D88  addi r5, r10, 0x6d88
	ctx.r[5].s64 = ctx.r[10].s64 + 28040;
	// 826A3208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A320C: 390B92A0  addi r8, r11, -0x6d60
	ctx.r[8].s64 = ctx.r[11].s64 + -28000;
	// 826A3210: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3214: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826A3218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A321C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3228: 386A70E8  addi r3, r10, 0x70e8
	ctx.r[3].s64 = ctx.r[10].s64 + 28904;
	// 826A322C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A324C: 4BDC3BD5  bl 0x82466e20
	ctx.lr = 0x826A3250;
	sub_82466E20(ctx, base);
	// 826A3250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3260 size=100
    let mut pc: u32 = 0x826A3260;
    'dispatch: loop {
        match pc {
            0x826A3260 => {
    //   block [0x826A3260..0x826A32C4)
	// 826A3260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A326C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3274: 38AA7748  addi r5, r10, 0x7748
	ctx.r[5].s64 = ctx.r[10].s64 + 30536;
	// 826A3278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A327C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3280: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826A3284: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A328C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3294: 386A7118  addi r3, r10, 0x7118
	ctx.r[3].s64 = ctx.r[10].s64 + 28952;
	// 826A3298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A329C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A32A0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826A32A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A32A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A32AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A32B0: 4BDC3B71  bl 0x82466e20
	ctx.lr = 0x826A32B4;
	sub_82466E20(ctx, base);
	// 826A32B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A32B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A32BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A32C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A32C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A32C8 size=112
    let mut pc: u32 = 0x826A32C8;
    'dispatch: loop {
        match pc {
            0x826A32C8 => {
    //   block [0x826A32C8..0x826A3338)
	// 826A32C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A32CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A32D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A32D4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A32D8: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826A32DC: 38EA92B8  addi r7, r10, -0x6d48
	ctx.r[7].s64 = ctx.r[10].s64 + -27976;
	// 826A32E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A32E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A32E8: 388A3100  addi r4, r10, 0x3100
	ctx.r[4].s64 = ctx.r[10].s64 + 12544;
	// 826A32EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A32F0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A32F4: 396BBAC8  addi r11, r11, -0x4538
	ctx.r[11].s64 = ctx.r[11].s64 + -17720;
	// 826A32F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A32FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3300: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3304: 386A7148  addi r3, r10, 0x7148
	ctx.r[3].s64 = ctx.r[10].s64 + 29000;
	// 826A3308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A330C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3310: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3314: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3318: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A331C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3320: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3324: 4BDC3AFD  bl 0x82466e20
	ctx.lr = 0x826A3328;
	sub_82466E20(ctx, base);
	// 826A3328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A332C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3338 size=112
    let mut pc: u32 = 0x826A3338;
    'dispatch: loop {
        match pc {
            0x826A3338 => {
    //   block [0x826A3338..0x826A33A8)
	// 826A3338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A333C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3344: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3348: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A334C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3354: 390B93F0  addi r8, r11, -0x6c10
	ctx.r[8].s64 = ctx.r[11].s64 + -27664;
	// 826A3358: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A335C: 388A312C  addi r4, r10, 0x312c
	ctx.r[4].s64 = ctx.r[10].s64 + 12588;
	// 826A3360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3364: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A336C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3370: 386A7178  addi r3, r10, 0x7178
	ctx.r[3].s64 = ctx.r[10].s64 + 29048;
	// 826A3374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A337C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A338C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3394: 4BDC3A8D  bl 0x82466e20
	ctx.lr = 0x826A3398;
	sub_82466E20(ctx, base);
	// 826A3398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A339C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A33A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A33A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A33A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A33A8 size=112
    let mut pc: u32 = 0x826A33A8;
    'dispatch: loop {
        match pc {
            0x826A33A8 => {
    //   block [0x826A33A8..0x826A3418)
	// 826A33A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A33AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A33B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A33B4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A33B8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A33BC: 38EA9420  addi r7, r10, -0x6be0
	ctx.r[7].s64 = ctx.r[10].s64 + -27616;
	// 826A33C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A33C4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A33C8: 388A3144  addi r4, r10, 0x3144
	ctx.r[4].s64 = ctx.r[10].s64 + 12612;
	// 826A33CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A33D0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A33D4: 396BBB1C  addi r11, r11, -0x44e4
	ctx.r[11].s64 = ctx.r[11].s64 + -17636;
	// 826A33D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A33DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A33E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A33E4: 386A71A8  addi r3, r10, 0x71a8
	ctx.r[3].s64 = ctx.r[10].s64 + 29096;
	// 826A33E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A33EC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A33F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A33F4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A33F8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A33FC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3400: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3404: 4BDC3A1D  bl 0x82466e20
	ctx.lr = 0x826A3408;
	sub_82466E20(ctx, base);
	// 826A3408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A340C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3418 size=112
    let mut pc: u32 = 0x826A3418;
    'dispatch: loop {
        match pc {
            0x826A3418 => {
    //   block [0x826A3418..0x826A3488)
	// 826A3418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A341C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3424: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3428: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A342C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3434: 390B9450  addi r8, r11, -0x6bb0
	ctx.r[8].s64 = ctx.r[11].s64 + -27568;
	// 826A3438: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A343C: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826A3440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3448: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A344C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3450: 386A71D8  addi r3, r10, 0x71d8
	ctx.r[3].s64 = ctx.r[10].s64 + 29144;
	// 826A3454: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A345C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3464: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A346C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3474: 4BDC39AD  bl 0x82466e20
	ctx.lr = 0x826A3478;
	sub_82466E20(ctx, base);
	// 826A3478: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A347C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3480: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3488 size=108
    let mut pc: u32 = 0x826A3488;
    'dispatch: loop {
        match pc {
            0x826A3488 => {
    //   block [0x826A3488..0x826A34F4)
	// 826A3488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A348C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3490: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3494: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3498: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A349C: 38EB9468  addi r7, r11, -0x6b98
	ctx.r[7].s64 = ctx.r[11].s64 + -27544;
	// 826A34A0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826A34A4: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826A34A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A34AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A34B0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A34B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A34B8: 386A7208  addi r3, r10, 0x7208
	ctx.r[3].s64 = ctx.r[10].s64 + 29192;
	// 826A34BC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A34C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A34C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A34C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A34CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A34D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A34D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A34D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A34DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A34E0: 4BDC3941  bl 0x82466e20
	ctx.lr = 0x826A34E4;
	sub_82466E20(ctx, base);
	// 826A34E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A34E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A34EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A34F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A34F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A34F8 size=112
    let mut pc: u32 = 0x826A34F8;
    'dispatch: loop {
        match pc {
            0x826A34F8 => {
    //   block [0x826A34F8..0x826A3568)
	// 826A34F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A34FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3504: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3508: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A350C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3514: 390B9480  addi r8, r11, -0x6b80
	ctx.r[8].s64 = ctx.r[11].s64 + -27520;
	// 826A3518: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A351C: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826A3520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3524: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A352C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3530: 386A7238  addi r3, r10, 0x7238
	ctx.r[3].s64 = ctx.r[10].s64 + 29240;
	// 826A3534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A353C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A354C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3554: 4BDC38CD  bl 0x82466e20
	ctx.lr = 0x826A3558;
	sub_82466E20(ctx, base);
	// 826A3558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A355C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3568 size=112
    let mut pc: u32 = 0x826A3568;
    'dispatch: loop {
        match pc {
            0x826A3568 => {
    //   block [0x826A3568..0x826A35D8)
	// 826A3568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A356C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3574: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3578: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A357C: 38EA9498  addi r7, r10, -0x6b68
	ctx.r[7].s64 = ctx.r[10].s64 + -27496;
	// 826A3580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3584: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3588: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826A358C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3590: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3594: 396BBB28  addi r11, r11, -0x44d8
	ctx.r[11].s64 = ctx.r[11].s64 + -17624;
	// 826A3598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A359C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A35A0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A35A4: 386A7268  addi r3, r10, 0x7268
	ctx.r[3].s64 = ctx.r[10].s64 + 29288;
	// 826A35A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A35AC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A35B0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A35B4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A35B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A35BC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A35C0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A35C4: 4BDC385D  bl 0x82466e20
	ctx.lr = 0x826A35C8;
	sub_82466E20(ctx, base);
	// 826A35C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A35CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A35D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A35D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A35D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A35D8 size=112
    let mut pc: u32 = 0x826A35D8;
    'dispatch: loop {
        match pc {
            0x826A35D8 => {
    //   block [0x826A35D8..0x826A3648)
	// 826A35D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A35DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A35E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A35E4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A35E8: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826A35EC: 38EA9570  addi r7, r10, -0x6a90
	ctx.r[7].s64 = ctx.r[10].s64 + -27280;
	// 826A35F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A35F4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A35F8: 388A31A4  addi r4, r10, 0x31a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12708;
	// 826A35FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3600: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3604: 396BBB68  addi r11, r11, -0x4498
	ctx.r[11].s64 = ctx.r[11].s64 + -17560;
	// 826A3608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A360C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3610: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3614: 386A7298  addi r3, r10, 0x7298
	ctx.r[3].s64 = ctx.r[10].s64 + 29336;
	// 826A3618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A361C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3620: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3624: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3628: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A362C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3630: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3634: 4BDC37ED  bl 0x82466e20
	ctx.lr = 0x826A3638;
	sub_82466E20(ctx, base);
	// 826A3638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A363C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3648 size=108
    let mut pc: u32 = 0x826A3648;
    'dispatch: loop {
        match pc {
            0x826A3648 => {
    //   block [0x826A3648..0x826A36B4)
	// 826A3648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A364C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3654: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3658: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A365C: 38EB96C0  addi r7, r11, -0x6940
	ctx.r[7].s64 = ctx.r[11].s64 + -26944;
	// 826A3660: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826A3664: 388A31BC  addi r4, r10, 0x31bc
	ctx.r[4].s64 = ctx.r[10].s64 + 12732;
	// 826A3668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A366C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3670: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A3674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3678: 386A72C8  addi r3, r10, 0x72c8
	ctx.r[3].s64 = ctx.r[10].s64 + 29384;
	// 826A367C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3680: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3684: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3688: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A368C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3690: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3694: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3698: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A369C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A36A0: 4BDC3781  bl 0x82466e20
	ctx.lr = 0x826A36A4;
	sub_82466E20(ctx, base);
	// 826A36A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A36A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A36AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A36B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A36B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A36B8 size=116
    let mut pc: u32 = 0x826A36B8;
    'dispatch: loop {
        match pc {
            0x826A36B8 => {
    //   block [0x826A36B8..0x826A372C)
	// 826A36B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A36BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A36C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A36C4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A36C8: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 826A36CC: 390A9720  addi r8, r10, -0x68e0
	ctx.r[8].s64 = ctx.r[10].s64 + -26848;
	// 826A36D0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A36D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A36D8: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A36DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A36E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A36E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A36E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A36EC: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826A36F0: 396BBC08  addi r11, r11, -0x43f8
	ctx.r[11].s64 = ctx.r[11].s64 + -17400;
	// 826A36F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A36F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A36FC: 386A72F8  addi r3, r10, 0x72f8
	ctx.r[3].s64 = ctx.r[10].s64 + 29432;
	// 826A3700: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3704: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3708: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A370C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3718: 4BDC3709  bl 0x82466e20
	ctx.lr = 0x826A371C;
	sub_82466E20(ctx, base);
	// 826A371C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3730 size=116
    let mut pc: u32 = 0x826A3730;
    'dispatch: loop {
        match pc {
            0x826A3730 => {
    //   block [0x826A3730..0x826A37A4)
	// 826A3730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A373C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3740: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826A3744: 390A9840  addi r8, r10, -0x67c0
	ctx.r[8].s64 = ctx.r[10].s64 + -26560;
	// 826A3748: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A374C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3750: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3754: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3758: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A375C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3764: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826A3768: 396BBC40  addi r11, r11, -0x43c0
	ctx.r[11].s64 = ctx.r[11].s64 + -17344;
	// 826A376C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3770: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3774: 386A7328  addi r3, r10, 0x7328
	ctx.r[3].s64 = ctx.r[10].s64 + 29480;
	// 826A3778: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A377C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3780: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A378C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3790: 4BDC3691  bl 0x82466e20
	ctx.lr = 0x826A3794;
	sub_82466E20(ctx, base);
	// 826A3794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A379C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A37A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A37A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A37A8 size=108
    let mut pc: u32 = 0x826A37A8;
    'dispatch: loop {
        match pc {
            0x826A37A8 => {
    //   block [0x826A37A8..0x826A3814)
	// 826A37A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A37AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A37B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A37B4: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A37B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A37BC: 38EB98A0  addi r7, r11, -0x6760
	ctx.r[7].s64 = ctx.r[11].s64 + -26464;
	// 826A37C0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826A37C4: 388AA8F8  addi r4, r10, -0x5708
	ctx.r[4].s64 = ctx.r[10].s64 + -22280;
	// 826A37C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A37CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A37D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A37D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A37D8: 386A7358  addi r3, r10, 0x7358
	ctx.r[3].s64 = ctx.r[10].s64 + 29528;
	// 826A37DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A37E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A37E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A37E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A37EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A37F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A37F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A37F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A37FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3800: 4BDC3621  bl 0x82466e20
	ctx.lr = 0x826A3804;
	sub_82466E20(ctx, base);
	// 826A3804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A380C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3818 size=112
    let mut pc: u32 = 0x826A3818;
    'dispatch: loop {
        match pc {
            0x826A3818 => {
    //   block [0x826A3818..0x826A3888)
	// 826A3818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A381C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3824: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3828: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826A382C: 38EA9948  addi r7, r10, -0x66b8
	ctx.r[7].s64 = ctx.r[10].s64 + -26296;
	// 826A3830: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A3834: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3838: 388AA910  addi r4, r10, -0x56f0
	ctx.r[4].s64 = ctx.r[10].s64 + -22256;
	// 826A383C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3840: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3844: 396BBC58  addi r11, r11, -0x43a8
	ctx.r[11].s64 = ctx.r[11].s64 + -17320;
	// 826A3848: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A384C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3850: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3854: 386A7388  addi r3, r10, 0x7388
	ctx.r[3].s64 = ctx.r[10].s64 + 29576;
	// 826A3858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A385C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3860: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3864: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3868: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A386C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3870: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3874: 4BDC35AD  bl 0x82466e20
	ctx.lr = 0x826A3878;
	sub_82466E20(ctx, base);
	// 826A3878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A387C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3888 size=112
    let mut pc: u32 = 0x826A3888;
    'dispatch: loop {
        match pc {
            0x826A3888 => {
    //   block [0x826A3888..0x826A38F8)
	// 826A3888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A388C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3894: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3898: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A389C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A38A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A38A4: 390B9A20  addi r8, r11, -0x65e0
	ctx.r[8].s64 = ctx.r[11].s64 + -26080;
	// 826A38A8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826A38AC: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826A38B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A38B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A38B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A38BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A38C0: 386A73B8  addi r3, r10, 0x73b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29624;
	// 826A38C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A38C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A38CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A38D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A38D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A38D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A38DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A38E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A38E4: 4BDC353D  bl 0x82466e20
	ctx.lr = 0x826A38E8;
	sub_82466E20(ctx, base);
	// 826A38E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A38EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A38F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A38F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A38F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A38F8 size=24
    let mut pc: u32 = 0x826A38F8;
    'dispatch: loop {
        match pc {
            0x826A38F8 => {
    //   block [0x826A38F8..0x826A3910)
	// 826A38F8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A38FC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3900: 394A2080  addi r10, r10, 0x2080
	ctx.r[10].s64 = ctx.r[10].s64 + 8320;
	// 826A3904: 816B9A68  lwz r11, -0x6598(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26008 as u32) ) } as u64;
	// 826A3908: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 826A390C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3910 size=116
    let mut pc: u32 = 0x826A3910;
    'dispatch: loop {
        match pc {
            0x826A3910 => {
    //   block [0x826A3910..0x826A3984)
	// 826A3910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A391C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3920: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A3924: 392BBCC8  addi r9, r11, -0x4338
	ctx.r[9].s64 = ctx.r[11].s64 + -17208;
	// 826A3928: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A392C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A3930: 38E90018  addi r7, r9, 0x18
	ctx.r[7].s64 = ctx.r[9].s64 + 24;
	// 826A3934: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 826A3938: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A393C: 388A7B20  addi r4, r10, 0x7b20
	ctx.r[4].s64 = ctx.r[10].s64 + 31520;
	// 826A3940: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3944: 396B2080  addi r11, r11, 0x2080
	ctx.r[11].s64 = ctx.r[11].s64 + 8320;
	// 826A3948: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826A394C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3950: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826A3954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3958: 386A73E8  addi r3, r10, 0x73e8
	ctx.r[3].s64 = ctx.r[10].s64 + 29672;
	// 826A395C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A3960: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826A3964: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3968: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826A396C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826A3970: 4BDC34B1  bl 0x82466e20
	ctx.lr = 0x826A3974;
	sub_82466E20(ctx, base);
	// 826A3974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3978: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A397C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3988 size=116
    let mut pc: u32 = 0x826A3988;
    'dispatch: loop {
        match pc {
            0x826A3988 => {
    //   block [0x826A3988..0x826A39FC)
	// 826A3988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A398C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3990: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3994: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3998: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826A399C: 390A9A70  addi r8, r10, -0x6590
	ctx.r[8].s64 = ctx.r[10].s64 + -26000;
	// 826A39A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A39A4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A39A8: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A39AC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A39B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A39B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A39B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A39BC: 388AB2FC  addi r4, r10, -0x4d04
	ctx.r[4].s64 = ctx.r[10].s64 + -19716;
	// 826A39C0: 396BBD24  addi r11, r11, -0x42dc
	ctx.r[11].s64 = ctx.r[11].s64 + -17116;
	// 826A39C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A39C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A39CC: 386A7418  addi r3, r10, 0x7418
	ctx.r[3].s64 = ctx.r[10].s64 + 29720;
	// 826A39D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A39D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A39D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A39DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A39E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A39E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A39E8: 4BDC3439  bl 0x82466e20
	ctx.lr = 0x826A39EC;
	sub_82466E20(ctx, base);
	// 826A39EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A39F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A39F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A39F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3A00 size=116
    let mut pc: u32 = 0x826A3A00;
    'dispatch: loop {
        match pc {
            0x826A3A00 => {
    //   block [0x826A3A00..0x826A3A74)
	// 826A3A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3A08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3A0C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3A10: 38E0000D  li r7, 0xd
	ctx.r[7].s64 = 13;
	// 826A3A14: 390A9AB8  addi r8, r10, -0x6548
	ctx.r[8].s64 = ctx.r[10].s64 + -25928;
	// 826A3A18: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3A1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3A20: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3A24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3A28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A3A2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3A30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3A34: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826A3A38: 396BBD38  addi r11, r11, -0x42c8
	ctx.r[11].s64 = ctx.r[11].s64 + -17096;
	// 826A3A3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3A40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3A44: 386A7448  addi r3, r10, 0x7448
	ctx.r[3].s64 = ctx.r[10].s64 + 29768;
	// 826A3A48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3A4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3A50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3A54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3A58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3A5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3A60: 4BDC33C1  bl 0x82466e20
	ctx.lr = 0x826A3A64;
	sub_82466E20(ctx, base);
	// 826A3A64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3A68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3A6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3A78 size=112
    let mut pc: u32 = 0x826A3A78;
    'dispatch: loop {
        match pc {
            0x826A3A78 => {
    //   block [0x826A3A78..0x826A3AE8)
	// 826A3A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3A84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3A88: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3A8C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3A90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826A3A94: 390B9BF0  addi r8, r11, -0x6410
	ctx.r[8].s64 = ctx.r[11].s64 + -25616;
	// 826A3A98: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3A9C: 388AA928  addi r4, r10, -0x56d8
	ctx.r[4].s64 = ctx.r[10].s64 + -22232;
	// 826A3AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3AB0: 386A7478  addi r3, r10, 0x7478
	ctx.r[3].s64 = ctx.r[10].s64 + 29816;
	// 826A3AB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3ABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3AC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3ACC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3AD4: 4BDC334D  bl 0x82466e20
	ctx.lr = 0x826A3AD8;
	sub_82466E20(ctx, base);
	// 826A3AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3AE8 size=112
    let mut pc: u32 = 0x826A3AE8;
    'dispatch: loop {
        match pc {
            0x826A3AE8 => {
    //   block [0x826A3AE8..0x826A3B58)
	// 826A3AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3AF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3AF8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3AFC: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3B00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3B04: 390B9C08  addi r8, r11, -0x63f8
	ctx.r[8].s64 = ctx.r[11].s64 + -25592;
	// 826A3B08: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3B0C: 388A3288  addi r4, r10, 0x3288
	ctx.r[4].s64 = ctx.r[10].s64 + 12936;
	// 826A3B10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3B14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3B18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3B20: 386A74A8  addi r3, r10, 0x74a8
	ctx.r[3].s64 = ctx.r[10].s64 + 29864;
	// 826A3B24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3B28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3B2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3B30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3B38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3B40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3B44: 4BDC32DD  bl 0x82466e20
	ctx.lr = 0x826A3B48;
	sub_82466E20(ctx, base);
	// 826A3B48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3B4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3B50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3B58 size=112
    let mut pc: u32 = 0x826A3B58;
    'dispatch: loop {
        match pc {
            0x826A3B58 => {
    //   block [0x826A3B58..0x826A3BC8)
	// 826A3B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3B64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826A3B68: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3B6C: 38AA85E8  addi r5, r10, -0x7a18
	ctx.r[5].s64 = ctx.r[10].s64 + -31256;
	// 826A3B70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3B74: 390B9C20  addi r8, r11, -0x63e0
	ctx.r[8].s64 = ctx.r[11].s64 + -25568;
	// 826A3B78: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A3B7C: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826A3B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3B84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3B8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3B90: 386A74D8  addi r3, r10, 0x74d8
	ctx.r[3].s64 = ctx.r[10].s64 + 29912;
	// 826A3B94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3B98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3B9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3BA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3BAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3BB4: 4BDC326D  bl 0x82466e20
	ctx.lr = 0x826A3BB8;
	sub_82466E20(ctx, base);
	// 826A3BB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3BBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3BC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3BC8 size=112
    let mut pc: u32 = 0x826A3BC8;
    'dispatch: loop {
        match pc {
            0x826A3BC8 => {
    //   block [0x826A3BC8..0x826A3C38)
	// 826A3BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3BD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3BD4: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3BD8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826A3BDC: 38EA9C50  addi r7, r10, -0x63b0
	ctx.r[7].s64 = ctx.r[10].s64 + -25520;
	// 826A3BE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3BE4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3BE8: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826A3BEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3BF0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3BF4: 396BBDB0  addi r11, r11, -0x4250
	ctx.r[11].s64 = ctx.r[11].s64 + -16976;
	// 826A3BF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A3BFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3C00: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3C04: 386A7508  addi r3, r10, 0x7508
	ctx.r[3].s64 = ctx.r[10].s64 + 29960;
	// 826A3C08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3C0C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3C10: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3C14: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3C18: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3C1C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3C20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3C24: 4BDC31FD  bl 0x82466e20
	ctx.lr = 0x826A3C28;
	sub_82466E20(ctx, base);
	// 826A3C28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3C38 size=112
    let mut pc: u32 = 0x826A3C38;
    'dispatch: loop {
        match pc {
            0x826A3C38 => {
    //   block [0x826A3C38..0x826A3CA8)
	// 826A3C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3C40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3C44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3C48: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3C4C: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3C50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3C54: 390B9CC8  addi r8, r11, -0x6338
	ctx.r[8].s64 = ctx.r[11].s64 + -25400;
	// 826A3C58: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826A3C5C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826A3C60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3C64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3C68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3C70: 386A7538  addi r3, r10, 0x7538
	ctx.r[3].s64 = ctx.r[10].s64 + 30008;
	// 826A3C74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3C78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3C7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3C80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3C84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3C88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3C8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3C90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3C94: 4BDC318D  bl 0x82466e20
	ctx.lr = 0x826A3C98;
	sub_82466E20(ctx, base);
	// 826A3C98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3CA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826A3CA8 size=24
    let mut pc: u32 = 0x826A3CA8;
    'dispatch: loop {
        match pc {
            0x826A3CA8 => {
    //   block [0x826A3CA8..0x826A3CC0)
	// 826A3CA8: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3CAC: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3CB0: 394A2188  addi r10, r10, 0x2188
	ctx.r[10].s64 = ctx.r[10].s64 + 8584;
	// 826A3CB4: 816B9A6C  lwz r11, -0x6594(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-26004 as u32) ) } as u64;
	// 826A3CB8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826A3CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3CC0 size=116
    let mut pc: u32 = 0x826A3CC0;
    'dispatch: loop {
        match pc {
            0x826A3CC0 => {
    //   block [0x826A3CC0..0x826A3D34)
	// 826A3CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3CCC: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3CD0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826A3CD4: 390B2188  addi r8, r11, 0x2188
	ctx.r[8].s64 = ctx.r[11].s64 + 8584;
	// 826A3CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3CDC: 392ABDF0  addi r9, r10, -0x4210
	ctx.r[9].s64 = ctx.r[10].s64 + -16912;
	// 826A3CE0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3CE4: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826A3CE8: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3CEC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3CF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3CF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3CF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3D00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3D04: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826A3D08: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826A3D0C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A3D10: 386B7568  addi r3, r11, 0x7568
	ctx.r[3].s64 = ctx.r[11].s64 + 30056;
	// 826A3D14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826A3D18: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3D20: 4BDC3101  bl 0x82466e20
	ctx.lr = 0x826A3D24;
	sub_82466E20(ctx, base);
	// 826A3D24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3D28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3D2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3D38 size=116
    let mut pc: u32 = 0x826A3D38;
    'dispatch: loop {
        match pc {
            0x826A3D38 => {
    //   block [0x826A3D38..0x826A3DAC)
	// 826A3D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3D44: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3D48: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826A3D4C: 390A9CF8  addi r8, r10, -0x6308
	ctx.r[8].s64 = ctx.r[10].s64 + -25352;
	// 826A3D50: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3D54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3D58: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3D5C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3D60: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A3D64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3D6C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826A3D70: 396BBE04  addi r11, r11, -0x41fc
	ctx.r[11].s64 = ctx.r[11].s64 + -16892;
	// 826A3D74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3D78: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3D7C: 386A7598  addi r3, r10, 0x7598
	ctx.r[3].s64 = ctx.r[10].s64 + 30104;
	// 826A3D80: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3D84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3D88: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3D8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3D94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3D98: 4BDC3089  bl 0x82466e20
	ctx.lr = 0x826A3D9C;
	sub_82466E20(ctx, base);
	// 826A3D9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3DB0 size=112
    let mut pc: u32 = 0x826A3DB0;
    'dispatch: loop {
        match pc {
            0x826A3DB0 => {
    //   block [0x826A3DB0..0x826A3E20)
	// 826A3DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3DB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3DBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3DC0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3DC4: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3DC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3DCC: 390B9DB8  addi r8, r11, -0x6248
	ctx.r[8].s64 = ctx.r[11].s64 + -25160;
	// 826A3DD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3DD4: 388A335C  addi r4, r10, 0x335c
	ctx.r[4].s64 = ctx.r[10].s64 + 13148;
	// 826A3DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3DE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3DE8: 386A75C8  addi r3, r10, 0x75c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30152;
	// 826A3DEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3DF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3DFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3E0C: 4BDC3015  bl 0x82466e20
	ctx.lr = 0x826A3E10;
	sub_82466E20(ctx, base);
	// 826A3E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3E14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3E18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3E1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3E20 size=112
    let mut pc: u32 = 0x826A3E20;
    'dispatch: loop {
        match pc {
            0x826A3E20 => {
    //   block [0x826A3E20..0x826A3E90)
	// 826A3E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3E28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3E2C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3E30: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826A3E34: 38EA9DD0  addi r7, r10, -0x6230
	ctx.r[7].s64 = ctx.r[10].s64 + -25136;
	// 826A3E38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3E3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3E40: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826A3E44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3E48: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826A3E4C: 396BBE28  addi r11, r11, -0x41d8
	ctx.r[11].s64 = ctx.r[11].s64 + -16856;
	// 826A3E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826A3E54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3E58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3E5C: 386A75F8  addi r3, r10, 0x75f8
	ctx.r[3].s64 = ctx.r[10].s64 + 30200;
	// 826A3E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3E64: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3E68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3E6C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3E70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3E74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3E78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826A3E7C: 4BDC2FA5  bl 0x82466e20
	ctx.lr = 0x826A3E80;
	sub_82466E20(ctx, base);
	// 826A3E80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3E90 size=112
    let mut pc: u32 = 0x826A3E90;
    'dispatch: loop {
        match pc {
            0x826A3E90 => {
    //   block [0x826A3E90..0x826A3F00)
	// 826A3E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3E9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3EA0: 3D60827D  lis r11, -0x7d83
	ctx.r[11].s64 = -2105737216;
	// 826A3EA4: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3EA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3EAC: 390B9E00  addi r8, r11, -0x6200
	ctx.r[8].s64 = ctx.r[11].s64 + -25088;
	// 826A3EB0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826A3EB4: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826A3EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826A3EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3EC8: 386A7628  addi r3, r10, 0x7628
	ctx.r[3].s64 = ctx.r[10].s64 + 30248;
	// 826A3ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826A3ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826A3EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826A3EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3EEC: 4BDC2F35  bl 0x82466e20
	ctx.lr = 0x826A3EF0;
	sub_82466E20(ctx, base);
	// 826A3EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826A3F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826A3F00 size=116
    let mut pc: u32 = 0x826A3F00;
    'dispatch: loop {
        match pc {
            0x826A3F00 => {
    //   block [0x826A3F00..0x826A3F74)
	// 826A3F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826A3F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826A3F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826A3F0C: 3D40827D  lis r10, -0x7d83
	ctx.r[10].s64 = -2105737216;
	// 826A3F10: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826A3F14: 390A9E18  addi r8, r10, -0x61e8
	ctx.r[8].s64 = ctx.r[10].s64 + -25064;
	// 826A3F18: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3F1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826A3F20: 38AA7118  addi r5, r10, 0x7118
	ctx.r[5].s64 = ctx.r[10].s64 + 28952;
	// 826A3F24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826A3F28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826A3F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826A3F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826A3F34: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826A3F38: 396BBE34  addi r11, r11, -0x41cc
	ctx.r[11].s64 = ctx.r[11].s64 + -16844;
	// 826A3F3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826A3F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826A3F44: 386A7658  addi r3, r10, 0x7658
	ctx.r[3].s64 = ctx.r[10].s64 + 30296;
	// 826A3F48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826A3F4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826A3F50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826A3F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826A3F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826A3F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826A3F60: 4BDC2EC1  bl 0x82466e20
	ctx.lr = 0x826A3F64;
	sub_82466E20(ctx, base);
	// 826A3F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826A3F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826A3F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826A3F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


