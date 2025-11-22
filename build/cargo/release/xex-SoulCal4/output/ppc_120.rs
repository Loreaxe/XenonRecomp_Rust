pub fn sub_826B7238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7238 size=112
    let mut pc: u32 = 0x826B7238;
    'dispatch: loop {
        match pc {
            0x826B7238 => {
    //   block [0x826B7238..0x826B72A8)
	// 826B7238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B723C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7244: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B7248: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 826B724C: 38EAA2D8  addi r7, r10, -0x5d28
	ctx.r[7].s64 = ctx.r[10].s64 + -23848;
	// 826B7250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7254: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B7258: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 826B725C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7260: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7264: 396BFD08  addi r11, r11, -0x2f8
	ctx.r[11].s64 = ctx.r[11].s64 + -760;
	// 826B7268: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B726C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7270: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7274: 386AF6E4  addi r3, r10, -0x91c
	ctx.r[3].s64 = ctx.r[10].s64 + -2332;
	// 826B7278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B727C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B7280: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7284: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B7288: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B728C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7290: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7294: 4BDAFB8D  bl 0x82466e20
	ctx.lr = 0x826B7298;
	sub_82466E20(ctx, base);
	// 826B7298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B729C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B72A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B72A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B72A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B72A8 size=112
    let mut pc: u32 = 0x826B72A8;
    'dispatch: loop {
        match pc {
            0x826B72A8 => {
    //   block [0x826B72A8..0x826B7318)
	// 826B72A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B72AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B72B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B72B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B72B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B72BC: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B72C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B72C4: 390BA4A0  addi r8, r11, -0x5b60
	ctx.r[8].s64 = ctx.r[11].s64 + -23392;
	// 826B72C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B72CC: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826B72D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B72D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B72D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B72DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B72E0: 386AF714  addi r3, r10, -0x8ec
	ctx.r[3].s64 = ctx.r[10].s64 + -2284;
	// 826B72E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B72E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B72EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B72F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B72F4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B72F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B72FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7304: 4BDAFB1D  bl 0x82466e20
	ctx.lr = 0x826B7308;
	sub_82466E20(ctx, base);
	// 826B7308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B730C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7318 size=112
    let mut pc: u32 = 0x826B7318;
    'dispatch: loop {
        match pc {
            0x826B7318 => {
    //   block [0x826B7318..0x826B7388)
	// 826B7318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B731C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7324: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7328: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B732C: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B7330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7334: 390BA4B8  addi r8, r11, -0x5b48
	ctx.r[8].s64 = ctx.r[11].s64 + -23368;
	// 826B7338: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B733C: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 826B7340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B734C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7350: 386AF744  addi r3, r10, -0x8bc
	ctx.r[3].s64 = ctx.r[10].s64 + -2236;
	// 826B7354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B735C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7364: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B7368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B736C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7374: 4BDAFAAD  bl 0x82466e20
	ctx.lr = 0x826B7378;
	sub_82466E20(ctx, base);
	// 826B7378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B737C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7388 size=112
    let mut pc: u32 = 0x826B7388;
    'dispatch: loop {
        match pc {
            0x826B7388 => {
    //   block [0x826B7388..0x826B73F8)
	// 826B7388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B738C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7394: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7398: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B739C: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B73A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B73A4: 390BA4D0  addi r8, r11, -0x5b30
	ctx.r[8].s64 = ctx.r[11].s64 + -23344;
	// 826B73A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B73AC: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826B73B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B73B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B73B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B73BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B73C0: 386AF774  addi r3, r10, -0x88c
	ctx.r[3].s64 = ctx.r[10].s64 + -2188;
	// 826B73C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B73C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B73CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B73D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B73D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B73D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B73DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B73E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B73E4: 4BDAFA3D  bl 0x82466e20
	ctx.lr = 0x826B73E8;
	sub_82466E20(ctx, base);
	// 826B73E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B73EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B73F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B73F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B73F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B73F8 size=108
    let mut pc: u32 = 0x826B73F8;
    'dispatch: loop {
        match pc {
            0x826B73F8 => {
    //   block [0x826B73F8..0x826B7464)
	// 826B73F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B73FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7404: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B740C: 38EBA500  addi r7, r11, -0x5b00
	ctx.r[7].s64 = ctx.r[11].s64 + -23296;
	// 826B7410: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B7414: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 826B7418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B741C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7428: 386AF7A4  addi r3, r10, -0x85c
	ctx.r[3].s64 = ctx.r[10].s64 + -2140;
	// 826B742C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B743C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B744C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7450: 4BDAF9D1  bl 0x82466e20
	ctx.lr = 0x826B7454;
	sub_82466E20(ctx, base);
	// 826B7454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B745C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7468 size=112
    let mut pc: u32 = 0x826B7468;
    'dispatch: loop {
        match pc {
            0x826B7468 => {
    //   block [0x826B7468..0x826B74D8)
	// 826B7468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B746C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7474: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7478: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B747C: 38AAD824  addi r5, r10, -0x27dc
	ctx.r[5].s64 = ctx.r[10].s64 + -10204;
	// 826B7480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7484: 390BA530  addi r8, r11, -0x5ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -23248;
	// 826B7488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B748C: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 826B7490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B749C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B74A0: 386AF7D4  addi r3, r10, -0x82c
	ctx.r[3].s64 = ctx.r[10].s64 + -2092;
	// 826B74A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B74A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B74AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B74B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B74B4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B74B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B74BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B74C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B74C4: 4BDAF95D  bl 0x82466e20
	ctx.lr = 0x826B74C8;
	sub_82466E20(ctx, base);
	// 826B74C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B74CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B74D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B74D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B74D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B74D8 size=108
    let mut pc: u32 = 0x826B74D8;
    'dispatch: loop {
        match pc {
            0x826B74D8 => {
    //   block [0x826B74D8..0x826B7544)
	// 826B74D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B74DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B74E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B74E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B74E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B74EC: 38EBA548  addi r7, r11, -0x5ab8
	ctx.r[7].s64 = ctx.r[11].s64 + -23224;
	// 826B74F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B74F4: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 826B74F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B74FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7508: 386AF804  addi r3, r10, -0x7fc
	ctx.r[3].s64 = ctx.r[10].s64 + -2044;
	// 826B750C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B751C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B752C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7530: 4BDAF8F1  bl 0x82466e20
	ctx.lr = 0x826B7534;
	sub_82466E20(ctx, base);
	// 826B7534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7548 size=108
    let mut pc: u32 = 0x826B7548;
    'dispatch: loop {
        match pc {
            0x826B7548 => {
    //   block [0x826B7548..0x826B75B4)
	// 826B7548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B754C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7554: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B755C: 38EBA578  addi r7, r11, -0x5a88
	ctx.r[7].s64 = ctx.r[11].s64 + -23176;
	// 826B7560: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B7564: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 826B7568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B756C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7570: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7578: 386AF834  addi r3, r10, -0x7cc
	ctx.r[3].s64 = ctx.r[10].s64 + -1996;
	// 826B757C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B758C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B759C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B75A0: 4BDAF881  bl 0x82466e20
	ctx.lr = 0x826B75A4;
	sub_82466E20(ctx, base);
	// 826B75A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B75A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B75AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B75B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B75B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B75B8 size=112
    let mut pc: u32 = 0x826B75B8;
    'dispatch: loop {
        match pc {
            0x826B75B8 => {
    //   block [0x826B75B8..0x826B7628)
	// 826B75B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B75BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B75C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B75C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B75C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B75CC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B75D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B75D4: 390BA5C0  addi r8, r11, -0x5a40
	ctx.r[8].s64 = ctx.r[11].s64 + -23104;
	// 826B75D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B75DC: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826B75E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B75E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B75E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B75EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B75F0: 386AF864  addi r3, r10, -0x79c
	ctx.r[3].s64 = ctx.r[10].s64 + -1948;
	// 826B75F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B75F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B75FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7600: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7604: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7608: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B760C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7610: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7614: 4BDAF80D  bl 0x82466e20
	ctx.lr = 0x826B7618;
	sub_82466E20(ctx, base);
	// 826B7618: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B761C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7620: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7628 size=112
    let mut pc: u32 = 0x826B7628;
    'dispatch: loop {
        match pc {
            0x826B7628 => {
    //   block [0x826B7628..0x826B7698)
	// 826B7628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7634: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7638: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B763C: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 826B7640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7644: 390BA608  addi r8, r11, -0x59f8
	ctx.r[8].s64 = ctx.r[11].s64 + -23032;
	// 826B7648: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826B764C: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 826B7650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7658: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B765C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7660: 386AF894  addi r3, r10, -0x76c
	ctx.r[3].s64 = ctx.r[10].s64 + -1900;
	// 826B7664: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B766C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7674: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B767C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7684: 4BDAF79D  bl 0x82466e20
	ctx.lr = 0x826B7688;
	sub_82466E20(ctx, base);
	// 826B7688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B768C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7698 size=108
    let mut pc: u32 = 0x826B7698;
    'dispatch: loop {
        match pc {
            0x826B7698 => {
    //   block [0x826B7698..0x826B7704)
	// 826B7698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B76A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B76A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B76A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B76AC: 38EBA698  addi r7, r11, -0x5968
	ctx.r[7].s64 = ctx.r[11].s64 + -22888;
	// 826B76B0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B76B4: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826B76B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B76BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B76C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B76C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B76C8: 386AF8C4  addi r3, r10, -0x73c
	ctx.r[3].s64 = ctx.r[10].s64 + -1852;
	// 826B76CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B76D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B76D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B76D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B76DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B76E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B76E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B76E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B76EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B76F0: 4BDAF731  bl 0x82466e20
	ctx.lr = 0x826B76F4;
	sub_82466E20(ctx, base);
	// 826B76F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B76F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B76FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7708 size=108
    let mut pc: u32 = 0x826B7708;
    'dispatch: loop {
        match pc {
            0x826B7708 => {
    //   block [0x826B7708..0x826B7774)
	// 826B7708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B770C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7714: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B771C: 38EBA6E0  addi r7, r11, -0x5920
	ctx.r[7].s64 = ctx.r[11].s64 + -22816;
	// 826B7720: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B7724: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 826B7728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B772C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7730: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7738: 386AF8F4  addi r3, r10, -0x70c
	ctx.r[3].s64 = ctx.r[10].s64 + -1804;
	// 826B773C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B774C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B775C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7760: 4BDAF6C1  bl 0x82466e20
	ctx.lr = 0x826B7764;
	sub_82466E20(ctx, base);
	// 826B7764: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B776C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7770: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7778 size=108
    let mut pc: u32 = 0x826B7778;
    'dispatch: loop {
        match pc {
            0x826B7778 => {
    //   block [0x826B7778..0x826B77E4)
	// 826B7778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B777C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7780: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7784: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B778C: 38EBA710  addi r7, r11, -0x58f0
	ctx.r[7].s64 = ctx.r[11].s64 + -22768;
	// 826B7790: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B7794: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826B7798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B779C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B77A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B77A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B77A8: 386AF924  addi r3, r10, -0x6dc
	ctx.r[3].s64 = ctx.r[10].s64 + -1756;
	// 826B77AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B77B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B77B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B77B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B77BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B77C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B77C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B77C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B77CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B77D0: 4BDAF651  bl 0x82466e20
	ctx.lr = 0x826B77D4;
	sub_82466E20(ctx, base);
	// 826B77D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B77D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B77DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B77E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B77E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B77E8 size=112
    let mut pc: u32 = 0x826B77E8;
    'dispatch: loop {
        match pc {
            0x826B77E8 => {
    //   block [0x826B77E8..0x826B7858)
	// 826B77E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B77EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B77F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B77F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B77F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B77FC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7804: 390BA740  addi r8, r11, -0x58c0
	ctx.r[8].s64 = ctx.r[11].s64 + -22720;
	// 826B7808: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B780C: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 826B7810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7814: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7818: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B781C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7820: 386AF954  addi r3, r10, -0x6ac
	ctx.r[3].s64 = ctx.r[10].s64 + -1708;
	// 826B7824: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B782C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B783C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7844: 4BDAF5DD  bl 0x82466e20
	ctx.lr = 0x826B7848;
	sub_82466E20(ctx, base);
	// 826B7848: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B784C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7858 size=112
    let mut pc: u32 = 0x826B7858;
    'dispatch: loop {
        match pc {
            0x826B7858 => {
    //   block [0x826B7858..0x826B78C8)
	// 826B7858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B785C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7860: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7868: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B786C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7874: 390BA770  addi r8, r11, -0x5890
	ctx.r[8].s64 = ctx.r[11].s64 + -22672;
	// 826B7878: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B787C: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 826B7880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7884: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7888: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B788C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7890: 386AF984  addi r3, r10, -0x67c
	ctx.r[3].s64 = ctx.r[10].s64 + -1660;
	// 826B7894: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B789C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B78A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B78A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B78A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B78AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B78B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B78B4: 4BDAF56D  bl 0x82466e20
	ctx.lr = 0x826B78B8;
	sub_82466E20(ctx, base);
	// 826B78B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B78BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B78C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B78C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B78C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B78C8 size=112
    let mut pc: u32 = 0x826B78C8;
    'dispatch: loop {
        match pc {
            0x826B78C8 => {
    //   block [0x826B78C8..0x826B7938)
	// 826B78C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B78CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B78D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B78D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B78D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B78DC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B78E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B78E4: 390BA788  addi r8, r11, -0x5878
	ctx.r[8].s64 = ctx.r[11].s64 + -22648;
	// 826B78E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B78EC: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826B78F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B78F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B78F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B78FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7900: 386AF9B4  addi r3, r10, -0x64c
	ctx.r[3].s64 = ctx.r[10].s64 + -1612;
	// 826B7904: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B790C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7914: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B791C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7924: 4BDAF4FD  bl 0x82466e20
	ctx.lr = 0x826B7928;
	sub_82466E20(ctx, base);
	// 826B7928: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B792C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7930: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7938 size=108
    let mut pc: u32 = 0x826B7938;
    'dispatch: loop {
        match pc {
            0x826B7938 => {
    //   block [0x826B7938..0x826B79A4)
	// 826B7938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B793C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7944: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7948: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B794C: 38EBA7A0  addi r7, r11, -0x5860
	ctx.r[7].s64 = ctx.r[11].s64 + -22624;
	// 826B7950: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B7954: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 826B7958: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B795C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7960: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7968: 386AF9E4  addi r3, r10, -0x61c
	ctx.r[3].s64 = ctx.r[10].s64 + -1564;
	// 826B796C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7970: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7978: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B797C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7980: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7988: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B798C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7990: 4BDAF491  bl 0x82466e20
	ctx.lr = 0x826B7994;
	sub_82466E20(ctx, base);
	// 826B7994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B799C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B79A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B79A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B79A8 size=112
    let mut pc: u32 = 0x826B79A8;
    'dispatch: loop {
        match pc {
            0x826B79A8 => {
    //   block [0x826B79A8..0x826B7A18)
	// 826B79A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B79AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B79B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B79B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B79B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B79BC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B79C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B79C4: 390BA7D0  addi r8, r11, -0x5830
	ctx.r[8].s64 = ctx.r[11].s64 + -22576;
	// 826B79C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B79CC: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 826B79D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B79D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B79D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B79DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B79E0: 386AFA14  addi r3, r10, -0x5ec
	ctx.r[3].s64 = ctx.r[10].s64 + -1516;
	// 826B79E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B79E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B79EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B79F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B79F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B79F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B79FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7A00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7A04: 4BDAF41D  bl 0x82466e20
	ctx.lr = 0x826B7A08;
	sub_82466E20(ctx, base);
	// 826B7A08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7A0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7A10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7A18 size=108
    let mut pc: u32 = 0x826B7A18;
    'dispatch: loop {
        match pc {
            0x826B7A18 => {
    //   block [0x826B7A18..0x826B7A84)
	// 826B7A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7A24: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7A28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7A2C: 38EBA7E8  addi r7, r11, -0x5818
	ctx.r[7].s64 = ctx.r[11].s64 + -22552;
	// 826B7A30: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826B7A34: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 826B7A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7A3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7A40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7A44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7A48: 386AFA44  addi r3, r10, -0x5bc
	ctx.r[3].s64 = ctx.r[10].s64 + -1468;
	// 826B7A4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7A50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7A54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7A58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7A5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7A60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7A64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7A68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7A6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7A70: 4BDAF3B1  bl 0x82466e20
	ctx.lr = 0x826B7A74;
	sub_82466E20(ctx, base);
	// 826B7A74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7A88 size=112
    let mut pc: u32 = 0x826B7A88;
    'dispatch: loop {
        match pc {
            0x826B7A88 => {
    //   block [0x826B7A88..0x826B7AF8)
	// 826B7A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7A8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7A94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7A98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7A9C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7AA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7AA4: 390BA8D8  addi r8, r11, -0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + -22312;
	// 826B7AA8: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 826B7AAC: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 826B7AB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7AB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7AB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7ABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7AC0: 386AFA74  addi r3, r10, -0x58c
	ctx.r[3].s64 = ctx.r[10].s64 + -1420;
	// 826B7AC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7AC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7AD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7AD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7AD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7ADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7AE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7AE4: 4BDAF33D  bl 0x82466e20
	ctx.lr = 0x826B7AE8;
	sub_82466E20(ctx, base);
	// 826B7AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7AEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7AF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7AF8 size=108
    let mut pc: u32 = 0x826B7AF8;
    'dispatch: loop {
        match pc {
            0x826B7AF8 => {
    //   block [0x826B7AF8..0x826B7B64)
	// 826B7AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7B04: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7B08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7B0C: 38EBAA88  addi r7, r11, -0x5578
	ctx.r[7].s64 = ctx.r[11].s64 + -21880;
	// 826B7B10: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 826B7B14: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 826B7B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7B1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7B20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7B24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7B28: 386AFAA4  addi r3, r10, -0x55c
	ctx.r[3].s64 = ctx.r[10].s64 + -1372;
	// 826B7B2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7B30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7B34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7B38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7B3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7B40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7B44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7B48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7B4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7B50: 4BDAF2D1  bl 0x82466e20
	ctx.lr = 0x826B7B54;
	sub_82466E20(ctx, base);
	// 826B7B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7B68 size=112
    let mut pc: u32 = 0x826B7B68;
    'dispatch: loop {
        match pc {
            0x826B7B68 => {
    //   block [0x826B7B68..0x826B7BD8)
	// 826B7B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7B70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7B74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7B78: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7B7C: 38AAE6F4  addi r5, r10, -0x190c
	ctx.r[5].s64 = ctx.r[10].s64 + -6412;
	// 826B7B80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7B84: 390BAC20  addi r8, r11, -0x53e0
	ctx.r[8].s64 = ctx.r[11].s64 + -21472;
	// 826B7B88: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 826B7B8C: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 826B7B90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7B94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7B98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7BA0: 386AFAD4  addi r3, r10, -0x52c
	ctx.r[3].s64 = ctx.r[10].s64 + -1324;
	// 826B7BA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7BB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7BB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7BB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7BC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7BC4: 4BDAF25D  bl 0x82466e20
	ctx.lr = 0x826B7BC8;
	sub_82466E20(ctx, base);
	// 826B7BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7BD8 size=100
    let mut pc: u32 = 0x826B7BD8;
    'dispatch: loop {
        match pc {
            0x826B7BD8 => {
    //   block [0x826B7BD8..0x826B7C3C)
	// 826B7BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7BE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7BEC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7BF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7BF8: 388A53C0  addi r4, r10, 0x53c0
	ctx.r[4].s64 = ctx.r[10].s64 + 21440;
	// 826B7BFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7C04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7C0C: 386AFB04  addi r3, r10, -0x4fc
	ctx.r[3].s64 = ctx.r[10].s64 + -1276;
	// 826B7C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7C14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7C18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7C20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7C24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7C28: 4BDAF1F9  bl 0x82466e20
	ctx.lr = 0x826B7C2C;
	sub_82466E20(ctx, base);
	// 826B7C2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7C38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7C40 size=112
    let mut pc: u32 = 0x826B7C40;
    'dispatch: loop {
        match pc {
            0x826B7C40 => {
    //   block [0x826B7C40..0x826B7CB0)
	// 826B7C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7C48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7C4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7C50: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7C54: 38AAFB04  addi r5, r10, -0x4fc
	ctx.r[5].s64 = ctx.r[10].s64 + -1276;
	// 826B7C58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7C5C: 390BAE78  addi r8, r11, -0x5188
	ctx.r[8].s64 = ctx.r[11].s64 + -20872;
	// 826B7C60: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826B7C64: 388A53D8  addi r4, r10, 0x53d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21464;
	// 826B7C68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7C6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7C70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7C74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7C78: 386AFB34  addi r3, r10, -0x4cc
	ctx.r[3].s64 = ctx.r[10].s64 + -1228;
	// 826B7C7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7C80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7C88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7C8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7C90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7C98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7C9C: 4BDAF185  bl 0x82466e20
	ctx.lr = 0x826B7CA0;
	sub_82466E20(ctx, base);
	// 826B7CA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7CB0 size=100
    let mut pc: u32 = 0x826B7CB0;
    'dispatch: loop {
        match pc {
            0x826B7CB0 => {
    //   block [0x826B7CB0..0x826B7D14)
	// 826B7CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7CBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7CC4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7CC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7CCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7CD0: 388A53F8  addi r4, r10, 0x53f8
	ctx.r[4].s64 = ctx.r[10].s64 + 21496;
	// 826B7CD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7CD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7CE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7CE4: 386AFB64  addi r3, r10, -0x49c
	ctx.r[3].s64 = ctx.r[10].s64 + -1180;
	// 826B7CE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7CEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7CF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7CF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7CF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7D00: 4BDAF121  bl 0x82466e20
	ctx.lr = 0x826B7D04;
	sub_82466E20(ctx, base);
	// 826B7D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7D18 size=108
    let mut pc: u32 = 0x826B7D18;
    'dispatch: loop {
        match pc {
            0x826B7D18 => {
    //   block [0x826B7D18..0x826B7D84)
	// 826B7D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7D24: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7D28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7D2C: 38EBAEF0  addi r7, r11, -0x5110
	ctx.r[7].s64 = ctx.r[11].s64 + -20752;
	// 826B7D30: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B7D34: 388A5408  addi r4, r10, 0x5408
	ctx.r[4].s64 = ctx.r[10].s64 + 21512;
	// 826B7D38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7D3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7D40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B7D44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7D48: 386AFB94  addi r3, r10, -0x46c
	ctx.r[3].s64 = ctx.r[10].s64 + -1132;
	// 826B7D4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B7D50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7D54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7D58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7D60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7D64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7D68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7D6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B7D70: 4BDAF0B1  bl 0x82466e20
	ctx.lr = 0x826B7D74;
	sub_82466E20(ctx, base);
	// 826B7D74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7D78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7D7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7D88 size=112
    let mut pc: u32 = 0x826B7D88;
    'dispatch: loop {
        match pc {
            0x826B7D88 => {
    //   block [0x826B7D88..0x826B7DF8)
	// 826B7D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7D94: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7D98: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7D9C: 38AAFB64  addi r5, r10, -0x49c
	ctx.r[5].s64 = ctx.r[10].s64 + -1180;
	// 826B7DA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7DA4: 390BAF38  addi r8, r11, -0x50c8
	ctx.r[8].s64 = ctx.r[11].s64 + -20680;
	// 826B7DA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B7DAC: 388A5434  addi r4, r10, 0x5434
	ctx.r[4].s64 = ctx.r[10].s64 + 21556;
	// 826B7DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7DB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7DB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7DBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7DC0: 386AFBC4  addi r3, r10, -0x43c
	ctx.r[3].s64 = ctx.r[10].s64 + -1084;
	// 826B7DC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7DC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7DE4: 4BDAF03D  bl 0x82466e20
	ctx.lr = 0x826B7DE8;
	sub_82466E20(ctx, base);
	// 826B7DE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7DF8 size=100
    let mut pc: u32 = 0x826B7DF8;
    'dispatch: loop {
        match pc {
            0x826B7DF8 => {
    //   block [0x826B7DF8..0x826B7E5C)
	// 826B7DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7E04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7E0C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7E18: 388A544C  addi r4, r10, 0x544c
	ctx.r[4].s64 = ctx.r[10].s64 + 21580;
	// 826B7E1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7E20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7E28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7E2C: 386AFBF4  addi r3, r10, -0x40c
	ctx.r[3].s64 = ctx.r[10].s64 + -1036;
	// 826B7E30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7E34: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7E38: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7E40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7E48: 4BDAEFD9  bl 0x82466e20
	ctx.lr = 0x826B7E4C;
	sub_82466E20(ctx, base);
	// 826B7E4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7E50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7E54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7E60 size=100
    let mut pc: u32 = 0x826B7E60;
    'dispatch: loop {
        match pc {
            0x826B7E60 => {
    //   block [0x826B7E60..0x826B7EC4)
	// 826B7E60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7E64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7E68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7E6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7E70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7E74: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7E78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7E7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7E80: 388A5468  addi r4, r10, 0x5468
	ctx.r[4].s64 = ctx.r[10].s64 + 21608;
	// 826B7E84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7E88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7E8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7E90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7E94: 386AFC24  addi r3, r10, -0x3dc
	ctx.r[3].s64 = ctx.r[10].s64 + -988;
	// 826B7E98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7E9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7EA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7EA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7EAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7EB0: 4BDAEF71  bl 0x82466e20
	ctx.lr = 0x826B7EB4;
	sub_82466E20(ctx, base);
	// 826B7EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7EC8 size=112
    let mut pc: u32 = 0x826B7EC8;
    'dispatch: loop {
        match pc {
            0x826B7EC8 => {
    //   block [0x826B7EC8..0x826B7F38)
	// 826B7EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7ED4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7ED8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7EDC: 38AAFBF4  addi r5, r10, -0x40c
	ctx.r[5].s64 = ctx.r[10].s64 + -1036;
	// 826B7EE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7EE4: 390BAF68  addi r8, r11, -0x5098
	ctx.r[8].s64 = ctx.r[11].s64 + -20632;
	// 826B7EE8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B7EEC: 388A5480  addi r4, r10, 0x5480
	ctx.r[4].s64 = ctx.r[10].s64 + 21632;
	// 826B7EF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7EF4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7EF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7EFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7F00: 386AFC54  addi r3, r10, -0x3ac
	ctx.r[3].s64 = ctx.r[10].s64 + -940;
	// 826B7F04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7F08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7F0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7F24: 4BDAEEFD  bl 0x82466e20
	ctx.lr = 0x826B7F28;
	sub_82466E20(ctx, base);
	// 826B7F28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7F2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7F30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7F34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7F38 size=112
    let mut pc: u32 = 0x826B7F38;
    'dispatch: loop {
        match pc {
            0x826B7F38 => {
    //   block [0x826B7F38..0x826B7FA8)
	// 826B7F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7F3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7F40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7F44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7F48: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B7F4C: 38AAFC24  addi r5, r10, -0x3dc
	ctx.r[5].s64 = ctx.r[10].s64 + -988;
	// 826B7F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7F54: 390BAFC8  addi r8, r11, -0x5038
	ctx.r[8].s64 = ctx.r[11].s64 + -20536;
	// 826B7F58: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B7F5C: 388A54A4  addi r4, r10, 0x54a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21668;
	// 826B7F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7F68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B7F6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7F70: 386AFC84  addi r3, r10, -0x37c
	ctx.r[3].s64 = ctx.r[10].s64 + -892;
	// 826B7F74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B7F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7F7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7F84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7F94: 4BDAEE8D  bl 0x82466e20
	ctx.lr = 0x826B7F98;
	sub_82466E20(ctx, base);
	// 826B7F98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B7F9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B7FA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B7FA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B7FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B7FA8 size=100
    let mut pc: u32 = 0x826B7FA8;
    'dispatch: loop {
        match pc {
            0x826B7FA8 => {
    //   block [0x826B7FA8..0x826B800C)
	// 826B7FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B7FAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B7FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B7FB4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7FB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B7FBC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B7FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B7FC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B7FC8: 388A54C8  addi r4, r10, 0x54c8
	ctx.r[4].s64 = ctx.r[10].s64 + 21704;
	// 826B7FCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B7FD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B7FD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B7FD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B7FDC: 386AFCB4  addi r3, r10, -0x34c
	ctx.r[3].s64 = ctx.r[10].s64 + -844;
	// 826B7FE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B7FE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B7FE8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B7FEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B7FF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B7FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B7FF8: 4BDAEE29  bl 0x82466e20
	ctx.lr = 0x826B7FFC;
	sub_82466E20(ctx, base);
	// 826B7FFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8010 size=112
    let mut pc: u32 = 0x826B8010;
    'dispatch: loop {
        match pc {
            0x826B8010 => {
    //   block [0x826B8010..0x826B8080)
	// 826B8010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8018: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B801C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8020: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8024: 38AAFCB4  addi r5, r10, -0x34c
	ctx.r[5].s64 = ctx.r[10].s64 + -844;
	// 826B8028: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B802C: 390BB028  addi r8, r11, -0x4fd8
	ctx.r[8].s64 = ctx.r[11].s64 + -20440;
	// 826B8030: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 826B8034: 388A54D8  addi r4, r10, 0x54d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21720;
	// 826B8038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B803C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8040: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8044: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8048: 386AFCE4  addi r3, r10, -0x31c
	ctx.r[3].s64 = ctx.r[10].s64 + -796;
	// 826B804C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8050: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8058: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B805C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8060: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8068: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B806C: 4BDAEDB5  bl 0x82466e20
	ctx.lr = 0x826B8070;
	sub_82466E20(ctx, base);
	// 826B8070: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B807C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8080 size=108
    let mut pc: u32 = 0x826B8080;
    'dispatch: loop {
        match pc {
            0x826B8080 => {
    //   block [0x826B8080..0x826B80EC)
	// 826B8080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8088: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B808C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8094: 38EBB118  addi r7, r11, -0x4ee8
	ctx.r[7].s64 = ctx.r[11].s64 + -20200;
	// 826B8098: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826B809C: 388A54F0  addi r4, r10, 0x54f0
	ctx.r[4].s64 = ctx.r[10].s64 + 21744;
	// 826B80A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B80A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B80A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B80AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B80B0: 386AFD14  addi r3, r10, -0x2ec
	ctx.r[3].s64 = ctx.r[10].s64 + -748;
	// 826B80B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B80B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B80BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B80C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B80C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B80C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B80CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B80D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B80D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B80D8: 4BDAED49  bl 0x82466e20
	ctx.lr = 0x826B80DC;
	sub_82466E20(ctx, base);
	// 826B80DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B80E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B80E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B80E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B80F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B80F0 size=108
    let mut pc: u32 = 0x826B80F0;
    'dispatch: loop {
        match pc {
            0x826B80F0 => {
    //   block [0x826B80F0..0x826B815C)
	// 826B80F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B80F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B80F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B80FC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8104: 38EBB208  addi r7, r11, -0x4df8
	ctx.r[7].s64 = ctx.r[11].s64 + -19960;
	// 826B8108: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B810C: 388A551C  addi r4, r10, 0x551c
	ctx.r[4].s64 = ctx.r[10].s64 + 21788;
	// 826B8110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8114: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8118: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B811C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8120: 386AFD44  addi r3, r10, -0x2bc
	ctx.r[3].s64 = ctx.r[10].s64 + -700;
	// 826B8124: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B812C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8134: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B813C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8144: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8148: 4BDAECD9  bl 0x82466e20
	ctx.lr = 0x826B814C;
	sub_82466E20(ctx, base);
	// 826B814C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8160 size=108
    let mut pc: u32 = 0x826B8160;
    'dispatch: loop {
        match pc {
            0x826B8160 => {
    //   block [0x826B8160..0x826B81CC)
	// 826B8160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B816C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8174: 38EBB250  addi r7, r11, -0x4db0
	ctx.r[7].s64 = ctx.r[11].s64 + -19888;
	// 826B8178: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826B817C: 388A553C  addi r4, r10, 0x553c
	ctx.r[4].s64 = ctx.r[10].s64 + 21820;
	// 826B8180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8184: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8188: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B818C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8190: 386AFD74  addi r3, r10, -0x28c
	ctx.r[3].s64 = ctx.r[10].s64 + -652;
	// 826B8194: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B819C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B81A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B81A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B81A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B81AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B81B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B81B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B81B8: 4BDAEC69  bl 0x82466e20
	ctx.lr = 0x826B81BC;
	sub_82466E20(ctx, base);
	// 826B81BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B81C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B81C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B81C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B81D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B81D0 size=108
    let mut pc: u32 = 0x826B81D0;
    'dispatch: loop {
        match pc {
            0x826B81D0 => {
    //   block [0x826B81D0..0x826B823C)
	// 826B81D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B81D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B81D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B81DC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B81E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B81E4: 38EBB328  addi r7, r11, -0x4cd8
	ctx.r[7].s64 = ctx.r[11].s64 + -19672;
	// 826B81E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B81EC: 388A5560  addi r4, r10, 0x5560
	ctx.r[4].s64 = ctx.r[10].s64 + 21856;
	// 826B81F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B81F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B81F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B81FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8200: 386AFDA4  addi r3, r10, -0x25c
	ctx.r[3].s64 = ctx.r[10].s64 + -604;
	// 826B8204: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B820C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8214: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B821C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8224: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8228: 4BDAEBF9  bl 0x82466e20
	ctx.lr = 0x826B822C;
	sub_82466E20(ctx, base);
	// 826B822C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8240 size=100
    let mut pc: u32 = 0x826B8240;
    'dispatch: loop {
        match pc {
            0x826B8240 => {
    //   block [0x826B8240..0x826B82A4)
	// 826B8240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B824C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8254: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B8258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B825C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8260: 388A5578  addi r4, r10, 0x5578
	ctx.r[4].s64 = ctx.r[10].s64 + 21880;
	// 826B8264: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8268: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B826C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8270: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8274: 386AFDD4  addi r3, r10, -0x22c
	ctx.r[3].s64 = ctx.r[10].s64 + -556;
	// 826B8278: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B827C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8280: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B8284: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8288: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B828C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8290: 4BDAEB91  bl 0x82466e20
	ctx.lr = 0x826B8294;
	sub_82466E20(ctx, base);
	// 826B8294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B829C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B82A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B82A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B82A8 size=112
    let mut pc: u32 = 0x826B82A8;
    'dispatch: loop {
        match pc {
            0x826B82A8 => {
    //   block [0x826B82A8..0x826B8318)
	// 826B82A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B82AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B82B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B82B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B82B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B82BC: 38AAFDD4  addi r5, r10, -0x22c
	ctx.r[5].s64 = ctx.r[10].s64 + -556;
	// 826B82C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B82C4: 390BB340  addi r8, r11, -0x4cc0
	ctx.r[8].s64 = ctx.r[11].s64 + -19648;
	// 826B82C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B82CC: 388A558C  addi r4, r10, 0x558c
	ctx.r[4].s64 = ctx.r[10].s64 + 21900;
	// 826B82D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B82D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B82D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B82DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B82E0: 386AFE04  addi r3, r10, -0x1fc
	ctx.r[3].s64 = ctx.r[10].s64 + -508;
	// 826B82E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B82E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B82EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B82F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B82F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B82F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B82FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8304: 4BDAEB1D  bl 0x82466e20
	ctx.lr = 0x826B8308;
	sub_82466E20(ctx, base);
	// 826B8308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B830C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8318 size=108
    let mut pc: u32 = 0x826B8318;
    'dispatch: loop {
        match pc {
            0x826B8318 => {
    //   block [0x826B8318..0x826B8384)
	// 826B8318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B831C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8324: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8328: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B832C: 38EBB388  addi r7, r11, -0x4c78
	ctx.r[7].s64 = ctx.r[11].s64 + -19576;
	// 826B8330: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B8334: 388A55A8  addi r4, r10, 0x55a8
	ctx.r[4].s64 = ctx.r[10].s64 + 21928;
	// 826B8338: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B833C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8340: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8344: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8348: 386AFE34  addi r3, r10, -0x1cc
	ctx.r[3].s64 = ctx.r[10].s64 + -460;
	// 826B834C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8350: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8354: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8358: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B835C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8360: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8364: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8368: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B836C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8370: 4BDAEAB1  bl 0x82466e20
	ctx.lr = 0x826B8374;
	sub_82466E20(ctx, base);
	// 826B8374: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8378: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B837C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8380: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8388 size=112
    let mut pc: u32 = 0x826B8388;
    'dispatch: loop {
        match pc {
            0x826B8388 => {
    //   block [0x826B8388..0x826B83F8)
	// 826B8388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B838C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8394: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8398: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B839C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B83A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B83A4: 390BB3D0  addi r8, r11, -0x4c30
	ctx.r[8].s64 = ctx.r[11].s64 + -19504;
	// 826B83A8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B83AC: 388A55D8  addi r4, r10, 0x55d8
	ctx.r[4].s64 = ctx.r[10].s64 + 21976;
	// 826B83B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B83B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B83B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B83BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B83C0: 386AFE64  addi r3, r10, -0x19c
	ctx.r[3].s64 = ctx.r[10].s64 + -412;
	// 826B83C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B83C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B83CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B83D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B83D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B83D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B83DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B83E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B83E4: 4BDAEA3D  bl 0x82466e20
	ctx.lr = 0x826B83E8;
	sub_82466E20(ctx, base);
	// 826B83E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B83EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B83F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B83F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B83F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B83F8 size=108
    let mut pc: u32 = 0x826B83F8;
    'dispatch: loop {
        match pc {
            0x826B83F8 => {
    //   block [0x826B83F8..0x826B8464)
	// 826B83F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B83FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8400: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8404: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8408: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B840C: 38EBB3E8  addi r7, r11, -0x4c18
	ctx.r[7].s64 = ctx.r[11].s64 + -19480;
	// 826B8410: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B8414: 388A55EC  addi r4, r10, 0x55ec
	ctx.r[4].s64 = ctx.r[10].s64 + 21996;
	// 826B8418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B841C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8420: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8424: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8428: 386AFE94  addi r3, r10, -0x16c
	ctx.r[3].s64 = ctx.r[10].s64 + -364;
	// 826B842C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8430: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8434: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8438: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B843C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8440: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8444: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8448: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B844C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8450: 4BDAE9D1  bl 0x82466e20
	ctx.lr = 0x826B8454;
	sub_82466E20(ctx, base);
	// 826B8454: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B845C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8468 size=112
    let mut pc: u32 = 0x826B8468;
    'dispatch: loop {
        match pc {
            0x826B8468 => {
    //   block [0x826B8468..0x826B84D8)
	// 826B8468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B846C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8474: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8478: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B847C: 38AAFE64  addi r5, r10, -0x19c
	ctx.r[5].s64 = ctx.r[10].s64 + -412;
	// 826B8480: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8484: 390BB430  addi r8, r11, -0x4bd0
	ctx.r[8].s64 = ctx.r[11].s64 + -19408;
	// 826B8488: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B848C: 388A5628  addi r4, r10, 0x5628
	ctx.r[4].s64 = ctx.r[10].s64 + 22056;
	// 826B8490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8494: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8498: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B849C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B84A0: 386AFEC4  addi r3, r10, -0x13c
	ctx.r[3].s64 = ctx.r[10].s64 + -316;
	// 826B84A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B84A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B84AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B84B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B84B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B84B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B84BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B84C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B84C4: 4BDAE95D  bl 0x82466e20
	ctx.lr = 0x826B84C8;
	sub_82466E20(ctx, base);
	// 826B84C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B84CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B84D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B84D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B84D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B84D8 size=100
    let mut pc: u32 = 0x826B84D8;
    'dispatch: loop {
        match pc {
            0x826B84D8 => {
    //   block [0x826B84D8..0x826B853C)
	// 826B84D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B84DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B84E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B84E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B84E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B84EC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B84F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B84F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B84F8: 388A5644  addi r4, r10, 0x5644
	ctx.r[4].s64 = ctx.r[10].s64 + 22084;
	// 826B84FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8500: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8504: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8508: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B850C: 386AFEF4  addi r3, r10, -0x10c
	ctx.r[3].s64 = ctx.r[10].s64 + -268;
	// 826B8510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8514: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8518: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B851C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8520: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B8524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8528: 4BDAE8F9  bl 0x82466e20
	ctx.lr = 0x826B852C;
	sub_82466E20(ctx, base);
	// 826B852C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8540 size=112
    let mut pc: u32 = 0x826B8540;
    'dispatch: loop {
        match pc {
            0x826B8540 => {
    //   block [0x826B8540..0x826B85B0)
	// 826B8540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8548: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B854C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8550: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8554: 38AAFEF4  addi r5, r10, -0x10c
	ctx.r[5].s64 = ctx.r[10].s64 + -268;
	// 826B8558: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B855C: 390BB448  addi r8, r11, -0x4bb8
	ctx.r[8].s64 = ctx.r[11].s64 + -19384;
	// 826B8560: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B8564: 388A565C  addi r4, r10, 0x565c
	ctx.r[4].s64 = ctx.r[10].s64 + 22108;
	// 826B8568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B856C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8570: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8578: 386AFF24  addi r3, r10, -0xdc
	ctx.r[3].s64 = ctx.r[10].s64 + -220;
	// 826B857C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8580: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8584: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8588: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B858C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8590: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8594: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8598: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B859C: 4BDAE885  bl 0x82466e20
	ctx.lr = 0x826B85A0;
	sub_82466E20(ctx, base);
	// 826B85A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B85A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B85A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B85AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B85B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B85B0 size=108
    let mut pc: u32 = 0x826B85B0;
    'dispatch: loop {
        match pc {
            0x826B85B0 => {
    //   block [0x826B85B0..0x826B861C)
	// 826B85B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B85B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B85B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B85BC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B85C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B85C4: 38EBB4F0  addi r7, r11, -0x4b10
	ctx.r[7].s64 = ctx.r[11].s64 + -19216;
	// 826B85C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B85CC: 388A567C  addi r4, r10, 0x567c
	ctx.r[4].s64 = ctx.r[10].s64 + 22140;
	// 826B85D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B85D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B85D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B85DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B85E0: 386AFF54  addi r3, r10, -0xac
	ctx.r[3].s64 = ctx.r[10].s64 + -172;
	// 826B85E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B85E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B85EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B85F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B85F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B85F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B85FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8604: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8608: 4BDAE819  bl 0x82466e20
	ctx.lr = 0x826B860C;
	sub_82466E20(ctx, base);
	// 826B860C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8620 size=112
    let mut pc: u32 = 0x826B8620;
    'dispatch: loop {
        match pc {
            0x826B8620 => {
    //   block [0x826B8620..0x826B8690)
	// 826B8620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B862C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8630: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8634: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B8638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B863C: 390BB520  addi r8, r11, -0x4ae0
	ctx.r[8].s64 = ctx.r[11].s64 + -19168;
	// 826B8640: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B8644: 388A568C  addi r4, r10, 0x568c
	ctx.r[4].s64 = ctx.r[10].s64 + 22156;
	// 826B8648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B864C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8650: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8658: 386AFF84  addi r3, r10, -0x7c
	ctx.r[3].s64 = ctx.r[10].s64 + -124;
	// 826B865C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8664: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B866C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B867C: 4BDAE7A5  bl 0x82466e20
	ctx.lr = 0x826B8680;
	sub_82466E20(ctx, base);
	// 826B8680: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B868C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8690 size=112
    let mut pc: u32 = 0x826B8690;
    'dispatch: loop {
        match pc {
            0x826B8690 => {
    //   block [0x826B8690..0x826B8700)
	// 826B8690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B869C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B86A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B86A4: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B86A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B86AC: 390BB568  addi r8, r11, -0x4a98
	ctx.r[8].s64 = ctx.r[11].s64 + -19096;
	// 826B86B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B86B4: 388A56A0  addi r4, r10, 0x56a0
	ctx.r[4].s64 = ctx.r[10].s64 + 22176;
	// 826B86B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B86BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B86C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B86C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B86C8: 386AFFB4  addi r3, r10, -0x4c
	ctx.r[3].s64 = ctx.r[10].s64 + -76;
	// 826B86CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B86D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B86D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B86D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B86DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B86E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B86E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B86E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B86EC: 4BDAE735  bl 0x82466e20
	ctx.lr = 0x826B86F0;
	sub_82466E20(ctx, base);
	// 826B86F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B86F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B86F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B86FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8700 size=100
    let mut pc: u32 = 0x826B8700;
    'dispatch: loop {
        match pc {
            0x826B8700 => {
    //   block [0x826B8700..0x826B8764)
	// 826B8700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B870C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8714: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B8718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B871C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8720: 388A56B0  addi r4, r10, 0x56b0
	ctx.r[4].s64 = ctx.r[10].s64 + 22192;
	// 826B8724: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B872C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8734: 386AFFE4  addi r3, r10, -0x1c
	ctx.r[3].s64 = ctx.r[10].s64 + -28;
	// 826B8738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B873C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8740: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B8744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8748: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B874C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8750: 4BDAE6D1  bl 0x82466e20
	ctx.lr = 0x826B8754;
	sub_82466E20(ctx, base);
	// 826B8754: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B875C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8768 size=112
    let mut pc: u32 = 0x826B8768;
    'dispatch: loop {
        match pc {
            0x826B8768 => {
    //   block [0x826B8768..0x826B87D8)
	// 826B8768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B876C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8774: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8778: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B877C: 38AAFFE4  addi r5, r10, -0x1c
	ctx.r[5].s64 = ctx.r[10].s64 + -28;
	// 826B8780: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8784: 390BB5B0  addi r8, r11, -0x4a50
	ctx.r[8].s64 = ctx.r[11].s64 + -19024;
	// 826B8788: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B878C: 388A56C8  addi r4, r10, 0x56c8
	ctx.r[4].s64 = ctx.r[10].s64 + 22216;
	// 826B8790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8794: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8798: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B879C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B87A0: 386A0014  addi r3, r10, 0x14
	ctx.r[3].s64 = ctx.r[10].s64 + 20;
	// 826B87A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B87A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B87AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B87B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B87B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B87B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B87BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B87C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B87C4: 4BDAE65D  bl 0x82466e20
	ctx.lr = 0x826B87C8;
	sub_82466E20(ctx, base);
	// 826B87C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B87CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B87D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B87D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B87D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B87D8 size=112
    let mut pc: u32 = 0x826B87D8;
    'dispatch: loop {
        match pc {
            0x826B87D8 => {
    //   block [0x826B87D8..0x826B8848)
	// 826B87D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B87DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B87E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B87E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B87E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B87EC: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B87F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B87F4: 390BB5F8  addi r8, r11, -0x4a08
	ctx.r[8].s64 = ctx.r[11].s64 + -18952;
	// 826B87F8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B87FC: 388A56E8  addi r4, r10, 0x56e8
	ctx.r[4].s64 = ctx.r[10].s64 + 22248;
	// 826B8800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8808: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B880C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8810: 386A0044  addi r3, r10, 0x44
	ctx.r[3].s64 = ctx.r[10].s64 + 68;
	// 826B8814: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B881C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8824: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B882C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8834: 4BDAE5ED  bl 0x82466e20
	ctx.lr = 0x826B8838;
	sub_82466E20(ctx, base);
	// 826B8838: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B883C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8840: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8844: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8848 size=112
    let mut pc: u32 = 0x826B8848;
    'dispatch: loop {
        match pc {
            0x826B8848 => {
    //   block [0x826B8848..0x826B88B8)
	// 826B8848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B884C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8850: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8854: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8858: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B885C: 38AACCB4  addi r5, r10, -0x334c
	ctx.r[5].s64 = ctx.r[10].s64 + -13132;
	// 826B8860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8864: 390BB610  addi r8, r11, -0x49f0
	ctx.r[8].s64 = ctx.r[11].s64 + -18928;
	// 826B8868: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B886C: 388A5700  addi r4, r10, 0x5700
	ctx.r[4].s64 = ctx.r[10].s64 + 22272;
	// 826B8870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8878: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B887C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8880: 386A0074  addi r3, r10, 0x74
	ctx.r[3].s64 = ctx.r[10].s64 + 116;
	// 826B8884: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B888C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8894: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B8898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B889C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B88A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B88A4: 4BDAE57D  bl 0x82466e20
	ctx.lr = 0x826B88A8;
	sub_82466E20(ctx, base);
	// 826B88A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B88AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B88B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B88B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B88B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B88B8 size=112
    let mut pc: u32 = 0x826B88B8;
    'dispatch: loop {
        match pc {
            0x826B88B8 => {
    //   block [0x826B88B8..0x826B8928)
	// 826B88B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B88BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B88C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B88C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B88C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B88CC: 38AA0044  addi r5, r10, 0x44
	ctx.r[5].s64 = ctx.r[10].s64 + 68;
	// 826B88D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B88D4: 390BB628  addi r8, r11, -0x49d8
	ctx.r[8].s64 = ctx.r[11].s64 + -18904;
	// 826B88D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B88DC: 388A571C  addi r4, r10, 0x571c
	ctx.r[4].s64 = ctx.r[10].s64 + 22300;
	// 826B88E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B88E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B88E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B88EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B88F0: 386A00A4  addi r3, r10, 0xa4
	ctx.r[3].s64 = ctx.r[10].s64 + 164;
	// 826B88F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B88F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B88FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8904: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B890C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8914: 4BDAE50D  bl 0x82466e20
	ctx.lr = 0x826B8918;
	sub_82466E20(ctx, base);
	// 826B8918: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B891C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8920: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8928 size=72
    let mut pc: u32 = 0x826B8928;
    'dispatch: loop {
        match pc {
            0x826B8928 => {
    //   block [0x826B8928..0x826B8970)
	// 826B8928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B892C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8930: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8934: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B8938: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 826B893C: 38CBEC80  addi r6, r11, -0x1380
	ctx.r[6].s64 = ctx.r[11].s64 + -4992;
	// 826B8940: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B8944: 388BFD60  addi r4, r11, -0x2a0
	ctx.r[4].s64 = ctx.r[11].s64 + -672;
	// 826B8948: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B894C: 386B00D4  addi r3, r11, 0xd4
	ctx.r[3].s64 = ctx.r[11].s64 + 212;
	// 826B8950: 4BDC3139  bl 0x8247ba88
	ctx.lr = 0x826B8954;
	sub_8247BA88(ctx, base);
	// 826B8954: 3D608271  lis r11, -0x7d8f
	ctx.r[11].s64 = -2106523648;
	// 826B8958: 386BCEB8  addi r3, r11, -0x3148
	ctx.r[3].s64 = ctx.r[11].s64 + -12616;
	// 826B895C: 4BE7A1DD  bl 0x82532b38
	ctx.lr = 0x826B8960;
	sub_82532B38(ctx, base);
	// 826B8960: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 826B8964: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8968: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B896C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8970 size=108
    let mut pc: u32 = 0x826B8970;
    'dispatch: loop {
        match pc {
            0x826B8970 => {
    //   block [0x826B8970..0x826B89DC)
	// 826B8970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B897C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8980: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8984: 38EBC4B0  addi r7, r11, -0x3b50
	ctx.r[7].s64 = ctx.r[11].s64 + -15184;
	// 826B8988: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B898C: 388A2B24  addi r4, r10, 0x2b24
	ctx.r[4].s64 = ctx.r[10].s64 + 11044;
	// 826B8990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8998: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B899C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B89A0: 386A00EC  addi r3, r10, 0xec
	ctx.r[3].s64 = ctx.r[10].s64 + 236;
	// 826B89A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B89A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B89AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B89B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B89B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B89B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B89BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B89C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B89C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B89C8: 4BDAE459  bl 0x82466e20
	ctx.lr = 0x826B89CC;
	sub_82466E20(ctx, base);
	// 826B89CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B89D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B89D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B89D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B89E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B89E0 size=24
    let mut pc: u32 = 0x826B89E0;
    'dispatch: loop {
        match pc {
            0x826B89E0 => {
    //   block [0x826B89E0..0x826B89F8)
	// 826B89E0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B89E4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B89E8: 394A28A0  addi r10, r10, 0x28a0
	ctx.r[10].s64 = ctx.r[10].s64 + 10400;
	// 826B89EC: 816BC528  lwz r11, -0x3ad8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15064 as u32) ) } as u64;
	// 826B89F0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826B89F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B89F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B89F8 size=112
    let mut pc: u32 = 0x826B89F8;
    'dispatch: loop {
        match pc {
            0x826B89F8 => {
    //   block [0x826B89F8..0x826B8A68)
	// 826B89F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B89FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8A00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8A04: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B8A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8A0C: 392B02DC  addi r9, r11, 0x2dc
	ctx.r[9].s64 = ctx.r[11].s64 + 732;
	// 826B8A10: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 826B8A14: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B8A18: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8A1C: 388A2B3C  addi r4, r10, 0x2b3c
	ctx.r[4].s64 = ctx.r[10].s64 + 11068;
	// 826B8A20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8A24: 396B28A0  addi r11, r11, 0x28a0
	ctx.r[11].s64 = ctx.r[11].s64 + 10400;
	// 826B8A28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B8A2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8A30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B8A34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8A38: 386A011C  addi r3, r10, 0x11c
	ctx.r[3].s64 = ctx.r[10].s64 + 284;
	// 826B8A3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B8A40: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B8A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8A48: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B8A4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8A50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B8A54: 4BDAE3CD  bl 0x82466e20
	ctx.lr = 0x826B8A58;
	sub_82466E20(ctx, base);
	// 826B8A58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8A68 size=108
    let mut pc: u32 = 0x826B8A68;
    'dispatch: loop {
        match pc {
            0x826B8A68 => {
    //   block [0x826B8A68..0x826B8AD4)
	// 826B8A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8A70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8A74: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8A78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8A7C: 38EBC52C  addi r7, r11, -0x3ad4
	ctx.r[7].s64 = ctx.r[11].s64 + -15060;
	// 826B8A80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B8A84: 388A2B50  addi r4, r10, 0x2b50
	ctx.r[4].s64 = ctx.r[10].s64 + 11088;
	// 826B8A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8A8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8A90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8A94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8A98: 386A014C  addi r3, r10, 0x14c
	ctx.r[3].s64 = ctx.r[10].s64 + 332;
	// 826B8A9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8AA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8ABC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8AC0: 4BDAE361  bl 0x82466e20
	ctx.lr = 0x826B8AC4;
	sub_82466E20(ctx, base);
	// 826B8AC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8AD8 size=108
    let mut pc: u32 = 0x826B8AD8;
    'dispatch: loop {
        match pc {
            0x826B8AD8 => {
    //   block [0x826B8AD8..0x826B8B44)
	// 826B8AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8AE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8AE4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8AE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8AEC: 38EBC55C  addi r7, r11, -0x3aa4
	ctx.r[7].s64 = ctx.r[11].s64 + -15012;
	// 826B8AF0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B8AF4: 388A2B6C  addi r4, r10, 0x2b6c
	ctx.r[4].s64 = ctx.r[10].s64 + 11116;
	// 826B8AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8AFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8B00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8B04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8B08: 386A017C  addi r3, r10, 0x17c
	ctx.r[3].s64 = ctx.r[10].s64 + 380;
	// 826B8B0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8B10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8B18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8B1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8B20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8B24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8B2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8B30: 4BDAE2F1  bl 0x82466e20
	ctx.lr = 0x826B8B34;
	sub_82466E20(ctx, base);
	// 826B8B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8B48 size=112
    let mut pc: u32 = 0x826B8B48;
    'dispatch: loop {
        match pc {
            0x826B8B48 => {
    //   block [0x826B8B48..0x826B8BB8)
	// 826B8B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8B50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8B54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8B58: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8B5C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B8B60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8B64: 390BC590  addi r8, r11, -0x3a70
	ctx.r[8].s64 = ctx.r[11].s64 + -14960;
	// 826B8B68: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B8B6C: 388A2BB4  addi r4, r10, 0x2bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 11188;
	// 826B8B70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8B74: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8B78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8B80: 386A01AC  addi r3, r10, 0x1ac
	ctx.r[3].s64 = ctx.r[10].s64 + 428;
	// 826B8B84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8B88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8B8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8B9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8BA4: 4BDAE27D  bl 0x82466e20
	ctx.lr = 0x826B8BA8;
	sub_82466E20(ctx, base);
	// 826B8BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8BAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8BB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8BB8 size=108
    let mut pc: u32 = 0x826B8BB8;
    'dispatch: loop {
        match pc {
            0x826B8BB8 => {
    //   block [0x826B8BB8..0x826B8C24)
	// 826B8BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8BC4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8BC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8BCC: 38EBC5F0  addi r7, r11, -0x3a10
	ctx.r[7].s64 = ctx.r[11].s64 + -14864;
	// 826B8BD0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B8BD4: 388A2BC8  addi r4, r10, 0x2bc8
	ctx.r[4].s64 = ctx.r[10].s64 + 11208;
	// 826B8BD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8BDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8BE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8BE8: 386A01DC  addi r3, r10, 0x1dc
	ctx.r[3].s64 = ctx.r[10].s64 + 476;
	// 826B8BEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8BF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8BF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8BF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8BFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8C00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8C04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8C08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8C10: 4BDAE211  bl 0x82466e20
	ctx.lr = 0x826B8C14;
	sub_82466E20(ctx, base);
	// 826B8C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8C28 size=112
    let mut pc: u32 = 0x826B8C28;
    'dispatch: loop {
        match pc {
            0x826B8C28 => {
    //   block [0x826B8C28..0x826B8C98)
	// 826B8C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8C34: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8C38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8C3C: 38AA01AC  addi r5, r10, 0x1ac
	ctx.r[5].s64 = ctx.r[10].s64 + 428;
	// 826B8C40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8C44: 390BC668  addi r8, r11, -0x3998
	ctx.r[8].s64 = ctx.r[11].s64 + -14744;
	// 826B8C48: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B8C4C: 388A2C00  addi r4, r10, 0x2c00
	ctx.r[4].s64 = ctx.r[10].s64 + 11264;
	// 826B8C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8C54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8C58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8C60: 386A020C  addi r3, r10, 0x20c
	ctx.r[3].s64 = ctx.r[10].s64 + 524;
	// 826B8C64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8C68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8C6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8C70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8C74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8C78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8C7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8C80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8C84: 4BDAE19D  bl 0x82466e20
	ctx.lr = 0x826B8C88;
	sub_82466E20(ctx, base);
	// 826B8C88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8C8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8C90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8C94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8C98 size=112
    let mut pc: u32 = 0x826B8C98;
    'dispatch: loop {
        match pc {
            0x826B8C98 => {
    //   block [0x826B8C98..0x826B8D08)
	// 826B8C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8CA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8CA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8CA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8CAC: 38AA01AC  addi r5, r10, 0x1ac
	ctx.r[5].s64 = ctx.r[10].s64 + 428;
	// 826B8CB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8CB4: 390BC710  addi r8, r11, -0x38f0
	ctx.r[8].s64 = ctx.r[11].s64 + -14576;
	// 826B8CB8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B8CBC: 388A2C24  addi r4, r10, 0x2c24
	ctx.r[4].s64 = ctx.r[10].s64 + 11300;
	// 826B8CC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8CC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8CC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8CD0: 386A023C  addi r3, r10, 0x23c
	ctx.r[3].s64 = ctx.r[10].s64 + 572;
	// 826B8CD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8CD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8CDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8CE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8CE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8CE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8CEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8CF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8CF4: 4BDAE12D  bl 0x82466e20
	ctx.lr = 0x826B8CF8;
	sub_82466E20(ctx, base);
	// 826B8CF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8D08 size=108
    let mut pc: u32 = 0x826B8D08;
    'dispatch: loop {
        match pc {
            0x826B8D08 => {
    //   block [0x826B8D08..0x826B8D74)
	// 826B8D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8D14: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8D18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8D1C: 38EBC728  addi r7, r11, -0x38d8
	ctx.r[7].s64 = ctx.r[11].s64 + -14552;
	// 826B8D20: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B8D24: 388A2C44  addi r4, r10, 0x2c44
	ctx.r[4].s64 = ctx.r[10].s64 + 11332;
	// 826B8D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8D2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8D30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8D34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8D38: 386A026C  addi r3, r10, 0x26c
	ctx.r[3].s64 = ctx.r[10].s64 + 620;
	// 826B8D3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8D40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8D44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8D48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8D50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8D58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8D5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8D60: 4BDAE0C1  bl 0x82466e20
	ctx.lr = 0x826B8D64;
	sub_82466E20(ctx, base);
	// 826B8D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8D68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8D6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8D78 size=112
    let mut pc: u32 = 0x826B8D78;
    'dispatch: loop {
        match pc {
            0x826B8D78 => {
    //   block [0x826B8D78..0x826B8DE8)
	// 826B8D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8D80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8D84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8D88: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8D8C: 38AA01AC  addi r5, r10, 0x1ac
	ctx.r[5].s64 = ctx.r[10].s64 + 428;
	// 826B8D90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8D94: 390BC7A0  addi r8, r11, -0x3860
	ctx.r[8].s64 = ctx.r[11].s64 + -14432;
	// 826B8D98: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826B8D9C: 388A2C74  addi r4, r10, 0x2c74
	ctx.r[4].s64 = ctx.r[10].s64 + 11380;
	// 826B8DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8DA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8DA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8DAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8DB0: 386A029C  addi r3, r10, 0x29c
	ctx.r[3].s64 = ctx.r[10].s64 + 668;
	// 826B8DB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B8DB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8DBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8DC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8DC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8DCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8DD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8DD4: 4BDAE04D  bl 0x82466e20
	ctx.lr = 0x826B8DD8;
	sub_82466E20(ctx, base);
	// 826B8DD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8DE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8DE8 size=108
    let mut pc: u32 = 0x826B8DE8;
    'dispatch: loop {
        match pc {
            0x826B8DE8 => {
    //   block [0x826B8DE8..0x826B8E54)
	// 826B8DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8DF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8DF4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8DF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8DFC: 38EBC848  addi r7, r11, -0x37b8
	ctx.r[7].s64 = ctx.r[11].s64 + -14264;
	// 826B8E00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826B8E04: 388A2C90  addi r4, r10, 0x2c90
	ctx.r[4].s64 = ctx.r[10].s64 + 11408;
	// 826B8E08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8E0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8E10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8E14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8E18: 386A02CC  addi r3, r10, 0x2cc
	ctx.r[3].s64 = ctx.r[10].s64 + 716;
	// 826B8E1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8E20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8E24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8E28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8E2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8E30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8E34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8E38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8E3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8E40: 4BDADFE1  bl 0x82466e20
	ctx.lr = 0x826B8E44;
	sub_82466E20(ctx, base);
	// 826B8E44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8E48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8E4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8E50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8E58 size=108
    let mut pc: u32 = 0x826B8E58;
    'dispatch: loop {
        match pc {
            0x826B8E58 => {
    //   block [0x826B8E58..0x826B8EC4)
	// 826B8E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8E5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8E60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8E64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8E68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8E6C: 38EBC860  addi r7, r11, -0x37a0
	ctx.r[7].s64 = ctx.r[11].s64 + -14240;
	// 826B8E70: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B8E74: 388A2CA8  addi r4, r10, 0x2ca8
	ctx.r[4].s64 = ctx.r[10].s64 + 11432;
	// 826B8E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8E7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8E80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8E84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8E88: 386A02FC  addi r3, r10, 0x2fc
	ctx.r[3].s64 = ctx.r[10].s64 + 764;
	// 826B8E8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8E90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8E94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8E98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8E9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8EA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8EA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8EA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8EAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8EB0: 4BDADF71  bl 0x82466e20
	ctx.lr = 0x826B8EB4;
	sub_82466E20(ctx, base);
	// 826B8EB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8EC8 size=116
    let mut pc: u32 = 0x826B8EC8;
    'dispatch: loop {
        match pc {
            0x826B8EC8 => {
    //   block [0x826B8EC8..0x826B8F3C)
	// 826B8EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8ED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8ED4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8ED8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B8EDC: 390BC8C0  addi r8, r11, -0x3740
	ctx.r[8].s64 = ctx.r[11].s64 + -14144;
	// 826B8EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8EE4: 392A0318  addi r9, r10, 0x318
	ctx.r[9].s64 = ctx.r[10].s64 + 792;
	// 826B8EE8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8EEC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826B8EF0: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B8EF4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B8EF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8EFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8F00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8F04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8F08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8F0C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826B8F10: 388A2CB8  addi r4, r10, 0x2cb8
	ctx.r[4].s64 = ctx.r[10].s64 + 11448;
	// 826B8F14: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B8F18: 386B032C  addi r3, r11, 0x32c
	ctx.r[3].s64 = ctx.r[11].s64 + 812;
	// 826B8F1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B8F20: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8F28: 4BDADEF9  bl 0x82466e20
	ctx.lr = 0x826B8F2C;
	sub_82466E20(ctx, base);
	// 826B8F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8F38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8F40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8F40 size=108
    let mut pc: u32 = 0x826B8F40;
    'dispatch: loop {
        match pc {
            0x826B8F40 => {
    //   block [0x826B8F40..0x826B8FAC)
	// 826B8F40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8F44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8F48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8F4C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8F50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8F54: 38EBC8D8  addi r7, r11, -0x3728
	ctx.r[7].s64 = ctx.r[11].s64 + -14120;
	// 826B8F58: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B8F5C: 388A2CCC  addi r4, r10, 0x2ccc
	ctx.r[4].s64 = ctx.r[10].s64 + 11468;
	// 826B8F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8F64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8F68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8F6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8F70: 386A035C  addi r3, r10, 0x35c
	ctx.r[3].s64 = ctx.r[10].s64 + 860;
	// 826B8F74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8F78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8F7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8F80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8F88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B8F90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B8F94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B8F98: 4BDADE89  bl 0x82466e20
	ctx.lr = 0x826B8F9C;
	sub_82466E20(ctx, base);
	// 826B8F9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B8FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B8FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B8FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B8FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B8FB0 size=108
    let mut pc: u32 = 0x826B8FB0;
    'dispatch: loop {
        match pc {
            0x826B8FB0 => {
    //   block [0x826B8FB0..0x826B901C)
	// 826B8FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B8FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B8FB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B8FBC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B8FC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B8FC4: 38EBC920  addi r7, r11, -0x36e0
	ctx.r[7].s64 = ctx.r[11].s64 + -14048;
	// 826B8FC8: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B8FCC: 388A2CF0  addi r4, r10, 0x2cf0
	ctx.r[4].s64 = ctx.r[10].s64 + 11504;
	// 826B8FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B8FD4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B8FD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B8FDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B8FE0: 386A038C  addi r3, r10, 0x38c
	ctx.r[3].s64 = ctx.r[10].s64 + 908;
	// 826B8FE4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B8FE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B8FEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B8FF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B8FF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B8FF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B8FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9000: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9004: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9008: 4BDADE19  bl 0x82466e20
	ctx.lr = 0x826B900C;
	sub_82466E20(ctx, base);
	// 826B900C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9020 size=108
    let mut pc: u32 = 0x826B9020;
    'dispatch: loop {
        match pc {
            0x826B9020 => {
    //   block [0x826B9020..0x826B908C)
	// 826B9020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9028: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B902C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9030: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9034: 38EBC9B0  addi r7, r11, -0x3650
	ctx.r[7].s64 = ctx.r[11].s64 + -13904;
	// 826B9038: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B903C: 388A2D14  addi r4, r10, 0x2d14
	ctx.r[4].s64 = ctx.r[10].s64 + 11540;
	// 826B9040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9044: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9048: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B904C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9050: 386A03BC  addi r3, r10, 0x3bc
	ctx.r[3].s64 = ctx.r[10].s64 + 956;
	// 826B9054: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9058: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B905C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9060: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9068: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B906C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9070: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9074: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9078: 4BDADDA9  bl 0x82466e20
	ctx.lr = 0x826B907C;
	sub_82466E20(ctx, base);
	// 826B907C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9090 size=100
    let mut pc: u32 = 0x826B9090;
    'dispatch: loop {
        match pc {
            0x826B9090 => {
    //   block [0x826B9090..0x826B90F4)
	// 826B9090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B909C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B90A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B90A4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B90A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B90AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B90B0: 388A2D2C  addi r4, r10, 0x2d2c
	ctx.r[4].s64 = ctx.r[10].s64 + 11564;
	// 826B90B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B90B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B90BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B90C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B90C4: 386A03EC  addi r3, r10, 0x3ec
	ctx.r[3].s64 = ctx.r[10].s64 + 1004;
	// 826B90C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B90CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B90D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B90D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B90D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B90DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B90E0: 4BDADD41  bl 0x82466e20
	ctx.lr = 0x826B90E4;
	sub_82466E20(ctx, base);
	// 826B90E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B90E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B90EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B90F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B90F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B90F8 size=112
    let mut pc: u32 = 0x826B90F8;
    'dispatch: loop {
        match pc {
            0x826B90F8 => {
    //   block [0x826B90F8..0x826B9168)
	// 826B90F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B90FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9104: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9108: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B910C: 38AA03EC  addi r5, r10, 0x3ec
	ctx.r[5].s64 = ctx.r[10].s64 + 1004;
	// 826B9110: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9114: 390BCA40  addi r8, r11, -0x35c0
	ctx.r[8].s64 = ctx.r[11].s64 + -13760;
	// 826B9118: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826B911C: 388A2D48  addi r4, r10, 0x2d48
	ctx.r[4].s64 = ctx.r[10].s64 + 11592;
	// 826B9120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9124: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9128: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B912C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9130: 386A041C  addi r3, r10, 0x41c
	ctx.r[3].s64 = ctx.r[10].s64 + 1052;
	// 826B9134: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9138: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B913C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9144: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B914C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9154: 4BDADCCD  bl 0x82466e20
	ctx.lr = 0x826B9158;
	sub_82466E20(ctx, base);
	// 826B9158: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B915C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9160: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9168 size=108
    let mut pc: u32 = 0x826B9168;
    'dispatch: loop {
        match pc {
            0x826B9168 => {
    //   block [0x826B9168..0x826B91D4)
	// 826B9168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B916C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9170: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9174: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9178: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B917C: 38EBCAA0  addi r7, r11, -0x3560
	ctx.r[7].s64 = ctx.r[11].s64 + -13664;
	// 826B9180: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B9184: 388A2D68  addi r4, r10, 0x2d68
	ctx.r[4].s64 = ctx.r[10].s64 + 11624;
	// 826B9188: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B918C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9190: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B9194: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9198: 386A044C  addi r3, r10, 0x44c
	ctx.r[3].s64 = ctx.r[10].s64 + 1100;
	// 826B919C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B91A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B91A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B91A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B91AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B91B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B91B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B91B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B91BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B91C0: 4BDADC61  bl 0x82466e20
	ctx.lr = 0x826B91C4;
	sub_82466E20(ctx, base);
	// 826B91C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B91C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B91CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B91D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B91D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B91D8 size=108
    let mut pc: u32 = 0x826B91D8;
    'dispatch: loop {
        match pc {
            0x826B91D8 => {
    //   block [0x826B91D8..0x826B9244)
	// 826B91D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B91DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B91E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B91E4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B91E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B91EC: 38EBCAD0  addi r7, r11, -0x3530
	ctx.r[7].s64 = ctx.r[11].s64 + -13616;
	// 826B91F0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B91F4: 388A2D70  addi r4, r10, 0x2d70
	ctx.r[4].s64 = ctx.r[10].s64 + 11632;
	// 826B91F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B91FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B9204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9208: 386A047C  addi r3, r10, 0x47c
	ctx.r[3].s64 = ctx.r[10].s64 + 1148;
	// 826B920C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B921C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B922C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9230: 4BDADBF1  bl 0x82466e20
	ctx.lr = 0x826B9234;
	sub_82466E20(ctx, base);
	// 826B9234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B923C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9248 size=108
    let mut pc: u32 = 0x826B9248;
    'dispatch: loop {
        match pc {
            0x826B9248 => {
    //   block [0x826B9248..0x826B92B4)
	// 826B9248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B924C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9254: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B925C: 38EBCB30  addi r7, r11, -0x34d0
	ctx.r[7].s64 = ctx.r[11].s64 + -13520;
	// 826B9260: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B9264: 388A2D84  addi r4, r10, 0x2d84
	ctx.r[4].s64 = ctx.r[10].s64 + 11652;
	// 826B9268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B926C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B9274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9278: 386A04AC  addi r3, r10, 0x4ac
	ctx.r[3].s64 = ctx.r[10].s64 + 1196;
	// 826B927C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B928C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B929C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B92A0: 4BDADB81  bl 0x82466e20
	ctx.lr = 0x826B92A4;
	sub_82466E20(ctx, base);
	// 826B92A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B92A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B92AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B92B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B92B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B92B8 size=108
    let mut pc: u32 = 0x826B92B8;
    'dispatch: loop {
        match pc {
            0x826B92B8 => {
    //   block [0x826B92B8..0x826B9324)
	// 826B92B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B92BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B92C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B92C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B92C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B92CC: 38EBCB90  addi r7, r11, -0x3470
	ctx.r[7].s64 = ctx.r[11].s64 + -13424;
	// 826B92D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826B92D4: 388A2DCC  addi r4, r10, 0x2dcc
	ctx.r[4].s64 = ctx.r[10].s64 + 11724;
	// 826B92D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B92DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B92E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B92E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B92E8: 386A04DC  addi r3, r10, 0x4dc
	ctx.r[3].s64 = ctx.r[10].s64 + 1244;
	// 826B92EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B92F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B92F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B92F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B92FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9304: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B930C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9310: 4BDADB11  bl 0x82466e20
	ctx.lr = 0x826B9314;
	sub_82466E20(ctx, base);
	// 826B9314: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B931C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9328 size=112
    let mut pc: u32 = 0x826B9328;
    'dispatch: loop {
        match pc {
            0x826B9328 => {
    //   block [0x826B9328..0x826B9398)
	// 826B9328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B932C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9334: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B9338: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B933C: 392A034C  addi r9, r10, 0x34c
	ctx.r[9].s64 = ctx.r[10].s64 + 844;
	// 826B9340: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9344: 390BCC10  addi r8, r11, -0x33f0
	ctx.r[8].s64 = ctx.r[11].s64 + -13296;
	// 826B9348: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 826B934C: 388A2E00  addi r4, r10, 0x2e00
	ctx.r[4].s64 = ctx.r[10].s64 + 11776;
	// 826B9350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9354: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9358: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B935C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9360: 386A050C  addi r3, r10, 0x50c
	ctx.r[3].s64 = ctx.r[10].s64 + 1292;
	// 826B9364: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9368: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B936C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9370: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9374: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9378: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B937C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9380: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9384: 4BDADA9D  bl 0x82466e20
	ctx.lr = 0x826B9388;
	sub_82466E20(ctx, base);
	// 826B9388: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B938C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9398 size=112
    let mut pc: u32 = 0x826B9398;
    'dispatch: loop {
        match pc {
            0x826B9398 => {
    //   block [0x826B9398..0x826B9408)
	// 826B9398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B939C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B93A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B93A4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B93A8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826B93AC: 38EACD18  addi r7, r10, -0x32e8
	ctx.r[7].s64 = ctx.r[10].s64 + -13032;
	// 826B93B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B93B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B93B8: 388A2E44  addi r4, r10, 0x2e44
	ctx.r[4].s64 = ctx.r[10].s64 + 11844;
	// 826B93BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B93C0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B93C4: 396B0360  addi r11, r11, 0x360
	ctx.r[11].s64 = ctx.r[11].s64 + 864;
	// 826B93C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B93CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B93D0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B93D4: 386A053C  addi r3, r10, 0x53c
	ctx.r[3].s64 = ctx.r[10].s64 + 1340;
	// 826B93D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B93DC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B93E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B93E4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B93E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B93EC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B93F0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B93F4: 4BDADA2D  bl 0x82466e20
	ctx.lr = 0x826B93F8;
	sub_82466E20(ctx, base);
	// 826B93F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B93FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9408 size=112
    let mut pc: u32 = 0x826B9408;
    'dispatch: loop {
        match pc {
            0x826B9408 => {
    //   block [0x826B9408..0x826B9478)
	// 826B9408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B940C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9414: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826B9418: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B941C: 392A03A4  addi r9, r10, 0x3a4
	ctx.r[9].s64 = ctx.r[10].s64 + 932;
	// 826B9420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9424: 390BCE20  addi r8, r11, -0x31e0
	ctx.r[8].s64 = ctx.r[11].s64 + -12768;
	// 826B9428: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826B942C: 388A2E64  addi r4, r10, 0x2e64
	ctx.r[4].s64 = ctx.r[10].s64 + 11876;
	// 826B9430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9434: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B943C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9440: 386A056C  addi r3, r10, 0x56c
	ctx.r[3].s64 = ctx.r[10].s64 + 1388;
	// 826B9444: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9448: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B944C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B945C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9464: 4BDAD9BD  bl 0x82466e20
	ctx.lr = 0x826B9468;
	sub_82466E20(ctx, base);
	// 826B9468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B946C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9478 size=100
    let mut pc: u32 = 0x826B9478;
    'dispatch: loop {
        match pc {
            0x826B9478 => {
    //   block [0x826B9478..0x826B94DC)
	// 826B9478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B947C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9484: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9488: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B948C: 38AA0B3C  addi r5, r10, 0xb3c
	ctx.r[5].s64 = ctx.r[10].s64 + 2876;
	// 826B9490: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9494: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9498: 388A2E70  addi r4, r10, 0x2e70
	ctx.r[4].s64 = ctx.r[10].s64 + 11888;
	// 826B949C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B94A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B94A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B94A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B94AC: 386A059C  addi r3, r10, 0x59c
	ctx.r[3].s64 = ctx.r[10].s64 + 1436;
	// 826B94B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B94B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B94B8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B94BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B94C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B94C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B94C8: 4BDAD959  bl 0x82466e20
	ctx.lr = 0x826B94CC;
	sub_82466E20(ctx, base);
	// 826B94CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B94D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B94D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B94D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B94E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B94E0 size=116
    let mut pc: u32 = 0x826B94E0;
    'dispatch: loop {
        match pc {
            0x826B94E0 => {
    //   block [0x826B94E0..0x826B9554)
	// 826B94E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B94E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B94E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B94EC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B94F0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826B94F4: 390ACE50  addi r8, r10, -0x31b0
	ctx.r[8].s64 = ctx.r[10].s64 + -12720;
	// 826B94F8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B94FC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9500: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B9504: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9508: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B950C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9514: 388A2EDC  addi r4, r10, 0x2edc
	ctx.r[4].s64 = ctx.r[10].s64 + 11996;
	// 826B9518: 396B03B8  addi r11, r11, 0x3b8
	ctx.r[11].s64 = ctx.r[11].s64 + 952;
	// 826B951C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9520: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9524: 386A05CC  addi r3, r10, 0x5cc
	ctx.r[3].s64 = ctx.r[10].s64 + 1484;
	// 826B9528: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B952C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9530: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B953C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9540: 4BDAD8E1  bl 0x82466e20
	ctx.lr = 0x826B9544;
	sub_82466E20(ctx, base);
	// 826B9544: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B954C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9558 size=100
    let mut pc: u32 = 0x826B9558;
    'dispatch: loop {
        match pc {
            0x826B9558 => {
    //   block [0x826B9558..0x826B95BC)
	// 826B9558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B955C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9564: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B956C: 38AA05CC  addi r5, r10, 0x5cc
	ctx.r[5].s64 = ctx.r[10].s64 + 1484;
	// 826B9570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9578: 388A2EF8  addi r4, r10, 0x2ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 12024;
	// 826B957C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B958C: 386A05FC  addi r3, r10, 0x5fc
	ctx.r[3].s64 = ctx.r[10].s64 + 1532;
	// 826B9590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9598: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B959C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B95A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B95A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B95A8: 4BDAD879  bl 0x82466e20
	ctx.lr = 0x826B95AC;
	sub_82466E20(ctx, base);
	// 826B95AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B95B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B95B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B95B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B95C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B95C0 size=112
    let mut pc: u32 = 0x826B95C0;
    'dispatch: loop {
        match pc {
            0x826B95C0 => {
    //   block [0x826B95C0..0x826B9630)
	// 826B95C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B95C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B95C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B95CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B95D0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B95D4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B95D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B95DC: 390BCEF8  addi r8, r11, -0x3108
	ctx.r[8].s64 = ctx.r[11].s64 + -12552;
	// 826B95E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826B95E4: 388A2F1C  addi r4, r10, 0x2f1c
	ctx.r[4].s64 = ctx.r[10].s64 + 12060;
	// 826B95E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B95EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B95F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B95F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B95F8: 386A062C  addi r3, r10, 0x62c
	ctx.r[3].s64 = ctx.r[10].s64 + 1580;
	// 826B95FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B960C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B961C: 4BDAD805  bl 0x82466e20
	ctx.lr = 0x826B9620;
	sub_82466E20(ctx, base);
	// 826B9620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B962C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9630 size=116
    let mut pc: u32 = 0x826B9630;
    'dispatch: loop {
        match pc {
            0x826B9630 => {
    //   block [0x826B9630..0x826B96A4)
	// 826B9630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B963C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9640: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826B9644: 390ACF40  addi r8, r10, -0x30c0
	ctx.r[8].s64 = ctx.r[10].s64 + -12480;
	// 826B9648: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B964C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9650: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B9654: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9658: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B965C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9664: 388A2F38  addi r4, r10, 0x2f38
	ctx.r[4].s64 = ctx.r[10].s64 + 12088;
	// 826B9668: 396B03E4  addi r11, r11, 0x3e4
	ctx.r[11].s64 = ctx.r[11].s64 + 996;
	// 826B966C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9670: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9674: 386A065C  addi r3, r10, 0x65c
	ctx.r[3].s64 = ctx.r[10].s64 + 1628;
	// 826B9678: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B967C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9680: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9684: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B968C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9690: 4BDAD791  bl 0x82466e20
	ctx.lr = 0x826B9694;
	sub_82466E20(ctx, base);
	// 826B9694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B969C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B96A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B96A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B96A8 size=108
    let mut pc: u32 = 0x826B96A8;
    'dispatch: loop {
        match pc {
            0x826B96A8 => {
    //   block [0x826B96A8..0x826B9714)
	// 826B96A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B96AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B96B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B96B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B96B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B96BC: 38EBD000  addi r7, r11, -0x3000
	ctx.r[7].s64 = ctx.r[11].s64 + -12288;
	// 826B96C0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826B96C4: 388A2F4C  addi r4, r10, 0x2f4c
	ctx.r[4].s64 = ctx.r[10].s64 + 12108;
	// 826B96C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B96CC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B96D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B96D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B96D8: 386A068C  addi r3, r10, 0x68c
	ctx.r[3].s64 = ctx.r[10].s64 + 1676;
	// 826B96DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B96E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B96E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B96E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B96EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B96F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B96F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B96F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B96FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9700: 4BDAD721  bl 0x82466e20
	ctx.lr = 0x826B9704;
	sub_82466E20(ctx, base);
	// 826B9704: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B970C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B9718 size=24
    let mut pc: u32 = 0x826B9718;
    'dispatch: loop {
        match pc {
            0x826B9718 => {
    //   block [0x826B9718..0x826B9730)
	// 826B9718: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B971C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9720: 394A28E8  addi r10, r10, 0x28e8
	ctx.r[10].s64 = ctx.r[10].s64 + 10472;
	// 826B9724: 816BD048  lwz r11, -0x2fb8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12216 as u32) ) } as u64;
	// 826B9728: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826B972C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9730 size=116
    let mut pc: u32 = 0x826B9730;
    'dispatch: loop {
        match pc {
            0x826B9730 => {
    //   block [0x826B9730..0x826B97A4)
	// 826B9730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B973C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9740: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9744: 392B042C  addi r9, r11, 0x42c
	ctx.r[9].s64 = ctx.r[11].s64 + 1068;
	// 826B9748: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B974C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9750: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B9754: 38C0000F  li r6, 0xf
	ctx.r[6].s64 = 15;
	// 826B9758: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B975C: 388A3040  addi r4, r10, 0x3040
	ctx.r[4].s64 = ctx.r[10].s64 + 12352;
	// 826B9760: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9764: 396B28E8  addi r11, r11, 0x28e8
	ctx.r[11].s64 = ctx.r[11].s64 + 10472;
	// 826B9768: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B976C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9770: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B9774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9778: 386A06BC  addi r3, r10, 0x6bc
	ctx.r[3].s64 = ctx.r[10].s64 + 1724;
	// 826B977C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B9780: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B9784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9788: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B978C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B9790: 4BDAD691  bl 0x82466e20
	ctx.lr = 0x826B9794;
	sub_82466E20(ctx, base);
	// 826B9794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B979C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B97A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B97A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B97A8 size=112
    let mut pc: u32 = 0x826B97A8;
    'dispatch: loop {
        match pc {
            0x826B97A8 => {
    //   block [0x826B97A8..0x826B9818)
	// 826B97A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B97AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B97B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B97B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B97B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B97BC: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B97C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B97C4: 390BD04C  addi r8, r11, -0x2fb4
	ctx.r[8].s64 = ctx.r[11].s64 + -12212;
	// 826B97C8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B97CC: 388A3054  addi r4, r10, 0x3054
	ctx.r[4].s64 = ctx.r[10].s64 + 12372;
	// 826B97D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B97D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B97D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B97DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B97E0: 386A06EC  addi r3, r10, 0x6ec
	ctx.r[3].s64 = ctx.r[10].s64 + 1772;
	// 826B97E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B97E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B97EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B97F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B97F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B97F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B97FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9804: 4BDAD61D  bl 0x82466e20
	ctx.lr = 0x826B9808;
	sub_82466E20(ctx, base);
	// 826B9808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B980C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9818 size=112
    let mut pc: u32 = 0x826B9818;
    'dispatch: loop {
        match pc {
            0x826B9818 => {
    //   block [0x826B9818..0x826B9888)
	// 826B9818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B981C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9824: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9828: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B982C: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826B9830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9834: 390BD07C  addi r8, r11, -0x2f84
	ctx.r[8].s64 = ctx.r[11].s64 + -12164;
	// 826B9838: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B983C: 388A30D8  addi r4, r10, 0x30d8
	ctx.r[4].s64 = ctx.r[10].s64 + 12504;
	// 826B9840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B984C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9850: 386A071C  addi r3, r10, 0x71c
	ctx.r[3].s64 = ctx.r[10].s64 + 1820;
	// 826B9854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B985C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B986C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9874: 4BDAD5AD  bl 0x82466e20
	ctx.lr = 0x826B9878;
	sub_82466E20(ctx, base);
	// 826B9878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B987C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9888 size=100
    let mut pc: u32 = 0x826B9888;
    'dispatch: loop {
        match pc {
            0x826B9888 => {
    //   block [0x826B9888..0x826B98EC)
	// 826B9888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B988C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9894: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B989C: 38AA0B3C  addi r5, r10, 0xb3c
	ctx.r[5].s64 = ctx.r[10].s64 + 2876;
	// 826B98A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B98A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B98A8: 388A30F4  addi r4, r10, 0x30f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12532;
	// 826B98AC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B98B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B98B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B98B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B98BC: 386A074C  addi r3, r10, 0x74c
	ctx.r[3].s64 = ctx.r[10].s64 + 1868;
	// 826B98C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B98C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B98C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826B98CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B98D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B98D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B98D8: 4BDAD549  bl 0x82466e20
	ctx.lr = 0x826B98DC;
	sub_82466E20(ctx, base);
	// 826B98DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B98E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B98E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B98E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B98F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B98F0 size=112
    let mut pc: u32 = 0x826B98F0;
    'dispatch: loop {
        match pc {
            0x826B98F0 => {
    //   block [0x826B98F0..0x826B9960)
	// 826B98F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B98F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B98F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B98FC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9900: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9904: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B990C: 390BD094  addi r8, r11, -0x2f6c
	ctx.r[8].s64 = ctx.r[11].s64 + -12140;
	// 826B9910: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B9914: 388A3164  addi r4, r10, 0x3164
	ctx.r[4].s64 = ctx.r[10].s64 + 12644;
	// 826B9918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B991C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9928: 386A077C  addi r3, r10, 0x77c
	ctx.r[3].s64 = ctx.r[10].s64 + 1916;
	// 826B992C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B993C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B994C: 4BDAD4D5  bl 0x82466e20
	ctx.lr = 0x826B9950;
	sub_82466E20(ctx, base);
	// 826B9950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B995C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9960 size=108
    let mut pc: u32 = 0x826B9960;
    'dispatch: loop {
        match pc {
            0x826B9960 => {
    //   block [0x826B9960..0x826B99CC)
	// 826B9960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B996C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9970: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9974: 38EBD0C8  addi r7, r11, -0x2f38
	ctx.r[7].s64 = ctx.r[11].s64 + -12088;
	// 826B9978: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826B997C: 388A317C  addi r4, r10, 0x317c
	ctx.r[4].s64 = ctx.r[10].s64 + 12668;
	// 826B9980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9984: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9988: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B998C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9990: 386A07AC  addi r3, r10, 0x7ac
	ctx.r[3].s64 = ctx.r[10].s64 + 1964;
	// 826B9994: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9998: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B999C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B99A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B99A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B99A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B99AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B99B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B99B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B99B8: 4BDAD469  bl 0x82466e20
	ctx.lr = 0x826B99BC;
	sub_82466E20(ctx, base);
	// 826B99BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B99C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B99C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B99C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B99D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B99D0 size=108
    let mut pc: u32 = 0x826B99D0;
    'dispatch: loop {
        match pc {
            0x826B99D0 => {
    //   block [0x826B99D0..0x826B9A3C)
	// 826B99D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B99D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B99D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B99DC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B99E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B99E4: 38EBD128  addi r7, r11, -0x2ed8
	ctx.r[7].s64 = ctx.r[11].s64 + -11992;
	// 826B99E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826B99EC: 388A318C  addi r4, r10, 0x318c
	ctx.r[4].s64 = ctx.r[10].s64 + 12684;
	// 826B99F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B99F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B99F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B99FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9A00: 386A07DC  addi r3, r10, 0x7dc
	ctx.r[3].s64 = ctx.r[10].s64 + 2012;
	// 826B9A04: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9A08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9A0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9A10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9A14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9A18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9A1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9A20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9A24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9A28: 4BDAD3F9  bl 0x82466e20
	ctx.lr = 0x826B9A2C;
	sub_82466E20(ctx, base);
	// 826B9A2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9A30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9A34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9A40 size=116
    let mut pc: u32 = 0x826B9A40;
    'dispatch: loop {
        match pc {
            0x826B9A40 => {
    //   block [0x826B9A40..0x826B9AB4)
	// 826B9A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9A4C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9A50: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826B9A54: 390AD158  addi r8, r10, -0x2ea8
	ctx.r[8].s64 = ctx.r[10].s64 + -11944;
	// 826B9A58: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9A5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9A60: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9A64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9A68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9A74: 388A31E0  addi r4, r10, 0x31e0
	ctx.r[4].s64 = ctx.r[10].s64 + 12768;
	// 826B9A78: 396B0480  addi r11, r11, 0x480
	ctx.r[11].s64 = ctx.r[11].s64 + 1152;
	// 826B9A7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9A80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9A84: 386A080C  addi r3, r10, 0x80c
	ctx.r[3].s64 = ctx.r[10].s64 + 2060;
	// 826B9A88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B9A8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9A90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9AA0: 4BDAD381  bl 0x82466e20
	ctx.lr = 0x826B9AA4;
	sub_82466E20(ctx, base);
	// 826B9AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9AB8 size=112
    let mut pc: u32 = 0x826B9AB8;
    'dispatch: loop {
        match pc {
            0x826B9AB8 => {
    //   block [0x826B9AB8..0x826B9B28)
	// 826B9AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9AC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9AC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9AC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9ACC: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9AD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9AD4: 390BD2D8  addi r8, r11, -0x2d28
	ctx.r[8].s64 = ctx.r[11].s64 + -11560;
	// 826B9AD8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826B9ADC: 388A31F4  addi r4, r10, 0x31f4
	ctx.r[4].s64 = ctx.r[10].s64 + 12788;
	// 826B9AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9AE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9AE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9AEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9AF0: 386A083C  addi r3, r10, 0x83c
	ctx.r[3].s64 = ctx.r[10].s64 + 2108;
	// 826B9AF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9AF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9B00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9B04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9B08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9B10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9B14: 4BDAD30D  bl 0x82466e20
	ctx.lr = 0x826B9B18;
	sub_82466E20(ctx, base);
	// 826B9B18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9B1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9B20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9B28 size=116
    let mut pc: u32 = 0x826B9B28;
    'dispatch: loop {
        match pc {
            0x826B9B28 => {
    //   block [0x826B9B28..0x826B9B9C)
	// 826B9B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9B30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9B34: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9B38: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826B9B3C: 390AD2F0  addi r8, r10, -0x2d10
	ctx.r[8].s64 = ctx.r[10].s64 + -11536;
	// 826B9B40: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9B44: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9B48: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9B4C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9B50: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9B54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9B58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9B5C: 388A3210  addi r4, r10, 0x3210
	ctx.r[4].s64 = ctx.r[10].s64 + 12816;
	// 826B9B60: 396B04CC  addi r11, r11, 0x4cc
	ctx.r[11].s64 = ctx.r[11].s64 + 1228;
	// 826B9B64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9B68: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9B6C: 386A086C  addi r3, r10, 0x86c
	ctx.r[3].s64 = ctx.r[10].s64 + 2156;
	// 826B9B70: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B9B74: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9B78: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9B80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9B88: 4BDAD299  bl 0x82466e20
	ctx.lr = 0x826B9B8C;
	sub_82466E20(ctx, base);
	// 826B9B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9BA0 size=112
    let mut pc: u32 = 0x826B9BA0;
    'dispatch: loop {
        match pc {
            0x826B9BA0 => {
    //   block [0x826B9BA0..0x826B9C10)
	// 826B9BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9BA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9BAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9BB0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9BB4: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9BBC: 390BD350  addi r8, r11, -0x2cb0
	ctx.r[8].s64 = ctx.r[11].s64 + -11440;
	// 826B9BC0: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 826B9BC4: 388A3260  addi r4, r10, 0x3260
	ctx.r[4].s64 = ctx.r[10].s64 + 12896;
	// 826B9BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9BCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9BD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9BD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9BD8: 386A089C  addi r3, r10, 0x89c
	ctx.r[3].s64 = ctx.r[10].s64 + 2204;
	// 826B9BDC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9BE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9BFC: 4BDAD225  bl 0x82466e20
	ctx.lr = 0x826B9C00;
	sub_82466E20(ctx, base);
	// 826B9C00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9C10 size=112
    let mut pc: u32 = 0x826B9C10;
    'dispatch: loop {
        match pc {
            0x826B9C10 => {
    //   block [0x826B9C10..0x826B9C80)
	// 826B9C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9C18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9C1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9C20: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9C24: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9C2C: 390BD428  addi r8, r11, -0x2bd8
	ctx.r[8].s64 = ctx.r[11].s64 + -11224;
	// 826B9C30: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 826B9C34: 388A3274  addi r4, r10, 0x3274
	ctx.r[4].s64 = ctx.r[10].s64 + 12916;
	// 826B9C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9C3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9C40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9C48: 386A08CC  addi r3, r10, 0x8cc
	ctx.r[3].s64 = ctx.r[10].s64 + 2252;
	// 826B9C4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9C5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9C6C: 4BDAD1B5  bl 0x82466e20
	ctx.lr = 0x826B9C70;
	sub_82466E20(ctx, base);
	// 826B9C70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9C80 size=112
    let mut pc: u32 = 0x826B9C80;
    'dispatch: loop {
        match pc {
            0x826B9C80 => {
    //   block [0x826B9C80..0x826B9CF0)
	// 826B9C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9C88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9C8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9C90: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9C94: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9C98: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B9C9C: 390BD530  addi r8, r11, -0x2ad0
	ctx.r[8].s64 = ctx.r[11].s64 + -10960;
	// 826B9CA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B9CA4: 388A43BC  addi r4, r10, 0x43bc
	ctx.r[4].s64 = ctx.r[10].s64 + 17340;
	// 826B9CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9CAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9CB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9CB8: 386A08FC  addi r3, r10, 0x8fc
	ctx.r[3].s64 = ctx.r[10].s64 + 2300;
	// 826B9CBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9CC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9CCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9CDC: 4BDAD145  bl 0x82466e20
	ctx.lr = 0x826B9CE0;
	sub_82466E20(ctx, base);
	// 826B9CE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B9CF0 size=24
    let mut pc: u32 = 0x826B9CF0;
    'dispatch: loop {
        match pc {
            0x826B9CF0 => {
    //   block [0x826B9CF0..0x826B9D08)
	// 826B9CF0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9CF4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9CF8: 394A2A50  addi r10, r10, 0x2a50
	ctx.r[10].s64 = ctx.r[10].s64 + 10832;
	// 826B9CFC: 816BD0C4  lwz r11, -0x2f3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12092 as u32) ) } as u64;
	// 826B9D00: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826B9D04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9D08 size=116
    let mut pc: u32 = 0x826B9D08;
    'dispatch: loop {
        match pc {
            0x826B9D08 => {
    //   block [0x826B9D08..0x826B9D7C)
	// 826B9D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9D0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9D10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9D14: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9D18: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9D1C: 392B04FC  addi r9, r11, 0x4fc
	ctx.r[9].s64 = ctx.r[11].s64 + 1276;
	// 826B9D20: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9D24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826B9D28: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B9D2C: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 826B9D30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9D34: 388A4454  addi r4, r10, 0x4454
	ctx.r[4].s64 = ctx.r[10].s64 + 17492;
	// 826B9D38: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9D3C: 396B2A50  addi r11, r11, 0x2a50
	ctx.r[11].s64 = ctx.r[11].s64 + 10832;
	// 826B9D40: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B9D44: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9D48: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B9D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9D50: 386A092C  addi r3, r10, 0x92c
	ctx.r[3].s64 = ctx.r[10].s64 + 2348;
	// 826B9D54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B9D58: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B9D5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9D60: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B9D64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B9D68: 4BDAD0B9  bl 0x82466e20
	ctx.lr = 0x826B9D6C;
	sub_82466E20(ctx, base);
	// 826B9D6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9D80 size=116
    let mut pc: u32 = 0x826B9D80;
    'dispatch: loop {
        match pc {
            0x826B9D80 => {
    //   block [0x826B9D80..0x826B9DF4)
	// 826B9D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9D84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9D88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9D8C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9D90: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826B9D94: 390AD560  addi r8, r10, -0x2aa0
	ctx.r[8].s64 = ctx.r[10].s64 + -10912;
	// 826B9D98: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9D9C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9DA0: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826B9DA4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9DA8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9DAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9DB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9DB4: 388A32A4  addi r4, r10, 0x32a4
	ctx.r[4].s64 = ctx.r[10].s64 + 12964;
	// 826B9DB8: 396B056C  addi r11, r11, 0x56c
	ctx.r[11].s64 = ctx.r[11].s64 + 1388;
	// 826B9DBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9DC0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9DC4: 386A095C  addi r3, r10, 0x95c
	ctx.r[3].s64 = ctx.r[10].s64 + 2396;
	// 826B9DC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B9DCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9DD0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9DD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9DD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9DE0: 4BDAD041  bl 0x82466e20
	ctx.lr = 0x826B9DE4;
	sub_82466E20(ctx, base);
	// 826B9DE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9DF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9DF8 size=108
    let mut pc: u32 = 0x826B9DF8;
    'dispatch: loop {
        match pc {
            0x826B9DF8 => {
    //   block [0x826B9DF8..0x826B9E64)
	// 826B9DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9E00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9E04: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9E08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9E0C: 38EBD590  addi r7, r11, -0x2a70
	ctx.r[7].s64 = ctx.r[11].s64 + -10864;
	// 826B9E10: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826B9E14: 388A32B4  addi r4, r10, 0x32b4
	ctx.r[4].s64 = ctx.r[10].s64 + 12980;
	// 826B9E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9E1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9E20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826B9E24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9E28: 386A098C  addi r3, r10, 0x98c
	ctx.r[3].s64 = ctx.r[10].s64 + 2444;
	// 826B9E2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826B9E30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9E34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9E38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9E3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9E40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9E44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9E48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9E4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826B9E50: 4BDACFD1  bl 0x82466e20
	ctx.lr = 0x826B9E54;
	sub_82466E20(ctx, base);
	// 826B9E54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826B9E68 size=24
    let mut pc: u32 = 0x826B9E68;
    'dispatch: loop {
        match pc {
            0x826B9E68 => {
    //   block [0x826B9E68..0x826B9E80)
	// 826B9E68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9E6C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9E70: 394A2BE8  addi r10, r10, 0x2be8
	ctx.r[10].s64 = ctx.r[10].s64 + 11240;
	// 826B9E74: 816BD620  lwz r11, -0x29e0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10720 as u32) ) } as u64;
	// 826B9E78: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826B9E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9E80 size=116
    let mut pc: u32 = 0x826B9E80;
    'dispatch: loop {
        match pc {
            0x826B9E80 => {
    //   block [0x826B9E80..0x826B9EF4)
	// 826B9E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9E8C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9E90: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9E94: 392B0590  addi r9, r11, 0x590
	ctx.r[9].s64 = ctx.r[11].s64 + 1424;
	// 826B9E98: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9E9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9EA0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826B9EA4: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 826B9EA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9EAC: 388A3300  addi r4, r10, 0x3300
	ctx.r[4].s64 = ctx.r[10].s64 + 13056;
	// 826B9EB0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9EB4: 396B2BE8  addi r11, r11, 0x2be8
	ctx.r[11].s64 = ctx.r[11].s64 + 11240;
	// 826B9EB8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826B9EBC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9EC0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826B9EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9EC8: 386A09BC  addi r3, r10, 0x9bc
	ctx.r[3].s64 = ctx.r[10].s64 + 2492;
	// 826B9ECC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826B9ED0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826B9ED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9ED8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826B9EDC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826B9EE0: 4BDACF41  bl 0x82466e20
	ctx.lr = 0x826B9EE4;
	sub_82466E20(ctx, base);
	// 826B9EE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9EF8 size=112
    let mut pc: u32 = 0x826B9EF8;
    'dispatch: loop {
        match pc {
            0x826B9EF8 => {
    //   block [0x826B9EF8..0x826B9F68)
	// 826B9EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9F04: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9F08: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9F0C: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9F14: 390BD624  addi r8, r11, -0x29dc
	ctx.r[8].s64 = ctx.r[11].s64 + -10716;
	// 826B9F18: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826B9F1C: 388A331C  addi r4, r10, 0x331c
	ctx.r[4].s64 = ctx.r[10].s64 + 13084;
	// 826B9F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826B9F24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9F28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9F30: 386A09EC  addi r3, r10, 0x9ec
	ctx.r[3].s64 = ctx.r[10].s64 + 2540;
	// 826B9F34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826B9F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826B9F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826B9F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9F54: 4BDACECD  bl 0x82466e20
	ctx.lr = 0x826B9F58;
	sub_82466E20(ctx, base);
	// 826B9F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9F68 size=116
    let mut pc: u32 = 0x826B9F68;
    'dispatch: loop {
        match pc {
            0x826B9F68 => {
    //   block [0x826B9F68..0x826B9FDC)
	// 826B9F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9F74: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826B9F78: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826B9F7C: 390AD658  addi r8, r10, -0x29a8
	ctx.r[8].s64 = ctx.r[10].s64 + -10664;
	// 826B9F80: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9F84: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826B9F88: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826B9F8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9F90: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826B9F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826B9F98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826B9F9C: 388A3340  addi r4, r10, 0x3340
	ctx.r[4].s64 = ctx.r[10].s64 + 13120;
	// 826B9FA0: 396B05D8  addi r11, r11, 0x5d8
	ctx.r[11].s64 = ctx.r[11].s64 + 1496;
	// 826B9FA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826B9FA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826B9FAC: 386A0A1C  addi r3, r10, 0xa1c
	ctx.r[3].s64 = ctx.r[10].s64 + 2588;
	// 826B9FB0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826B9FB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826B9FB8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826B9FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826B9FC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826B9FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826B9FC8: 4BDACE59  bl 0x82466e20
	ctx.lr = 0x826B9FCC;
	sub_82466E20(ctx, base);
	// 826B9FCC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826B9FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826B9FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826B9FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826B9FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826B9FE0 size=108
    let mut pc: u32 = 0x826B9FE0;
    'dispatch: loop {
        match pc {
            0x826B9FE0 => {
    //   block [0x826B9FE0..0x826BA04C)
	// 826B9FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826B9FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826B9FE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826B9FEC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826B9FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826B9FF4: 38EBD6E8  addi r7, r11, -0x2918
	ctx.r[7].s64 = ctx.r[11].s64 + -10520;
	// 826B9FF8: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 826B9FFC: 388A3374  addi r4, r10, 0x3374
	ctx.r[4].s64 = ctx.r[10].s64 + 13172;
	// 826BA000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA004: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA008: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA00C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA010: 386A0A4C  addi r3, r10, 0xa4c
	ctx.r[3].s64 = ctx.r[10].s64 + 2636;
	// 826BA014: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA01C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA024: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA034: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA038: 4BDACDE9  bl 0x82466e20
	ctx.lr = 0x826BA03C;
	sub_82466E20(ctx, base);
	// 826BA03C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA050 size=116
    let mut pc: u32 = 0x826BA050;
    'dispatch: loop {
        match pc {
            0x826BA050 => {
    //   block [0x826BA050..0x826BA0C4)
	// 826BA050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA05C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA060: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826BA064: 390AD838  addi r8, r10, -0x27c8
	ctx.r[8].s64 = ctx.r[10].s64 + -10184;
	// 826BA068: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA06C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA070: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826BA074: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA078: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA080: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA084: 388A3394  addi r4, r10, 0x3394
	ctx.r[4].s64 = ctx.r[10].s64 + 13204;
	// 826BA088: 396B05FC  addi r11, r11, 0x5fc
	ctx.r[11].s64 = ctx.r[11].s64 + 1532;
	// 826BA08C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA090: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA094: 386A0A7C  addi r3, r10, 0xa7c
	ctx.r[3].s64 = ctx.r[10].s64 + 2684;
	// 826BA098: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BA09C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA0A0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BA0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA0A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA0AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA0B0: 4BDACD71  bl 0x82466e20
	ctx.lr = 0x826BA0B4;
	sub_82466E20(ctx, base);
	// 826BA0B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA0C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA0C8 size=112
    let mut pc: u32 = 0x826BA0C8;
    'dispatch: loop {
        match pc {
            0x826BA0C8 => {
    //   block [0x826BA0C8..0x826BA138)
	// 826BA0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA0D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA0D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA0D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA0DC: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826BA0E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA0E4: 390BD8F8  addi r8, r11, -0x2708
	ctx.r[8].s64 = ctx.r[11].s64 + -9992;
	// 826BA0E8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BA0EC: 388A33B0  addi r4, r10, 0x33b0
	ctx.r[4].s64 = ctx.r[10].s64 + 13232;
	// 826BA0F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA0F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA0F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA100: 386A0AAC  addi r3, r10, 0xaac
	ctx.r[3].s64 = ctx.r[10].s64 + 2732;
	// 826BA104: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA10C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA114: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA11C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA124: 4BDACCFD  bl 0x82466e20
	ctx.lr = 0x826BA128;
	sub_82466E20(ctx, base);
	// 826BA128: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA12C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA130: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA134: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA138 size=112
    let mut pc: u32 = 0x826BA138;
    'dispatch: loop {
        match pc {
            0x826BA138 => {
    //   block [0x826BA138..0x826BA1A8)
	// 826BA138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA140: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA148: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA14C: 38AA089C  addi r5, r10, 0x89c
	ctx.r[5].s64 = ctx.r[10].s64 + 2204;
	// 826BA150: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA154: 390BD910  addi r8, r11, -0x26f0
	ctx.r[8].s64 = ctx.r[11].s64 + -9968;
	// 826BA158: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826BA15C: 388A33D4  addi r4, r10, 0x33d4
	ctx.r[4].s64 = ctx.r[10].s64 + 13268;
	// 826BA160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA164: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA168: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA170: 386A0ADC  addi r3, r10, 0xadc
	ctx.r[3].s64 = ctx.r[10].s64 + 2780;
	// 826BA174: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA178: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA180: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA184: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA188: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA18C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA190: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA194: 4BDACC8D  bl 0x82466e20
	ctx.lr = 0x826BA198;
	sub_82466E20(ctx, base);
	// 826BA198: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA19C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA1A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA1A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA1A8 size=112
    let mut pc: u32 = 0x826BA1A8;
    'dispatch: loop {
        match pc {
            0x826BA1A8 => {
    //   block [0x826BA1A8..0x826BA218)
	// 826BA1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA1B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA1B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA1B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA1BC: 38AA074C  addi r5, r10, 0x74c
	ctx.r[5].s64 = ctx.r[10].s64 + 1868;
	// 826BA1C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA1C4: 390BD9A0  addi r8, r11, -0x2660
	ctx.r[8].s64 = ctx.r[11].s64 + -9824;
	// 826BA1C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BA1CC: 388A33E8  addi r4, r10, 0x33e8
	ctx.r[4].s64 = ctx.r[10].s64 + 13288;
	// 826BA1D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA1D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA1D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA1DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA1E0: 386A0B0C  addi r3, r10, 0xb0c
	ctx.r[3].s64 = ctx.r[10].s64 + 2828;
	// 826BA1E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA1E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA1EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA1F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA1F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA1F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA200: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA204: 4BDACC1D  bl 0x82466e20
	ctx.lr = 0x826BA208;
	sub_82466E20(ctx, base);
	// 826BA208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA20C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA210: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA218 size=116
    let mut pc: u32 = 0x826BA218;
    'dispatch: loop {
        match pc {
            0x826BA218 => {
    //   block [0x826BA218..0x826BA28C)
	// 826BA218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA224: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA228: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BA22C: 390BD9B8  addi r8, r11, -0x2648
	ctx.r[8].s64 = ctx.r[11].s64 + -9800;
	// 826BA230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA234: 392A0644  addi r9, r10, 0x644
	ctx.r[9].s64 = ctx.r[10].s64 + 1604;
	// 826BA238: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA23C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826BA240: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BA244: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA24C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA254: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA25C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BA260: 388A33FC  addi r4, r10, 0x33fc
	ctx.r[4].s64 = ctx.r[10].s64 + 13308;
	// 826BA264: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA268: 386B0B3C  addi r3, r11, 0xb3c
	ctx.r[3].s64 = ctx.r[11].s64 + 2876;
	// 826BA26C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BA270: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA274: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA278: 4BDACBA9  bl 0x82466e20
	ctx.lr = 0x826BA27C;
	sub_82466E20(ctx, base);
	// 826BA27C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA280: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA284: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA290 size=100
    let mut pc: u32 = 0x826BA290;
    'dispatch: loop {
        match pc {
            0x826BA290 => {
    //   block [0x826BA290..0x826BA2F4)
	// 826BA290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA298: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA29C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA2A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA2A4: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BA2A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA2AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA2B0: 388A3404  addi r4, r10, 0x3404
	ctx.r[4].s64 = ctx.r[10].s64 + 13316;
	// 826BA2B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA2B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA2BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA2C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA2C4: 386A0B6C  addi r3, r10, 0xb6c
	ctx.r[3].s64 = ctx.r[10].s64 + 2924;
	// 826BA2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA2CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA2D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BA2D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA2D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA2E0: 4BDACB41  bl 0x82466e20
	ctx.lr = 0x826BA2E4;
	sub_82466E20(ctx, base);
	// 826BA2E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA2F8 size=112
    let mut pc: u32 = 0x826BA2F8;
    'dispatch: loop {
        match pc {
            0x826BA2F8 => {
    //   block [0x826BA2F8..0x826BA368)
	// 826BA2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA304: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA308: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA30C: 38AA0B6C  addi r5, r10, 0xb6c
	ctx.r[5].s64 = ctx.r[10].s64 + 2924;
	// 826BA310: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA314: 390BD9D0  addi r8, r11, -0x2630
	ctx.r[8].s64 = ctx.r[11].s64 + -9776;
	// 826BA318: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BA31C: 388A3414  addi r4, r10, 0x3414
	ctx.r[4].s64 = ctx.r[10].s64 + 13332;
	// 826BA320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA324: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA330: 386A0B9C  addi r3, r10, 0xb9c
	ctx.r[3].s64 = ctx.r[10].s64 + 2972;
	// 826BA334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA354: 4BDACACD  bl 0x82466e20
	ctx.lr = 0x826BA358;
	sub_82466E20(ctx, base);
	// 826BA358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA368 size=108
    let mut pc: u32 = 0x826BA368;
    'dispatch: loop {
        match pc {
            0x826BA368 => {
    //   block [0x826BA368..0x826BA3D4)
	// 826BA368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA374: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA378: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826BA37C: 38EBD9E8  addi r7, r11, -0x2618
	ctx.r[7].s64 = ctx.r[11].s64 + -9752;
	// 826BA380: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BA384: 388A446C  addi r4, r10, 0x446c
	ctx.r[4].s64 = ctx.r[10].s64 + 17516;
	// 826BA388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA38C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA398: 386A0BCC  addi r3, r10, 0xbcc
	ctx.r[3].s64 = ctx.r[10].s64 + 3020;
	// 826BA39C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA3BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA3C0: 4BDACA61  bl 0x82466e20
	ctx.lr = 0x826BA3C4;
	sub_82466E20(ctx, base);
	// 826BA3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA3D8 size=112
    let mut pc: u32 = 0x826BA3D8;
    'dispatch: loop {
        match pc {
            0x826BA3D8 => {
    //   block [0x826BA3D8..0x826BA448)
	// 826BA3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA3E4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA3E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA3EC: 392B06B0  addi r9, r11, 0x6b0
	ctx.r[9].s64 = ctx.r[11].s64 + 1712;
	// 826BA3F0: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826BA3F4: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826BA3F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA3FC: 388A35CC  addi r4, r10, 0x35cc
	ctx.r[4].s64 = ctx.r[10].s64 + 13772;
	// 826BA400: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA404: 396BDA50  addi r11, r11, -0x25b0
	ctx.r[11].s64 = ctx.r[11].s64 + -9648;
	// 826BA408: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826BA40C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA410: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826BA414: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA418: 386A0BFC  addi r3, r10, 0xbfc
	ctx.r[3].s64 = ctx.r[10].s64 + 3068;
	// 826BA41C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BA420: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826BA424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA428: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826BA42C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA430: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA434: 4BDAC9ED  bl 0x82466e20
	ctx.lr = 0x826BA438;
	sub_82466E20(ctx, base);
	// 826BA438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA448 size=108
    let mut pc: u32 = 0x826BA448;
    'dispatch: loop {
        match pc {
            0x826BA448 => {
    //   block [0x826BA448..0x826BA4B4)
	// 826BA448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA454: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA458: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA45C: 38EBDB10  addi r7, r11, -0x24f0
	ctx.r[7].s64 = ctx.r[11].s64 + -9456;
	// 826BA460: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BA464: 388A3610  addi r4, r10, 0x3610
	ctx.r[4].s64 = ctx.r[10].s64 + 13840;
	// 826BA468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA46C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA478: 386A0C2C  addi r3, r10, 0xc2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3116;
	// 826BA47C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA4A0: 4BDAC981  bl 0x82466e20
	ctx.lr = 0x826BA4A4;
	sub_82466E20(ctx, base);
	// 826BA4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA4B8 size=116
    let mut pc: u32 = 0x826BA4B8;
    'dispatch: loop {
        match pc {
            0x826BA4B8 => {
    //   block [0x826BA4B8..0x826BA52C)
	// 826BA4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA4C4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA4C8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 826BA4CC: 390ADB58  addi r8, r10, -0x24a8
	ctx.r[8].s64 = ctx.r[10].s64 + -9384;
	// 826BA4D0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA4D4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA4D8: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826BA4DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA4E0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA4E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA4EC: 388A362C  addi r4, r10, 0x362c
	ctx.r[4].s64 = ctx.r[10].s64 + 13868;
	// 826BA4F0: 396B06E8  addi r11, r11, 0x6e8
	ctx.r[11].s64 = ctx.r[11].s64 + 1768;
	// 826BA4F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA4F8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA4FC: 386A0C5C  addi r3, r10, 0xc5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3164;
	// 826BA500: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BA504: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA508: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BA50C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA510: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA518: 4BDAC909  bl 0x82466e20
	ctx.lr = 0x826BA51C;
	sub_82466E20(ctx, base);
	// 826BA51C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA530 size=100
    let mut pc: u32 = 0x826BA530;
    'dispatch: loop {
        match pc {
            0x826BA530 => {
    //   block [0x826BA530..0x826BA594)
	// 826BA530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA53C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA544: 38AA059C  addi r5, r10, 0x59c
	ctx.r[5].s64 = ctx.r[10].s64 + 1436;
	// 826BA548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA54C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA550: 388A363C  addi r4, r10, 0x363c
	ctx.r[4].s64 = ctx.r[10].s64 + 13884;
	// 826BA554: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA564: 386A0C8C  addi r3, r10, 0xc8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3212;
	// 826BA568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA56C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA570: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BA574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA578: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA580: 4BDAC8A1  bl 0x82466e20
	ctx.lr = 0x826BA584;
	sub_82466E20(ctx, base);
	// 826BA584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BA598 size=24
    let mut pc: u32 = 0x826BA598;
    'dispatch: loop {
        match pc {
            0x826BA598 => {
    //   block [0x826BA598..0x826BA5B0)
	// 826BA598: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA59C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA5A0: 394A2CF0  addi r10, r10, 0x2cf0
	ctx.r[10].s64 = ctx.r[10].s64 + 11504;
	// 826BA5A4: 816BDCD8  lwz r11, -0x2328(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-9000 as u32) ) } as u64;
	// 826BA5A8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826BA5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA5B0 size=116
    let mut pc: u32 = 0x826BA5B0;
    'dispatch: loop {
        match pc {
            0x826BA5B0 => {
    //   block [0x826BA5B0..0x826BA624)
	// 826BA5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA5BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA5C0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA5C4: 392B0768  addi r9, r11, 0x768
	ctx.r[9].s64 = ctx.r[11].s64 + 1896;
	// 826BA5C8: 38AA0C8C  addi r5, r10, 0xc8c
	ctx.r[5].s64 = ctx.r[10].s64 + 3212;
	// 826BA5CC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA5D0: 38E90028  addi r7, r9, 0x28
	ctx.r[7].s64 = ctx.r[9].s64 + 40;
	// 826BA5D4: 38C0000C  li r6, 0xc
	ctx.r[6].s64 = 12;
	// 826BA5D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA5DC: 388A3690  addi r4, r10, 0x3690
	ctx.r[4].s64 = ctx.r[10].s64 + 13968;
	// 826BA5E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA5E4: 396B2CF0  addi r11, r11, 0x2cf0
	ctx.r[11].s64 = ctx.r[11].s64 + 11504;
	// 826BA5E8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826BA5EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA5F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826BA5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA5F8: 386A0CBC  addi r3, r10, 0xcbc
	ctx.r[3].s64 = ctx.r[10].s64 + 3260;
	// 826BA5FC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826BA600: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826BA604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA608: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826BA60C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA610: 4BDAC811  bl 0x82466e20
	ctx.lr = 0x826BA614;
	sub_82466E20(ctx, base);
	// 826BA614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA61C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA628 size=112
    let mut pc: u32 = 0x826BA628;
    'dispatch: loop {
        match pc {
            0x826BA628 => {
    //   block [0x826BA628..0x826BA698)
	// 826BA628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA634: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA638: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BA63C: 38EADCE0  addi r7, r10, -0x2320
	ctx.r[7].s64 = ctx.r[10].s64 + -8992;
	// 826BA640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA644: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BA648: 388A3770  addi r4, r10, 0x3770
	ctx.r[4].s64 = ctx.r[10].s64 + 14192;
	// 826BA64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA650: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA654: 396B07F4  addi r11, r11, 0x7f4
	ctx.r[11].s64 = ctx.r[11].s64 + 2036;
	// 826BA658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA65C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA660: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA664: 386A0CEC  addi r3, r10, 0xcec
	ctx.r[3].s64 = ctx.r[10].s64 + 3308;
	// 826BA668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA66C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BA670: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA674: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BA678: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA67C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA680: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA684: 4BDAC79D  bl 0x82466e20
	ctx.lr = 0x826BA688;
	sub_82466E20(ctx, base);
	// 826BA688: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA68C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA698 size=108
    let mut pc: u32 = 0x826BA698;
    'dispatch: loop {
        match pc {
            0x826BA698 => {
    //   block [0x826BA698..0x826BA704)
	// 826BA698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA69C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA6A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA6A4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA6A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA6AC: 38EBDD40  addi r7, r11, -0x22c0
	ctx.r[7].s64 = ctx.r[11].s64 + -8896;
	// 826BA6B0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BA6B4: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 826BA6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA6BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA6C0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA6C8: 386A0D1C  addi r3, r10, 0xd1c
	ctx.r[3].s64 = ctx.r[10].s64 + 3356;
	// 826BA6CC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA6D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA6DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA6EC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA6F0: 4BDAC731  bl 0x82466e20
	ctx.lr = 0x826BA6F4;
	sub_82466E20(ctx, base);
	// 826BA6F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA6F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA6FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA708 size=116
    let mut pc: u32 = 0x826BA708;
    'dispatch: loop {
        match pc {
            0x826BA708 => {
    //   block [0x826BA708..0x826BA77C)
	// 826BA708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA714: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA718: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BA71C: 390BDD70  addi r8, r11, -0x2290
	ctx.r[8].s64 = ctx.r[11].s64 + -8848;
	// 826BA720: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA724: 392A07E0  addi r9, r10, 0x7e0
	ctx.r[9].s64 = ctx.r[10].s64 + 2016;
	// 826BA728: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA72C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826BA730: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BA734: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA73C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BA740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA744: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA74C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BA750: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826BA754: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA758: 386B0D4C  addi r3, r11, 0xd4c
	ctx.r[3].s64 = ctx.r[11].s64 + 3404;
	// 826BA75C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BA760: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA768: 4BDAC6B9  bl 0x82466e20
	ctx.lr = 0x826BA76C;
	sub_82466E20(ctx, base);
	// 826BA76C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA780 size=96
    let mut pc: u32 = 0x826BA780;
    'dispatch: loop {
        match pc {
            0x826BA780 => {
    //   block [0x826BA780..0x826BA7E0)
	// 826BA780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA78C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA794: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 826BA798: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA7A0: 386A0D7C  addi r3, r10, 0xd7c
	ctx.r[3].s64 = ctx.r[10].s64 + 3452;
	// 826BA7A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA7AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BA7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA7C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BA7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA7C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BA7CC: 4BDAC655  bl 0x82466e20
	ctx.lr = 0x826BA7D0;
	sub_82466E20(ctx, base);
	// 826BA7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA7E0 size=112
    let mut pc: u32 = 0x826BA7E0;
    'dispatch: loop {
        match pc {
            0x826BA7E0 => {
    //   block [0x826BA7E0..0x826BA850)
	// 826BA7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA7EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA7F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA7F4: 38AA0D7C  addi r5, r10, 0xd7c
	ctx.r[5].s64 = ctx.r[10].s64 + 3452;
	// 826BA7F8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA7FC: 390BDD88  addi r8, r11, -0x2278
	ctx.r[8].s64 = ctx.r[11].s64 + -8824;
	// 826BA800: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BA804: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 826BA808: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA80C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA810: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA818: 386A0DAC  addi r3, r10, 0xdac
	ctx.r[3].s64 = ctx.r[10].s64 + 3500;
	// 826BA81C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BA820: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA828: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA830: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA838: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA83C: 4BDAC5E5  bl 0x82466e20
	ctx.lr = 0x826BA840;
	sub_82466E20(ctx, base);
	// 826BA840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA84C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA850 size=112
    let mut pc: u32 = 0x826BA850;
    'dispatch: loop {
        match pc {
            0x826BA850 => {
    //   block [0x826BA850..0x826BA8C0)
	// 826BA850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA85C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BA860: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA864: 392A0810  addi r9, r10, 0x810
	ctx.r[9].s64 = ctx.r[10].s64 + 2064;
	// 826BA868: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA86C: 390BDDC0  addi r8, r11, -0x2240
	ctx.r[8].s64 = ctx.r[11].s64 + -8768;
	// 826BA870: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826BA874: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 826BA878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA87C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA888: 386A0DDC  addi r3, r10, 0xddc
	ctx.r[3].s64 = ctx.r[10].s64 + 3548;
	// 826BA88C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BA890: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BA894: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA89C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA8AC: 4BDAC575  bl 0x82466e20
	ctx.lr = 0x826BA8B0;
	sub_82466E20(ctx, base);
	// 826BA8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA8C0 size=108
    let mut pc: u32 = 0x826BA8C0;
    'dispatch: loop {
        match pc {
            0x826BA8C0 => {
    //   block [0x826BA8C0..0x826BA92C)
	// 826BA8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA8CC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA8D0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA8D4: 38EBDE68  addi r7, r11, -0x2198
	ctx.r[7].s64 = ctx.r[11].s64 + -8600;
	// 826BA8D8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BA8DC: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 826BA8E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA8E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA8E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA8EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA8F0: 386A0E0C  addi r3, r10, 0xe0c
	ctx.r[3].s64 = ctx.r[10].s64 + 3596;
	// 826BA8F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA8F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA900: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA904: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA908: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA910: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA914: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA918: 4BDAC509  bl 0x82466e20
	ctx.lr = 0x826BA91C;
	sub_82466E20(ctx, base);
	// 826BA91C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA920: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA924: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA928: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA930 size=108
    let mut pc: u32 = 0x826BA930;
    'dispatch: loop {
        match pc {
            0x826BA930 => {
    //   block [0x826BA930..0x826BA99C)
	// 826BA930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA93C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA940: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA944: 38EBDE98  addi r7, r11, -0x2168
	ctx.r[7].s64 = ctx.r[11].s64 + -8552;
	// 826BA948: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BA94C: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 826BA950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA954: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA958: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BA95C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BA960: 386A0E3C  addi r3, r10, 0xe3c
	ctx.r[3].s64 = ctx.r[10].s64 + 3644;
	// 826BA964: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BA968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BA96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BA970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BA974: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BA978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BA97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BA984: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BA988: 4BDAC499  bl 0x82466e20
	ctx.lr = 0x826BA98C;
	sub_82466E20(ctx, base);
	// 826BA98C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BA990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BA994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BA998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BA9A0 size=28
    let mut pc: u32 = 0x826BA9A0;
    'dispatch: loop {
        match pc {
            0x826BA9A0 => {
    //   block [0x826BA9A0..0x826BA9BC)
	// 826BA9A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA9A4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BA9A8: 394A2E10  addi r10, r10, 0x2e10
	ctx.r[10].s64 = ctx.r[10].s64 + 11792;
	// 826BA9AC: 816BDDBC  lwz r11, -0x2244(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8772 as u32) ) } as u64;
	// 826BA9B0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826BA9B4: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826BA9B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BA9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BA9C0 size=112
    let mut pc: u32 = 0x826BA9C0;
    'dispatch: loop {
        match pc {
            0x826BA9C0 => {
    //   block [0x826BA9C0..0x826BAA30)
	// 826BA9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BA9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BA9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BA9CC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BA9D0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BA9D4: 392A0988  addi r9, r10, 0x988
	ctx.r[9].s64 = ctx.r[10].s64 + 2440;
	// 826BA9D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BA9DC: 390B2E10  addi r8, r11, 0x2e10
	ctx.r[8].s64 = ctx.r[11].s64 + 11792;
	// 826BA9E0: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826BA9E4: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 826BA9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BA9EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BA9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BA9F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BA9F8: 386A0E6C  addi r3, r10, 0xe6c
	ctx.r[3].s64 = ctx.r[10].s64 + 3692;
	// 826BA9FC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BAA00: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826BAA04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAA0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAA14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAA1C: 4BDAC405  bl 0x82466e20
	ctx.lr = 0x826BAA20;
	sub_82466E20(ctx, base);
	// 826BAA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAA30 size=108
    let mut pc: u32 = 0x826BAA30;
    'dispatch: loop {
        match pc {
            0x826BAA30 => {
    //   block [0x826BAA30..0x826BAA9C)
	// 826BAA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAA3C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAA40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAA44: 38EBDED0  addi r7, r11, -0x2130
	ctx.r[7].s64 = ctx.r[11].s64 + -8496;
	// 826BAA48: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAA4C: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 826BAA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAA54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAA58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAA60: 386A0E9C  addi r3, r10, 0xe9c
	ctx.r[3].s64 = ctx.r[10].s64 + 3740;
	// 826BAA64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAA74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAA84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAA88: 4BDAC399  bl 0x82466e20
	ctx.lr = 0x826BAA8C;
	sub_82466E20(ctx, base);
	// 826BAA8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAA90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAA94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAA98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAAA0 size=108
    let mut pc: u32 = 0x826BAAA0;
    'dispatch: loop {
        match pc {
            0x826BAAA0 => {
    //   block [0x826BAAA0..0x826BAB0C)
	// 826BAAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAAAC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAAB0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAAB4: 38EBDF00  addi r7, r11, -0x2100
	ctx.r[7].s64 = ctx.r[11].s64 + -8448;
	// 826BAAB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BAABC: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826BAAC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAAC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAAC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAAD0: 386A0ECC  addi r3, r10, 0xecc
	ctx.r[3].s64 = ctx.r[10].s64 + 3788;
	// 826BAAD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAAD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAADC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAAE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAAE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAAE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAAF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAAF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAAF8: 4BDAC329  bl 0x82466e20
	ctx.lr = 0x826BAAFC;
	sub_82466E20(ctx, base);
	// 826BAAFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAB00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAB04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BAB10 size=24
    let mut pc: u32 = 0x826BAB10;
    'dispatch: loop {
        match pc {
            0x826BAB10 => {
    //   block [0x826BAB10..0x826BAB28)
	// 826BAB10: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAB14: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BAB18: 394A2ED0  addi r10, r10, 0x2ed0
	ctx.r[10].s64 = ctx.r[10].s64 + 11984;
	// 826BAB1C: 816BDF18  lwz r11, -0x20e8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8424 as u32) ) } as u64;
	// 826BAB20: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 826BAB24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAB28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAB28 size=112
    let mut pc: u32 = 0x826BAB28;
    'dispatch: loop {
        match pc {
            0x826BAB28 => {
    //   block [0x826BAB28..0x826BAB98)
	// 826BAB28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAB2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAB30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAB34: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BAB38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAB3C: 392A09DC  addi r9, r10, 0x9dc
	ctx.r[9].s64 = ctx.r[10].s64 + 2524;
	// 826BAB40: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAB44: 390B2ED0  addi r8, r11, 0x2ed0
	ctx.r[8].s64 = ctx.r[11].s64 + 11984;
	// 826BAB48: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 826BAB4C: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 826BAB50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAB54: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAB58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BAB5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAB60: 386A0EFC  addi r3, r10, 0xefc
	ctx.r[3].s64 = ctx.r[10].s64 + 3836;
	// 826BAB64: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BAB68: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BAB6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAB70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAB74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAB78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAB7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAB80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAB84: 4BDAC29D  bl 0x82466e20
	ctx.lr = 0x826BAB88;
	sub_82466E20(ctx, base);
	// 826BAB88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAB8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAB90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAB98 size=112
    let mut pc: u32 = 0x826BAB98;
    'dispatch: loop {
        match pc {
            0x826BAB98 => {
    //   block [0x826BAB98..0x826BAC08)
	// 826BAB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BABA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BABA4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BABA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BABAC: 392A0A18  addi r9, r10, 0xa18
	ctx.r[9].s64 = ctx.r[10].s64 + 2584;
	// 826BABB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BABB4: 390BDF28  addi r8, r11, -0x20d8
	ctx.r[8].s64 = ctx.r[11].s64 + -8408;
	// 826BABB8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826BABBC: 388A37B0  addi r4, r10, 0x37b0
	ctx.r[4].s64 = ctx.r[10].s64 + 14256;
	// 826BABC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BABC4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BABC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BABCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BABD0: 386A0F2C  addi r3, r10, 0xf2c
	ctx.r[3].s64 = ctx.r[10].s64 + 3884;
	// 826BABD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BABD8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826BABDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BABE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BABE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BABE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BABEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BABF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BABF4: 4BDAC22D  bl 0x82466e20
	ctx.lr = 0x826BABF8;
	sub_82466E20(ctx, base);
	// 826BABF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BABFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAC00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAC04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAC08 size=108
    let mut pc: u32 = 0x826BAC08;
    'dispatch: loop {
        match pc {
            0x826BAC08 => {
    //   block [0x826BAC08..0x826BAC74)
	// 826BAC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAC0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAC10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAC14: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAC18: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAC1C: 38EBDF70  addi r7, r11, -0x2090
	ctx.r[7].s64 = ctx.r[11].s64 + -8336;
	// 826BAC20: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAC24: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 826BAC28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAC2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAC30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAC38: 386A0F5C  addi r3, r10, 0xf5c
	ctx.r[3].s64 = ctx.r[10].s64 + 3932;
	// 826BAC3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAC40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAC44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAC48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAC4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAC50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAC54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAC58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAC5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAC60: 4BDAC1C1  bl 0x82466e20
	ctx.lr = 0x826BAC64;
	sub_82466E20(ctx, base);
	// 826BAC64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAC68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAC6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAC78 size=108
    let mut pc: u32 = 0x826BAC78;
    'dispatch: loop {
        match pc {
            0x826BAC78 => {
    //   block [0x826BAC78..0x826BACE4)
	// 826BAC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAC84: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAC88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAC8C: 38EBDFA0  addi r7, r11, -0x2060
	ctx.r[7].s64 = ctx.r[11].s64 + -8288;
	// 826BAC90: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAC94: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 826BAC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAC9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BACA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BACA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BACA8: 386A0F8C  addi r3, r10, 0xf8c
	ctx.r[3].s64 = ctx.r[10].s64 + 3980;
	// 826BACAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BACB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BACB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BACB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BACBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BACC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BACC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BACC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BACCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BACD0: 4BDAC151  bl 0x82466e20
	ctx.lr = 0x826BACD4;
	sub_82466E20(ctx, base);
	// 826BACD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BACD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BACDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BACE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BACE8 size=112
    let mut pc: u32 = 0x826BACE8;
    'dispatch: loop {
        match pc {
            0x826BACE8 => {
    //   block [0x826BACE8..0x826BAD58)
	// 826BACE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BACEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BACF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BACF4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BACF8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BACFC: 392A0A50  addi r9, r10, 0xa50
	ctx.r[9].s64 = ctx.r[10].s64 + 2640;
	// 826BAD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAD04: 390BDFD0  addi r8, r11, -0x2030
	ctx.r[8].s64 = ctx.r[11].s64 + -8240;
	// 826BAD08: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 826BAD0C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 826BAD10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAD14: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAD18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BAD1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAD20: 386A0FBC  addi r3, r10, 0xfbc
	ctx.r[3].s64 = ctx.r[10].s64 + 4028;
	// 826BAD24: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BAD28: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BAD2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAD30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAD38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAD3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAD40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAD44: 4BDAC0DD  bl 0x82466e20
	ctx.lr = 0x826BAD48;
	sub_82466E20(ctx, base);
	// 826BAD48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAD4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAD50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAD54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAD58 size=108
    let mut pc: u32 = 0x826BAD58;
    'dispatch: loop {
        match pc {
            0x826BAD58 => {
    //   block [0x826BAD58..0x826BADC4)
	// 826BAD58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAD5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAD60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAD64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAD68: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BAD6C: 38EBE030  addi r7, r11, -0x1fd0
	ctx.r[7].s64 = ctx.r[11].s64 + -8144;
	// 826BAD70: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 826BAD74: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826BAD78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAD7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAD80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAD84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAD88: 386A0FEC  addi r3, r10, 0xfec
	ctx.r[3].s64 = ctx.r[10].s64 + 4076;
	// 826BAD8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAD90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAD94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAD9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BADA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BADA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BADA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BADAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BADB0: 4BDAC071  bl 0x82466e20
	ctx.lr = 0x826BADB4;
	sub_82466E20(ctx, base);
	// 826BADB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BADB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BADBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BADC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BADC8 size=108
    let mut pc: u32 = 0x826BADC8;
    'dispatch: loop {
        match pc {
            0x826BADC8 => {
    //   block [0x826BADC8..0x826BAE34)
	// 826BADC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BADCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BADD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BADD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BADD8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826BADDC: 38EBE120  addi r7, r11, -0x1ee0
	ctx.r[7].s64 = ctx.r[11].s64 + -7904;
	// 826BADE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BADE4: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 826BADE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BADEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BADF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BADF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BADF8: 386A101C  addi r3, r10, 0x101c
	ctx.r[3].s64 = ctx.r[10].s64 + 4124;
	// 826BADFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAE00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAE1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAE20: 4BDAC001  bl 0x82466e20
	ctx.lr = 0x826BAE24;
	sub_82466E20(ctx, base);
	// 826BAE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAE38 size=108
    let mut pc: u32 = 0x826BAE38;
    'dispatch: loop {
        match pc {
            0x826BAE38 => {
    //   block [0x826BAE38..0x826BAEA4)
	// 826BAE38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAE3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAE40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAE44: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAE48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAE4C: 38EBE138  addi r7, r11, -0x1ec8
	ctx.r[7].s64 = ctx.r[11].s64 + -7880;
	// 826BAE50: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826BAE54: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 826BAE58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAE5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAE60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAE68: 386A104C  addi r3, r10, 0x104c
	ctx.r[3].s64 = ctx.r[10].s64 + 4172;
	// 826BAE6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAE70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAE74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAE78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAE7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAE80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAE84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAE88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAE8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAE90: 4BDABF91  bl 0x82466e20
	ctx.lr = 0x826BAE94;
	sub_82466E20(ctx, base);
	// 826BAE94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAE98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAE9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BAEA8 size=24
    let mut pc: u32 = 0x826BAEA8;
    'dispatch: loop {
        match pc {
            0x826BAEA8 => {
    //   block [0x826BAEA8..0x826BAEC0)
	// 826BAEA8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAEAC: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BAEB0: 394A2F60  addi r10, r10, 0x2f60
	ctx.r[10].s64 = ctx.r[10].s64 + 12128;
	// 826BAEB4: 816BE1C8  lwz r11, -0x1e38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7736 as u32) ) } as u64;
	// 826BAEB8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826BAEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAEC0 size=108
    let mut pc: u32 = 0x826BAEC0;
    'dispatch: loop {
        match pc {
            0x826BAEC0 => {
    //   block [0x826BAEC0..0x826BAF2C)
	// 826BAEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAECC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAED0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAED4: 38EB2F60  addi r7, r11, 0x2f60
	ctx.r[7].s64 = ctx.r[11].s64 + 12128;
	// 826BAED8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAEDC: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 826BAEE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAEE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAEE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAEEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAEF0: 386A107C  addi r3, r10, 0x107c
	ctx.r[3].s64 = ctx.r[10].s64 + 4220;
	// 826BAEF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAEF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAF00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAF08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAF10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAF14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAF18: 4BDABF09  bl 0x82466e20
	ctx.lr = 0x826BAF1C;
	sub_82466E20(ctx, base);
	// 826BAF1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAF20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAF24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAF28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BAF30 size=24
    let mut pc: u32 = 0x826BAF30;
    'dispatch: loop {
        match pc {
            0x826BAF30 => {
    //   block [0x826BAF30..0x826BAF48)
	// 826BAF30: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAF34: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BAF38: 394A2F90  addi r10, r10, 0x2f90
	ctx.r[10].s64 = ctx.r[10].s64 + 12176;
	// 826BAF3C: 816BE1C8  lwz r11, -0x1e38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7736 as u32) ) } as u64;
	// 826BAF40: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826BAF44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAF48 size=108
    let mut pc: u32 = 0x826BAF48;
    'dispatch: loop {
        match pc {
            0x826BAF48 => {
    //   block [0x826BAF48..0x826BAFB4)
	// 826BAF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAF54: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAF58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAF5C: 38EB2F90  addi r7, r11, 0x2f90
	ctx.r[7].s64 = ctx.r[11].s64 + 12176;
	// 826BAF60: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BAF64: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826BAF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAF6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAF70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAF78: 386A10AC  addi r3, r10, 0x10ac
	ctx.r[3].s64 = ctx.r[10].s64 + 4268;
	// 826BAF7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAF80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAF84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAF88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BAF90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BAF94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BAF98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BAF9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BAFA0: 4BDABE81  bl 0x82466e20
	ctx.lr = 0x826BAFA4;
	sub_82466E20(ctx, base);
	// 826BAFA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BAFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BAFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BAFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BAFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BAFB8 size=108
    let mut pc: u32 = 0x826BAFB8;
    'dispatch: loop {
        match pc {
            0x826BAFB8 => {
    //   block [0x826BAFB8..0x826BB024)
	// 826BAFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BAFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BAFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BAFC4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BAFC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BAFCC: 38EBE1B0  addi r7, r11, -0x1e50
	ctx.r[7].s64 = ctx.r[11].s64 + -7760;
	// 826BAFD0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BAFD4: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 826BAFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BAFDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BAFE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BAFE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BAFE8: 386A10DC  addi r3, r10, 0x10dc
	ctx.r[3].s64 = ctx.r[10].s64 + 4316;
	// 826BAFEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BAFF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BAFF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BAFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BAFFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB004: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB00C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB010: 4BDABE11  bl 0x82466e20
	ctx.lr = 0x826BB014;
	sub_82466E20(ctx, base);
	// 826BB014: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BB028 size=24
    let mut pc: u32 = 0x826BB028;
    'dispatch: loop {
        match pc {
            0x826BB028 => {
    //   block [0x826BB028..0x826BB040)
	// 826BB028: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB02C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BB030: 394A2FC0  addi r10, r10, 0x2fc0
	ctx.r[10].s64 = ctx.r[10].s64 + 12224;
	// 826BB034: 816BE1C8  lwz r11, -0x1e38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7736 as u32) ) } as u64;
	// 826BB038: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826BB03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB040 size=108
    let mut pc: u32 = 0x826BB040;
    'dispatch: loop {
        match pc {
            0x826BB040 => {
    //   block [0x826BB040..0x826BB0AC)
	// 826BB040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB04C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB054: 38EB2FC0  addi r7, r11, 0x2fc0
	ctx.r[7].s64 = ctx.r[11].s64 + 12224;
	// 826BB058: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB05C: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826BB060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB064: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB070: 386A110C  addi r3, r10, 0x110c
	ctx.r[3].s64 = ctx.r[10].s64 + 4364;
	// 826BB074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB098: 4BDABD89  bl 0x82466e20
	ctx.lr = 0x826BB09C;
	sub_82466E20(ctx, base);
	// 826BB09C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB0B0 size=112
    let mut pc: u32 = 0x826BB0B0;
    'dispatch: loop {
        match pc {
            0x826BB0B0 => {
    //   block [0x826BB0B0..0x826BB120)
	// 826BB0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB0BC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB0C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB0C4: 392A0A94  addi r9, r10, 0xa94
	ctx.r[9].s64 = ctx.r[10].s64 + 2708;
	// 826BB0C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB0CC: 390BE1CC  addi r8, r11, -0x1e34
	ctx.r[8].s64 = ctx.r[11].s64 + -7732;
	// 826BB0D0: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826BB0D4: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 826BB0D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB0DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB0E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB0E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB0E8: 386A113C  addi r3, r10, 0x113c
	ctx.r[3].s64 = ctx.r[10].s64 + 4412;
	// 826BB0EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB0F0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB0F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB0FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB100: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB108: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB10C: 4BDABD15  bl 0x82466e20
	ctx.lr = 0x826BB110;
	sub_82466E20(ctx, base);
	// 826BB110: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB114: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB118: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB11C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB120 size=108
    let mut pc: u32 = 0x826BB120;
    'dispatch: loop {
        match pc {
            0x826BB120 => {
    //   block [0x826BB120..0x826BB18C)
	// 826BB120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB12C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB134: 38EBE1FC  addi r7, r11, -0x1e04
	ctx.r[7].s64 = ctx.r[11].s64 + -7684;
	// 826BB138: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB13C: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826BB140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB144: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB148: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB150: 386A116C  addi r3, r10, 0x116c
	ctx.r[3].s64 = ctx.r[10].s64 + 4460;
	// 826BB154: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB16C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB178: 4BDABCA9  bl 0x82466e20
	ctx.lr = 0x826BB17C;
	sub_82466E20(ctx, base);
	// 826BB17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB190 size=108
    let mut pc: u32 = 0x826BB190;
    'dispatch: loop {
        match pc {
            0x826BB190 => {
    //   block [0x826BB190..0x826BB1FC)
	// 826BB190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB19C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB1A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB1A4: 38EBE22C  addi r7, r11, -0x1dd4
	ctx.r[7].s64 = ctx.r[11].s64 + -7636;
	// 826BB1A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB1AC: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826BB1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB1B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB1B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB1C0: 386A119C  addi r3, r10, 0x119c
	ctx.r[3].s64 = ctx.r[10].s64 + 4508;
	// 826BB1C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB1E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB1E8: 4BDABC39  bl 0x82466e20
	ctx.lr = 0x826BB1EC;
	sub_82466E20(ctx, base);
	// 826BB1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB200 size=112
    let mut pc: u32 = 0x826BB200;
    'dispatch: loop {
        match pc {
            0x826BB200 => {
    //   block [0x826BB200..0x826BB270)
	// 826BB200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB20C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB210: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB214: 38AA11FC  addi r5, r10, 0x11fc
	ctx.r[5].s64 = ctx.r[10].s64 + 4604;
	// 826BB218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB21C: 390BE25C  addi r8, r11, -0x1da4
	ctx.r[8].s64 = ctx.r[11].s64 + -7588;
	// 826BB220: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BB224: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 826BB228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB22C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB238: 386A11CC  addi r3, r10, 0x11cc
	ctx.r[3].s64 = ctx.r[10].s64 + 4556;
	// 826BB23C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BB240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB25C: 4BDABBC5  bl 0x82466e20
	ctx.lr = 0x826BB260;
	sub_82466E20(ctx, base);
	// 826BB260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB270 size=108
    let mut pc: u32 = 0x826BB270;
    'dispatch: loop {
        match pc {
            0x826BB270 => {
    //   block [0x826BB270..0x826BB2DC)
	// 826BB270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB27C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB284: 38EBE274  addi r7, r11, -0x1d8c
	ctx.r[7].s64 = ctx.r[11].s64 + -7564;
	// 826BB288: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB28C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 826BB290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB294: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB298: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB2A0: 386A11FC  addi r3, r10, 0x11fc
	ctx.r[3].s64 = ctx.r[10].s64 + 4604;
	// 826BB2A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB2B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB2B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB2C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB2C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB2C8: 4BDABB59  bl 0x82466e20
	ctx.lr = 0x826BB2CC;
	sub_82466E20(ctx, base);
	// 826BB2CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB2E0 size=108
    let mut pc: u32 = 0x826BB2E0;
    'dispatch: loop {
        match pc {
            0x826BB2E0 => {
    //   block [0x826BB2E0..0x826BB34C)
	// 826BB2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB2EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB2F4: 38EBE2A4  addi r7, r11, -0x1d5c
	ctx.r[7].s64 = ctx.r[11].s64 + -7516;
	// 826BB2F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BB2FC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 826BB300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB304: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB308: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB310: 386A122C  addi r3, r10, 0x122c
	ctx.r[3].s64 = ctx.r[10].s64 + 4652;
	// 826BB314: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB338: 4BDABAE9  bl 0x82466e20
	ctx.lr = 0x826BB33C;
	sub_82466E20(ctx, base);
	// 826BB33C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB350 size=108
    let mut pc: u32 = 0x826BB350;
    'dispatch: loop {
        match pc {
            0x826BB350 => {
    //   block [0x826BB350..0x826BB3BC)
	// 826BB350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB35C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB360: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB364: 38EBE2BC  addi r7, r11, -0x1d44
	ctx.r[7].s64 = ctx.r[11].s64 + -7492;
	// 826BB368: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB36C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 826BB370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB374: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB378: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB37C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB380: 386A125C  addi r3, r10, 0x125c
	ctx.r[3].s64 = ctx.r[10].s64 + 4700;
	// 826BB384: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB388: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB38C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB394: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB39C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB3A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB3A8: 4BDABA79  bl 0x82466e20
	ctx.lr = 0x826BB3AC;
	sub_82466E20(ctx, base);
	// 826BB3AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB3B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB3C0 size=108
    let mut pc: u32 = 0x826BB3C0;
    'dispatch: loop {
        match pc {
            0x826BB3C0 => {
    //   block [0x826BB3C0..0x826BB42C)
	// 826BB3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB3CC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB3D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB3D4: 38EBE2F0  addi r7, r11, -0x1d10
	ctx.r[7].s64 = ctx.r[11].s64 + -7440;
	// 826BB3D8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826BB3DC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826BB3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB3E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB3E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB3F0: 386A128C  addi r3, r10, 0x128c
	ctx.r[3].s64 = ctx.r[10].s64 + 4748;
	// 826BB3F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB3F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB3FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB414: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB418: 4BDABA09  bl 0x82466e20
	ctx.lr = 0x826BB41C;
	sub_82466E20(ctx, base);
	// 826BB41C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB430 size=108
    let mut pc: u32 = 0x826BB430;
    'dispatch: loop {
        match pc {
            0x826BB430 => {
    //   block [0x826BB430..0x826BB49C)
	// 826BB430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB43C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB444: 38EBE398  addi r7, r11, -0x1c68
	ctx.r[7].s64 = ctx.r[11].s64 + -7272;
	// 826BB448: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB44C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 826BB450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB454: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB460: 386A12BC  addi r3, r10, 0x12bc
	ctx.r[3].s64 = ctx.r[10].s64 + 4796;
	// 826BB464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB47C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB488: 4BDAB999  bl 0x82466e20
	ctx.lr = 0x826BB48C;
	sub_82466E20(ctx, base);
	// 826BB48C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB4A0 size=108
    let mut pc: u32 = 0x826BB4A0;
    'dispatch: loop {
        match pc {
            0x826BB4A0 => {
    //   block [0x826BB4A0..0x826BB50C)
	// 826BB4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB4AC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB4B4: 38EBE3C8  addi r7, r11, -0x1c38
	ctx.r[7].s64 = ctx.r[11].s64 + -7224;
	// 826BB4B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BB4BC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826BB4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB4C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB4D0: 386A12EC  addi r3, r10, 0x12ec
	ctx.r[3].s64 = ctx.r[10].s64 + 4844;
	// 826BB4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB4F8: 4BDAB929  bl 0x82466e20
	ctx.lr = 0x826BB4FC;
	sub_82466E20(ctx, base);
	// 826BB4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB510 size=108
    let mut pc: u32 = 0x826BB510;
    'dispatch: loop {
        match pc {
            0x826BB510 => {
    //   block [0x826BB510..0x826BB57C)
	// 826BB510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB51C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB524: 38EBE3E0  addi r7, r11, -0x1c20
	ctx.r[7].s64 = ctx.r[11].s64 + -7200;
	// 826BB528: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB52C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 826BB530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB534: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB540: 386A131C  addi r3, r10, 0x131c
	ctx.r[3].s64 = ctx.r[10].s64 + 4892;
	// 826BB544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB568: 4BDAB8B9  bl 0x82466e20
	ctx.lr = 0x826BB56C;
	sub_82466E20(ctx, base);
	// 826BB56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB580 size=108
    let mut pc: u32 = 0x826BB580;
    'dispatch: loop {
        match pc {
            0x826BB580 => {
    //   block [0x826BB580..0x826BB5EC)
	// 826BB580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB58C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB594: 38EBE410  addi r7, r11, -0x1bf0
	ctx.r[7].s64 = ctx.r[11].s64 + -7152;
	// 826BB598: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826BB59C: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826BB5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB5A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB5B0: 386A134C  addi r3, r10, 0x134c
	ctx.r[3].s64 = ctx.r[10].s64 + 4940;
	// 826BB5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB5D8: 4BDAB849  bl 0x82466e20
	ctx.lr = 0x826BB5DC;
	sub_82466E20(ctx, base);
	// 826BB5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BB5F0 size=24
    let mut pc: u32 = 0x826BB5F0;
    'dispatch: loop {
        match pc {
            0x826BB5F0 => {
    //   block [0x826BB5F0..0x826BB608)
	// 826BB5F0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB5F4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BB5F8: 394A2FF0  addi r10, r10, 0x2ff0
	ctx.r[10].s64 = ctx.r[10].s64 + 12272;
	// 826BB5FC: 816BE2EC  lwz r11, -0x1d14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7444 as u32) ) } as u64;
	// 826BB600: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826BB604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB608 size=112
    let mut pc: u32 = 0x826BB608;
    'dispatch: loop {
        match pc {
            0x826BB608 => {
    //   block [0x826BB608..0x826BB678)
	// 826BB608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB614: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB618: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB61C: 392A0AC0  addi r9, r10, 0xac0
	ctx.r[9].s64 = ctx.r[10].s64 + 2752;
	// 826BB620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB624: 390B2FF0  addi r8, r11, 0x2ff0
	ctx.r[8].s64 = ctx.r[11].s64 + 12272;
	// 826BB628: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826BB62C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 826BB630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB634: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB638: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB63C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB640: 386A137C  addi r3, r10, 0x137c
	ctx.r[3].s64 = ctx.r[10].s64 + 4988;
	// 826BB644: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB648: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB64C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB650: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB654: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB658: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB660: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB664: 4BDAB7BD  bl 0x82466e20
	ctx.lr = 0x826BB668;
	sub_82466E20(ctx, base);
	// 826BB668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB678 size=108
    let mut pc: u32 = 0x826BB678;
    'dispatch: loop {
        match pc {
            0x826BB678 => {
    //   block [0x826BB678..0x826BB6E4)
	// 826BB678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB684: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB68C: 38EBE4D4  addi r7, r11, -0x1b2c
	ctx.r[7].s64 = ctx.r[11].s64 + -6956;
	// 826BB690: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB694: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 826BB698: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB69C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB6A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB6A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB6A8: 386A13AC  addi r3, r10, 0x13ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5036;
	// 826BB6AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB6B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB6B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB6B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB6BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB6C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB6C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB6C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB6CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB6D0: 4BDAB751  bl 0x82466e20
	ctx.lr = 0x826BB6D4;
	sub_82466E20(ctx, base);
	// 826BB6D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB6D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB6DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB6E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB6E8 size=112
    let mut pc: u32 = 0x826BB6E8;
    'dispatch: loop {
        match pc {
            0x826BB6E8 => {
    //   block [0x826BB6E8..0x826BB758)
	// 826BB6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB6F4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB6F8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB6FC: 392A0B04  addi r9, r10, 0xb04
	ctx.r[9].s64 = ctx.r[10].s64 + 2820;
	// 826BB700: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB704: 390BE508  addi r8, r11, -0x1af8
	ctx.r[8].s64 = ctx.r[11].s64 + -6904;
	// 826BB708: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 826BB70C: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 826BB710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB714: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB718: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB720: 386A13DC  addi r3, r10, 0x13dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5084;
	// 826BB724: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB728: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB72C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB730: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB738: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB740: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB744: 4BDAB6DD  bl 0x82466e20
	ctx.lr = 0x826BB748;
	sub_82466E20(ctx, base);
	// 826BB748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB74C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BB758 size=24
    let mut pc: u32 = 0x826BB758;
    'dispatch: loop {
        match pc {
            0x826BB758 => {
    //   block [0x826BB758..0x826BB770)
	// 826BB758: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB75C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BB760: 394A3068  addi r10, r10, 0x3068
	ctx.r[10].s64 = ctx.r[10].s64 + 12392;
	// 826BB764: 816BE504  lwz r11, -0x1afc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6908 as u32) ) } as u64;
	// 826BB768: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826BB76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB770 size=112
    let mut pc: u32 = 0x826BB770;
    'dispatch: loop {
        match pc {
            0x826BB770 => {
    //   block [0x826BB770..0x826BB7E0)
	// 826BB770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB77C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB780: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB784: 392A0B40  addi r9, r10, 0xb40
	ctx.r[9].s64 = ctx.r[10].s64 + 2880;
	// 826BB788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB78C: 390B3068  addi r8, r11, 0x3068
	ctx.r[8].s64 = ctx.r[11].s64 + 12392;
	// 826BB790: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826BB794: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 826BB798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB79C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB7A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB7A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB7A8: 386A140C  addi r3, r10, 0x140c
	ctx.r[3].s64 = ctx.r[10].s64 + 5132;
	// 826BB7AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB7B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB7B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB7B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB7BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB7C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB7C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB7C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB7CC: 4BDAB655  bl 0x82466e20
	ctx.lr = 0x826BB7D0;
	sub_82466E20(ctx, base);
	// 826BB7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB7D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB7D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB7DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB7E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB7E0 size=108
    let mut pc: u32 = 0x826BB7E0;
    'dispatch: loop {
        match pc {
            0x826BB7E0 => {
    //   block [0x826BB7E0..0x826BB84C)
	// 826BB7E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB7E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB7E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB7EC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB7F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB7F4: 38EBE5C8  addi r7, r11, -0x1a38
	ctx.r[7].s64 = ctx.r[11].s64 + -6712;
	// 826BB7F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BB7FC: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 826BB800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB804: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB808: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB810: 386A143C  addi r3, r10, 0x143c
	ctx.r[3].s64 = ctx.r[10].s64 + 5180;
	// 826BB814: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB818: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB820: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB824: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB828: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB82C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB830: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB834: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB838: 4BDAB5E9  bl 0x82466e20
	ctx.lr = 0x826BB83C;
	sub_82466E20(ctx, base);
	// 826BB83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB850 size=108
    let mut pc: u32 = 0x826BB850;
    'dispatch: loop {
        match pc {
            0x826BB850 => {
    //   block [0x826BB850..0x826BB8BC)
	// 826BB850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB85C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB860: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB864: 38EBE5E0  addi r7, r11, -0x1a20
	ctx.r[7].s64 = ctx.r[11].s64 + -6688;
	// 826BB868: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BB86C: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 826BB870: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB874: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB878: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB880: 386A146C  addi r3, r10, 0x146c
	ctx.r[3].s64 = ctx.r[10].s64 + 5228;
	// 826BB884: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB888: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB890: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB894: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB898: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB89C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB8A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB8A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB8A8: 4BDAB579  bl 0x82466e20
	ctx.lr = 0x826BB8AC;
	sub_82466E20(ctx, base);
	// 826BB8AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB8B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB8B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB8B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BB8C0 size=24
    let mut pc: u32 = 0x826BB8C0;
    'dispatch: loop {
        match pc {
            0x826BB8C0 => {
    //   block [0x826BB8C0..0x826BB8D8)
	// 826BB8C0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB8C4: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BB8C8: 394A30B0  addi r10, r10, 0x30b0
	ctx.r[10].s64 = ctx.r[10].s64 + 12464;
	// 826BB8CC: 816BE610  lwz r11, -0x19f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6640 as u32) ) } as u64;
	// 826BB8D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826BB8D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB8D8 size=112
    let mut pc: u32 = 0x826BB8D8;
    'dispatch: loop {
        match pc {
            0x826BB8D8 => {
    //   block [0x826BB8D8..0x826BB948)
	// 826BB8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB8E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB8E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BB8E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB8EC: 392A0B7C  addi r9, r10, 0xb7c
	ctx.r[9].s64 = ctx.r[10].s64 + 2940;
	// 826BB8F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB8F4: 390B30B0  addi r8, r11, 0x30b0
	ctx.r[8].s64 = ctx.r[11].s64 + 12464;
	// 826BB8F8: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826BB8FC: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 826BB900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB904: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB908: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BB90C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB910: 386A149C  addi r3, r10, 0x149c
	ctx.r[3].s64 = ctx.r[10].s64 + 5276;
	// 826BB914: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BB918: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BB91C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB92C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB934: 4BDAB4ED  bl 0x82466e20
	ctx.lr = 0x826BB938;
	sub_82466E20(ctx, base);
	// 826BB938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB944: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB948 size=108
    let mut pc: u32 = 0x826BB948;
    'dispatch: loop {
        match pc {
            0x826BB948 => {
    //   block [0x826BB948..0x826BB9B4)
	// 826BB948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB94C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB950: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB954: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB95C: 38EBE614  addi r7, r11, -0x19ec
	ctx.r[7].s64 = ctx.r[11].s64 + -6636;
	// 826BB960: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BB964: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 826BB968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB96C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB970: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB974: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB978: 386A14CC  addi r3, r10, 0x14cc
	ctx.r[3].s64 = ctx.r[10].s64 + 5324;
	// 826BB97C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB980: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB984: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB988: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB98C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BB990: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BB994: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BB998: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BB99C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BB9A0: 4BDAB481  bl 0x82466e20
	ctx.lr = 0x826BB9A4;
	sub_82466E20(ctx, base);
	// 826BB9A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BB9A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BB9AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BB9B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BB9B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BB9B8 size=108
    let mut pc: u32 = 0x826BB9B8;
    'dispatch: loop {
        match pc {
            0x826BB9B8 => {
    //   block [0x826BB9B8..0x826BBA24)
	// 826BB9B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BB9BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BB9C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BB9C4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BB9C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BB9CC: 38EBE630  addi r7, r11, -0x19d0
	ctx.r[7].s64 = ctx.r[11].s64 + -6608;
	// 826BB9D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BB9D4: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 826BB9D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BB9DC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BB9E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BB9E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BB9E8: 386A14FC  addi r3, r10, 0x14fc
	ctx.r[3].s64 = ctx.r[10].s64 + 5372;
	// 826BB9EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BB9F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BB9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BB9F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BB9FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBA00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBA04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBA0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBA10: 4BDAB411  bl 0x82466e20
	ctx.lr = 0x826BBA14;
	sub_82466E20(ctx, base);
	// 826BBA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBA28 size=108
    let mut pc: u32 = 0x826BBA28;
    'dispatch: loop {
        match pc {
            0x826BBA28 => {
    //   block [0x826BBA28..0x826BBA94)
	// 826BBA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBA34: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBA38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBA3C: 38EBE678  addi r7, r11, -0x1988
	ctx.r[7].s64 = ctx.r[11].s64 + -6536;
	// 826BBA40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BBA44: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 826BBA48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBA4C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBA50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBA54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBA58: 386A152C  addi r3, r10, 0x152c
	ctx.r[3].s64 = ctx.r[10].s64 + 5420;
	// 826BBA5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBA60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBA6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBA74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBA7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBA80: 4BDAB3A1  bl 0x82466e20
	ctx.lr = 0x826BBA84;
	sub_82466E20(ctx, base);
	// 826BBA84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBA88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBA8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBA90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBA98 size=108
    let mut pc: u32 = 0x826BBA98;
    'dispatch: loop {
        match pc {
            0x826BBA98 => {
    //   block [0x826BBA98..0x826BBB04)
	// 826BBA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBAA4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBAA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBAAC: 38EBE6A8  addi r7, r11, -0x1958
	ctx.r[7].s64 = ctx.r[11].s64 + -6488;
	// 826BBAB0: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826BBAB4: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 826BBAB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBABC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBAC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBAC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBAC8: 386A155C  addi r3, r10, 0x155c
	ctx.r[3].s64 = ctx.r[10].s64 + 5468;
	// 826BBACC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBAD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBAD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBAE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBAE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBAEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBAF0: 4BDAB331  bl 0x82466e20
	ctx.lr = 0x826BBAF4;
	sub_82466E20(ctx, base);
	// 826BBAF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBB00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBB08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBB08 size=108
    let mut pc: u32 = 0x826BBB08;
    'dispatch: loop {
        match pc {
            0x826BBB08 => {
    //   block [0x826BBB08..0x826BBB74)
	// 826BBB08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBB0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBB10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBB14: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBB18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBB1C: 38EBE7C8  addi r7, r11, -0x1838
	ctx.r[7].s64 = ctx.r[11].s64 + -6200;
	// 826BBB20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 826BBB24: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 826BBB28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBB2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBB30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBB34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBB38: 386A158C  addi r3, r10, 0x158c
	ctx.r[3].s64 = ctx.r[10].s64 + 5516;
	// 826BBB3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBB40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBB44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBB48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBB50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBB54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBB58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBB5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBB60: 4BDAB2C1  bl 0x82466e20
	ctx.lr = 0x826BBB64;
	sub_82466E20(ctx, base);
	// 826BBB64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBB68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBB6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBB70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBB78 size=108
    let mut pc: u32 = 0x826BBB78;
    'dispatch: loop {
        match pc {
            0x826BBB78 => {
    //   block [0x826BBB78..0x826BBBE4)
	// 826BBB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBB7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBB80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBB84: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBB8C: 38EBE858  addi r7, r11, -0x17a8
	ctx.r[7].s64 = ctx.r[11].s64 + -6056;
	// 826BBB90: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826BBB94: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 826BBB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBB9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBBA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBBA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBBA8: 386A15BC  addi r3, r10, 0x15bc
	ctx.r[3].s64 = ctx.r[10].s64 + 5564;
	// 826BBBAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBBB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBBB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBBBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBBC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBBC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBBC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBBCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBBD0: 4BDAB251  bl 0x82466e20
	ctx.lr = 0x826BBBD4;
	sub_82466E20(ctx, base);
	// 826BBBD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBBD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBBDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBBE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBBE8 size=108
    let mut pc: u32 = 0x826BBBE8;
    'dispatch: loop {
        match pc {
            0x826BBBE8 => {
    //   block [0x826BBBE8..0x826BBC54)
	// 826BBBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBBF4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBBF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBBFC: 38EBE918  addi r7, r11, -0x16e8
	ctx.r[7].s64 = ctx.r[11].s64 + -5864;
	// 826BBC00: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826BBC04: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 826BBC08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBC0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBC10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBC14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBC18: 386A15EC  addi r3, r10, 0x15ec
	ctx.r[3].s64 = ctx.r[10].s64 + 5612;
	// 826BBC1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBC20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBC24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBC28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBC30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBC38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBC3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBC40: 4BDAB1E1  bl 0x82466e20
	ctx.lr = 0x826BBC44;
	sub_82466E20(ctx, base);
	// 826BBC44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBC48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBC4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBC58 size=108
    let mut pc: u32 = 0x826BBC58;
    'dispatch: loop {
        match pc {
            0x826BBC58 => {
    //   block [0x826BBC58..0x826BBCC4)
	// 826BBC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBC60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBC64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBC68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBC6C: 38EBE9F0  addi r7, r11, -0x1610
	ctx.r[7].s64 = ctx.r[11].s64 + -5648;
	// 826BBC70: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 826BBC74: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 826BBC78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBC7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBC80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBC84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBC88: 386A161C  addi r3, r10, 0x161c
	ctx.r[3].s64 = ctx.r[10].s64 + 5660;
	// 826BBC8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBC90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBC94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBC98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBC9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBCA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBCA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBCA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBCAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBCB0: 4BDAB171  bl 0x82466e20
	ctx.lr = 0x826BBCB4;
	sub_82466E20(ctx, base);
	// 826BBCB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBCB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBCBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBCC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBCC8 size=108
    let mut pc: u32 = 0x826BBCC8;
    'dispatch: loop {
        match pc {
            0x826BBCC8 => {
    //   block [0x826BBCC8..0x826BBD34)
	// 826BBCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBCCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBCD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBCD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBCD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBCDC: 38EBEAB0  addi r7, r11, -0x1550
	ctx.r[7].s64 = ctx.r[11].s64 + -5456;
	// 826BBCE0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826BBCE4: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 826BBCE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBCEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBCF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBCF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBCF8: 386A164C  addi r3, r10, 0x164c
	ctx.r[3].s64 = ctx.r[10].s64 + 5708;
	// 826BBCFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBD04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBD0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBD1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBD20: 4BDAB101  bl 0x82466e20
	ctx.lr = 0x826BBD24;
	sub_82466E20(ctx, base);
	// 826BBD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBD30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBD38 size=112
    let mut pc: u32 = 0x826BBD38;
    'dispatch: loop {
        match pc {
            0x826BBD38 => {
    //   block [0x826BBD38..0x826BBDA8)
	// 826BBD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBD44: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BBD48: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 826BBD4C: 38EAEB58  addi r7, r10, -0x14a8
	ctx.r[7].s64 = ctx.r[10].s64 + -5288;
	// 826BBD50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBD54: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BBD58: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 826BBD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBD60: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBD64: 396B0B90  addi r11, r11, 0xb90
	ctx.r[11].s64 = ctx.r[11].s64 + 2960;
	// 826BBD68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBD6C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBD70: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBD74: 386A167C  addi r3, r10, 0x167c
	ctx.r[3].s64 = ctx.r[10].s64 + 5756;
	// 826BBD78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBD7C: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826BBD80: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBD84: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826BBD88: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBD8C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBD90: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBD94: 4BDAB08D  bl 0x82466e20
	ctx.lr = 0x826BBD98;
	sub_82466E20(ctx, base);
	// 826BBD98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBDA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBDA8 size=108
    let mut pc: u32 = 0x826BBDA8;
    'dispatch: loop {
        match pc {
            0x826BBDA8 => {
    //   block [0x826BBDA8..0x826BBE14)
	// 826BBDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBDB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBDB4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBDB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBDBC: 38EBEC78  addi r7, r11, -0x1388
	ctx.r[7].s64 = ctx.r[11].s64 + -5000;
	// 826BBDC0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BBDC4: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 826BBDC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBDCC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBDD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBDD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBDD8: 386A16AC  addi r3, r10, 0x16ac
	ctx.r[3].s64 = ctx.r[10].s64 + 5804;
	// 826BBDDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBDE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBDE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBDE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBDEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBDF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBDF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBDF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBDFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBE00: 4BDAB021  bl 0x82466e20
	ctx.lr = 0x826BBE04;
	sub_82466E20(ctx, base);
	// 826BBE04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBE10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBE18 size=108
    let mut pc: u32 = 0x826BBE18;
    'dispatch: loop {
        match pc {
            0x826BBE18 => {
    //   block [0x826BBE18..0x826BBE84)
	// 826BBE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBE1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBE24: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBE28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBE2C: 38EBECD8  addi r7, r11, -0x1328
	ctx.r[7].s64 = ctx.r[11].s64 + -4904;
	// 826BBE30: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 826BBE34: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 826BBE38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBE3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBE40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBE44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBE48: 386A16DC  addi r3, r10, 0x16dc
	ctx.r[3].s64 = ctx.r[10].s64 + 5852;
	// 826BBE4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBE50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBE54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBE58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBE5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBE60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBE64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBE68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBE6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBE70: 4BDAAFB1  bl 0x82466e20
	ctx.lr = 0x826BBE74;
	sub_82466E20(ctx, base);
	// 826BBE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBE78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBE7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBE88 size=108
    let mut pc: u32 = 0x826BBE88;
    'dispatch: loop {
        match pc {
            0x826BBE88 => {
    //   block [0x826BBE88..0x826BBEF4)
	// 826BBE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBE90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBE94: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBE98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBE9C: 38EBEDE0  addi r7, r11, -0x1220
	ctx.r[7].s64 = ctx.r[11].s64 + -4640;
	// 826BBEA0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826BBEA4: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 826BBEA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBEAC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBEB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBEB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBEB8: 386A170C  addi r3, r10, 0x170c
	ctx.r[3].s64 = ctx.r[10].s64 + 5900;
	// 826BBEBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBEC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBEC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBEC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBECC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBED0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBED4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBED8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBEDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBEE0: 4BDAAF41  bl 0x82466e20
	ctx.lr = 0x826BBEE4;
	sub_82466E20(ctx, base);
	// 826BBEE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBEE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBEEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBEF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBEF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBEF8 size=108
    let mut pc: u32 = 0x826BBEF8;
    'dispatch: loop {
        match pc {
            0x826BBEF8 => {
    //   block [0x826BBEF8..0x826BBF64)
	// 826BBEF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBEFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBF00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBF04: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBF0C: 38EBEEB8  addi r7, r11, -0x1148
	ctx.r[7].s64 = ctx.r[11].s64 + -4424;
	// 826BBF10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BBF14: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 826BBF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBF1C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBF20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBF24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBF28: 386A173C  addi r3, r10, 0x173c
	ctx.r[3].s64 = ctx.r[10].s64 + 5948;
	// 826BBF2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBF34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBF3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBF4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBF50: 4BDAAED1  bl 0x82466e20
	ctx.lr = 0x826BBF54;
	sub_82466E20(ctx, base);
	// 826BBF54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBF58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBF5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBF60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBF68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBF68 size=108
    let mut pc: u32 = 0x826BBF68;
    'dispatch: loop {
        match pc {
            0x826BBF68 => {
    //   block [0x826BBF68..0x826BBFD4)
	// 826BBF68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBF6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBF70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBF74: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBF78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBF7C: 38EBEF00  addi r7, r11, -0x1100
	ctx.r[7].s64 = ctx.r[11].s64 + -4352;
	// 826BBF80: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BBF84: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 826BBF88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBF8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BBF90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BBF94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BBF98: 386A176C  addi r3, r10, 0x176c
	ctx.r[3].s64 = ctx.r[10].s64 + 5996;
	// 826BBF9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BBFA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BBFA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BBFA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BBFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BBFB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BBFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BBFB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BBFBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BBFC0: 4BDAAE61  bl 0x82466e20
	ctx.lr = 0x826BBFC4;
	sub_82466E20(ctx, base);
	// 826BBFC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BBFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BBFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BBFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BBFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BBFD8 size=108
    let mut pc: u32 = 0x826BBFD8;
    'dispatch: loop {
        match pc {
            0x826BBFD8 => {
    //   block [0x826BBFD8..0x826BC044)
	// 826BBFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BBFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BBFE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BBFE4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BBFE8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BBFEC: 38EBEF18  addi r7, r11, -0x10e8
	ctx.r[7].s64 = ctx.r[11].s64 + -4328;
	// 826BBFF0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BBFF4: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 826BBFF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BBFFC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC000: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BC004: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC008: 386A179C  addi r3, r10, 0x179c
	ctx.r[3].s64 = ctx.r[10].s64 + 6044;
	// 826BC00C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BC010: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC014: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC018: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC01C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC020: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC028: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC02C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC030: 4BDAADF1  bl 0x82466e20
	ctx.lr = 0x826BC034;
	sub_82466E20(ctx, base);
	// 826BC034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC048 size=108
    let mut pc: u32 = 0x826BC048;
    'dispatch: loop {
        match pc {
            0x826BC048 => {
    //   block [0x826BC048..0x826BC0B4)
	// 826BC048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC054: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC05C: 38EBEF60  addi r7, r11, -0x10a0
	ctx.r[7].s64 = ctx.r[11].s64 + -4256;
	// 826BC060: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC064: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 826BC068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC06C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BC074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC078: 386A17CC  addi r3, r10, 0x17cc
	ctx.r[3].s64 = ctx.r[10].s64 + 6092;
	// 826BC07C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BC080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC08C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC09C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC0A0: 4BDAAD81  bl 0x82466e20
	ctx.lr = 0x826BC0A4;
	sub_82466E20(ctx, base);
	// 826BC0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC0B8 size=112
    let mut pc: u32 = 0x826BC0B8;
    'dispatch: loop {
        match pc {
            0x826BC0B8 => {
    //   block [0x826BC0B8..0x826BC128)
	// 826BC0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC0C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC0C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC0CC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BC0D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC0D4: 390BEF78  addi r8, r11, -0x1088
	ctx.r[8].s64 = ctx.r[11].s64 + -4232;
	// 826BC0D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BC0DC: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 826BC0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC0E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC0E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC0F0: 386A17FC  addi r3, r10, 0x17fc
	ctx.r[3].s64 = ctx.r[10].s64 + 6140;
	// 826BC0F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC114: 4BDAAD0D  bl 0x82466e20
	ctx.lr = 0x826BC118;
	sub_82466E20(ctx, base);
	// 826BC118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC128 size=108
    let mut pc: u32 = 0x826BC128;
    'dispatch: loop {
        match pc {
            0x826BC128 => {
    //   block [0x826BC128..0x826BC194)
	// 826BC128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC134: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC13C: 38EBEFC0  addi r7, r11, -0x1040
	ctx.r[7].s64 = ctx.r[11].s64 + -4160;
	// 826BC140: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BC144: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 826BC148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC14C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC150: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BC154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC158: 386A182C  addi r3, r10, 0x182c
	ctx.r[3].s64 = ctx.r[10].s64 + 6188;
	// 826BC15C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BC160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC164: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC17C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC180: 4BDAACA1  bl 0x82466e20
	ctx.lr = 0x826BC184;
	sub_82466E20(ctx, base);
	// 826BC184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC198 size=112
    let mut pc: u32 = 0x826BC198;
    'dispatch: loop {
        match pc {
            0x826BC198 => {
    //   block [0x826BC198..0x826BC208)
	// 826BC198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC1A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC1A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC1A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC1AC: 38AA182C  addi r5, r10, 0x182c
	ctx.r[5].s64 = ctx.r[10].s64 + 6188;
	// 826BC1B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC1B4: 390BF020  addi r8, r11, -0xfe0
	ctx.r[8].s64 = ctx.r[11].s64 + -4064;
	// 826BC1B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BC1BC: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 826BC1C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC1C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC1C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC1CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC1D0: 386A185C  addi r3, r10, 0x185c
	ctx.r[3].s64 = ctx.r[10].s64 + 6236;
	// 826BC1D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC1D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC1DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC1E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC1E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC1E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC1F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC1F4: 4BDAAC2D  bl 0x82466e20
	ctx.lr = 0x826BC1F8;
	sub_82466E20(ctx, base);
	// 826BC1F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC208 size=96
    let mut pc: u32 = 0x826BC208;
    'dispatch: loop {
        match pc {
            0x826BC208 => {
    //   block [0x826BC208..0x826BC268)
	// 826BC208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC214: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC21C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 826BC220: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC224: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC228: 386A188C  addi r3, r10, 0x188c
	ctx.r[3].s64 = ctx.r[10].s64 + 6284;
	// 826BC22C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC234: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC23C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC248: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC24C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC254: 4BDAABCD  bl 0x82466e20
	ctx.lr = 0x826BC258;
	sub_82466E20(ctx, base);
	// 826BC258: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC25C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC260: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC268 size=112
    let mut pc: u32 = 0x826BC268;
    'dispatch: loop {
        match pc {
            0x826BC268 => {
    //   block [0x826BC268..0x826BC2D8)
	// 826BC268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC274: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC278: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC27C: 38AA356C  addi r5, r10, 0x356c
	ctx.r[5].s64 = ctx.r[10].s64 + 13676;
	// 826BC280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC284: 390BF068  addi r8, r11, -0xf98
	ctx.r[8].s64 = ctx.r[11].s64 + -3992;
	// 826BC288: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826BC28C: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 826BC290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC294: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC29C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC2A0: 386A18BC  addi r3, r10, 0x18bc
	ctx.r[3].s64 = ctx.r[10].s64 + 6332;
	// 826BC2A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC2AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC2B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC2B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC2B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC2C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC2C4: 4BDAAB5D  bl 0x82466e20
	ctx.lr = 0x826BC2C8;
	sub_82466E20(ctx, base);
	// 826BC2C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC2D8 size=96
    let mut pc: u32 = 0x826BC2D8;
    'dispatch: loop {
        match pc {
            0x826BC2D8 => {
    //   block [0x826BC2D8..0x826BC338)
	// 826BC2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC2E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC2E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC2EC: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 826BC2F0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC2F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC2F8: 386A18EC  addi r3, r10, 0x18ec
	ctx.r[3].s64 = ctx.r[10].s64 + 6380;
	// 826BC2FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC304: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC314: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC318: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC31C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC320: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC324: 4BDAAAFD  bl 0x82466e20
	ctx.lr = 0x826BC328;
	sub_82466E20(ctx, base);
	// 826BC328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC32C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC330: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC338 size=100
    let mut pc: u32 = 0x826BC338;
    'dispatch: loop {
        match pc {
            0x826BC338 => {
    //   block [0x826BC338..0x826BC39C)
	// 826BC338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC344: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC34C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BC350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC358: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 826BC35C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC364: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BC368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC36C: 386A191C  addi r3, r10, 0x191c
	ctx.r[3].s64 = ctx.r[10].s64 + 6428;
	// 826BC370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC374: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC378: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC37C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC380: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC384: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC388: 4BDAAA99  bl 0x82466e20
	ctx.lr = 0x826BC38C;
	sub_82466E20(ctx, base);
	// 826BC38C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC3A0 size=96
    let mut pc: u32 = 0x826BC3A0;
    'dispatch: loop {
        match pc {
            0x826BC3A0 => {
    //   block [0x826BC3A0..0x826BC400)
	// 826BC3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC3A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC3AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC3B4: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 826BC3B8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC3BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC3C0: 386A194C  addi r3, r10, 0x194c
	ctx.r[3].s64 = ctx.r[10].s64 + 6476;
	// 826BC3C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC3C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC3CC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC3D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC3D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC3D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC3DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC3E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC3E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC3E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC3EC: 4BDAAA35  bl 0x82466e20
	ctx.lr = 0x826BC3F0;
	sub_82466E20(ctx, base);
	// 826BC3F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC3F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC3F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC400 size=112
    let mut pc: u32 = 0x826BC400;
    'dispatch: loop {
        match pc {
            0x826BC400 => {
    //   block [0x826BC400..0x826BC470)
	// 826BC400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC40C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC410: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC414: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826BC418: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC41C: 390BF0C8  addi r8, r11, -0xf38
	ctx.r[8].s64 = ctx.r[11].s64 + -3896;
	// 826BC420: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BC424: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 826BC428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC42C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC438: 386A197C  addi r3, r10, 0x197c
	ctx.r[3].s64 = ctx.r[10].s64 + 6524;
	// 826BC43C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC440: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC448: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC44C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC450: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC454: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC45C: 4BDAA9C5  bl 0x82466e20
	ctx.lr = 0x826BC460;
	sub_82466E20(ctx, base);
	// 826BC460: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC470 size=112
    let mut pc: u32 = 0x826BC470;
    'dispatch: loop {
        match pc {
            0x826BC470 => {
    //   block [0x826BC470..0x826BC4E0)
	// 826BC470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC47C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC480: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC484: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826BC488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC48C: 390BF0F8  addi r8, r11, -0xf08
	ctx.r[8].s64 = ctx.r[11].s64 + -3848;
	// 826BC490: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC494: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 826BC498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC49C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC4A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC4A8: 386A19AC  addi r3, r10, 0x19ac
	ctx.r[3].s64 = ctx.r[10].s64 + 6572;
	// 826BC4AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC4B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC4CC: 4BDAA955  bl 0x82466e20
	ctx.lr = 0x826BC4D0;
	sub_82466E20(ctx, base);
	// 826BC4D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC4E0 size=100
    let mut pc: u32 = 0x826BC4E0;
    'dispatch: loop {
        match pc {
            0x826BC4E0 => {
    //   block [0x826BC4E0..0x826BC544)
	// 826BC4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC4EC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC4F4: 38AA191C  addi r5, r10, 0x191c
	ctx.r[5].s64 = ctx.r[10].s64 + 6428;
	// 826BC4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC4FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC500: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 826BC504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC508: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC50C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC510: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC514: 386A19DC  addi r3, r10, 0x19dc
	ctx.r[3].s64 = ctx.r[10].s64 + 6620;
	// 826BC518: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC51C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC520: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC524: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC528: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC52C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC530: 4BDAA8F1  bl 0x82466e20
	ctx.lr = 0x826BC534;
	sub_82466E20(ctx, base);
	// 826BC534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC548 size=96
    let mut pc: u32 = 0x826BC548;
    'dispatch: loop {
        match pc {
            0x826BC548 => {
    //   block [0x826BC548..0x826BC5A8)
	// 826BC548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC54C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC554: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC55C: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 826BC560: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC568: 386A1A0C  addi r3, r10, 0x1a0c
	ctx.r[3].s64 = ctx.r[10].s64 + 6668;
	// 826BC56C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC570: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC574: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC578: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC580: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC588: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC590: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC594: 4BDAA88D  bl 0x82466e20
	ctx.lr = 0x826BC598;
	sub_82466E20(ctx, base);
	// 826BC598: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC59C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC5A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC5A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC5A8 size=112
    let mut pc: u32 = 0x826BC5A8;
    'dispatch: loop {
        match pc {
            0x826BC5A8 => {
    //   block [0x826BC5A8..0x826BC618)
	// 826BC5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC5B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC5B8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC5BC: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BC5C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC5C4: 390BF110  addi r8, r11, -0xef0
	ctx.r[8].s64 = ctx.r[11].s64 + -3824;
	// 826BC5C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC5CC: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 826BC5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC5D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC5E0: 386A1A3C  addi r3, r10, 0x1a3c
	ctx.r[3].s64 = ctx.r[10].s64 + 6716;
	// 826BC5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC604: 4BDAA81D  bl 0x82466e20
	ctx.lr = 0x826BC608;
	sub_82466E20(ctx, base);
	// 826BC608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC618 size=96
    let mut pc: u32 = 0x826BC618;
    'dispatch: loop {
        match pc {
            0x826BC618 => {
    //   block [0x826BC618..0x826BC678)
	// 826BC618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC624: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC62C: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 826BC630: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC638: 386A1A6C  addi r3, r10, 0x1a6c
	ctx.r[3].s64 = ctx.r[10].s64 + 6764;
	// 826BC63C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC644: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC64C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC658: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BC65C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC660: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BC664: 4BDAA7BD  bl 0x82466e20
	ctx.lr = 0x826BC668;
	sub_82466E20(ctx, base);
	// 826BC668: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC66C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC670: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC678 size=112
    let mut pc: u32 = 0x826BC678;
    'dispatch: loop {
        match pc {
            0x826BC678 => {
    //   block [0x826BC678..0x826BC6E8)
	// 826BC678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC684: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC688: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC68C: 38AA1A6C  addi r5, r10, 0x1a6c
	ctx.r[5].s64 = ctx.r[10].s64 + 6764;
	// 826BC690: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC694: 390BF128  addi r8, r11, -0xed8
	ctx.r[8].s64 = ctx.r[11].s64 + -3800;
	// 826BC698: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC69C: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 826BC6A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC6A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC6A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC6AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC6B0: 386A1A9C  addi r3, r10, 0x1a9c
	ctx.r[3].s64 = ctx.r[10].s64 + 6812;
	// 826BC6B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC6B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC6BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC6C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC6C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC6C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC6CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC6D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC6D4: 4BDAA74D  bl 0x82466e20
	ctx.lr = 0x826BC6D8;
	sub_82466E20(ctx, base);
	// 826BC6D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC6DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC6E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC6E8 size=108
    let mut pc: u32 = 0x826BC6E8;
    'dispatch: loop {
        match pc {
            0x826BC6E8 => {
    //   block [0x826BC6E8..0x826BC754)
	// 826BC6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC6F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC6F4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC6F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC6FC: 38EBF140  addi r7, r11, -0xec0
	ctx.r[7].s64 = ctx.r[11].s64 + -3776;
	// 826BC700: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826BC704: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 826BC708: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC70C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC710: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BC714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC718: 386A1ACC  addi r3, r10, 0x1acc
	ctx.r[3].s64 = ctx.r[10].s64 + 6860;
	// 826BC71C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BC720: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC724: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC728: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC72C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC730: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC738: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC73C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BC740: 4BDAA6E1  bl 0x82466e20
	ctx.lr = 0x826BC744;
	sub_82466E20(ctx, base);
	// 826BC744: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC748: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC74C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC758 size=112
    let mut pc: u32 = 0x826BC758;
    'dispatch: loop {
        match pc {
            0x826BC758 => {
    //   block [0x826BC758..0x826BC7C8)
	// 826BC758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC75C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC764: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC768: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC76C: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BC770: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC774: 390BF1A0  addi r8, r11, -0xe60
	ctx.r[8].s64 = ctx.r[11].s64 + -3680;
	// 826BC778: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC77C: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 826BC780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC784: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC788: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC78C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC790: 386A1AFC  addi r3, r10, 0x1afc
	ctx.r[3].s64 = ctx.r[10].s64 + 6908;
	// 826BC794: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC798: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC79C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC7A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC7A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC7A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC7AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC7B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC7B4: 4BDAA66D  bl 0x82466e20
	ctx.lr = 0x826BC7B8;
	sub_82466E20(ctx, base);
	// 826BC7B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC7BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC7C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC7C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC7C8 size=112
    let mut pc: u32 = 0x826BC7C8;
    'dispatch: loop {
        match pc {
            0x826BC7C8 => {
    //   block [0x826BC7C8..0x826BC838)
	// 826BC7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC7CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC7D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC7D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC7D8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC7DC: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BC7E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC7E4: 390BF1B8  addi r8, r11, -0xe48
	ctx.r[8].s64 = ctx.r[11].s64 + -3656;
	// 826BC7E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BC7EC: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 826BC7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC7F4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC7F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC7FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC800: 386A1B2C  addi r3, r10, 0x1b2c
	ctx.r[3].s64 = ctx.r[10].s64 + 6956;
	// 826BC804: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC80C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC824: 4BDAA5FD  bl 0x82466e20
	ctx.lr = 0x826BC828;
	sub_82466E20(ctx, base);
	// 826BC828: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC82C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC830: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC838 size=112
    let mut pc: u32 = 0x826BC838;
    'dispatch: loop {
        match pc {
            0x826BC838 => {
    //   block [0x826BC838..0x826BC8A8)
	// 826BC838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC844: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC848: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC84C: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BC850: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC854: 390BF1E8  addi r8, r11, -0xe18
	ctx.r[8].s64 = ctx.r[11].s64 + -3608;
	// 826BC858: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC85C: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 826BC860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC864: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC868: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC870: 386A1B5C  addi r3, r10, 0x1b5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7004;
	// 826BC874: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC878: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC87C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC880: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC884: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC888: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC88C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC890: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC894: 4BDAA58D  bl 0x82466e20
	ctx.lr = 0x826BC898;
	sub_82466E20(ctx, base);
	// 826BC898: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC89C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC8A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC8A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC8A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC8A8 size=116
    let mut pc: u32 = 0x826BC8A8;
    'dispatch: loop {
        match pc {
            0x826BC8A8 => {
    //   block [0x826BC8A8..0x826BC91C)
	// 826BC8A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC8AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC8B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC8B4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC8B8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BC8BC: 390BF200  addi r8, r11, -0xe00
	ctx.r[8].s64 = ctx.r[11].s64 + -3584;
	// 826BC8C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC8C4: 392A0C08  addi r9, r10, 0xc08
	ctx.r[9].s64 = ctx.r[10].s64 + 3080;
	// 826BC8C8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC8CC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 826BC8D0: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BC8D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC8DC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC8E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC8EC: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BC8F0: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 826BC8F4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BC8F8: 386B1B8C  addi r3, r11, 0x1b8c
	ctx.r[3].s64 = ctx.r[11].s64 + 7052;
	// 826BC8FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BC900: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC908: 4BDAA519  bl 0x82466e20
	ctx.lr = 0x826BC90C;
	sub_82466E20(ctx, base);
	// 826BC90C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC920 size=112
    let mut pc: u32 = 0x826BC920;
    'dispatch: loop {
        match pc {
            0x826BC920 => {
    //   block [0x826BC920..0x826BC990)
	// 826BC920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC92C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC930: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC934: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BC938: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC93C: 390BF230  addi r8, r11, -0xdd0
	ctx.r[8].s64 = ctx.r[11].s64 + -3536;
	// 826BC940: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC944: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 826BC948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC94C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC958: 386A1BBC  addi r3, r10, 0x1bbc
	ctx.r[3].s64 = ctx.r[10].s64 + 7100;
	// 826BC95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC96C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BC970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC97C: 4BDAA4A5  bl 0x82466e20
	ctx.lr = 0x826BC980;
	sub_82466E20(ctx, base);
	// 826BC980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BC990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BC990 size=112
    let mut pc: u32 = 0x826BC990;
    'dispatch: loop {
        match pc {
            0x826BC990 => {
    //   block [0x826BC990..0x826BCA00)
	// 826BC990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BC994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BC998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BC99C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC9A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BC9A4: 38AA209C  addi r5, r10, 0x209c
	ctx.r[5].s64 = ctx.r[10].s64 + 8348;
	// 826BC9A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BC9AC: 390BF248  addi r8, r11, -0xdb8
	ctx.r[8].s64 = ctx.r[11].s64 + -3512;
	// 826BC9B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BC9B4: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 826BC9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BC9BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BC9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BC9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BC9C8: 386A1BEC  addi r3, r10, 0x1bec
	ctx.r[3].s64 = ctx.r[10].s64 + 7148;
	// 826BC9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BC9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BC9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BC9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BC9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BC9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BC9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BC9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BC9EC: 4BDAA435  bl 0x82466e20
	ctx.lr = 0x826BC9F0;
	sub_82466E20(ctx, base);
	// 826BC9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BC9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BC9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BC9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCA00 size=112
    let mut pc: u32 = 0x826BCA00;
    'dispatch: loop {
        match pc {
            0x826BCA00 => {
    //   block [0x826BCA00..0x826BCA70)
	// 826BCA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCA0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCA10: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCA14: 38AA1DFC  addi r5, r10, 0x1dfc
	ctx.r[5].s64 = ctx.r[10].s64 + 7676;
	// 826BCA18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCA1C: 390BF260  addi r8, r11, -0xda0
	ctx.r[8].s64 = ctx.r[11].s64 + -3488;
	// 826BCA20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BCA24: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 826BCA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCA2C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCA30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCA38: 386A1C1C  addi r3, r10, 0x1c1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7196;
	// 826BCA3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCA40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCA44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCA48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCA50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCA58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCA5C: 4BDAA3C5  bl 0x82466e20
	ctx.lr = 0x826BCA60;
	sub_82466E20(ctx, base);
	// 826BCA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCA70 size=112
    let mut pc: u32 = 0x826BCA70;
    'dispatch: loop {
        match pc {
            0x826BCA70 => {
    //   block [0x826BCA70..0x826BCAE0)
	// 826BCA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCA7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCA80: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCA84: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BCA88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCA8C: 390BF278  addi r8, r11, -0xd88
	ctx.r[8].s64 = ctx.r[11].s64 + -3464;
	// 826BCA90: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BCA94: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 826BCA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCA9C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCAA8: 386A1C4C  addi r3, r10, 0x1c4c
	ctx.r[3].s64 = ctx.r[10].s64 + 7244;
	// 826BCAAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCACC: 4BDAA355  bl 0x82466e20
	ctx.lr = 0x826BCAD0;
	sub_82466E20(ctx, base);
	// 826BCAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCAE0 size=112
    let mut pc: u32 = 0x826BCAE0;
    'dispatch: loop {
        match pc {
            0x826BCAE0 => {
    //   block [0x826BCAE0..0x826BCB50)
	// 826BCAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCAEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCAF0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCAF4: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BCAF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCAFC: 390BF2C0  addi r8, r11, -0xd40
	ctx.r[8].s64 = ctx.r[11].s64 + -3392;
	// 826BCB00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BCB04: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 826BCB08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCB0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCB14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCB18: 386A1C7C  addi r3, r10, 0x1c7c
	ctx.r[3].s64 = ctx.r[10].s64 + 7292;
	// 826BCB1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCB20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCB28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCB2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCB30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCB34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCB3C: 4BDAA2E5  bl 0x82466e20
	ctx.lr = 0x826BCB40;
	sub_82466E20(ctx, base);
	// 826BCB40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCB50 size=112
    let mut pc: u32 = 0x826BCB50;
    'dispatch: loop {
        match pc {
            0x826BCB50 => {
    //   block [0x826BCB50..0x826BCBC0)
	// 826BCB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCB58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCB5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCB60: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCB64: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BCB68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCB6C: 390BF2F0  addi r8, r11, -0xd10
	ctx.r[8].s64 = ctx.r[11].s64 + -3344;
	// 826BCB70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BCB74: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 826BCB78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCB7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCB80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCB84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCB88: 386A1CAC  addi r3, r10, 0x1cac
	ctx.r[3].s64 = ctx.r[10].s64 + 7340;
	// 826BCB8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCB90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCB98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCB9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCBA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCBA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCBA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCBAC: 4BDAA275  bl 0x82466e20
	ctx.lr = 0x826BCBB0;
	sub_82466E20(ctx, base);
	// 826BCBB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCBB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCBB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCBC0 size=108
    let mut pc: u32 = 0x826BCBC0;
    'dispatch: loop {
        match pc {
            0x826BCBC0 => {
    //   block [0x826BCBC0..0x826BCC2C)
	// 826BCBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCBC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCBCC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCBD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCBD4: 38EBF320  addi r7, r11, -0xce0
	ctx.r[7].s64 = ctx.r[11].s64 + -3296;
	// 826BCBD8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BCBDC: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 826BCBE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCBE4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCBE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BCBEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCBF0: 386A1CDC  addi r3, r10, 0x1cdc
	ctx.r[3].s64 = ctx.r[10].s64 + 7388;
	// 826BCBF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BCBF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCBFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCC00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCC04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCC08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCC0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCC10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCC14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BCC18: 4BDAA209  bl 0x82466e20
	ctx.lr = 0x826BCC1C;
	sub_82466E20(ctx, base);
	// 826BCC1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCC20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCC24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCC28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCC30 size=112
    let mut pc: u32 = 0x826BCC30;
    'dispatch: loop {
        match pc {
            0x826BCC30 => {
    //   block [0x826BCC30..0x826BCCA0)
	// 826BCC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCC3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCC40: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCC44: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BCC48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCC4C: 390BF368  addi r8, r11, -0xc98
	ctx.r[8].s64 = ctx.r[11].s64 + -3224;
	// 826BCC50: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826BCC54: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 826BCC58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCC5C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCC60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCC64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCC68: 386A1D0C  addi r3, r10, 0x1d0c
	ctx.r[3].s64 = ctx.r[10].s64 + 7436;
	// 826BCC6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCC70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCC74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCC78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCC7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCC80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCC88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCC8C: 4BDAA195  bl 0x82466e20
	ctx.lr = 0x826BCC90;
	sub_82466E20(ctx, base);
	// 826BCC90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCC94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCC98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCCA0 size=116
    let mut pc: u32 = 0x826BCCA0;
    'dispatch: loop {
        match pc {
            0x826BCCA0 => {
    //   block [0x826BCCA0..0x826BCD14)
	// 826BCCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCCA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCCAC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826BCCB0: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCCB4: 392B0C44  addi r9, r11, 0xc44
	ctx.r[9].s64 = ctx.r[11].s64 + 3140;
	// 826BCCB8: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BCCBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCCC0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826BCCC4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 826BCCC8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCCCC: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 826BCCD0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCCD4: 396BF3E8  addi r11, r11, -0xc18
	ctx.r[11].s64 = ctx.r[11].s64 + -3096;
	// 826BCCD8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826BCCDC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCCE0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826BCCE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCCE8: 386A1D3C  addi r3, r10, 0x1d3c
	ctx.r[3].s64 = ctx.r[10].s64 + 7484;
	// 826BCCEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826BCCF0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826BCCF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCCF8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826BCCFC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BCD00: 4BDAA121  bl 0x82466e20
	ctx.lr = 0x826BCD04;
	sub_82466E20(ctx, base);
	// 826BCD04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCD08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCD0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCD10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCD18 size=100
    let mut pc: u32 = 0x826BCD18;
    'dispatch: loop {
        match pc {
            0x826BCD18 => {
    //   block [0x826BCD18..0x826BCD7C)
	// 826BCD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCD1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCD20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCD24: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCD2C: 38AA1E8C  addi r5, r10, 0x1e8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7820;
	// 826BCD30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCD34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCD38: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 826BCD3C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCD40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCD44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCD48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCD4C: 386A1D6C  addi r3, r10, 0x1d6c
	ctx.r[3].s64 = ctx.r[10].s64 + 7532;
	// 826BCD50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCD54: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCD58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BCD5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCD60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BCD64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCD68: 4BDAA0B9  bl 0x82466e20
	ctx.lr = 0x826BCD6C;
	sub_82466E20(ctx, base);
	// 826BCD6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCD70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCD74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCD78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCD80 size=100
    let mut pc: u32 = 0x826BCD80;
    'dispatch: loop {
        match pc {
            0x826BCD80 => {
    //   block [0x826BCD80..0x826BCDE4)
	// 826BCD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCD88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCD8C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCD90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCD94: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BCD98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCD9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCDA0: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 826BCDA4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCDA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCDAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCDB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCDB4: 386A1D9C  addi r3, r10, 0x1d9c
	ctx.r[3].s64 = ctx.r[10].s64 + 7580;
	// 826BCDB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCDBC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCDC0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BCDC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCDC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BCDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCDD0: 4BDAA051  bl 0x82466e20
	ctx.lr = 0x826BCDD4;
	sub_82466E20(ctx, base);
	// 826BCDD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCDD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCDDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCDE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCDE8 size=108
    let mut pc: u32 = 0x826BCDE8;
    'dispatch: loop {
        match pc {
            0x826BCDE8 => {
    //   block [0x826BCDE8..0x826BCE54)
	// 826BCDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCDF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCDF4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCDF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCDFC: 38EBF478  addi r7, r11, -0xb88
	ctx.r[7].s64 = ctx.r[11].s64 + -2952;
	// 826BCE00: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BCE04: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 826BCE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCE0C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCE10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BCE14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCE18: 386A1DCC  addi r3, r10, 0x1dcc
	ctx.r[3].s64 = ctx.r[10].s64 + 7628;
	// 826BCE1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BCE20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCE24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCE28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCE2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCE30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCE38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCE3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BCE40: 4BDA9FE1  bl 0x82466e20
	ctx.lr = 0x826BCE44;
	sub_82466E20(ctx, base);
	// 826BCE44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCE48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCE4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCE50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCE58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCE58 size=112
    let mut pc: u32 = 0x826BCE58;
    'dispatch: loop {
        match pc {
            0x826BCE58 => {
    //   block [0x826BCE58..0x826BCEC8)
	// 826BCE58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCE5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCE60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCE64: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCE68: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCE6C: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BCE70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCE74: 390BF4A8  addi r8, r11, -0xb58
	ctx.r[8].s64 = ctx.r[11].s64 + -2904;
	// 826BCE78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BCE7C: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 826BCE80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCE84: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCE88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCE8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCE90: 386A1DFC  addi r3, r10, 0x1dfc
	ctx.r[3].s64 = ctx.r[10].s64 + 7676;
	// 826BCE94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BCE98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCE9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCEB4: 4BDA9F6D  bl 0x82466e20
	ctx.lr = 0x826BCEB8;
	sub_82466E20(ctx, base);
	// 826BCEB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCEBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCEC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCEC8 size=108
    let mut pc: u32 = 0x826BCEC8;
    'dispatch: loop {
        match pc {
            0x826BCEC8 => {
    //   block [0x826BCEC8..0x826BCF34)
	// 826BCEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCED4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCEDC: 38EBF4C0  addi r7, r11, -0xb40
	ctx.r[7].s64 = ctx.r[11].s64 + -2880;
	// 826BCEE0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BCEE4: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 826BCEE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCEEC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCEF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BCEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCEF8: 386A1E2C  addi r3, r10, 0x1e2c
	ctx.r[3].s64 = ctx.r[10].s64 + 7724;
	// 826BCEFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BCF00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCF04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCF08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCF0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCF10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCF14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCF18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCF1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BCF20: 4BDA9F01  bl 0x82466e20
	ctx.lr = 0x826BCF24;
	sub_82466E20(ctx, base);
	// 826BCF24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCF28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCF2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCF30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BCF38 size=28
    let mut pc: u32 = 0x826BCF38;
    'dispatch: loop {
        match pc {
            0x826BCF38 => {
    //   block [0x826BCF38..0x826BCF54)
	// 826BCF38: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCF3C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BCF40: 394A3128  addi r10, r10, 0x3128
	ctx.r[10].s64 = ctx.r[10].s64 + 12584;
	// 826BCF44: 816BF3E4  lwz r11, -0xc1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3100 as u32) ) } as u64;
	// 826BCF48: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 826BCF4C: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 826BCF50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCF58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCF58 size=108
    let mut pc: u32 = 0x826BCF58;
    'dispatch: loop {
        match pc {
            0x826BCF58 => {
    //   block [0x826BCF58..0x826BCFC4)
	// 826BCF58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCF5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCF60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCF64: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCF68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BCF6C: 38EB3128  addi r7, r11, 0x3128
	ctx.r[7].s64 = ctx.r[11].s64 + 12584;
	// 826BCF70: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 826BCF74: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 826BCF78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCF7C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCF80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BCF84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BCF88: 386A1E5C  addi r3, r10, 0x1e5c
	ctx.r[3].s64 = ctx.r[10].s64 + 7772;
	// 826BCF8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BCF90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BCF94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BCF98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCF9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BCFA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BCFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BCFA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BCFAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BCFB0: 4BDA9E71  bl 0x82466e20
	ctx.lr = 0x826BCFB4;
	sub_82466E20(ctx, base);
	// 826BCFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BCFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BCFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BCFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BCFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BCFC8 size=116
    let mut pc: u32 = 0x826BCFC8;
    'dispatch: loop {
        match pc {
            0x826BCFC8 => {
    //   block [0x826BCFC8..0x826BD03C)
	// 826BCFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BCFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BCFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BCFD4: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BCFD8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BCFDC: 390BF4E0  addi r8, r11, -0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + -2848;
	// 826BCFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BCFE4: 392A0CB4  addi r9, r10, 0xcb4
	ctx.r[9].s64 = ctx.r[10].s64 + 3252;
	// 826BCFE8: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BCFEC: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826BCFF0: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BCFF4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BCFF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BCFFC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD000: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD008: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD00C: 3D608298  lis r11, -0x7d68
	ctx.r[11].s64 = -2103967744;
	// 826BD010: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 826BD014: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BD018: 386B1E8C  addi r3, r11, 0x1e8c
	ctx.r[3].s64 = ctx.r[11].s64 + 7820;
	// 826BD01C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826BD020: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD028: 4BDA9DF9  bl 0x82466e20
	ctx.lr = 0x826BD02C;
	sub_82466E20(ctx, base);
	// 826BD02C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD040 size=112
    let mut pc: u32 = 0x826BD040;
    'dispatch: loop {
        match pc {
            0x826BD040 => {
    //   block [0x826BD040..0x826BD0B0)
	// 826BD040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD04C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD050: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD054: 38AA1B5C  addi r5, r10, 0x1b5c
	ctx.r[5].s64 = ctx.r[10].s64 + 7004;
	// 826BD058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD05C: 390BF558  addi r8, r11, -0xaa8
	ctx.r[8].s64 = ctx.r[11].s64 + -2728;
	// 826BD060: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BD064: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 826BD068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD06C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD070: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD074: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD078: 386A1EBC  addi r3, r10, 0x1ebc
	ctx.r[3].s64 = ctx.r[10].s64 + 7868;
	// 826BD07C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD084: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD08C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD09C: 4BDA9D85  bl 0x82466e20
	ctx.lr = 0x826BD0A0;
	sub_82466E20(ctx, base);
	// 826BD0A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD0A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD0A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD0AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD0B0 size=108
    let mut pc: u32 = 0x826BD0B0;
    'dispatch: loop {
        match pc {
            0x826BD0B0 => {
    //   block [0x826BD0B0..0x826BD11C)
	// 826BD0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD0BC: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD0C4: 38EBF570  addi r7, r11, -0xa90
	ctx.r[7].s64 = ctx.r[11].s64 + -2704;
	// 826BD0C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826BD0CC: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 826BD0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD0D4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD0D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BD0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD0E0: 386A1EEC  addi r3, r10, 0x1eec
	ctx.r[3].s64 = ctx.r[10].s64 + 7916;
	// 826BD0E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BD0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD0F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BD108: 4BDA9D19  bl 0x82466e20
	ctx.lr = 0x826BD10C;
	sub_82466E20(ctx, base);
	// 826BD10C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD120 size=112
    let mut pc: u32 = 0x826BD120;
    'dispatch: loop {
        match pc {
            0x826BD120 => {
    //   block [0x826BD120..0x826BD190)
	// 826BD120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD12C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD130: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD134: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BD138: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD13C: 390BF5A0  addi r8, r11, -0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + -2656;
	// 826BD140: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD144: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 826BD148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD14C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD150: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD158: 386A1F1C  addi r3, r10, 0x1f1c
	ctx.r[3].s64 = ctx.r[10].s64 + 7964;
	// 826BD15C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD160: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD164: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD168: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD16C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD170: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD178: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD17C: 4BDA9CA5  bl 0x82466e20
	ctx.lr = 0x826BD180;
	sub_82466E20(ctx, base);
	// 826BD180: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD190 size=112
    let mut pc: u32 = 0x826BD190;
    'dispatch: loop {
        match pc {
            0x826BD190 => {
    //   block [0x826BD190..0x826BD200)
	// 826BD190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD19C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD1A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD1A4: 38AA209C  addi r5, r10, 0x209c
	ctx.r[5].s64 = ctx.r[10].s64 + 8348;
	// 826BD1A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD1AC: 390BF5D0  addi r8, r11, -0xa30
	ctx.r[8].s64 = ctx.r[11].s64 + -2608;
	// 826BD1B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD1B4: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 826BD1B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD1BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD1C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD1C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD1C8: 386A1F4C  addi r3, r10, 0x1f4c
	ctx.r[3].s64 = ctx.r[10].s64 + 8012;
	// 826BD1CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD1D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD1D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD1D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD1DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD1E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD1E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD1E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD1EC: 4BDA9C35  bl 0x82466e20
	ctx.lr = 0x826BD1F0;
	sub_82466E20(ctx, base);
	// 826BD1F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD200 size=100
    let mut pc: u32 = 0x826BD200;
    'dispatch: loop {
        match pc {
            0x826BD200 => {
    //   block [0x826BD200..0x826BD264)
	// 826BD200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD20C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD214: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BD218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD21C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD220: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 826BD224: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD228: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD22C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD230: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD234: 386A1F7C  addi r3, r10, 0x1f7c
	ctx.r[3].s64 = ctx.r[10].s64 + 8060;
	// 826BD238: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD23C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD240: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BD244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD248: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BD24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD250: 4BDA9BD1  bl 0x82466e20
	ctx.lr = 0x826BD254;
	sub_82466E20(ctx, base);
	// 826BD254: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD25C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD268 size=112
    let mut pc: u32 = 0x826BD268;
    'dispatch: loop {
        match pc {
            0x826BD268 => {
    //   block [0x826BD268..0x826BD2D8)
	// 826BD268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD26C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD274: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD278: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD27C: 38AA1D9C  addi r5, r10, 0x1d9c
	ctx.r[5].s64 = ctx.r[10].s64 + 7580;
	// 826BD280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD284: 390BF600  addi r8, r11, -0xa00
	ctx.r[8].s64 = ctx.r[11].s64 + -2560;
	// 826BD288: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BD28C: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 826BD290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD294: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD29C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD2A0: 386A1FAC  addi r3, r10, 0x1fac
	ctx.r[3].s64 = ctx.r[10].s64 + 8108;
	// 826BD2A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD2AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD2B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD2B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD2B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD2C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD2C4: 4BDA9B5D  bl 0x82466e20
	ctx.lr = 0x826BD2C8;
	sub_82466E20(ctx, base);
	// 826BD2C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD2CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD2D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD2D8 size=112
    let mut pc: u32 = 0x826BD2D8;
    'dispatch: loop {
        match pc {
            0x826BD2D8 => {
    //   block [0x826BD2D8..0x826BD348)
	// 826BD2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD2E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD2E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD2E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD2EC: 38AA1D9C  addi r5, r10, 0x1d9c
	ctx.r[5].s64 = ctx.r[10].s64 + 7580;
	// 826BD2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD2F4: 390BF648  addi r8, r11, -0x9b8
	ctx.r[8].s64 = ctx.r[11].s64 + -2488;
	// 826BD2F8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826BD2FC: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 826BD300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD304: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD308: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD30C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD310: 386A1FDC  addi r3, r10, 0x1fdc
	ctx.r[3].s64 = ctx.r[10].s64 + 8156;
	// 826BD314: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD31C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD334: 4BDA9AED  bl 0x82466e20
	ctx.lr = 0x826BD338;
	sub_82466E20(ctx, base);
	// 826BD338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD348 size=108
    let mut pc: u32 = 0x826BD348;
    'dispatch: loop {
        match pc {
            0x826BD348 => {
    //   block [0x826BD348..0x826BD3B4)
	// 826BD348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD350: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD354: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD35C: 38EBF6F0  addi r7, r11, -0x910
	ctx.r[7].s64 = ctx.r[11].s64 + -2320;
	// 826BD360: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826BD364: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 826BD368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD36C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BD374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD378: 386A200C  addi r3, r10, 0x200c
	ctx.r[3].s64 = ctx.r[10].s64 + 8204;
	// 826BD37C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BD380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD38C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD394: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD39C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BD3A0: 4BDA9A81  bl 0x82466e20
	ctx.lr = 0x826BD3A4;
	sub_82466E20(ctx, base);
	// 826BD3A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD3A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD3AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD3B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD3B8 size=112
    let mut pc: u32 = 0x826BD3B8;
    'dispatch: loop {
        match pc {
            0x826BD3B8 => {
    //   block [0x826BD3B8..0x826BD428)
	// 826BD3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD3C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD3C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD3C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD3CC: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BD3D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD3D4: 390BF738  addi r8, r11, -0x8c8
	ctx.r[8].s64 = ctx.r[11].s64 + -2248;
	// 826BD3D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826BD3DC: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 826BD3E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD3E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD3E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD3EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD3F0: 386A203C  addi r3, r10, 0x203c
	ctx.r[3].s64 = ctx.r[10].s64 + 8252;
	// 826BD3F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD3F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD3FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD414: 4BDA9A0D  bl 0x82466e20
	ctx.lr = 0x826BD418;
	sub_82466E20(ctx, base);
	// 826BD418: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD41C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD428 size=100
    let mut pc: u32 = 0x826BD428;
    'dispatch: loop {
        match pc {
            0x826BD428 => {
    //   block [0x826BD428..0x826BD48C)
	// 826BD428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD42C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD434: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD43C: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BD440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD444: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD448: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 826BD44C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD45C: 386A206C  addi r3, r10, 0x206c
	ctx.r[3].s64 = ctx.r[10].s64 + 8300;
	// 826BD460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD464: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD468: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BD46C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD470: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BD474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD478: 4BDA99A9  bl 0x82466e20
	ctx.lr = 0x826BD47C;
	sub_82466E20(ctx, base);
	// 826BD47C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD490 size=100
    let mut pc: u32 = 0x826BD490;
    'dispatch: loop {
        match pc {
            0x826BD490 => {
    //   block [0x826BD490..0x826BD4F4)
	// 826BD490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD49C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD4A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD4A4: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BD4A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD4AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD4B0: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 826BD4B4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD4BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD4C4: 386A209C  addi r3, r10, 0x209c
	ctx.r[3].s64 = ctx.r[10].s64 + 8348;
	// 826BD4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD4CC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD4D0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BD4D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD4D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BD4DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD4E0: 4BDA9941  bl 0x82466e20
	ctx.lr = 0x826BD4E4;
	sub_82466E20(ctx, base);
	// 826BD4E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD4E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD4EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD4F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD4F8 size=112
    let mut pc: u32 = 0x826BD4F8;
    'dispatch: loop {
        match pc {
            0x826BD4F8 => {
    //   block [0x826BD4F8..0x826BD568)
	// 826BD4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD504: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD508: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD50C: 38AA0DAC  addi r5, r10, 0xdac
	ctx.r[5].s64 = ctx.r[10].s64 + 3500;
	// 826BD510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD514: 390BF798  addi r8, r11, -0x868
	ctx.r[8].s64 = ctx.r[11].s64 + -2152;
	// 826BD518: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826BD51C: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 826BD520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD524: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD528: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD52C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD530: 386A20CC  addi r3, r10, 0x20cc
	ctx.r[3].s64 = ctx.r[10].s64 + 8396;
	// 826BD534: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD544: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD54C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD554: 4BDA98CD  bl 0x82466e20
	ctx.lr = 0x826BD558;
	sub_82466E20(ctx, base);
	// 826BD558: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD564: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD568 size=112
    let mut pc: u32 = 0x826BD568;
    'dispatch: loop {
        match pc {
            0x826BD568 => {
    //   block [0x826BD568..0x826BD5D8)
	// 826BD568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD56C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD574: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD578: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD57C: 38AA1E8C  addi r5, r10, 0x1e8c
	ctx.r[5].s64 = ctx.r[10].s64 + 7820;
	// 826BD580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD584: 390BF828  addi r8, r11, -0x7d8
	ctx.r[8].s64 = ctx.r[11].s64 + -2008;
	// 826BD588: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BD58C: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 826BD590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD594: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD5A0: 386A20FC  addi r3, r10, 0x20fc
	ctx.r[3].s64 = ctx.r[10].s64 + 8444;
	// 826BD5A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD5A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD5B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD5B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD5B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD5BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD5C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD5C4: 4BDA985D  bl 0x82466e20
	ctx.lr = 0x826BD5C8;
	sub_82466E20(ctx, base);
	// 826BD5C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD5CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD5D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD5D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD5D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD5D8 size=112
    let mut pc: u32 = 0x826BD5D8;
    'dispatch: loop {
        match pc {
            0x826BD5D8 => {
    //   block [0x826BD5D8..0x826BD648)
	// 826BD5D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD5DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD5E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD5E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD5E8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD5EC: 38AA1FDC  addi r5, r10, 0x1fdc
	ctx.r[5].s64 = ctx.r[10].s64 + 8156;
	// 826BD5F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD5F4: 390BF840  addi r8, r11, -0x7c0
	ctx.r[8].s64 = ctx.r[11].s64 + -1984;
	// 826BD5F8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD5FC: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 826BD600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD604: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD608: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD60C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD610: 386A212C  addi r3, r10, 0x212c
	ctx.r[3].s64 = ctx.r[10].s64 + 8492;
	// 826BD614: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD62C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD634: 4BDA97ED  bl 0x82466e20
	ctx.lr = 0x826BD638;
	sub_82466E20(ctx, base);
	// 826BD638: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD63C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD640: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD648 size=112
    let mut pc: u32 = 0x826BD648;
    'dispatch: loop {
        match pc {
            0x826BD648 => {
    //   block [0x826BD648..0x826BD6B8)
	// 826BD648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD64C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD650: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD654: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD658: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD65C: 38AA1A3C  addi r5, r10, 0x1a3c
	ctx.r[5].s64 = ctx.r[10].s64 + 6716;
	// 826BD660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD664: 390BF870  addi r8, r11, -0x790
	ctx.r[8].s64 = ctx.r[11].s64 + -1936;
	// 826BD668: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BD66C: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 826BD670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD674: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD678: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD67C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD680: 386A215C  addi r3, r10, 0x215c
	ctx.r[3].s64 = ctx.r[10].s64 + 8540;
	// 826BD684: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD69C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD6A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD6A4: 4BDA977D  bl 0x82466e20
	ctx.lr = 0x826BD6A8;
	sub_82466E20(ctx, base);
	// 826BD6A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD6AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD6B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD6B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD6B8 size=112
    let mut pc: u32 = 0x826BD6B8;
    'dispatch: loop {
        match pc {
            0x826BD6B8 => {
    //   block [0x826BD6B8..0x826BD728)
	// 826BD6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD6C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD6C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD6C8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD6CC: 38AA1BEC  addi r5, r10, 0x1bec
	ctx.r[5].s64 = ctx.r[10].s64 + 7148;
	// 826BD6D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD6D4: 390BF8B8  addi r8, r11, -0x748
	ctx.r[8].s64 = ctx.r[11].s64 + -1864;
	// 826BD6D8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826BD6DC: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 826BD6E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD6E4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD6E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD6EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD6F0: 386A218C  addi r3, r10, 0x218c
	ctx.r[3].s64 = ctx.r[10].s64 + 8588;
	// 826BD6F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD6F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD70C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD714: 4BDA970D  bl 0x82466e20
	ctx.lr = 0x826BD718;
	sub_82466E20(ctx, base);
	// 826BD718: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD71C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD720: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD728 size=112
    let mut pc: u32 = 0x826BD728;
    'dispatch: loop {
        match pc {
            0x826BD728 => {
    //   block [0x826BD728..0x826BD798)
	// 826BD728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD734: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD738: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD73C: 38AA1B5C  addi r5, r10, 0x1b5c
	ctx.r[5].s64 = ctx.r[10].s64 + 7004;
	// 826BD740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD744: 390BF900  addi r8, r11, -0x700
	ctx.r[8].s64 = ctx.r[11].s64 + -1792;
	// 826BD748: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826BD74C: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 826BD750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD754: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD760: 386A21BC  addi r3, r10, 0x21bc
	ctx.r[3].s64 = ctx.r[10].s64 + 8636;
	// 826BD764: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD76C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD77C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD784: 4BDA969D  bl 0x82466e20
	ctx.lr = 0x826BD788;
	sub_82466E20(ctx, base);
	// 826BD788: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD78C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD790: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD798 size=112
    let mut pc: u32 = 0x826BD798;
    'dispatch: loop {
        match pc {
            0x826BD798 => {
    //   block [0x826BD798..0x826BD808)
	// 826BD798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD79C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD7A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD7A4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD7A8: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD7AC: 38AA1BBC  addi r5, r10, 0x1bbc
	ctx.r[5].s64 = ctx.r[10].s64 + 7100;
	// 826BD7B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD7B4: 390BF918  addi r8, r11, -0x6e8
	ctx.r[8].s64 = ctx.r[11].s64 + -1768;
	// 826BD7B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD7BC: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 826BD7C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD7C4: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD7C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD7CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD7D0: 386A21EC  addi r3, r10, 0x21ec
	ctx.r[3].s64 = ctx.r[10].s64 + 8684;
	// 826BD7D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD7D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD7DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD7E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD7E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD7E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD7F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD7F4: 4BDA962D  bl 0x82466e20
	ctx.lr = 0x826BD7F8;
	sub_82466E20(ctx, base);
	// 826BD7F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD7FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826BD808 size=24
    let mut pc: u32 = 0x826BD808;
    'dispatch: loop {
        match pc {
            0x826BD808 => {
    //   block [0x826BD808..0x826BD820)
	// 826BD808: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD80C: 3D40827E  lis r10, -0x7d82
	ctx.r[10].s64 = -2105671680;
	// 826BD810: 394A3260  addi r10, r10, 0x3260
	ctx.r[10].s64 = ctx.r[10].s64 + 12896;
	// 826BD814: 816BF4DC  lwz r11, -0xb24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2852 as u32) ) } as u64;
	// 826BD818: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 826BD81C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD820 size=112
    let mut pc: u32 = 0x826BD820;
    'dispatch: loop {
        match pc {
            0x826BD820 => {
    //   block [0x826BD820..0x826BD890)
	// 826BD820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD82C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826BD830: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD834: 392A0DD8  addi r9, r10, 0xdd8
	ctx.r[9].s64 = ctx.r[10].s64 + 3544;
	// 826BD838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD83C: 390B3260  addi r8, r11, 0x3260
	ctx.r[8].s64 = ctx.r[11].s64 + 12896;
	// 826BD840: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 826BD844: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 826BD848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD84C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD858: 386A221C  addi r3, r10, 0x221c
	ctx.r[3].s64 = ctx.r[10].s64 + 8732;
	// 826BD85C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826BD860: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826BD864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD86C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BD878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD87C: 4BDA95A5  bl 0x82466e20
	ctx.lr = 0x826BD880;
	sub_82466E20(ctx, base);
	// 826BD880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD88C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD890 size=112
    let mut pc: u32 = 0x826BD890;
    'dispatch: loop {
        match pc {
            0x826BD890 => {
    //   block [0x826BD890..0x826BD900)
	// 826BD890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD89C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD8A0: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD8A4: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BD8A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD8AC: 390BF94C  addi r8, r11, -0x6b4
	ctx.r[8].s64 = ctx.r[11].s64 + -1716;
	// 826BD8B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826BD8B4: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 826BD8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD8BC: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD8C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826BD8C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD8C8: 386A224C  addi r3, r10, 0x224c
	ctx.r[3].s64 = ctx.r[10].s64 + 8780;
	// 826BD8CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826BD8D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD8D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD8D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD8DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD8E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD8E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD8E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD8EC: 4BDA9535  bl 0x82466e20
	ctx.lr = 0x826BD8F0;
	sub_82466E20(ctx, base);
	// 826BD8F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD8FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD900 size=108
    let mut pc: u32 = 0x826BD900;
    'dispatch: loop {
        match pc {
            0x826BD900 => {
    //   block [0x826BD900..0x826BD96C)
	// 826BD900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD90C: 3D60827E  lis r11, -0x7d82
	ctx.r[11].s64 = -2105671680;
	// 826BD910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD914: 38EBF97C  addi r7, r11, -0x684
	ctx.r[7].s64 = ctx.r[11].s64 + -1668;
	// 826BD918: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826BD91C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 826BD920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD924: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826BD92C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD930: 386A227C  addi r3, r10, 0x227c
	ctx.r[3].s64 = ctx.r[10].s64 + 8828;
	// 826BD934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826BD938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD93C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD94C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826BD958: 4BDA94C9  bl 0x82466e20
	ctx.lr = 0x826BD95C;
	sub_82466E20(ctx, base);
	// 826BD95C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826BD970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826BD970 size=100
    let mut pc: u32 = 0x826BD970;
    'dispatch: loop {
        match pc {
            0x826BD970 => {
    //   block [0x826BD970..0x826BD9D4)
	// 826BD970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826BD974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826BD978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826BD97C: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD980: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826BD984: 38AA221C  addi r5, r10, 0x221c
	ctx.r[5].s64 = ctx.r[10].s64 + 8732;
	// 826BD988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826BD98C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826BD990: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 826BD994: 3D408298  lis r10, -0x7d68
	ctx.r[10].s64 = -2103967744;
	// 826BD998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826BD99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826BD9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826BD9A4: 386A22AC  addi r3, r10, 0x22ac
	ctx.r[3].s64 = ctx.r[10].s64 + 8876;
	// 826BD9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826BD9AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826BD9B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826BD9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826BD9B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826BD9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826BD9C0: 4BDA9461  bl 0x82466e20
	ctx.lr = 0x826BD9C4;
	sub_82466E20(ctx, base);
	// 826BD9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826BD9C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826BD9CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826BD9D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


