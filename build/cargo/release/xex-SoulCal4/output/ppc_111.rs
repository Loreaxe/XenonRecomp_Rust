pub fn sub_8267D630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D630 size=108
    let mut pc: u32 = 0x8267D630;
    'dispatch: loop {
        match pc {
            0x8267D630 => {
    //   block [0x8267D630..0x8267D69C)
	// 8267D630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D63C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D640: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D644: 38EB9DF0  addi r7, r11, -0x6210
	ctx.r[7].s64 = ctx.r[11].s64 + -25104;
	// 8267D648: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8267D64C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 8267D650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D654: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D660: 386A72D8  addi r3, r10, 0x72d8
	ctx.r[3].s64 = ctx.r[10].s64 + 29400;
	// 8267D664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D688: 4BDE9799  bl 0x82466e20
	ctx.lr = 0x8267D68C;
	sub_82466E20(ctx, base);
	// 8267D68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D6A0 size=108
    let mut pc: u32 = 0x8267D6A0;
    'dispatch: loop {
        match pc {
            0x8267D6A0 => {
    //   block [0x8267D6A0..0x8267D70C)
	// 8267D6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D6AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D6B4: 38EB9F10  addi r7, r11, -0x60f0
	ctx.r[7].s64 = ctx.r[11].s64 + -24816;
	// 8267D6B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D6BC: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 8267D6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D6C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D6C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D6D0: 386A7308  addi r3, r10, 0x7308
	ctx.r[3].s64 = ctx.r[10].s64 + 29448;
	// 8267D6D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D6F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D6F8: 4BDE9729  bl 0x82466e20
	ctx.lr = 0x8267D6FC;
	sub_82466E20(ctx, base);
	// 8267D6FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D710 size=108
    let mut pc: u32 = 0x8267D710;
    'dispatch: loop {
        match pc {
            0x8267D710 => {
    //   block [0x8267D710..0x8267D77C)
	// 8267D710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D71C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D720: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D724: 38EB9F28  addi r7, r11, -0x60d8
	ctx.r[7].s64 = ctx.r[11].s64 + -24792;
	// 8267D728: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D72C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 8267D730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D734: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D738: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D73C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D740: 386A7338  addi r3, r10, 0x7338
	ctx.r[3].s64 = ctx.r[10].s64 + 29496;
	// 8267D744: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D748: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D750: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D754: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D758: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D75C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D760: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D768: 4BDE96B9  bl 0x82466e20
	ctx.lr = 0x8267D76C;
	sub_82466E20(ctx, base);
	// 8267D76C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D780 size=108
    let mut pc: u32 = 0x8267D780;
    'dispatch: loop {
        match pc {
            0x8267D780 => {
    //   block [0x8267D780..0x8267D7EC)
	// 8267D780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D78C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D790: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D794: 38EB9F40  addi r7, r11, -0x60c0
	ctx.r[7].s64 = ctx.r[11].s64 + -24768;
	// 8267D798: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D79C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 8267D7A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D7A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D7A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D7AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D7B0: 386A7368  addi r3, r10, 0x7368
	ctx.r[3].s64 = ctx.r[10].s64 + 29544;
	// 8267D7B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D7B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D7BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D7C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D7C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D7C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D7CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D7D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D7D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D7D8: 4BDE9649  bl 0x82466e20
	ctx.lr = 0x8267D7DC;
	sub_82466E20(ctx, base);
	// 8267D7DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D7E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D7E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D7E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D7F0 size=108
    let mut pc: u32 = 0x8267D7F0;
    'dispatch: loop {
        match pc {
            0x8267D7F0 => {
    //   block [0x8267D7F0..0x8267D85C)
	// 8267D7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D7FC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D800: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D804: 38EB9F58  addi r7, r11, -0x60a8
	ctx.r[7].s64 = ctx.r[11].s64 + -24744;
	// 8267D808: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D80C: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 8267D810: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D818: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D81C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D820: 386A7398  addi r3, r10, 0x7398
	ctx.r[3].s64 = ctx.r[10].s64 + 29592;
	// 8267D824: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D828: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D82C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D834: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D83C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D848: 4BDE95D9  bl 0x82466e20
	ctx.lr = 0x8267D84C;
	sub_82466E20(ctx, base);
	// 8267D84C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D860 size=108
    let mut pc: u32 = 0x8267D860;
    'dispatch: loop {
        match pc {
            0x8267D860 => {
    //   block [0x8267D860..0x8267D8CC)
	// 8267D860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D864: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D86C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D870: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D874: 38EB9F70  addi r7, r11, -0x6090
	ctx.r[7].s64 = ctx.r[11].s64 + -24720;
	// 8267D878: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D87C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 8267D880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D884: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D888: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D88C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D890: 386A73C8  addi r3, r10, 0x73c8
	ctx.r[3].s64 = ctx.r[10].s64 + 29640;
	// 8267D894: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D898: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D89C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D8A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D8A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D8AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D8B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D8B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D8B8: 4BDE9569  bl 0x82466e20
	ctx.lr = 0x8267D8BC;
	sub_82466E20(ctx, base);
	// 8267D8BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D8C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D8C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D8C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D8D0 size=108
    let mut pc: u32 = 0x8267D8D0;
    'dispatch: loop {
        match pc {
            0x8267D8D0 => {
    //   block [0x8267D8D0..0x8267D93C)
	// 8267D8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D8D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D8D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D8DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D8E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D8E4: 38EB9F88  addi r7, r11, -0x6078
	ctx.r[7].s64 = ctx.r[11].s64 + -24696;
	// 8267D8E8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267D8EC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 8267D8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D8F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D8F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D8FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D900: 386A73F8  addi r3, r10, 0x73f8
	ctx.r[3].s64 = ctx.r[10].s64 + 29688;
	// 8267D904: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D908: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D90C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D910: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D918: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D920: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D928: 4BDE94F9  bl 0x82466e20
	ctx.lr = 0x8267D92C;
	sub_82466E20(ctx, base);
	// 8267D92C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D940 size=108
    let mut pc: u32 = 0x8267D940;
    'dispatch: loop {
        match pc {
            0x8267D940 => {
    //   block [0x8267D940..0x8267D9AC)
	// 8267D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D94C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D954: 38EB9FA0  addi r7, r11, -0x6060
	ctx.r[7].s64 = ctx.r[11].s64 + -24672;
	// 8267D958: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8267D95C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8267D960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D964: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D968: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D970: 386A7428  addi r3, r10, 0x7428
	ctx.r[3].s64 = ctx.r[10].s64 + 29736;
	// 8267D974: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D97C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267D990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267D994: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267D998: 4BDE9489  bl 0x82466e20
	ctx.lr = 0x8267D99C;
	sub_82466E20(ctx, base);
	// 8267D99C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267D9A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267D9A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267D9A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267D9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267D9B0 size=108
    let mut pc: u32 = 0x8267D9B0;
    'dispatch: loop {
        match pc {
            0x8267D9B0 => {
    //   block [0x8267D9B0..0x8267DA1C)
	// 8267D9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267D9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267D9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267D9BC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267D9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267D9C4: 38EBA030  addi r7, r11, -0x5fd0
	ctx.r[7].s64 = ctx.r[11].s64 + -24528;
	// 8267D9C8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8267D9CC: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8267D9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267D9D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267D9D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267D9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267D9E0: 386A7458  addi r3, r10, 0x7458
	ctx.r[3].s64 = ctx.r[10].s64 + 29784;
	// 8267D9E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267D9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267D9EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267D9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267D9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267D9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267D9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DA04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DA08: 4BDE9419  bl 0x82466e20
	ctx.lr = 0x8267DA0C;
	sub_82466E20(ctx, base);
	// 8267DA0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DA10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DA14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DA18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DA20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DA20 size=108
    let mut pc: u32 = 0x8267DA20;
    'dispatch: loop {
        match pc {
            0x8267DA20 => {
    //   block [0x8267DA20..0x8267DA8C)
	// 8267DA20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DA24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DA28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DA2C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DA34: 38EBA0F0  addi r7, r11, -0x5f10
	ctx.r[7].s64 = ctx.r[11].s64 + -24336;
	// 8267DA38: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8267DA3C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8267DA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DA44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DA48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DA4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DA50: 386A7488  addi r3, r10, 0x7488
	ctx.r[3].s64 = ctx.r[10].s64 + 29832;
	// 8267DA54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DA5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DA74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DA78: 4BDE93A9  bl 0x82466e20
	ctx.lr = 0x8267DA7C;
	sub_82466E20(ctx, base);
	// 8267DA7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DA90 size=108
    let mut pc: u32 = 0x8267DA90;
    'dispatch: loop {
        match pc {
            0x8267DA90 => {
    //   block [0x8267DA90..0x8267DAFC)
	// 8267DA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DA98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DA9C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DAA4: 38EBA1C8  addi r7, r11, -0x5e38
	ctx.r[7].s64 = ctx.r[11].s64 + -24120;
	// 8267DAA8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8267DAAC: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8267DAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DAB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DAB8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DABC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DAC0: 386A74B8  addi r3, r10, 0x74b8
	ctx.r[3].s64 = ctx.r[10].s64 + 29880;
	// 8267DAC4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DACC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DAE4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DAE8: 4BDE9339  bl 0x82466e20
	ctx.lr = 0x8267DAEC;
	sub_82466E20(ctx, base);
	// 8267DAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DB00 size=108
    let mut pc: u32 = 0x8267DB00;
    'dispatch: loop {
        match pc {
            0x8267DB00 => {
    //   block [0x8267DB00..0x8267DB6C)
	// 8267DB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DB0C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DB10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DB14: 38EBA288  addi r7, r11, -0x5d78
	ctx.r[7].s64 = ctx.r[11].s64 + -23928;
	// 8267DB18: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8267DB1C: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8267DB20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DB24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DB28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DB2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DB30: 386A74E8  addi r3, r10, 0x74e8
	ctx.r[3].s64 = ctx.r[10].s64 + 29928;
	// 8267DB34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DB38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DB3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DB40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DB48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DB4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DB50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DB54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DB58: 4BDE92C9  bl 0x82466e20
	ctx.lr = 0x8267DB5C;
	sub_82466E20(ctx, base);
	// 8267DB5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DB60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DB64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DB68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DB70 size=112
    let mut pc: u32 = 0x8267DB70;
    'dispatch: loop {
        match pc {
            0x8267DB70 => {
    //   block [0x8267DB70..0x8267DBE0)
	// 8267DB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DB74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DB7C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267DB80: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 8267DB84: 38EAA330  addi r7, r10, -0x5cd0
	ctx.r[7].s64 = ctx.r[10].s64 + -23760;
	// 8267DB88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DB8C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267DB90: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8267DB94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DB98: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DB9C: 396B4448  addi r11, r11, 0x4448
	ctx.r[11].s64 = ctx.r[11].s64 + 17480;
	// 8267DBA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DBA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DBA8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DBAC: 386A7518  addi r3, r10, 0x7518
	ctx.r[3].s64 = ctx.r[10].s64 + 29976;
	// 8267DBB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DBB4: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267DBB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DBBC: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267DBC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DBC4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DBC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DBCC: 4BDE9255  bl 0x82466e20
	ctx.lr = 0x8267DBD0;
	sub_82466E20(ctx, base);
	// 8267DBD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DBD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DBD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DBDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DBE0 size=108
    let mut pc: u32 = 0x8267DBE0;
    'dispatch: loop {
        match pc {
            0x8267DBE0 => {
    //   block [0x8267DBE0..0x8267DC4C)
	// 8267DBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DBE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DBEC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DBF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DBF4: 38EBA450  addi r7, r11, -0x5bb0
	ctx.r[7].s64 = ctx.r[11].s64 + -23472;
	// 8267DBF8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267DBFC: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8267DC00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DC04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DC08: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DC0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DC10: 386A7548  addi r3, r10, 0x7548
	ctx.r[3].s64 = ctx.r[10].s64 + 30024;
	// 8267DC14: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DC1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DC20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DC28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DC30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DC34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DC38: 4BDE91E9  bl 0x82466e20
	ctx.lr = 0x8267DC3C;
	sub_82466E20(ctx, base);
	// 8267DC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DC50 size=108
    let mut pc: u32 = 0x8267DC50;
    'dispatch: loop {
        match pc {
            0x8267DC50 => {
    //   block [0x8267DC50..0x8267DCBC)
	// 8267DC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DC54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DC5C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DC60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DC64: 38EBA4B0  addi r7, r11, -0x5b50
	ctx.r[7].s64 = ctx.r[11].s64 + -23376;
	// 8267DC68: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8267DC6C: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8267DC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DC74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DC78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DC80: 386A7578  addi r3, r10, 0x7578
	ctx.r[3].s64 = ctx.r[10].s64 + 30072;
	// 8267DC84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DC88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DC8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DC90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DC98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DCA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DCA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DCA8: 4BDE9179  bl 0x82466e20
	ctx.lr = 0x8267DCAC;
	sub_82466E20(ctx, base);
	// 8267DCAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DCB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DCB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DCB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DCC0 size=108
    let mut pc: u32 = 0x8267DCC0;
    'dispatch: loop {
        match pc {
            0x8267DCC0 => {
    //   block [0x8267DCC0..0x8267DD2C)
	// 8267DCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DCC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DCCC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DCD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DCD4: 38EBA5B8  addi r7, r11, -0x5a48
	ctx.r[7].s64 = ctx.r[11].s64 + -23112;
	// 8267DCD8: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8267DCDC: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8267DCE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DCE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DCE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DCEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DCF0: 386A75A8  addi r3, r10, 0x75a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30120;
	// 8267DCF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DCF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DCFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DD00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DD04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DD08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DD10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DD14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DD18: 4BDE9109  bl 0x82466e20
	ctx.lr = 0x8267DD1C;
	sub_82466E20(ctx, base);
	// 8267DD1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DD20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DD24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DD28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DD30 size=108
    let mut pc: u32 = 0x8267DD30;
    'dispatch: loop {
        match pc {
            0x8267DD30 => {
    //   block [0x8267DD30..0x8267DD9C)
	// 8267DD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DD3C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DD40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DD44: 38EBA690  addi r7, r11, -0x5970
	ctx.r[7].s64 = ctx.r[11].s64 + -22896;
	// 8267DD48: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267DD4C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8267DD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DD54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DD58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DD60: 386A75D8  addi r3, r10, 0x75d8
	ctx.r[3].s64 = ctx.r[10].s64 + 30168;
	// 8267DD64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DD68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DD6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DD70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DD78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DD7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DD80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DD84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DD88: 4BDE9099  bl 0x82466e20
	ctx.lr = 0x8267DD8C;
	sub_82466E20(ctx, base);
	// 8267DD8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DD90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DD94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DD98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DDA0 size=108
    let mut pc: u32 = 0x8267DDA0;
    'dispatch: loop {
        match pc {
            0x8267DDA0 => {
    //   block [0x8267DDA0..0x8267DE0C)
	// 8267DDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DDA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DDA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DDAC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DDB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DDB4: 38EBA6D8  addi r7, r11, -0x5928
	ctx.r[7].s64 = ctx.r[11].s64 + -22824;
	// 8267DDB8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267DDBC: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8267DDC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DDC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DDC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DDCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DDD0: 386A7608  addi r3, r10, 0x7608
	ctx.r[3].s64 = ctx.r[10].s64 + 30216;
	// 8267DDD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DDD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DDDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DDE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DDE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DDEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DDF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DDF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DDF8: 4BDE9029  bl 0x82466e20
	ctx.lr = 0x8267DDFC;
	sub_82466E20(ctx, base);
	// 8267DDFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DE00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DE04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DE08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DE10 size=108
    let mut pc: u32 = 0x8267DE10;
    'dispatch: loop {
        match pc {
            0x8267DE10 => {
    //   block [0x8267DE10..0x8267DE7C)
	// 8267DE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DE14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DE1C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DE20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DE24: 38EBA6F0  addi r7, r11, -0x5910
	ctx.r[7].s64 = ctx.r[11].s64 + -22800;
	// 8267DE28: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267DE2C: 388A37C4  addi r4, r10, 0x37c4
	ctx.r[4].s64 = ctx.r[10].s64 + 14276;
	// 8267DE30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DE34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DE38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DE3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DE40: 386A7638  addi r3, r10, 0x7638
	ctx.r[3].s64 = ctx.r[10].s64 + 30264;
	// 8267DE44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DE48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DE4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DE50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DE58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DE5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DE60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DE64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DE68: 4BDE8FB9  bl 0x82466e20
	ctx.lr = 0x8267DE6C;
	sub_82466E20(ctx, base);
	// 8267DE6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DE70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DE74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DE78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DE80 size=108
    let mut pc: u32 = 0x8267DE80;
    'dispatch: loop {
        match pc {
            0x8267DE80 => {
    //   block [0x8267DE80..0x8267DEEC)
	// 8267DE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DE88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DE8C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DE94: 38EBA738  addi r7, r11, -0x58c8
	ctx.r[7].s64 = ctx.r[11].s64 + -22728;
	// 8267DE98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267DE9C: 388A37F4  addi r4, r10, 0x37f4
	ctx.r[4].s64 = ctx.r[10].s64 + 14324;
	// 8267DEA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DEA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DEA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DEAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DEB0: 386A7668  addi r3, r10, 0x7668
	ctx.r[3].s64 = ctx.r[10].s64 + 30312;
	// 8267DEB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DEB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DEBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DEC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DEC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DED8: 4BDE8F49  bl 0x82466e20
	ctx.lr = 0x8267DEDC;
	sub_82466E20(ctx, base);
	// 8267DEDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DEE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DEE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DEE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DEF0 size=112
    let mut pc: u32 = 0x8267DEF0;
    'dispatch: loop {
        match pc {
            0x8267DEF0 => {
    //   block [0x8267DEF0..0x8267DF60)
	// 8267DEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DEF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DEFC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DF00: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DF04: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267DF08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DF0C: 390BA750  addi r8, r11, -0x58b0
	ctx.r[8].s64 = ctx.r[11].s64 + -22704;
	// 8267DF10: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267DF14: 388A3810  addi r4, r10, 0x3810
	ctx.r[4].s64 = ctx.r[10].s64 + 14352;
	// 8267DF18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DF1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DF20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267DF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DF28: 386A7698  addi r3, r10, 0x7698
	ctx.r[3].s64 = ctx.r[10].s64 + 30360;
	// 8267DF2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267DF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DF34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DF38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DF3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DF40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DF44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DF48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DF4C: 4BDE8ED5  bl 0x82466e20
	ctx.lr = 0x8267DF50;
	sub_82466E20(ctx, base);
	// 8267DF50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DF54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DF58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DF60 size=108
    let mut pc: u32 = 0x8267DF60;
    'dispatch: loop {
        match pc {
            0x8267DF60 => {
    //   block [0x8267DF60..0x8267DFCC)
	// 8267DF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DF68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DF6C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DF70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267DF74: 38EBA798  addi r7, r11, -0x5868
	ctx.r[7].s64 = ctx.r[11].s64 + -22632;
	// 8267DF78: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267DF7C: 388A3A30  addi r4, r10, 0x3a30
	ctx.r[4].s64 = ctx.r[10].s64 + 14896;
	// 8267DF80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DF84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DF88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267DF8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267DF90: 386A76C8  addi r3, r10, 0x76c8
	ctx.r[3].s64 = ctx.r[10].s64 + 30408;
	// 8267DF94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267DF98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267DF9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267DFA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267DFA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267DFA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267DFAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267DFB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267DFB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267DFB8: 4BDE8E69  bl 0x82466e20
	ctx.lr = 0x8267DFBC;
	sub_82466E20(ctx, base);
	// 8267DFBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267DFC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267DFC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267DFC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267DFD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267DFD0 size=116
    let mut pc: u32 = 0x8267DFD0;
    'dispatch: loop {
        match pc {
            0x8267DFD0 => {
    //   block [0x8267DFD0..0x8267E044)
	// 8267DFD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267DFD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267DFD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267DFDC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267DFE0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267DFE4: 390BA7F8  addi r8, r11, -0x5808
	ctx.r[8].s64 = ctx.r[11].s64 + -22536;
	// 8267DFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267DFEC: 392A44C0  addi r9, r10, 0x44c0
	ctx.r[9].s64 = ctx.r[10].s64 + 17600;
	// 8267DFF0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267DFF4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267DFF8: 38AA76C8  addi r5, r10, 0x76c8
	ctx.r[5].s64 = ctx.r[10].s64 + 30408;
	// 8267DFFC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E004: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E00C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E014: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267E018: 388A3A3C  addi r4, r10, 0x3a3c
	ctx.r[4].s64 = ctx.r[10].s64 + 14908;
	// 8267E01C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267E020: 386B76F8  addi r3, r11, 0x76f8
	ctx.r[3].s64 = ctx.r[11].s64 + 30456;
	// 8267E024: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E028: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E02C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E030: 4BDE8DF1  bl 0x82466e20
	ctx.lr = 0x8267E034;
	sub_82466E20(ctx, base);
	// 8267E034: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E03C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E048 size=96
    let mut pc: u32 = 0x8267E048;
    'dispatch: loop {
        match pc {
            0x8267E048 => {
    //   block [0x8267E048..0x8267E0A8)
	// 8267E048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E04C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E054: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E05C: 388A3A4C  addi r4, r10, 0x3a4c
	ctx.r[4].s64 = ctx.r[10].s64 + 14924;
	// 8267E060: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E064: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E068: 386A7728  addi r3, r10, 0x7728
	ctx.r[3].s64 = ctx.r[10].s64 + 30504;
	// 8267E06C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E070: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E074: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E078: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E07C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E080: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E088: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E08C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E090: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E094: 4BDE8D8D  bl 0x82466e20
	ctx.lr = 0x8267E098;
	sub_82466E20(ctx, base);
	// 8267E098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E0A8 size=112
    let mut pc: u32 = 0x8267E0A8;
    'dispatch: loop {
        match pc {
            0x8267E0A8 => {
    //   block [0x8267E0A8..0x8267E118)
	// 8267E0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E0B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267E0B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E0BC: 38AA9738  addi r5, r10, -0x68c8
	ctx.r[5].s64 = ctx.r[10].s64 + -26824;
	// 8267E0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E0C4: 390BA870  addi r8, r11, -0x5790
	ctx.r[8].s64 = ctx.r[11].s64 + -22416;
	// 8267E0C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267E0CC: 388A3A64  addi r4, r10, 0x3a64
	ctx.r[4].s64 = ctx.r[10].s64 + 14948;
	// 8267E0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E0D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E0D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E0DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E0E0: 386A7758  addi r3, r10, 0x7758
	ctx.r[3].s64 = ctx.r[10].s64 + 30552;
	// 8267E0E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E0EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E0F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E104: 4BDE8D1D  bl 0x82466e20
	ctx.lr = 0x8267E108;
	sub_82466E20(ctx, base);
	// 8267E108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E10C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E118 size=96
    let mut pc: u32 = 0x8267E118;
    'dispatch: loop {
        match pc {
            0x8267E118 => {
    //   block [0x8267E118..0x8267E178)
	// 8267E118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E124: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E128: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E12C: 388A3A7C  addi r4, r10, 0x3a7c
	ctx.r[4].s64 = ctx.r[10].s64 + 14972;
	// 8267E130: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E138: 386A7788  addi r3, r10, 0x7788
	ctx.r[3].s64 = ctx.r[10].s64 + 30600;
	// 8267E13C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E140: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E144: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E148: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E14C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E150: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E154: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E158: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E15C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E160: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E164: 4BDE8CBD  bl 0x82466e20
	ctx.lr = 0x8267E168;
	sub_82466E20(ctx, base);
	// 8267E168: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267E178 size=24
    let mut pc: u32 = 0x8267E178;
    'dispatch: loop {
        match pc {
            0x8267E178 => {
    //   block [0x8267E178..0x8267E190)
	// 8267E178: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E17C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267E180: 394AEE70  addi r10, r10, -0x1190
	ctx.r[10].s64 = ctx.r[10].s64 + -4496;
	// 8267E184: 816BA8D0  lwz r11, -0x5730(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22320 as u32) ) } as u64;
	// 8267E188: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267E18C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E190 size=116
    let mut pc: u32 = 0x8267E190;
    'dispatch: loop {
        match pc {
            0x8267E190 => {
    //   block [0x8267E190..0x8267E204)
	// 8267E190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E19C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E1A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267E1A4: 390BEE70  addi r8, r11, -0x1190
	ctx.r[8].s64 = ctx.r[11].s64 + -4496;
	// 8267E1A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E1AC: 392A44FC  addi r9, r10, 0x44fc
	ctx.r[9].s64 = ctx.r[10].s64 + 17660;
	// 8267E1B0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E1B4: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267E1B8: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267E1BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E1C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E1C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E1C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E1CC: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267E1D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E1D4: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267E1D8: 388A3A9C  addi r4, r10, 0x3a9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15004;
	// 8267E1DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267E1E0: 386B77B8  addi r3, r11, 0x77b8
	ctx.r[3].s64 = ctx.r[11].s64 + 30648;
	// 8267E1E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E1E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E1EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E1F0: 4BDE8C31  bl 0x82466e20
	ctx.lr = 0x8267E1F4;
	sub_82466E20(ctx, base);
	// 8267E1F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E1F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E1FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E208 size=104
    let mut pc: u32 = 0x8267E208;
    'dispatch: loop {
        match pc {
            0x8267E208 => {
    //   block [0x8267E208..0x8267E270)
	// 8267E208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E214: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267E218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E21C: 392A4528  addi r9, r10, 0x4528
	ctx.r[9].s64 = ctx.r[10].s64 + 17704;
	// 8267E220: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E224: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E228: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267E22C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E230: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E238: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E23C: 388A3AB0  addi r4, r10, 0x3ab0
	ctx.r[4].s64 = ctx.r[10].s64 + 15024;
	// 8267E240: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E244: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E248: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E254: 386A77E8  addi r3, r10, 0x77e8
	ctx.r[3].s64 = ctx.r[10].s64 + 30696;
	// 8267E258: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E25C: 4BDE8BC5  bl 0x82466e20
	ctx.lr = 0x8267E260;
	sub_82466E20(ctx, base);
	// 8267E260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E270 size=96
    let mut pc: u32 = 0x8267E270;
    'dispatch: loop {
        match pc {
            0x8267E270 => {
    //   block [0x8267E270..0x8267E2D0)
	// 8267E270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E27C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E284: 388A3AC4  addi r4, r10, 0x3ac4
	ctx.r[4].s64 = ctx.r[10].s64 + 15044;
	// 8267E288: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E28C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E290: 386A7818  addi r3, r10, 0x7818
	ctx.r[3].s64 = ctx.r[10].s64 + 30744;
	// 8267E294: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E298: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E29C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E2A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E2A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E2AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E2B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E2B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E2B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E2BC: 4BDE8B65  bl 0x82466e20
	ctx.lr = 0x8267E2C0;
	sub_82466E20(ctx, base);
	// 8267E2C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E2C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E2C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E2CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E2D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E2D0 size=100
    let mut pc: u32 = 0x8267E2D0;
    'dispatch: loop {
        match pc {
            0x8267E2D0 => {
    //   block [0x8267E2D0..0x8267E334)
	// 8267E2D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E2D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E2D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E2DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E2E4: 38AA77E8  addi r5, r10, 0x77e8
	ctx.r[5].s64 = ctx.r[10].s64 + 30696;
	// 8267E2E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E2EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E2F0: 388A3ADC  addi r4, r10, 0x3adc
	ctx.r[4].s64 = ctx.r[10].s64 + 15068;
	// 8267E2F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E2F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E2FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E300: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E304: 386A7848  addi r3, r10, 0x7848
	ctx.r[3].s64 = ctx.r[10].s64 + 30792;
	// 8267E308: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E30C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E310: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E318: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E31C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E320: 4BDE8B01  bl 0x82466e20
	ctx.lr = 0x8267E324;
	sub_82466E20(ctx, base);
	// 8267E324: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E32C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E338 size=112
    let mut pc: u32 = 0x8267E338;
    'dispatch: loop {
        match pc {
            0x8267E338 => {
    //   block [0x8267E338..0x8267E3A8)
	// 8267E338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E33C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E344: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E348: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E34C: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 8267E350: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E354: 390BA8D8  addi r8, r11, -0x5728
	ctx.r[8].s64 = ctx.r[11].s64 + -22312;
	// 8267E358: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267E35C: 388A3AF8  addi r4, r10, 0x3af8
	ctx.r[4].s64 = ctx.r[10].s64 + 15096;
	// 8267E360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E364: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E368: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E370: 386A7878  addi r3, r10, 0x7878
	ctx.r[3].s64 = ctx.r[10].s64 + 30840;
	// 8267E374: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E378: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E37C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E380: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E388: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E38C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E390: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E394: 4BDE8A8D  bl 0x82466e20
	ctx.lr = 0x8267E398;
	sub_82466E20(ctx, base);
	// 8267E398: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E39C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E3A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E3A8 size=112
    let mut pc: u32 = 0x8267E3A8;
    'dispatch: loop {
        match pc {
            0x8267E3A8 => {
    //   block [0x8267E3A8..0x8267E418)
	// 8267E3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E3AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E3B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E3B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E3B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E3BC: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 8267E3C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E3C4: 390BA920  addi r8, r11, -0x56e0
	ctx.r[8].s64 = ctx.r[11].s64 + -22240;
	// 8267E3C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267E3CC: 388A3B08  addi r4, r10, 0x3b08
	ctx.r[4].s64 = ctx.r[10].s64 + 15112;
	// 8267E3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E3D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E3D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E3DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E3E0: 386A78A8  addi r3, r10, 0x78a8
	ctx.r[3].s64 = ctx.r[10].s64 + 30888;
	// 8267E3E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E3EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E3F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E3F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E3FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E404: 4BDE8A1D  bl 0x82466e20
	ctx.lr = 0x8267E408;
	sub_82466E20(ctx, base);
	// 8267E408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E40C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E414: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E418 size=100
    let mut pc: u32 = 0x8267E418;
    'dispatch: loop {
        match pc {
            0x8267E418 => {
    //   block [0x8267E418..0x8267E47C)
	// 8267E418: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E41C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E420: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E424: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E428: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E42C: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 8267E430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E434: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E438: 388A3B20  addi r4, r10, 0x3b20
	ctx.r[4].s64 = ctx.r[10].s64 + 15136;
	// 8267E43C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E440: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E444: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E448: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E44C: 386A78D8  addi r3, r10, 0x78d8
	ctx.r[3].s64 = ctx.r[10].s64 + 30936;
	// 8267E450: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E454: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E458: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E45C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E460: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E464: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E468: 4BDE89B9  bl 0x82466e20
	ctx.lr = 0x8267E46C;
	sub_82466E20(ctx, base);
	// 8267E46C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E480 size=96
    let mut pc: u32 = 0x8267E480;
    'dispatch: loop {
        match pc {
            0x8267E480 => {
    //   block [0x8267E480..0x8267E4E0)
	// 8267E480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E488: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E48C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E494: 388A3B38  addi r4, r10, 0x3b38
	ctx.r[4].s64 = ctx.r[10].s64 + 15160;
	// 8267E498: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E49C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E4A0: 386A7908  addi r3, r10, 0x7908
	ctx.r[3].s64 = ctx.r[10].s64 + 30984;
	// 8267E4A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E4A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E4AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E4B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E4B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E4B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E4BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E4C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E4C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E4C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E4CC: 4BDE8955  bl 0x82466e20
	ctx.lr = 0x8267E4D0;
	sub_82466E20(ctx, base);
	// 8267E4D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E4DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E4E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E4E0 size=112
    let mut pc: u32 = 0x8267E4E0;
    'dispatch: loop {
        match pc {
            0x8267E4E0 => {
    //   block [0x8267E4E0..0x8267E550)
	// 8267E4E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E4E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E4E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E4EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E4F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E4F4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267E4F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E4FC: 390BA938  addi r8, r11, -0x56c8
	ctx.r[8].s64 = ctx.r[11].s64 + -22216;
	// 8267E500: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267E504: 388A3B54  addi r4, r10, 0x3b54
	ctx.r[4].s64 = ctx.r[10].s64 + 15188;
	// 8267E508: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E50C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E510: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E514: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E518: 386A7938  addi r3, r10, 0x7938
	ctx.r[3].s64 = ctx.r[10].s64 + 31032;
	// 8267E51C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E520: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E524: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E528: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E52C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E530: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E534: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E538: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E53C: 4BDE88E5  bl 0x82466e20
	ctx.lr = 0x8267E540;
	sub_82466E20(ctx, base);
	// 8267E540: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E544: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E548: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E54C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E550 size=96
    let mut pc: u32 = 0x8267E550;
    'dispatch: loop {
        match pc {
            0x8267E550 => {
    //   block [0x8267E550..0x8267E5B0)
	// 8267E550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E55C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E564: 388A3B5C  addi r4, r10, 0x3b5c
	ctx.r[4].s64 = ctx.r[10].s64 + 15196;
	// 8267E568: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E56C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E570: 386A7968  addi r3, r10, 0x7968
	ctx.r[3].s64 = ctx.r[10].s64 + 31080;
	// 8267E574: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E57C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E584: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E58C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E590: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E594: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E598: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E59C: 4BDE8885  bl 0x82466e20
	ctx.lr = 0x8267E5A0;
	sub_82466E20(ctx, base);
	// 8267E5A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E5A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E5A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E5AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E5B0 size=112
    let mut pc: u32 = 0x8267E5B0;
    'dispatch: loop {
        match pc {
            0x8267E5B0 => {
    //   block [0x8267E5B0..0x8267E620)
	// 8267E5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E5B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E5BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E5C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E5C4: 38AA7968  addi r5, r10, 0x7968
	ctx.r[5].s64 = ctx.r[10].s64 + 31080;
	// 8267E5C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E5CC: 390BA968  addi r8, r11, -0x5698
	ctx.r[8].s64 = ctx.r[11].s64 + -22168;
	// 8267E5D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267E5D4: 388A3B70  addi r4, r10, 0x3b70
	ctx.r[4].s64 = ctx.r[10].s64 + 15216;
	// 8267E5D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E5DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E5E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E5E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E5E8: 386A7998  addi r3, r10, 0x7998
	ctx.r[3].s64 = ctx.r[10].s64 + 31128;
	// 8267E5EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E5F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E5F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E5F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E5FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E60C: 4BDE8815  bl 0x82466e20
	ctx.lr = 0x8267E610;
	sub_82466E20(ctx, base);
	// 8267E610: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E61C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E620 size=108
    let mut pc: u32 = 0x8267E620;
    'dispatch: loop {
        match pc {
            0x8267E620 => {
    //   block [0x8267E620..0x8267E68C)
	// 8267E620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E628: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E62C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E630: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E634: 38EBA980  addi r7, r11, -0x5680
	ctx.r[7].s64 = ctx.r[11].s64 + -22144;
	// 8267E638: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267E63C: 388A3B88  addi r4, r10, 0x3b88
	ctx.r[4].s64 = ctx.r[10].s64 + 15240;
	// 8267E640: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E644: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E648: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267E64C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E650: 386A79C8  addi r3, r10, 0x79c8
	ctx.r[3].s64 = ctx.r[10].s64 + 31176;
	// 8267E654: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267E658: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E65C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E660: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E668: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E670: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E674: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267E678: 4BDE87A9  bl 0x82466e20
	ctx.lr = 0x8267E67C;
	sub_82466E20(ctx, base);
	// 8267E67C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E690 size=112
    let mut pc: u32 = 0x8267E690;
    'dispatch: loop {
        match pc {
            0x8267E690 => {
    //   block [0x8267E690..0x8267E700)
	// 8267E690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E69C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E6A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E6A4: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267E6A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E6AC: 390BA9E0  addi r8, r11, -0x5620
	ctx.r[8].s64 = ctx.r[11].s64 + -22048;
	// 8267E6B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267E6B4: 388A3B9C  addi r4, r10, 0x3b9c
	ctx.r[4].s64 = ctx.r[10].s64 + 15260;
	// 8267E6B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E6BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E6C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E6C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E6C8: 386A79F8  addi r3, r10, 0x79f8
	ctx.r[3].s64 = ctx.r[10].s64 + 31224;
	// 8267E6CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E6D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E6D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E6D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E6E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E6E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E6E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E6EC: 4BDE8735  bl 0x82466e20
	ctx.lr = 0x8267E6F0;
	sub_82466E20(ctx, base);
	// 8267E6F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E6F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E6F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E6FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E700 size=112
    let mut pc: u32 = 0x8267E700;
    'dispatch: loop {
        match pc {
            0x8267E700 => {
    //   block [0x8267E700..0x8267E770)
	// 8267E700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E708: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E70C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E710: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E714: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267E718: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E71C: 390BA9F8  addi r8, r11, -0x5608
	ctx.r[8].s64 = ctx.r[11].s64 + -22024;
	// 8267E720: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267E724: 388A3BA8  addi r4, r10, 0x3ba8
	ctx.r[4].s64 = ctx.r[10].s64 + 15272;
	// 8267E728: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E72C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E730: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E734: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E738: 386A7A28  addi r3, r10, 0x7a28
	ctx.r[3].s64 = ctx.r[10].s64 + 31272;
	// 8267E73C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E740: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E744: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E748: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E74C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E750: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E754: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E758: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E75C: 4BDE86C5  bl 0x82466e20
	ctx.lr = 0x8267E760;
	sub_82466E20(ctx, base);
	// 8267E760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E764: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E768: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E76C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E770 size=100
    let mut pc: u32 = 0x8267E770;
    'dispatch: loop {
        match pc {
            0x8267E770 => {
    //   block [0x8267E770..0x8267E7D4)
	// 8267E770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E778: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E77C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E780: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E784: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267E788: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E78C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E790: 388A3BB4  addi r4, r10, 0x3bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 15284;
	// 8267E794: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E798: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E79C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E7A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E7A4: 386A7A58  addi r3, r10, 0x7a58
	ctx.r[3].s64 = ctx.r[10].s64 + 31320;
	// 8267E7A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E7AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E7B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267E7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E7B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267E7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E7C0: 4BDE8661  bl 0x82466e20
	ctx.lr = 0x8267E7C4;
	sub_82466E20(ctx, base);
	// 8267E7C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E7C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E7CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E7D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E7D8 size=116
    let mut pc: u32 = 0x8267E7D8;
    'dispatch: loop {
        match pc {
            0x8267E7D8 => {
    //   block [0x8267E7D8..0x8267E84C)
	// 8267E7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E7E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E7E4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E7E8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267E7EC: 390BAA2C  addi r8, r11, -0x55d4
	ctx.r[8].s64 = ctx.r[11].s64 + -21972;
	// 8267E7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E7F4: 392A4554  addi r9, r10, 0x4554
	ctx.r[9].s64 = ctx.r[10].s64 + 17748;
	// 8267E7F8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E7FC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8267E800: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267E804: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E808: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E80C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E810: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E818: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E81C: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267E820: 388A3BC4  addi r4, r10, 0x3bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15300;
	// 8267E824: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267E828: 386B7A88  addi r3, r11, 0x7a88
	ctx.r[3].s64 = ctx.r[11].s64 + 31368;
	// 8267E82C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E830: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E838: 4BDE85E9  bl 0x82466e20
	ctx.lr = 0x8267E83C;
	sub_82466E20(ctx, base);
	// 8267E83C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E840: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E844: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E850 size=112
    let mut pc: u32 = 0x8267E850;
    'dispatch: loop {
        match pc {
            0x8267E850 => {
    //   block [0x8267E850..0x8267E8C0)
	// 8267E850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E858: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E85C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E860: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E864: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267E868: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E86C: 390BAA5C  addi r8, r11, -0x55a4
	ctx.r[8].s64 = ctx.r[11].s64 + -21924;
	// 8267E870: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267E874: 388A3BD4  addi r4, r10, 0x3bd4
	ctx.r[4].s64 = ctx.r[10].s64 + 15316;
	// 8267E878: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E87C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E880: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E884: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E888: 386A7AB8  addi r3, r10, 0x7ab8
	ctx.r[3].s64 = ctx.r[10].s64 + 31416;
	// 8267E88C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E890: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E894: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E898: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E89C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E8A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E8A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E8A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E8AC: 4BDE8575  bl 0x82466e20
	ctx.lr = 0x8267E8B0;
	sub_82466E20(ctx, base);
	// 8267E8B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E8B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E8B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E8BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E8C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E8C0 size=116
    let mut pc: u32 = 0x8267E8C0;
    'dispatch: loop {
        match pc {
            0x8267E8C0 => {
    //   block [0x8267E8C0..0x8267E934)
	// 8267E8C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E8C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E8C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E8CC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E8D0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267E8D4: 390BAA78  addi r8, r11, -0x5588
	ctx.r[8].s64 = ctx.r[11].s64 + -21896;
	// 8267E8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E8DC: 392A4580  addi r9, r10, 0x4580
	ctx.r[9].s64 = ctx.r[10].s64 + 17792;
	// 8267E8E0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267E8E4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8267E8E8: 38AA8118  addi r5, r10, -0x7ee8
	ctx.r[5].s64 = ctx.r[10].s64 + -32488;
	// 8267E8EC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E8F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E8F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E8F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E900: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E904: 3D608296  lis r11, -0x7d6a
	ctx.r[11].s64 = -2104098816;
	// 8267E908: 388A3BE8  addi r4, r10, 0x3be8
	ctx.r[4].s64 = ctx.r[10].s64 + 15336;
	// 8267E90C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267E910: 386B7AE8  addi r3, r11, 0x7ae8
	ctx.r[3].s64 = ctx.r[11].s64 + 31464;
	// 8267E914: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267E918: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E91C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E920: 4BDE8501  bl 0x82466e20
	ctx.lr = 0x8267E924;
	sub_82466E20(ctx, base);
	// 8267E924: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E928: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E92C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E930: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E938 size=112
    let mut pc: u32 = 0x8267E938;
    'dispatch: loop {
        match pc {
            0x8267E938 => {
    //   block [0x8267E938..0x8267E9A8)
	// 8267E938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E93C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E940: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E944: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E948: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E94C: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267E950: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E954: 390BAA90  addi r8, r11, -0x5570
	ctx.r[8].s64 = ctx.r[11].s64 + -21872;
	// 8267E958: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267E95C: 388A3BF8  addi r4, r10, 0x3bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 15352;
	// 8267E960: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E964: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E968: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E96C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E970: 386A7B18  addi r3, r10, 0x7b18
	ctx.r[3].s64 = ctx.r[10].s64 + 31512;
	// 8267E974: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E978: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E97C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E980: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E984: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267E988: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E98C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267E990: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267E994: 4BDE848D  bl 0x82466e20
	ctx.lr = 0x8267E998;
	sub_82466E20(ctx, base);
	// 8267E998: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267E99C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267E9A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267E9A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267E9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267E9A8 size=112
    let mut pc: u32 = 0x8267E9A8;
    'dispatch: loop {
        match pc {
            0x8267E9A8 => {
    //   block [0x8267E9A8..0x8267EA18)
	// 8267E9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267E9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267E9B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267E9B4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E9B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267E9BC: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267E9C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267E9C4: 390BAB08  addi r8, r11, -0x54f8
	ctx.r[8].s64 = ctx.r[11].s64 + -21752;
	// 8267E9C8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267E9CC: 388A3C0C  addi r4, r10, 0x3c0c
	ctx.r[4].s64 = ctx.r[10].s64 + 15372;
	// 8267E9D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267E9D4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267E9D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267E9DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267E9E0: 386A7B48  addi r3, r10, 0x7b48
	ctx.r[3].s64 = ctx.r[10].s64 + 31560;
	// 8267E9E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267E9E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267E9EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267E9F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267E9F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267E9F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267E9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EA00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EA04: 4BDE841D  bl 0x82466e20
	ctx.lr = 0x8267EA08;
	sub_82466E20(ctx, base);
	// 8267EA08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EA0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EA10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EA14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EA18 size=112
    let mut pc: u32 = 0x8267EA18;
    'dispatch: loop {
        match pc {
            0x8267EA18 => {
    //   block [0x8267EA18..0x8267EA88)
	// 8267EA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EA20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EA24: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EA28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EA2C: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267EA30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EA34: 390BAB50  addi r8, r11, -0x54b0
	ctx.r[8].s64 = ctx.r[11].s64 + -21680;
	// 8267EA38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267EA3C: 388A3C24  addi r4, r10, 0x3c24
	ctx.r[4].s64 = ctx.r[10].s64 + 15396;
	// 8267EA40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EA44: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EA48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EA4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EA50: 386A7B78  addi r3, r10, 0x7b78
	ctx.r[3].s64 = ctx.r[10].s64 + 31608;
	// 8267EA54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EA58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EA60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EA64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EA68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EA6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EA70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EA74: 4BDE83AD  bl 0x82466e20
	ctx.lr = 0x8267EA78;
	sub_82466E20(ctx, base);
	// 8267EA78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EA7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EA80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EA84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EA88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EA88 size=112
    let mut pc: u32 = 0x8267EA88;
    'dispatch: loop {
        match pc {
            0x8267EA88 => {
    //   block [0x8267EA88..0x8267EAF8)
	// 8267EA88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EA8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EA90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EA94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EA98: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EA9C: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267EAA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EAA4: 390BAB98  addi r8, r11, -0x5468
	ctx.r[8].s64 = ctx.r[11].s64 + -21608;
	// 8267EAA8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267EAAC: 388A3C3C  addi r4, r10, 0x3c3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15420;
	// 8267EAB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EAB4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EAB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EABC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EAC0: 386A7BA8  addi r3, r10, 0x7ba8
	ctx.r[3].s64 = ctx.r[10].s64 + 31656;
	// 8267EAC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EAD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EAD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EAD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EADC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EAE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EAE4: 4BDE833D  bl 0x82466e20
	ctx.lr = 0x8267EAE8;
	sub_82466E20(ctx, base);
	// 8267EAE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EAEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EAF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EAF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EAF8 size=108
    let mut pc: u32 = 0x8267EAF8;
    'dispatch: loop {
        match pc {
            0x8267EAF8 => {
    //   block [0x8267EAF8..0x8267EB64)
	// 8267EAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EAFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EB00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EB04: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EB0C: 38EBABE0  addi r7, r11, -0x5420
	ctx.r[7].s64 = ctx.r[11].s64 + -21536;
	// 8267EB10: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267EB14: 388A3C54  addi r4, r10, 0x3c54
	ctx.r[4].s64 = ctx.r[10].s64 + 15444;
	// 8267EB18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EB1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EB20: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267EB24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EB28: 386A7BD8  addi r3, r10, 0x7bd8
	ctx.r[3].s64 = ctx.r[10].s64 + 31704;
	// 8267EB2C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267EB30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EB34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EB38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EB3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EB40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EB44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EB48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EB4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267EB50: 4BDE82D1  bl 0x82466e20
	ctx.lr = 0x8267EB54;
	sub_82466E20(ctx, base);
	// 8267EB54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EB58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EB5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EB60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EB68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EB68 size=112
    let mut pc: u32 = 0x8267EB68;
    'dispatch: loop {
        match pc {
            0x8267EB68 => {
    //   block [0x8267EB68..0x8267EBD8)
	// 8267EB68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EB6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EB70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EB74: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EB78: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EB7C: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267EB80: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EB84: 390BAC28  addi r8, r11, -0x53d8
	ctx.r[8].s64 = ctx.r[11].s64 + -21464;
	// 8267EB88: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8267EB8C: 388A3C78  addi r4, r10, 0x3c78
	ctx.r[4].s64 = ctx.r[10].s64 + 15480;
	// 8267EB90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EB94: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EB98: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EB9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EBA0: 386A7C08  addi r3, r10, 0x7c08
	ctx.r[3].s64 = ctx.r[10].s64 + 31752;
	// 8267EBA4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EBA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EBAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EBB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EBB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EBB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EBBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EBC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EBC4: 4BDE825D  bl 0x82466e20
	ctx.lr = 0x8267EBC8;
	sub_82466E20(ctx, base);
	// 8267EBC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EBCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EBD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EBD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EBD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EBD8 size=116
    let mut pc: u32 = 0x8267EBD8;
    'dispatch: loop {
        match pc {
            0x8267EBD8 => {
    //   block [0x8267EBD8..0x8267EC4C)
	// 8267EBD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EBDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EBE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EBE4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267EBE8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EBEC: 392B45BC  addi r9, r11, 0x45bc
	ctx.r[9].s64 = ctx.r[11].s64 + 17852;
	// 8267EBF0: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267EBF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EBF8: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8267EBFC: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8267EC00: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EC04: 388A3C90  addi r4, r10, 0x3c90
	ctx.r[4].s64 = ctx.r[10].s64 + 15504;
	// 8267EC08: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EC0C: 396BACA8  addi r11, r11, -0x5358
	ctx.r[11].s64 = ctx.r[11].s64 + -21336;
	// 8267EC10: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267EC14: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EC18: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267EC1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EC20: 386A7C38  addi r3, r10, 0x7c38
	ctx.r[3].s64 = ctx.r[10].s64 + 31800;
	// 8267EC24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8267EC28: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267EC2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EC30: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267EC34: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267EC38: 4BDE81E9  bl 0x82466e20
	ctx.lr = 0x8267EC3C;
	sub_82466E20(ctx, base);
	// 8267EC3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EC40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EC44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EC48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267EC50 size=36
    let mut pc: u32 = 0x8267EC50;
    'dispatch: loop {
        match pc {
            0x8267EC50 => {
    //   block [0x8267EC50..0x8267EC74)
	// 8267EC50: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EC54: 814BAD3C  lwz r10, -0x52c4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21188 as u32) ) } as u64;
	// 8267EC58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EC5C: 396BEEA0  addi r11, r11, -0x1160
	ctx.r[11].s64 = ctx.r[11].s64 + -4448;
	// 8267EC60: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8267EC64: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267EC68: 814AACA4  lwz r10, -0x535c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-21340 as u32) ) } as u64;
	// 8267EC6C: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8267EC70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EC78 size=108
    let mut pc: u32 = 0x8267EC78;
    'dispatch: loop {
        match pc {
            0x8267EC78 => {
    //   block [0x8267EC78..0x8267ECE4)
	// 8267EC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EC7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EC80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EC84: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EC88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EC8C: 38EBEEA0  addi r7, r11, -0x1160
	ctx.r[7].s64 = ctx.r[11].s64 + -4448;
	// 8267EC90: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8267EC94: 388A3D1C  addi r4, r10, 0x3d1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15644;
	// 8267EC98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EC9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ECA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267ECA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267ECA8: 386A7C68  addi r3, r10, 0x7c68
	ctx.r[3].s64 = ctx.r[10].s64 + 31848;
	// 8267ECAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267ECB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267ECB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ECB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267ECBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ECC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267ECC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ECC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ECCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267ECD0: 4BDE8151  bl 0x82466e20
	ctx.lr = 0x8267ECD4;
	sub_82466E20(ctx, base);
	// 8267ECD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267ECD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267ECDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ECE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ECE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267ECE8 size=24
    let mut pc: u32 = 0x8267ECE8;
    'dispatch: loop {
        match pc {
            0x8267ECE8 => {
    //   block [0x8267ECE8..0x8267ED00)
	// 8267ECE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ECEC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267ECF0: 394AEF48  addi r10, r10, -0x10b8
	ctx.r[10].s64 = ctx.r[10].s64 + -4280;
	// 8267ECF4: 816BACA4  lwz r11, -0x535c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21340 as u32) ) } as u64;
	// 8267ECF8: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8267ECFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ED00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ED00 size=116
    let mut pc: u32 = 0x8267ED00;
    'dispatch: loop {
        match pc {
            0x8267ED00 => {
    //   block [0x8267ED00..0x8267ED74)
	// 8267ED00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ED04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ED08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ED0C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267ED10: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 8267ED14: 390AEF48  addi r8, r10, -0x10b8
	ctx.r[8].s64 = ctx.r[10].s64 + -4280;
	// 8267ED18: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ED1C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267ED20: 38AA7C68  addi r5, r10, 0x7c68
	ctx.r[5].s64 = ctx.r[10].s64 + 31848;
	// 8267ED24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ED28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267ED2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267ED30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267ED34: 388A3D58  addi r4, r10, 0x3d58
	ctx.r[4].s64 = ctx.r[10].s64 + 15704;
	// 8267ED38: 396B4678  addi r11, r11, 0x4678
	ctx.r[11].s64 = ctx.r[11].s64 + 18040;
	// 8267ED3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ED40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267ED44: 386A7C98  addi r3, r10, 0x7c98
	ctx.r[3].s64 = ctx.r[10].s64 + 31896;
	// 8267ED48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267ED4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267ED50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267ED54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267ED58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267ED5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267ED60: 4BDE80C1  bl 0x82466e20
	ctx.lr = 0x8267ED64;
	sub_82466E20(ctx, base);
	// 8267ED64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267ED68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267ED6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267ED70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267ED78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267ED78 size=112
    let mut pc: u32 = 0x8267ED78;
    'dispatch: loop {
        match pc {
            0x8267ED78 => {
    //   block [0x8267ED78..0x8267EDE8)
	// 8267ED78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267ED7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267ED80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267ED84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267ED88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267ED8C: 38AA7C68  addi r5, r10, 0x7c68
	ctx.r[5].s64 = ctx.r[10].s64 + 31848;
	// 8267ED90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267ED94: 390BAD40  addi r8, r11, -0x52c0
	ctx.r[8].s64 = ctx.r[11].s64 + -21184;
	// 8267ED98: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267ED9C: 388A3D7C  addi r4, r10, 0x3d7c
	ctx.r[4].s64 = ctx.r[10].s64 + 15740;
	// 8267EDA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EDA4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EDA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EDAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EDB0: 386A7CC8  addi r3, r10, 0x7cc8
	ctx.r[3].s64 = ctx.r[10].s64 + 31944;
	// 8267EDB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EDB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EDBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EDC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EDC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EDC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EDCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EDD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EDD4: 4BDE804D  bl 0x82466e20
	ctx.lr = 0x8267EDD8;
	sub_82466E20(ctx, base);
	// 8267EDD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EDE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267EDE8 size=24
    let mut pc: u32 = 0x8267EDE8;
    'dispatch: loop {
        match pc {
            0x8267EDE8 => {
    //   block [0x8267EDE8..0x8267EE00)
	// 8267EDE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EDEC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267EDF0: 394AF008  addi r10, r10, -0xff8
	ctx.r[10].s64 = ctx.r[10].s64 + -4088;
	// 8267EDF4: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267EDF8: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8267EDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EE00 size=116
    let mut pc: u32 = 0x8267EE00;
    'dispatch: loop {
        match pc {
            0x8267EE00 => {
    //   block [0x8267EE00..0x8267EE74)
	// 8267EE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EE04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EE08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EE0C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267EE10: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EE14: 392B463C  addi r9, r11, 0x463c
	ctx.r[9].s64 = ctx.r[11].s64 + 17980;
	// 8267EE18: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267EE1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EE20: 38E90060  addi r7, r9, 0x60
	ctx.r[7].s64 = ctx.r[9].s64 + 96;
	// 8267EE24: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8267EE28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EE2C: 388A3DA0  addi r4, r10, 0x3da0
	ctx.r[4].s64 = ctx.r[10].s64 + 15776;
	// 8267EE30: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EE34: 396BF008  addi r11, r11, -0xff8
	ctx.r[11].s64 = ctx.r[11].s64 + -4088;
	// 8267EE38: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267EE3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EE40: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267EE44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EE48: 386A7CF8  addi r3, r10, 0x7cf8
	ctx.r[3].s64 = ctx.r[10].s64 + 31992;
	// 8267EE4C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8267EE50: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267EE54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EE58: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267EE5C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267EE60: 4BDE7FC1  bl 0x82466e20
	ctx.lr = 0x8267EE64;
	sub_82466E20(ctx, base);
	// 8267EE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EE70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EE78 size=100
    let mut pc: u32 = 0x8267EE78;
    'dispatch: loop {
        match pc {
            0x8267EE78 => {
    //   block [0x8267EE78..0x8267EEDC)
	// 8267EE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EE80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EE84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EE88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EE8C: 38AA7E48  addi r5, r10, 0x7e48
	ctx.r[5].s64 = ctx.r[10].s64 + 32328;
	// 8267EE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EE94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EE98: 388A3DB4  addi r4, r10, 0x3db4
	ctx.r[4].s64 = ctx.r[10].s64 + 15796;
	// 8267EE9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EEA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EEA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EEA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EEAC: 386A7D28  addi r3, r10, 0x7d28
	ctx.r[3].s64 = ctx.r[10].s64 + 32040;
	// 8267EEB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EEB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EEB8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267EEBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EEC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267EEC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EEC8: 4BDE7F59  bl 0x82466e20
	ctx.lr = 0x8267EECC;
	sub_82466E20(ctx, base);
	// 8267EECC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EED8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EEE0 size=100
    let mut pc: u32 = 0x8267EEE0;
    'dispatch: loop {
        match pc {
            0x8267EEE0 => {
    //   block [0x8267EEE0..0x8267EF44)
	// 8267EEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EEE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EEE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EEEC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EEF4: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267EEF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EEFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EF00: 388A3DC4  addi r4, r10, 0x3dc4
	ctx.r[4].s64 = ctx.r[10].s64 + 15812;
	// 8267EF04: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EF08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EF0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EF10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EF14: 386A7D58  addi r3, r10, 0x7d58
	ctx.r[3].s64 = ctx.r[10].s64 + 32088;
	// 8267EF18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EF1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EF20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267EF24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EF28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267EF2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EF30: 4BDE7EF1  bl 0x82466e20
	ctx.lr = 0x8267EF34;
	sub_82466E20(ctx, base);
	// 8267EF34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EF38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EF3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EF40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EF48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EF48 size=108
    let mut pc: u32 = 0x8267EF48;
    'dispatch: loop {
        match pc {
            0x8267EF48 => {
    //   block [0x8267EF48..0x8267EFB4)
	// 8267EF48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EF4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EF50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EF54: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EF58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EF5C: 38EBADA0  addi r7, r11, -0x5260
	ctx.r[7].s64 = ctx.r[11].s64 + -21088;
	// 8267EF60: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267EF64: 388A3DD8  addi r4, r10, 0x3dd8
	ctx.r[4].s64 = ctx.r[10].s64 + 15832;
	// 8267EF68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EF6C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EF70: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267EF74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267EF78: 386A7D88  addi r3, r10, 0x7d88
	ctx.r[3].s64 = ctx.r[10].s64 + 32136;
	// 8267EF7C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267EF80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EF84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267EF88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267EF8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EF90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267EF94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267EF98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267EF9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267EFA0: 4BDE7E81  bl 0x82466e20
	ctx.lr = 0x8267EFA4;
	sub_82466E20(ctx, base);
	// 8267EFA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267EFA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267EFAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267EFB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267EFB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267EFB8 size=112
    let mut pc: u32 = 0x8267EFB8;
    'dispatch: loop {
        match pc {
            0x8267EFB8 => {
    //   block [0x8267EFB8..0x8267F028)
	// 8267EFB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267EFBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267EFC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267EFC4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EFC8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267EFCC: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267EFD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267EFD4: 390BADE8  addi r8, r11, -0x5218
	ctx.r[8].s64 = ctx.r[11].s64 + -21016;
	// 8267EFD8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267EFDC: 388A3DF0  addi r4, r10, 0x3df0
	ctx.r[4].s64 = ctx.r[10].s64 + 15856;
	// 8267EFE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267EFE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267EFE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267EFEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267EFF0: 386A7DB8  addi r3, r10, 0x7db8
	ctx.r[3].s64 = ctx.r[10].s64 + 32184;
	// 8267EFF4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267EFF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267EFFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F000: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F008: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F00C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F010: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F014: 4BDE7E0D  bl 0x82466e20
	ctx.lr = 0x8267F018;
	sub_82466E20(ctx, base);
	// 8267F018: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F01C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F020: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F024: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F028 size=108
    let mut pc: u32 = 0x8267F028;
    'dispatch: loop {
        match pc {
            0x8267F028 => {
    //   block [0x8267F028..0x8267F094)
	// 8267F028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F02C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F030: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F034: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F038: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F03C: 38EBAE30  addi r7, r11, -0x51d0
	ctx.r[7].s64 = ctx.r[11].s64 + -20944;
	// 8267F040: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267F044: 388A3DFC  addi r4, r10, 0x3dfc
	ctx.r[4].s64 = ctx.r[10].s64 + 15868;
	// 8267F048: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F04C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F050: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F054: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F058: 386A7DE8  addi r3, r10, 0x7de8
	ctx.r[3].s64 = ctx.r[10].s64 + 32232;
	// 8267F05C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F060: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F064: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F068: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F070: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F074: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F078: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F07C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F080: 4BDE7DA1  bl 0x82466e20
	ctx.lr = 0x8267F084;
	sub_82466E20(ctx, base);
	// 8267F084: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267F098 size=28
    let mut pc: u32 = 0x8267F098;
    'dispatch: loop {
        match pc {
            0x8267F098 => {
    //   block [0x8267F098..0x8267F0B4)
	// 8267F098: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F09C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F0A0: 394AF0C8  addi r10, r10, -0xf38
	ctx.r[10].s64 = ctx.r[10].s64 + -3896;
	// 8267F0A4: 816BAE48  lwz r11, -0x51b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20920 as u32) ) } as u64;
	// 8267F0A8: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267F0AC: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8267F0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F0B8 size=112
    let mut pc: u32 = 0x8267F0B8;
    'dispatch: loop {
        match pc {
            0x8267F0B8 => {
    //   block [0x8267F0B8..0x8267F128)
	// 8267F0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F0C4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F0C8: 3900000E  li r8, 0xe
	ctx.r[8].s64 = 14;
	// 8267F0CC: 38EAF0C8  addi r7, r10, -0xf38
	ctx.r[7].s64 = ctx.r[10].s64 + -3896;
	// 8267F0D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F0D4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267F0D8: 388A3E1C  addi r4, r10, 0x3e1c
	ctx.r[4].s64 = ctx.r[10].s64 + 15900;
	// 8267F0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F0E0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F0E4: 396B4728  addi r11, r11, 0x4728
	ctx.r[11].s64 = ctx.r[11].s64 + 18216;
	// 8267F0E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F0EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F0F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F0F4: 386A7E18  addi r3, r10, 0x7e18
	ctx.r[3].s64 = ctx.r[10].s64 + 32280;
	// 8267F0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F0FC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267F100: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F104: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267F108: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F10C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F110: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F114: 4BDE7D0D  bl 0x82466e20
	ctx.lr = 0x8267F118;
	sub_82466E20(ctx, base);
	// 8267F118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267F128 size=24
    let mut pc: u32 = 0x8267F128;
    'dispatch: loop {
        match pc {
            0x8267F128 => {
    //   block [0x8267F128..0x8267F140)
	// 8267F128: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F12C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F130: 394AF218  addi r10, r10, -0xde8
	ctx.r[10].s64 = ctx.r[10].s64 + -3560;
	// 8267F134: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267F138: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267F13C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F140 size=116
    let mut pc: u32 = 0x8267F140;
    'dispatch: loop {
        match pc {
            0x8267F140 => {
    //   block [0x8267F140..0x8267F1B4)
	// 8267F140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F14C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267F150: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F154: 392B4700  addi r9, r11, 0x4700
	ctx.r[9].s64 = ctx.r[11].s64 + 18176;
	// 8267F158: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267F15C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F160: 38E90064  addi r7, r9, 0x64
	ctx.r[7].s64 = ctx.r[9].s64 + 100;
	// 8267F164: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8267F168: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F16C: 388A3E30  addi r4, r10, 0x3e30
	ctx.r[4].s64 = ctx.r[10].s64 + 15920;
	// 8267F170: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F174: 396BF218  addi r11, r11, -0xde8
	ctx.r[11].s64 = ctx.r[11].s64 + -3560;
	// 8267F178: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8267F17C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F180: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8267F184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F188: 386A7E48  addi r3, r10, 0x7e48
	ctx.r[3].s64 = ctx.r[10].s64 + 32328;
	// 8267F18C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267F190: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8267F194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F198: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8267F19C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267F1A0: 4BDE7C81  bl 0x82466e20
	ctx.lr = 0x8267F1A4;
	sub_82466E20(ctx, base);
	// 8267F1A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F1A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F1AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F1B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F1B8 size=112
    let mut pc: u32 = 0x8267F1B8;
    'dispatch: loop {
        match pc {
            0x8267F1B8 => {
    //   block [0x8267F1B8..0x8267F228)
	// 8267F1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F1C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F1C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F1C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F1CC: 38AA7A58  addi r5, r10, 0x7a58
	ctx.r[5].s64 = ctx.r[10].s64 + 31320;
	// 8267F1D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F1D4: 390BAE50  addi r8, r11, -0x51b0
	ctx.r[8].s64 = ctx.r[11].s64 + -20912;
	// 8267F1D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F1DC: 388A3E3C  addi r4, r10, 0x3e3c
	ctx.r[4].s64 = ctx.r[10].s64 + 15932;
	// 8267F1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F1E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F1E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F1EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F1F0: 386A7E78  addi r3, r10, 0x7e78
	ctx.r[3].s64 = ctx.r[10].s64 + 32376;
	// 8267F1F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F1F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F1FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F200: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F204: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F208: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F20C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F210: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F214: 4BDE7C0D  bl 0x82466e20
	ctx.lr = 0x8267F218;
	sub_82466E20(ctx, base);
	// 8267F218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F21C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F220: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267F228 size=24
    let mut pc: u32 = 0x8267F228;
    'dispatch: loop {
        match pc {
            0x8267F228 => {
    //   block [0x8267F228..0x8267F240)
	// 8267F228: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F22C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F230: 394AF2C0  addi r10, r10, -0xd40
	ctx.r[10].s64 = ctx.r[10].s64 + -3392;
	// 8267F234: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267F238: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8267F23C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F240 size=116
    let mut pc: u32 = 0x8267F240;
    'dispatch: loop {
        match pc {
            0x8267F240 => {
    //   block [0x8267F240..0x8267F2B4)
	// 8267F240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F248: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F24C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F250: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 8267F254: 390AF2C0  addi r8, r10, -0xd40
	ctx.r[8].s64 = ctx.r[10].s64 + -3392;
	// 8267F258: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F25C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267F260: 38AA7A58  addi r5, r10, 0x7a58
	ctx.r[5].s64 = ctx.r[10].s64 + 31320;
	// 8267F264: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F268: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267F26C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F270: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F274: 388A3E64  addi r4, r10, 0x3e64
	ctx.r[4].s64 = ctx.r[10].s64 + 15972;
	// 8267F278: 396B4784  addi r11, r11, 0x4784
	ctx.r[11].s64 = ctx.r[11].s64 + 18308;
	// 8267F27C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F280: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267F284: 386A7EA8  addi r3, r10, 0x7ea8
	ctx.r[3].s64 = ctx.r[10].s64 + 32424;
	// 8267F288: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267F28C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F290: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267F294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F29C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F2A0: 4BDE7B81  bl 0x82466e20
	ctx.lr = 0x8267F2A4;
	sub_82466E20(ctx, base);
	// 8267F2A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F2A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F2AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F2B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F2B8 size=112
    let mut pc: u32 = 0x8267F2B8;
    'dispatch: loop {
        match pc {
            0x8267F2B8 => {
    //   block [0x8267F2B8..0x8267F328)
	// 8267F2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F2BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F2C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F2C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F2C8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F2CC: 38AA9828  addi r5, r10, -0x67d8
	ctx.r[5].s64 = ctx.r[10].s64 + -26584;
	// 8267F2D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F2D4: 390BAE80  addi r8, r11, -0x5180
	ctx.r[8].s64 = ctx.r[11].s64 + -20864;
	// 8267F2D8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F2DC: 388A3E78  addi r4, r10, 0x3e78
	ctx.r[4].s64 = ctx.r[10].s64 + 15992;
	// 8267F2E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F2E4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F2E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F2EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F2F0: 386A7ED8  addi r3, r10, 0x7ed8
	ctx.r[3].s64 = ctx.r[10].s64 + 32472;
	// 8267F2F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F2F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F2FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F300: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F304: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F308: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F30C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F310: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F314: 4BDE7B0D  bl 0x82466e20
	ctx.lr = 0x8267F318;
	sub_82466E20(ctx, base);
	// 8267F318: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F31C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F320: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F328 size=108
    let mut pc: u32 = 0x8267F328;
    'dispatch: loop {
        match pc {
            0x8267F328 => {
    //   block [0x8267F328..0x8267F394)
	// 8267F328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F330: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F334: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F338: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F33C: 38EBAEB0  addi r7, r11, -0x5150
	ctx.r[7].s64 = ctx.r[11].s64 + -20816;
	// 8267F340: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8267F344: 388A3E98  addi r4, r10, 0x3e98
	ctx.r[4].s64 = ctx.r[10].s64 + 16024;
	// 8267F348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F34C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F350: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F354: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F358: 386A7F08  addi r3, r10, 0x7f08
	ctx.r[3].s64 = ctx.r[10].s64 + 32520;
	// 8267F35C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F360: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F368: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F36C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F370: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F378: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F37C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F380: 4BDE7AA1  bl 0x82466e20
	ctx.lr = 0x8267F384;
	sub_82466E20(ctx, base);
	// 8267F384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F38C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F398 size=112
    let mut pc: u32 = 0x8267F398;
    'dispatch: loop {
        match pc {
            0x8267F398 => {
    //   block [0x8267F398..0x8267F408)
	// 8267F398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F3A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F3A4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F3A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F3AC: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267F3B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F3B4: 390BAEE0  addi r8, r11, -0x5120
	ctx.r[8].s64 = ctx.r[11].s64 + -20768;
	// 8267F3B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F3BC: 388A3EAC  addi r4, r10, 0x3eac
	ctx.r[4].s64 = ctx.r[10].s64 + 16044;
	// 8267F3C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F3C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F3C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F3CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F3D0: 386A7F38  addi r3, r10, 0x7f38
	ctx.r[3].s64 = ctx.r[10].s64 + 32568;
	// 8267F3D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F3D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F3DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F3E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F3E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F3E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F3EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F3F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F3F4: 4BDE7A2D  bl 0x82466e20
	ctx.lr = 0x8267F3F8;
	sub_82466E20(ctx, base);
	// 8267F3F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F408 size=112
    let mut pc: u32 = 0x8267F408;
    'dispatch: loop {
        match pc {
            0x8267F408 => {
    //   block [0x8267F408..0x8267F478)
	// 8267F408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F410: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F414: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F418: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F41C: 38AA8118  addi r5, r10, -0x7ee8
	ctx.r[5].s64 = ctx.r[10].s64 + -32488;
	// 8267F420: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F424: 390BAF10  addi r8, r11, -0x50f0
	ctx.r[8].s64 = ctx.r[11].s64 + -20720;
	// 8267F428: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F42C: 388A3EBC  addi r4, r10, 0x3ebc
	ctx.r[4].s64 = ctx.r[10].s64 + 16060;
	// 8267F430: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F434: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F438: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F43C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F440: 386A7F68  addi r3, r10, 0x7f68
	ctx.r[3].s64 = ctx.r[10].s64 + 32616;
	// 8267F444: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F448: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F44C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F450: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F454: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F458: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F45C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F460: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F464: 4BDE79BD  bl 0x82466e20
	ctx.lr = 0x8267F468;
	sub_82466E20(ctx, base);
	// 8267F468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F478 size=108
    let mut pc: u32 = 0x8267F478;
    'dispatch: loop {
        match pc {
            0x8267F478 => {
    //   block [0x8267F478..0x8267F4E4)
	// 8267F478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F47C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F484: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F488: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F48C: 38EBAF40  addi r7, r11, -0x50c0
	ctx.r[7].s64 = ctx.r[11].s64 + -20672;
	// 8267F490: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8267F494: 388A3ED0  addi r4, r10, 0x3ed0
	ctx.r[4].s64 = ctx.r[10].s64 + 16080;
	// 8267F498: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F49C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F4A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F4A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F4A8: 386A7F98  addi r3, r10, 0x7f98
	ctx.r[3].s64 = ctx.r[10].s64 + 32664;
	// 8267F4AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F4B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F4B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F4B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F4BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F4C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F4C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F4C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F4CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F4D0: 4BDE7951  bl 0x82466e20
	ctx.lr = 0x8267F4D4;
	sub_82466E20(ctx, base);
	// 8267F4D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F4D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F4DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F4E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F4E8 size=112
    let mut pc: u32 = 0x8267F4E8;
    'dispatch: loop {
        match pc {
            0x8267F4E8 => {
    //   block [0x8267F4E8..0x8267F558)
	// 8267F4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F4EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F4F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F4F4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F4F8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F4FC: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267F500: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F504: 390BAF88  addi r8, r11, -0x5078
	ctx.r[8].s64 = ctx.r[11].s64 + -20600;
	// 8267F508: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267F50C: 388A3EF8  addi r4, r10, 0x3ef8
	ctx.r[4].s64 = ctx.r[10].s64 + 16120;
	// 8267F510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F514: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F518: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F51C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F520: 386A7FC8  addi r3, r10, 0x7fc8
	ctx.r[3].s64 = ctx.r[10].s64 + 32712;
	// 8267F524: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F528: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F52C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F530: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F534: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F538: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F53C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F540: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F544: 4BDE78DD  bl 0x82466e20
	ctx.lr = 0x8267F548;
	sub_82466E20(ctx, base);
	// 8267F548: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F558 size=100
    let mut pc: u32 = 0x8267F558;
    'dispatch: loop {
        match pc {
            0x8267F558 => {
    //   block [0x8267F558..0x8267F5BC)
	// 8267F558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F564: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F56C: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267F570: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F574: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F578: 388A3F14  addi r4, r10, 0x3f14
	ctx.r[4].s64 = ctx.r[10].s64 + 16148;
	// 8267F57C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F580: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F584: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F588: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F58C: 386A7FF8  addi r3, r10, 0x7ff8
	ctx.r[3].s64 = ctx.r[10].s64 + 32760;
	// 8267F590: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F594: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F598: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267F59C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F5A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267F5A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F5A8: 4BDE7879  bl 0x82466e20
	ctx.lr = 0x8267F5AC;
	sub_82466E20(ctx, base);
	// 8267F5AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F5B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F5B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F5C0 size=112
    let mut pc: u32 = 0x8267F5C0;
    'dispatch: loop {
        match pc {
            0x8267F5C0 => {
    //   block [0x8267F5C0..0x8267F630)
	// 8267F5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F5C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F5CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F5D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F5D4: 38AA7D58  addi r5, r10, 0x7d58
	ctx.r[5].s64 = ctx.r[10].s64 + 32088;
	// 8267F5D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F5DC: 390BAFE8  addi r8, r11, -0x5018
	ctx.r[8].s64 = ctx.r[11].s64 + -20504;
	// 8267F5E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267F5E4: 388A3F2C  addi r4, r10, 0x3f2c
	ctx.r[4].s64 = ctx.r[10].s64 + 16172;
	// 8267F5E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F5EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F5F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F5F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F5F8: 386A8028  addi r3, r10, -0x7fd8
	ctx.r[3].s64 = ctx.r[10].s64 + -32728;
	// 8267F5FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F604: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F608: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F60C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F610: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F618: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F61C: 4BDE7805  bl 0x82466e20
	ctx.lr = 0x8267F620;
	sub_82466E20(ctx, base);
	// 8267F620: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F62C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F630 size=112
    let mut pc: u32 = 0x8267F630;
    'dispatch: loop {
        match pc {
            0x8267F630 => {
    //   block [0x8267F630..0x8267F6A0)
	// 8267F630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F63C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F640: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F644: 38AA7D58  addi r5, r10, 0x7d58
	ctx.r[5].s64 = ctx.r[10].s64 + 32088;
	// 8267F648: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F64C: 390BB030  addi r8, r11, -0x4fd0
	ctx.r[8].s64 = ctx.r[11].s64 + -20432;
	// 8267F650: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8267F654: 388A3F3C  addi r4, r10, 0x3f3c
	ctx.r[4].s64 = ctx.r[10].s64 + 16188;
	// 8267F658: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F65C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F660: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F664: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F668: 386A8058  addi r3, r10, -0x7fa8
	ctx.r[3].s64 = ctx.r[10].s64 + -32680;
	// 8267F66C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F670: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F674: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F678: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F67C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F680: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F684: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F688: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F68C: 4BDE7795  bl 0x82466e20
	ctx.lr = 0x8267F690;
	sub_82466E20(ctx, base);
	// 8267F690: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F69C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F6A0 size=108
    let mut pc: u32 = 0x8267F6A0;
    'dispatch: loop {
        match pc {
            0x8267F6A0 => {
    //   block [0x8267F6A0..0x8267F70C)
	// 8267F6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F6AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F6B4: 38EBB0D8  addi r7, r11, -0x4f28
	ctx.r[7].s64 = ctx.r[11].s64 + -20264;
	// 8267F6B8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8267F6BC: 388A3F58  addi r4, r10, 0x3f58
	ctx.r[4].s64 = ctx.r[10].s64 + 16216;
	// 8267F6C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F6C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F6C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267F6CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F6D0: 386A8088  addi r3, r10, -0x7f78
	ctx.r[3].s64 = ctx.r[10].s64 + -32632;
	// 8267F6D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267F6D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F6DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F6E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F6E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F6E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F6EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F6F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F6F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267F6F8: 4BDE7729  bl 0x82466e20
	ctx.lr = 0x8267F6FC;
	sub_82466E20(ctx, base);
	// 8267F6FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267F710 size=24
    let mut pc: u32 = 0x8267F710;
    'dispatch: loop {
        match pc {
            0x8267F710 => {
    //   block [0x8267F710..0x8267F728)
	// 8267F710: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F714: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F718: 394AF3E0  addi r10, r10, -0xc20
	ctx.r[10].s64 = ctx.r[10].s64 + -3104;
	// 8267F71C: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267F720: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8267F724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F728 size=116
    let mut pc: u32 = 0x8267F728;
    'dispatch: loop {
        match pc {
            0x8267F728 => {
    //   block [0x8267F728..0x8267F79C)
	// 8267F728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F72C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F730: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F734: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267F738: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8267F73C: 390AF3E0  addi r8, r10, -0xc20
	ctx.r[8].s64 = ctx.r[10].s64 + -3104;
	// 8267F740: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F744: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267F748: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267F74C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F750: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267F754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F758: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F75C: 388A3F74  addi r4, r10, 0x3f74
	ctx.r[4].s64 = ctx.r[10].s64 + 16244;
	// 8267F760: 396B47B8  addi r11, r11, 0x47b8
	ctx.r[11].s64 = ctx.r[11].s64 + 18360;
	// 8267F764: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F768: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F76C: 386A80B8  addi r3, r10, -0x7f48
	ctx.r[3].s64 = ctx.r[10].s64 + -32584;
	// 8267F770: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267F774: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F778: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267F77C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F788: 4BDE7699  bl 0x82466e20
	ctx.lr = 0x8267F78C;
	sub_82466E20(ctx, base);
	// 8267F78C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F7A0 size=100
    let mut pc: u32 = 0x8267F7A0;
    'dispatch: loop {
        match pc {
            0x8267F7A0 => {
    //   block [0x8267F7A0..0x8267F804)
	// 8267F7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F7A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F7AC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F7B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F7B4: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267F7B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F7BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F7C0: 388A3F88  addi r4, r10, 0x3f88
	ctx.r[4].s64 = ctx.r[10].s64 + 16264;
	// 8267F7C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F7D4: 386A80E8  addi r3, r10, -0x7f18
	ctx.r[3].s64 = ctx.r[10].s64 + -32536;
	// 8267F7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F7DC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F7E0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267F7E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F7E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267F7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F7F0: 4BDE7631  bl 0x82466e20
	ctx.lr = 0x8267F7F4;
	sub_82466E20(ctx, base);
	// 8267F7F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F7F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F7FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F808 size=100
    let mut pc: u32 = 0x8267F808;
    'dispatch: loop {
        match pc {
            0x8267F808 => {
    //   block [0x8267F808..0x8267F86C)
	// 8267F808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F80C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F814: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F81C: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267F820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F824: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F828: 388A3F98  addi r4, r10, 0x3f98
	ctx.r[4].s64 = ctx.r[10].s64 + 16280;
	// 8267F82C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F830: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F838: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F83C: 386A8118  addi r3, r10, -0x7ee8
	ctx.r[3].s64 = ctx.r[10].s64 + -32488;
	// 8267F840: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F844: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F848: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267F84C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F850: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267F854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F858: 4BDE75C9  bl 0x82466e20
	ctx.lr = 0x8267F85C;
	sub_82466E20(ctx, base);
	// 8267F85C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F868: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F870 size=112
    let mut pc: u32 = 0x8267F870;
    'dispatch: loop {
        match pc {
            0x8267F870 => {
    //   block [0x8267F870..0x8267F8E0)
	// 8267F870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F87C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F880: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F884: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267F888: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F88C: 390BB138  addi r8, r11, -0x4ec8
	ctx.r[8].s64 = ctx.r[11].s64 + -20168;
	// 8267F890: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8267F894: 388A3FAC  addi r4, r10, 0x3fac
	ctx.r[4].s64 = ctx.r[10].s64 + 16300;
	// 8267F898: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F89C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F8A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F8A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F8A8: 386A8148  addi r3, r10, -0x7eb8
	ctx.r[3].s64 = ctx.r[10].s64 + -32440;
	// 8267F8AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F8B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F8B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F8B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F8BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F8C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F8C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F8C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F8CC: 4BDE7555  bl 0x82466e20
	ctx.lr = 0x8267F8D0;
	sub_82466E20(ctx, base);
	// 8267F8D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F8E0 size=112
    let mut pc: u32 = 0x8267F8E0;
    'dispatch: loop {
        match pc {
            0x8267F8E0 => {
    //   block [0x8267F8E0..0x8267F950)
	// 8267F8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F8E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F8EC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F8F0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F8F4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267F8F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F8FC: 390BB1C8  addi r8, r11, -0x4e38
	ctx.r[8].s64 = ctx.r[11].s64 + -20024;
	// 8267F900: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8267F904: 388A3FDC  addi r4, r10, 0x3fdc
	ctx.r[4].s64 = ctx.r[10].s64 + 16348;
	// 8267F908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F90C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F910: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F914: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F918: 386A8178  addi r3, r10, -0x7e88
	ctx.r[3].s64 = ctx.r[10].s64 + -32392;
	// 8267F91C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F920: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F928: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F92C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F930: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F934: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F938: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F93C: 4BDE74E5  bl 0x82466e20
	ctx.lr = 0x8267F940;
	sub_82466E20(ctx, base);
	// 8267F940: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F944: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F948: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F950 size=112
    let mut pc: u32 = 0x8267F950;
    'dispatch: loop {
        match pc {
            0x8267F950 => {
    //   block [0x8267F950..0x8267F9C0)
	// 8267F950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F958: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F95C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F960: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F964: 38AA7CF8  addi r5, r10, 0x7cf8
	ctx.r[5].s64 = ctx.r[10].s64 + 31992;
	// 8267F968: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F96C: 390BB228  addi r8, r11, -0x4dd8
	ctx.r[8].s64 = ctx.r[11].s64 + -19928;
	// 8267F970: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267F974: 388A400C  addi r4, r10, 0x400c
	ctx.r[4].s64 = ctx.r[10].s64 + 16396;
	// 8267F978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F97C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F980: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F984: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F988: 386A81A8  addi r3, r10, -0x7e58
	ctx.r[3].s64 = ctx.r[10].s64 + -32344;
	// 8267F98C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267F990: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267F994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267F998: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267F99C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267F9A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267F9A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267F9A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267F9AC: 4BDE7475  bl 0x82466e20
	ctx.lr = 0x8267F9B0;
	sub_82466E20(ctx, base);
	// 8267F9B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267F9B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267F9B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267F9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267F9C0 size=112
    let mut pc: u32 = 0x8267F9C0;
    'dispatch: loop {
        match pc {
            0x8267F9C0 => {
    //   block [0x8267F9C0..0x8267FA30)
	// 8267F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267F9C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267F9CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267F9D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267F9D4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8267F9D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267F9DC: 390BB258  addi r8, r11, -0x4da8
	ctx.r[8].s64 = ctx.r[11].s64 + -19880;
	// 8267F9E0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8267F9E4: 388A4028  addi r4, r10, 0x4028
	ctx.r[4].s64 = ctx.r[10].s64 + 16424;
	// 8267F9E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267F9EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267F9F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267F9F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267F9F8: 386A81D8  addi r3, r10, -0x7e28
	ctx.r[3].s64 = ctx.r[10].s64 + -32296;
	// 8267F9FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FA00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FA04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FA0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FA14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FA1C: 4BDE7405  bl 0x82466e20
	ctx.lr = 0x8267FA20;
	sub_82466E20(ctx, base);
	// 8267FA20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FA24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FA28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FA30 size=112
    let mut pc: u32 = 0x8267FA30;
    'dispatch: loop {
        match pc {
            0x8267FA30 => {
    //   block [0x8267FA30..0x8267FAA0)
	// 8267FA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FA38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FA3C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FA40: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FA44: 38AA7E48  addi r5, r10, 0x7e48
	ctx.r[5].s64 = ctx.r[10].s64 + 32328;
	// 8267FA48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FA4C: 390BB2E8  addi r8, r11, -0x4d18
	ctx.r[8].s64 = ctx.r[11].s64 + -19736;
	// 8267FA50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267FA54: 388A404C  addi r4, r10, 0x404c
	ctx.r[4].s64 = ctx.r[10].s64 + 16460;
	// 8267FA58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FA5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FA60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FA64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FA68: 386A8208  addi r3, r10, -0x7df8
	ctx.r[3].s64 = ctx.r[10].s64 + -32248;
	// 8267FA6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FA70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FA74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FA78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FA7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FA80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FA88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FA8C: 4BDE7395  bl 0x82466e20
	ctx.lr = 0x8267FA90;
	sub_82466E20(ctx, base);
	// 8267FA90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FA94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FA98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FAA0 size=112
    let mut pc: u32 = 0x8267FAA0;
    'dispatch: loop {
        match pc {
            0x8267FAA0 => {
    //   block [0x8267FAA0..0x8267FB10)
	// 8267FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FAA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FAAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FAB0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FAB4: 38AA8058  addi r5, r10, -0x7fa8
	ctx.r[5].s64 = ctx.r[10].s64 + -32680;
	// 8267FAB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FABC: 390BB300  addi r8, r11, -0x4d00
	ctx.r[8].s64 = ctx.r[11].s64 + -19712;
	// 8267FAC0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267FAC4: 388A4060  addi r4, r10, 0x4060
	ctx.r[4].s64 = ctx.r[10].s64 + 16480;
	// 8267FAC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FACC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FAD0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FAD4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FAD8: 386A8238  addi r3, r10, -0x7dc8
	ctx.r[3].s64 = ctx.r[10].s64 + -32200;
	// 8267FADC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FAE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FAE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FAE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FAF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FAF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FAF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FAFC: 4BDE7325  bl 0x82466e20
	ctx.lr = 0x8267FB00;
	sub_82466E20(ctx, base);
	// 8267FB00: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FB10 size=112
    let mut pc: u32 = 0x8267FB10;
    'dispatch: loop {
        match pc {
            0x8267FB10 => {
    //   block [0x8267FB10..0x8267FB80)
	// 8267FB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FB18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FB1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FB20: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FB24: 38AA7938  addi r5, r10, 0x7938
	ctx.r[5].s64 = ctx.r[10].s64 + 31032;
	// 8267FB28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FB2C: 390BB330  addi r8, r11, -0x4cd0
	ctx.r[8].s64 = ctx.r[11].s64 + -19664;
	// 8267FB30: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8267FB34: 388A4084  addi r4, r10, 0x4084
	ctx.r[4].s64 = ctx.r[10].s64 + 16516;
	// 8267FB38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FB3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FB40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FB44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FB48: 386A8268  addi r3, r10, -0x7d98
	ctx.r[3].s64 = ctx.r[10].s64 + -32152;
	// 8267FB4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FB50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FB54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FB58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FB5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FB60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FB64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FB68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FB6C: 4BDE72B5  bl 0x82466e20
	ctx.lr = 0x8267FB70;
	sub_82466E20(ctx, base);
	// 8267FB70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FB74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FB78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267FB80 size=24
    let mut pc: u32 = 0x8267FB80;
    'dispatch: loop {
        match pc {
            0x8267FB80 => {
    //   block [0x8267FB80..0x8267FB98)
	// 8267FB80: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FB84: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267FB88: 394AF458  addi r10, r10, -0xba8
	ctx.r[10].s64 = ctx.r[10].s64 + -2984;
	// 8267FB8C: 816BB3D8  lwz r11, -0x4c28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 8267FB90: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8267FB94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FB98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FB98 size=116
    let mut pc: u32 = 0x8267FB98;
    'dispatch: loop {
        match pc {
            0x8267FB98 => {
    //   block [0x8267FB98..0x8267FC0C)
	// 8267FB98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FB9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FBA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FBA4: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267FBA8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8267FBAC: 390AF458  addi r8, r10, -0xba8
	ctx.r[8].s64 = ctx.r[10].s64 + -2984;
	// 8267FBB0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FBB4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 8267FBB8: 38AA7AE8  addi r5, r10, 0x7ae8
	ctx.r[5].s64 = ctx.r[10].s64 + 31464;
	// 8267FBBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FBC0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267FBC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FBC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FBCC: 388A40B8  addi r4, r10, 0x40b8
	ctx.r[4].s64 = ctx.r[10].s64 + 16568;
	// 8267FBD0: 396B47D0  addi r11, r11, 0x47d0
	ctx.r[11].s64 = ctx.r[11].s64 + 18384;
	// 8267FBD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FBD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FBDC: 386A8298  addi r3, r10, -0x7d68
	ctx.r[3].s64 = ctx.r[10].s64 + -32104;
	// 8267FBE0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8267FBE4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FBE8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8267FBEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FBF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FBF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FBF8: 4BDE7229  bl 0x82466e20
	ctx.lr = 0x8267FBFC;
	sub_82466E20(ctx, base);
	// 8267FBFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FC00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FC04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FC08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FC10 size=112
    let mut pc: u32 = 0x8267FC10;
    'dispatch: loop {
        match pc {
            0x8267FC10 => {
    //   block [0x8267FC10..0x8267FC80)
	// 8267FC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FC14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FC1C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FC20: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FC24: 38AA7A58  addi r5, r10, 0x7a58
	ctx.r[5].s64 = ctx.r[10].s64 + 31320;
	// 8267FC28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FC2C: 390BB378  addi r8, r11, -0x4c88
	ctx.r[8].s64 = ctx.r[11].s64 + -19592;
	// 8267FC30: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267FC34: 388A40C8  addi r4, r10, 0x40c8
	ctx.r[4].s64 = ctx.r[10].s64 + 16584;
	// 8267FC38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FC3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FC40: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FC44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FC48: 386A82C8  addi r3, r10, -0x7d38
	ctx.r[3].s64 = ctx.r[10].s64 + -32056;
	// 8267FC4C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FC50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FC54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FC58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FC5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FC60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FC64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FC68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FC6C: 4BDE71B5  bl 0x82466e20
	ctx.lr = 0x8267FC70;
	sub_82466E20(ctx, base);
	// 8267FC70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FC74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FC78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FC7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FC80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FC80 size=112
    let mut pc: u32 = 0x8267FC80;
    'dispatch: loop {
        match pc {
            0x8267FC80 => {
    //   block [0x8267FC80..0x8267FCF0)
	// 8267FC80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FC84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FC88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FC8C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 8267FC90: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FC94: 38AA7AB8  addi r5, r10, 0x7ab8
	ctx.r[5].s64 = ctx.r[10].s64 + 31416;
	// 8267FC98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FC9C: 390BB3A8  addi r8, r11, -0x4c58
	ctx.r[8].s64 = ctx.r[11].s64 + -19544;
	// 8267FCA0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267FCA4: 388A40EC  addi r4, r10, 0x40ec
	ctx.r[4].s64 = ctx.r[10].s64 + 16620;
	// 8267FCA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FCAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FCB0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FCB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FCB8: 386A82F8  addi r3, r10, -0x7d08
	ctx.r[3].s64 = ctx.r[10].s64 + -32008;
	// 8267FCBC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FCC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FCC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FCC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FCCC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FCD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FCD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FCD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FCDC: 4BDE7145  bl 0x82466e20
	ctx.lr = 0x8267FCE0;
	sub_82466E20(ctx, base);
	// 8267FCE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FCE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FCE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FCEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FCF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FCF0 size=100
    let mut pc: u32 = 0x8267FCF0;
    'dispatch: loop {
        match pc {
            0x8267FCF0 => {
    //   block [0x8267FCF0..0x8267FD54)
	// 8267FCF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FCF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FCF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FCFC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267FD00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FD04: 392A4840  addi r9, r10, 0x4840
	ctx.r[9].s64 = ctx.r[10].s64 + 18496;
	// 8267FD08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FD10: 388A4110  addi r4, r10, 0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + 16656;
	// 8267FD14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FD1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FD24: 386A8328  addi r3, r10, -0x7cd8
	ctx.r[3].s64 = ctx.r[10].s64 + -31960;
	// 8267FD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FD2C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8267FD30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267FD34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FD38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267FD3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267FD40: 4BDE70E1  bl 0x82466e20
	ctx.lr = 0x8267FD44;
	sub_82466E20(ctx, base);
	// 8267FD44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FD48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FD4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FD50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FD58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8267FD58 size=24
    let mut pc: u32 = 0x8267FD58;
    'dispatch: loop {
        match pc {
            0x8267FD58 => {
    //   block [0x8267FD58..0x8267FD70)
	// 8267FD58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FD5C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 8267FD60: 394AF500  addi r10, r10, -0xb00
	ctx.r[10].s64 = ctx.r[10].s64 + -2816;
	// 8267FD64: 816BB3E4  lwz r11, -0x4c1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19484 as u32) ) } as u64;
	// 8267FD68: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8267FD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FD70 size=112
    let mut pc: u32 = 0x8267FD70;
    'dispatch: loop {
        match pc {
            0x8267FD70 => {
    //   block [0x8267FD70..0x8267FDE0)
	// 8267FD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FD7C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 8267FD80: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FD84: 392A4980  addi r9, r10, 0x4980
	ctx.r[9].s64 = ctx.r[10].s64 + 18816;
	// 8267FD88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FD8C: 390BF500  addi r8, r11, -0xb00
	ctx.r[8].s64 = ctx.r[11].s64 + -2816;
	// 8267FD90: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8267FD94: 388A4124  addi r4, r10, 0x4124
	ctx.r[4].s64 = ctx.r[10].s64 + 16676;
	// 8267FD98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FD9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FDA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FDA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FDA8: 386A8358  addi r3, r10, -0x7ca8
	ctx.r[3].s64 = ctx.r[10].s64 + -31912;
	// 8267FDAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8267FDB0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8267FDB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FDB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FDBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FDC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FDC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267FDC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FDCC: 4BDE7055  bl 0x82466e20
	ctx.lr = 0x8267FDD0;
	sub_82466E20(ctx, base);
	// 8267FDD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FDD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FDD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FDE0 size=112
    let mut pc: u32 = 0x8267FDE0;
    'dispatch: loop {
        match pc {
            0x8267FDE0 => {
    //   block [0x8267FDE0..0x8267FE50)
	// 8267FDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FDE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FDEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FDF0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FDF4: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 8267FDF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FDFC: 390BB3EC  addi r8, r11, -0x4c14
	ctx.r[8].s64 = ctx.r[11].s64 + -19476;
	// 8267FE00: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267FE04: 388A4138  addi r4, r10, 0x4138
	ctx.r[4].s64 = ctx.r[10].s64 + 16696;
	// 8267FE08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FE0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FE10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FE14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FE18: 386A8388  addi r3, r10, -0x7c78
	ctx.r[3].s64 = ctx.r[10].s64 + -31864;
	// 8267FE1C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FE20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FE24: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FE28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FE2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FE30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FE34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FE38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FE3C: 4BDE6FE5  bl 0x82466e20
	ctx.lr = 0x8267FE40;
	sub_82466E20(ctx, base);
	// 8267FE40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FE44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FE48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FE50 size=108
    let mut pc: u32 = 0x8267FE50;
    'dispatch: loop {
        match pc {
            0x8267FE50 => {
    //   block [0x8267FE50..0x8267FEBC)
	// 8267FE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FE5C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FE60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FE64: 38EBB41C  addi r7, r11, -0x4be4
	ctx.r[7].s64 = ctx.r[11].s64 + -19428;
	// 8267FE68: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8267FE6C: 388A4150  addi r4, r10, 0x4150
	ctx.r[4].s64 = ctx.r[10].s64 + 16720;
	// 8267FE70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FE74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FE78: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8267FE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FE80: 386A83B8  addi r3, r10, -0x7c48
	ctx.r[3].s64 = ctx.r[10].s64 + -31816;
	// 8267FE84: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8267FE88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FE90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FE94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FE98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FEA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FEA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8267FEA8: 4BDE6F79  bl 0x82466e20
	ctx.lr = 0x8267FEAC;
	sub_82466E20(ctx, base);
	// 8267FEAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FEB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FEB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FEB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FEC0 size=100
    let mut pc: u32 = 0x8267FEC0;
    'dispatch: loop {
        match pc {
            0x8267FEC0 => {
    //   block [0x8267FEC0..0x8267FF24)
	// 8267FEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FEC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FECC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FED0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FED4: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 8267FED8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FEDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FEE0: 388A4160  addi r4, r10, 0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + 16736;
	// 8267FEE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FEE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FEEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FEF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FEF4: 386A83E8  addi r3, r10, -0x7c18
	ctx.r[3].s64 = ctx.r[10].s64 + -31768;
	// 8267FEF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FEFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FF00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8267FF04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FF08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8267FF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FF10: 4BDE6F11  bl 0x82466e20
	ctx.lr = 0x8267FF14;
	sub_82466E20(ctx, base);
	// 8267FF14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FF18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FF1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FF28 size=112
    let mut pc: u32 = 0x8267FF28;
    'dispatch: loop {
        match pc {
            0x8267FF28 => {
    //   block [0x8267FF28..0x8267FF98)
	// 8267FF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FF30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FF34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FF38: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FF3C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 8267FF40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FF44: 390BB434  addi r8, r11, -0x4bcc
	ctx.r[8].s64 = ctx.r[11].s64 + -19404;
	// 8267FF48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8267FF4C: 388A417C  addi r4, r10, 0x417c
	ctx.r[4].s64 = ctx.r[10].s64 + 16764;
	// 8267FF50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FF54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FF58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FF5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FF60: 386A8418  addi r3, r10, -0x7be8
	ctx.r[3].s64 = ctx.r[10].s64 + -31720;
	// 8267FF64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FF68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FF6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FF70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FF74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FF78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FF7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FF80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FF84: 4BDE6E9D  bl 0x82466e20
	ctx.lr = 0x8267FF88;
	sub_82466E20(ctx, base);
	// 8267FF88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FF8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8267FF90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8267FF94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8267FF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8267FF98 size=112
    let mut pc: u32 = 0x8267FF98;
    'dispatch: loop {
        match pc {
            0x8267FF98 => {
    //   block [0x8267FF98..0x82680008)
	// 8267FF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8267FF9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8267FFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8267FFA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FFA8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8267FFAC: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 8267FFB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8267FFB4: 390BB44C  addi r8, r11, -0x4bb4
	ctx.r[8].s64 = ctx.r[11].s64 + -19380;
	// 8267FFB8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8267FFBC: 388A4198  addi r4, r10, 0x4198
	ctx.r[4].s64 = ctx.r[10].s64 + 16792;
	// 8267FFC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8267FFC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8267FFC8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8267FFCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8267FFD0: 386A8448  addi r3, r10, -0x7bb8
	ctx.r[3].s64 = ctx.r[10].s64 + -31672;
	// 8267FFD4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8267FFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8267FFDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8267FFE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8267FFE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8267FFE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8267FFEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8267FFF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8267FFF4: 4BDE6E2D  bl 0x82466e20
	ctx.lr = 0x8267FFF8;
	sub_82466E20(ctx, base);
	// 8267FFF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8267FFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680008 size=112
    let mut pc: u32 = 0x82680008;
    'dispatch: loop {
        match pc {
            0x82680008 => {
    //   block [0x82680008..0x82680078)
	// 82680008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268000C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680010: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680014: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680018: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268001C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680020: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680024: 390BB47C  addi r8, r11, -0x4b84
	ctx.r[8].s64 = ctx.r[11].s64 + -19332;
	// 82680028: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268002C: 388A41BC  addi r4, r10, 0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + 16828;
	// 82680030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680034: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680038: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268003C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680040: 386A8478  addi r3, r10, -0x7b88
	ctx.r[3].s64 = ctx.r[10].s64 + -31624;
	// 82680044: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680048: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268004C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680050: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680054: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680058: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268005C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680060: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680064: 4BDE6DBD  bl 0x82466e20
	ctx.lr = 0x82680068;
	sub_82466E20(ctx, base);
	// 82680068: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268006C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680078 size=112
    let mut pc: u32 = 0x82680078;
    'dispatch: loop {
        match pc {
            0x82680078 => {
    //   block [0x82680078..0x826800E8)
	// 82680078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268007C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680088: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268008C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680090: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680094: 390BB4AC  addi r8, r11, -0x4b54
	ctx.r[8].s64 = ctx.r[11].s64 + -19284;
	// 82680098: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268009C: 388A41E4  addi r4, r10, 0x41e4
	ctx.r[4].s64 = ctx.r[10].s64 + 16868;
	// 826800A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826800A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826800A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826800AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826800B0: 386A84A8  addi r3, r10, -0x7b58
	ctx.r[3].s64 = ctx.r[10].s64 + -31576;
	// 826800B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826800B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826800BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826800C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826800C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826800C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826800CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826800D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826800D4: 4BDE6D4D  bl 0x82466e20
	ctx.lr = 0x826800D8;
	sub_82466E20(ctx, base);
	// 826800D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826800DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826800E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826800E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826800E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826800E8 size=112
    let mut pc: u32 = 0x826800E8;
    'dispatch: loop {
        match pc {
            0x826800E8 => {
    //   block [0x826800E8..0x82680158)
	// 826800E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826800EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826800F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826800F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826800F8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826800FC: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680100: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680104: 390BB4DC  addi r8, r11, -0x4b24
	ctx.r[8].s64 = ctx.r[11].s64 + -19236;
	// 82680108: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268010C: 388A4208  addi r4, r10, 0x4208
	ctx.r[4].s64 = ctx.r[10].s64 + 16904;
	// 82680110: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680114: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680118: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268011C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680120: 386A84D8  addi r3, r10, -0x7b28
	ctx.r[3].s64 = ctx.r[10].s64 + -31528;
	// 82680124: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680128: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268012C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680130: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680134: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680138: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268013C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680140: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680144: 4BDE6CDD  bl 0x82466e20
	ctx.lr = 0x82680148;
	sub_82466E20(ctx, base);
	// 82680148: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268014C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680158 size=112
    let mut pc: u32 = 0x82680158;
    'dispatch: loop {
        match pc {
            0x82680158 => {
    //   block [0x82680158..0x826801C8)
	// 82680158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268015C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680160: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680164: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680168: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268016C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680170: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680174: 390BB4F4  addi r8, r11, -0x4b0c
	ctx.r[8].s64 = ctx.r[11].s64 + -19212;
	// 82680178: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268017C: 388A4228  addi r4, r10, 0x4228
	ctx.r[4].s64 = ctx.r[10].s64 + 16936;
	// 82680180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680184: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680188: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268018C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680190: 386A8508  addi r3, r10, -0x7af8
	ctx.r[3].s64 = ctx.r[10].s64 + -31480;
	// 82680194: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680198: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268019C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826801A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826801A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826801A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826801AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826801B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826801B4: 4BDE6C6D  bl 0x82466e20
	ctx.lr = 0x826801B8;
	sub_82466E20(ctx, base);
	// 826801B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826801BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826801C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826801C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826801C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826801C8 size=112
    let mut pc: u32 = 0x826801C8;
    'dispatch: loop {
        match pc {
            0x826801C8 => {
    //   block [0x826801C8..0x82680238)
	// 826801C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826801CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826801D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826801D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826801D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826801DC: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 826801E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826801E4: 390BB510  addi r8, r11, -0x4af0
	ctx.r[8].s64 = ctx.r[11].s64 + -19184;
	// 826801E8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826801EC: 388A423C  addi r4, r10, 0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + 16956;
	// 826801F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826801F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826801F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826801FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680200: 386A8538  addi r3, r10, -0x7ac8
	ctx.r[3].s64 = ctx.r[10].s64 + -31432;
	// 82680204: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680208: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268020C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680210: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680218: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268021C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680220: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680224: 4BDE6BFD  bl 0x82466e20
	ctx.lr = 0x82680228;
	sub_82466E20(ctx, base);
	// 82680228: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268022C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680230: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680234: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680238 size=112
    let mut pc: u32 = 0x82680238;
    'dispatch: loop {
        match pc {
            0x82680238 => {
    //   block [0x82680238..0x826802A8)
	// 82680238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268023C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680248: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268024C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680250: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680254: 390BB558  addi r8, r11, -0x4aa8
	ctx.r[8].s64 = ctx.r[11].s64 + -19112;
	// 82680258: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268025C: 388A4254  addi r4, r10, 0x4254
	ctx.r[4].s64 = ctx.r[10].s64 + 16980;
	// 82680260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680264: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680268: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268026C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680270: 386A8568  addi r3, r10, -0x7a98
	ctx.r[3].s64 = ctx.r[10].s64 + -31384;
	// 82680274: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680278: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268027C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680280: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680288: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268028C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680290: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680294: 4BDE6B8D  bl 0x82466e20
	ctx.lr = 0x82680298;
	sub_82466E20(ctx, base);
	// 82680298: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268029C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826802A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826802A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826802A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826802A8 size=112
    let mut pc: u32 = 0x826802A8;
    'dispatch: loop {
        match pc {
            0x826802A8 => {
    //   block [0x826802A8..0x82680318)
	// 826802A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826802AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826802B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826802B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826802B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826802BC: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 826802C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826802C4: 390BB5A0  addi r8, r11, -0x4a60
	ctx.r[8].s64 = ctx.r[11].s64 + -19040;
	// 826802C8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826802CC: 388A4270  addi r4, r10, 0x4270
	ctx.r[4].s64 = ctx.r[10].s64 + 17008;
	// 826802D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826802D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826802D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826802DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826802E0: 386A8598  addi r3, r10, -0x7a68
	ctx.r[3].s64 = ctx.r[10].s64 + -31336;
	// 826802E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826802E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826802EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826802F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826802F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826802F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826802FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680300: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680304: 4BDE6B1D  bl 0x82466e20
	ctx.lr = 0x82680308;
	sub_82466E20(ctx, base);
	// 82680308: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268030C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680314: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680318 size=112
    let mut pc: u32 = 0x82680318;
    'dispatch: loop {
        match pc {
            0x82680318 => {
    //   block [0x82680318..0x82680388)
	// 82680318: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268031C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680320: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680324: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680328: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268032C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680330: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680334: 390BB5B8  addi r8, r11, -0x4a48
	ctx.r[8].s64 = ctx.r[11].s64 + -19016;
	// 82680338: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268033C: 388A4288  addi r4, r10, 0x4288
	ctx.r[4].s64 = ctx.r[10].s64 + 17032;
	// 82680340: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680344: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680348: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268034C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680350: 386A85C8  addi r3, r10, -0x7a38
	ctx.r[3].s64 = ctx.r[10].s64 + -31288;
	// 82680354: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680358: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268035C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680360: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680364: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680368: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268036C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680370: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680374: 4BDE6AAD  bl 0x82466e20
	ctx.lr = 0x82680378;
	sub_82466E20(ctx, base);
	// 82680378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268037C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680380: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680388 size=116
    let mut pc: u32 = 0x82680388;
    'dispatch: loop {
        match pc {
            0x82680388 => {
    //   block [0x82680388..0x826803FC)
	// 82680388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268038C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680394: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82680398: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8268039C: 390AB5E8  addi r8, r10, -0x4a18
	ctx.r[8].s64 = ctx.r[10].s64 + -18968;
	// 826803A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826803A4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826803A8: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 826803AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826803B0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826803B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826803B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826803BC: 388A429C  addi r4, r10, 0x429c
	ctx.r[4].s64 = ctx.r[10].s64 + 17052;
	// 826803C0: 396B49A8  addi r11, r11, 0x49a8
	ctx.r[11].s64 = ctx.r[11].s64 + 18856;
	// 826803C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826803C8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826803CC: 386A85F8  addi r3, r10, -0x7a08
	ctx.r[3].s64 = ctx.r[10].s64 + -31240;
	// 826803D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826803D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826803D8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826803DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826803E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826803E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826803E8: 4BDE6A39  bl 0x82466e20
	ctx.lr = 0x826803EC;
	sub_82466E20(ctx, base);
	// 826803EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826803F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826803F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826803F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680400 size=116
    let mut pc: u32 = 0x82680400;
    'dispatch: loop {
        match pc {
            0x82680400 => {
    //   block [0x82680400..0x82680474)
	// 82680400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268040C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82680410: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82680414: 390AB660  addi r8, r10, -0x49a0
	ctx.r[8].s64 = ctx.r[10].s64 + -18848;
	// 82680418: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268041C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82680420: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680424: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680428: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268042C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680430: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680434: 388A42B8  addi r4, r10, 0x42b8
	ctx.r[4].s64 = ctx.r[10].s64 + 17080;
	// 82680438: 396B49C0  addi r11, r11, 0x49c0
	ctx.r[11].s64 = ctx.r[11].s64 + 18880;
	// 8268043C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680440: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680444: 386A8628  addi r3, r10, -0x79d8
	ctx.r[3].s64 = ctx.r[10].s64 + -31192;
	// 82680448: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8268044C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680450: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82680454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680458: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268045C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680460: 4BDE69C1  bl 0x82466e20
	ctx.lr = 0x82680464;
	sub_82466E20(ctx, base);
	// 82680464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268046C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82680478 size=24
    let mut pc: u32 = 0x82680478;
    'dispatch: loop {
        match pc {
            0x82680478 => {
    //   block [0x82680478..0x82680490)
	// 82680478: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268047C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82680480: 394AF518  addi r10, r10, -0xae8
	ctx.r[10].s64 = ctx.r[10].s64 + -2792;
	// 82680484: 816BB50C  lwz r11, -0x4af4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19188 as u32) ) } as u64;
	// 82680488: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8268048C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680490 size=116
    let mut pc: u32 = 0x82680490;
    'dispatch: loop {
        match pc {
            0x82680490 => {
    //   block [0x82680490..0x82680504)
	// 82680490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268049C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 826804A0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826804A4: 392B49EC  addi r9, r11, 0x49ec
	ctx.r[9].s64 = ctx.r[11].s64 + 18924;
	// 826804A8: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 826804AC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826804B0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 826804B4: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 826804B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826804BC: 388A42D4  addi r4, r10, 0x42d4
	ctx.r[4].s64 = ctx.r[10].s64 + 17108;
	// 826804C0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826804C4: 396BF518  addi r11, r11, -0xae8
	ctx.r[11].s64 = ctx.r[11].s64 + -2792;
	// 826804C8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826804CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826804D0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826804D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826804D8: 386A8658  addi r3, r10, -0x79a8
	ctx.r[3].s64 = ctx.r[10].s64 + -31144;
	// 826804DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826804E0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826804E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826804E8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826804EC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826804F0: 4BDE6931  bl 0x82466e20
	ctx.lr = 0x826804F4;
	sub_82466E20(ctx, base);
	// 826804F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826804F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826804FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680500: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680508 size=112
    let mut pc: u32 = 0x82680508;
    'dispatch: loop {
        match pc {
            0x82680508 => {
    //   block [0x82680508..0x82680578)
	// 82680508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268050C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680514: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680518: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268051C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680524: 390BB6F0  addi r8, r11, -0x4910
	ctx.r[8].s64 = ctx.r[11].s64 + -18704;
	// 82680528: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268052C: 388A42F0  addi r4, r10, 0x42f0
	ctx.r[4].s64 = ctx.r[10].s64 + 17136;
	// 82680530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680534: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680538: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268053C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680540: 386A8688  addi r3, r10, -0x7978
	ctx.r[3].s64 = ctx.r[10].s64 + -31096;
	// 82680544: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268054C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680554: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268055C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680564: 4BDE68BD  bl 0x82466e20
	ctx.lr = 0x82680568;
	sub_82466E20(ctx, base);
	// 82680568: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268056C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680578 size=112
    let mut pc: u32 = 0x82680578;
    'dispatch: loop {
        match pc {
            0x82680578 => {
    //   block [0x82680578..0x826805E8)
	// 82680578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268057C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680584: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680588: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268058C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680594: 390BB750  addi r8, r11, -0x48b0
	ctx.r[8].s64 = ctx.r[11].s64 + -18608;
	// 82680598: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8268059C: 388A430C  addi r4, r10, 0x430c
	ctx.r[4].s64 = ctx.r[10].s64 + 17164;
	// 826805A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826805A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826805A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826805AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826805B0: 386A86B8  addi r3, r10, -0x7948
	ctx.r[3].s64 = ctx.r[10].s64 + -31048;
	// 826805B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826805B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826805BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826805C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826805C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826805C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826805CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826805D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826805D4: 4BDE684D  bl 0x82466e20
	ctx.lr = 0x826805D8;
	sub_82466E20(ctx, base);
	// 826805D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826805DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826805E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826805E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826805E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826805E8 size=112
    let mut pc: u32 = 0x826805E8;
    'dispatch: loop {
        match pc {
            0x826805E8 => {
    //   block [0x826805E8..0x82680658)
	// 826805E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826805EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826805F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826805F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826805F8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826805FC: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680604: 390BB7F8  addi r8, r11, -0x4808
	ctx.r[8].s64 = ctx.r[11].s64 + -18440;
	// 82680608: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8268060C: 388A4328  addi r4, r10, 0x4328
	ctx.r[4].s64 = ctx.r[10].s64 + 17192;
	// 82680610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680614: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680618: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268061C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680620: 386A86E8  addi r3, r10, -0x7918
	ctx.r[3].s64 = ctx.r[10].s64 + -31000;
	// 82680624: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268062C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268063C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680644: 4BDE67DD  bl 0x82466e20
	ctx.lr = 0x82680648;
	sub_82466E20(ctx, base);
	// 82680648: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268064C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680658 size=112
    let mut pc: u32 = 0x82680658;
    'dispatch: loop {
        match pc {
            0x82680658 => {
    //   block [0x82680658..0x826806C8)
	// 82680658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268065C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680664: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680668: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268066C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680670: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680674: 390BB870  addi r8, r11, -0x4790
	ctx.r[8].s64 = ctx.r[11].s64 + -18320;
	// 82680678: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268067C: 388A4348  addi r4, r10, 0x4348
	ctx.r[4].s64 = ctx.r[10].s64 + 17224;
	// 82680680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680684: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680688: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268068C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680690: 386A8718  addi r3, r10, -0x78e8
	ctx.r[3].s64 = ctx.r[10].s64 + -30952;
	// 82680694: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268069C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826806A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826806A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826806A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826806AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826806B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826806B4: 4BDE676D  bl 0x82466e20
	ctx.lr = 0x826806B8;
	sub_82466E20(ctx, base);
	// 826806B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826806BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826806C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826806C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826806C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826806C8 size=112
    let mut pc: u32 = 0x826806C8;
    'dispatch: loop {
        match pc {
            0x826806C8 => {
    //   block [0x826806C8..0x82680738)
	// 826806C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826806CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826806D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826806D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826806D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826806DC: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 826806E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826806E4: 390BB8B8  addi r8, r11, -0x4748
	ctx.r[8].s64 = ctx.r[11].s64 + -18248;
	// 826806E8: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826806EC: 388A4364  addi r4, r10, 0x4364
	ctx.r[4].s64 = ctx.r[10].s64 + 17252;
	// 826806F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826806F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826806F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826806FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680700: 386A8748  addi r3, r10, -0x78b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30904;
	// 82680704: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268070C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680714: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268071C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680724: 4BDE66FD  bl 0x82466e20
	ctx.lr = 0x82680728;
	sub_82466E20(ctx, base);
	// 82680728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268072C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680730: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680738 size=112
    let mut pc: u32 = 0x82680738;
    'dispatch: loop {
        match pc {
            0x82680738 => {
    //   block [0x82680738..0x826807A8)
	// 82680738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268073C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680744: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680748: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268074C: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 82680750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680754: 390BB948  addi r8, r11, -0x46b8
	ctx.r[8].s64 = ctx.r[11].s64 + -18104;
	// 82680758: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8268075C: 388A4380  addi r4, r10, 0x4380
	ctx.r[4].s64 = ctx.r[10].s64 + 17280;
	// 82680760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680764: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680768: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268076C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680770: 386A8778  addi r3, r10, -0x7888
	ctx.r[3].s64 = ctx.r[10].s64 + -30856;
	// 82680774: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268077C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680784: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268078C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680794: 4BDE668D  bl 0x82466e20
	ctx.lr = 0x82680798;
	sub_82466E20(ctx, base);
	// 82680798: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268079C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826807A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826807A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826807A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826807A8 size=112
    let mut pc: u32 = 0x826807A8;
    'dispatch: loop {
        match pc {
            0x826807A8 => {
    //   block [0x826807A8..0x82680818)
	// 826807A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826807AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826807B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826807B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826807B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826807BC: 38AA8358  addi r5, r10, -0x7ca8
	ctx.r[5].s64 = ctx.r[10].s64 + -31912;
	// 826807C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826807C4: 390BB9A8  addi r8, r11, -0x4658
	ctx.r[8].s64 = ctx.r[11].s64 + -18008;
	// 826807C8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826807CC: 388A4398  addi r4, r10, 0x4398
	ctx.r[4].s64 = ctx.r[10].s64 + 17304;
	// 826807D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826807D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826807D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826807DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826807E0: 386A87A8  addi r3, r10, -0x7858
	ctx.r[3].s64 = ctx.r[10].s64 + -30808;
	// 826807E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826807E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826807EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826807F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826807F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826807F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826807FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680800: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680804: 4BDE661D  bl 0x82466e20
	ctx.lr = 0x82680808;
	sub_82466E20(ctx, base);
	// 82680808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268080C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680818 size=112
    let mut pc: u32 = 0x82680818;
    'dispatch: loop {
        match pc {
            0x82680818 => {
    //   block [0x82680818..0x82680888)
	// 82680818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268081C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680824: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680828: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268082C: 38AA87A8  addi r5, r10, -0x7858
	ctx.r[5].s64 = ctx.r[10].s64 + -30808;
	// 82680830: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680834: 390BBA08  addi r8, r11, -0x45f8
	ctx.r[8].s64 = ctx.r[11].s64 + -17912;
	// 82680838: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8268083C: 388A43B4  addi r4, r10, 0x43b4
	ctx.r[4].s64 = ctx.r[10].s64 + 17332;
	// 82680840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680844: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680848: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268084C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680850: 386A87D8  addi r3, r10, -0x7828
	ctx.r[3].s64 = ctx.r[10].s64 + -30760;
	// 82680854: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680858: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268085C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680860: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680868: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268086C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680870: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680874: 4BDE65AD  bl 0x82466e20
	ctx.lr = 0x82680878;
	sub_82466E20(ctx, base);
	// 82680878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268087C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680888 size=112
    let mut pc: u32 = 0x82680888;
    'dispatch: loop {
        match pc {
            0x82680888 => {
    //   block [0x82680888..0x826808F8)
	// 82680888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268088C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680894: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680898: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268089C: 38AA87A8  addi r5, r10, -0x7858
	ctx.r[5].s64 = ctx.r[10].s64 + -30808;
	// 826808A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826808A4: 390BBA38  addi r8, r11, -0x45c8
	ctx.r[8].s64 = ctx.r[11].s64 + -17864;
	// 826808A8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826808AC: 388A43D8  addi r4, r10, 0x43d8
	ctx.r[4].s64 = ctx.r[10].s64 + 17368;
	// 826808B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826808B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826808B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826808BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826808C0: 386A8808  addi r3, r10, -0x77f8
	ctx.r[3].s64 = ctx.r[10].s64 + -30712;
	// 826808C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826808C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826808CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826808D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826808D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826808D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826808DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826808E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826808E4: 4BDE653D  bl 0x82466e20
	ctx.lr = 0x826808E8;
	sub_82466E20(ctx, base);
	// 826808E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826808EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826808F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826808F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826808F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826808F8 size=100
    let mut pc: u32 = 0x826808F8;
    'dispatch: loop {
        match pc {
            0x826808F8 => {
    //   block [0x826808F8..0x8268095C)
	// 826808F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826808FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680904: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268090C: 38AA87A8  addi r5, r10, -0x7858
	ctx.r[5].s64 = ctx.r[10].s64 + -30808;
	// 82680910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680914: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680918: 388A43FC  addi r4, r10, 0x43fc
	ctx.r[4].s64 = ctx.r[10].s64 + 17404;
	// 8268091C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680920: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680924: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680928: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268092C: 386A8838  addi r3, r10, -0x77c8
	ctx.r[3].s64 = ctx.r[10].s64 + -30664;
	// 82680930: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680934: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680938: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268093C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680940: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82680944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680948: 4BDE64D9  bl 0x82466e20
	ctx.lr = 0x8268094C;
	sub_82466E20(ctx, base);
	// 8268094C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680950: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680954: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680960 size=112
    let mut pc: u32 = 0x82680960;
    'dispatch: loop {
        match pc {
            0x82680960 => {
    //   block [0x82680960..0x826809D0)
	// 82680960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268096C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680970: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680974: 38AA87A8  addi r5, r10, -0x7858
	ctx.r[5].s64 = ctx.r[10].s64 + -30808;
	// 82680978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268097C: 390BBA68  addi r8, r11, -0x4598
	ctx.r[8].s64 = ctx.r[11].s64 + -17816;
	// 82680980: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82680984: 388A4424  addi r4, r10, 0x4424
	ctx.r[4].s64 = ctx.r[10].s64 + 17444;
	// 82680988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268098C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680998: 386A8868  addi r3, r10, -0x7798
	ctx.r[3].s64 = ctx.r[10].s64 + -30616;
	// 8268099C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826809A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826809A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826809A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826809AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826809B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826809B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826809B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826809BC: 4BDE6465  bl 0x82466e20
	ctx.lr = 0x826809C0;
	sub_82466E20(ctx, base);
	// 826809C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826809C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826809C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826809CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826809D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826809D0 size=112
    let mut pc: u32 = 0x826809D0;
    'dispatch: loop {
        match pc {
            0x826809D0 => {
    //   block [0x826809D0..0x82680A40)
	// 826809D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826809D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826809D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826809DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826809E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826809E4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826809E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826809EC: 390BBA80  addi r8, r11, -0x4580
	ctx.r[8].s64 = ctx.r[11].s64 + -17792;
	// 826809F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826809F4: 388A444C  addi r4, r10, 0x444c
	ctx.r[4].s64 = ctx.r[10].s64 + 17484;
	// 826809F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826809FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680A08: 386A8898  addi r3, r10, -0x7768
	ctx.r[3].s64 = ctx.r[10].s64 + -30568;
	// 82680A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680A2C: 4BDE63F5  bl 0x82466e20
	ctx.lr = 0x82680A30;
	sub_82466E20(ctx, base);
	// 82680A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680A40 size=112
    let mut pc: u32 = 0x82680A40;
    'dispatch: loop {
        match pc {
            0x82680A40 => {
    //   block [0x82680A40..0x82680AB0)
	// 82680A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680A4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680A50: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680A54: 38AA8898  addi r5, r10, -0x7768
	ctx.r[5].s64 = ctx.r[10].s64 + -30568;
	// 82680A58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680A5C: 390BBAE0  addi r8, r11, -0x4520
	ctx.r[8].s64 = ctx.r[11].s64 + -17696;
	// 82680A60: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82680A64: 388A4458  addi r4, r10, 0x4458
	ctx.r[4].s64 = ctx.r[10].s64 + 17496;
	// 82680A68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680A6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680A74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680A78: 386A88C8  addi r3, r10, -0x7738
	ctx.r[3].s64 = ctx.r[10].s64 + -30520;
	// 82680A7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680A80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680A88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680A8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680A90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680A9C: 4BDE6385  bl 0x82466e20
	ctx.lr = 0x82680AA0;
	sub_82466E20(ctx, base);
	// 82680AA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680AB0 size=112
    let mut pc: u32 = 0x82680AB0;
    'dispatch: loop {
        match pc {
            0x82680AB0 => {
    //   block [0x82680AB0..0x82680B20)
	// 82680AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680ABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680AC0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680AC4: 38AA8898  addi r5, r10, -0x7768
	ctx.r[5].s64 = ctx.r[10].s64 + -30568;
	// 82680AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680ACC: 390BBAF8  addi r8, r11, -0x4508
	ctx.r[8].s64 = ctx.r[11].s64 + -17672;
	// 82680AD0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82680AD4: 388A4468  addi r4, r10, 0x4468
	ctx.r[4].s64 = ctx.r[10].s64 + 17512;
	// 82680AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680ADC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680AE8: 386A88F8  addi r3, r10, -0x7708
	ctx.r[3].s64 = ctx.r[10].s64 + -30472;
	// 82680AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680B0C: 4BDE6315  bl 0x82466e20
	ctx.lr = 0x82680B10;
	sub_82466E20(ctx, base);
	// 82680B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680B20 size=112
    let mut pc: u32 = 0x82680B20;
    'dispatch: loop {
        match pc {
            0x82680B20 => {
    //   block [0x82680B20..0x82680B90)
	// 82680B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680B2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680B30: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680B34: 38AA8898  addi r5, r10, -0x7768
	ctx.r[5].s64 = ctx.r[10].s64 + -30568;
	// 82680B38: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680B3C: 390BBB28  addi r8, r11, -0x44d8
	ctx.r[8].s64 = ctx.r[11].s64 + -17624;
	// 82680B40: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82680B44: 388A4478  addi r4, r10, 0x4478
	ctx.r[4].s64 = ctx.r[10].s64 + 17528;
	// 82680B48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680B4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680B50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680B54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680B58: 386A8928  addi r3, r10, -0x76d8
	ctx.r[3].s64 = ctx.r[10].s64 + -30424;
	// 82680B5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680B60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680B68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680B6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680B70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680B74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680B78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680B7C: 4BDE62A5  bl 0x82466e20
	ctx.lr = 0x82680B80;
	sub_82466E20(ctx, base);
	// 82680B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680B84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680B88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680B8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82680B90 size=24
    let mut pc: u32 = 0x82680B90;
    'dispatch: loop {
        match pc {
            0x82680B90 => {
    //   block [0x82680B90..0x82680BA8)
	// 82680B90: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680B94: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82680B98: 394AF5C0  addi r10, r10, -0xa40
	ctx.r[10].s64 = ctx.r[10].s64 + -2624;
	// 82680B9C: 816BBB40  lwz r11, -0x44c0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17600 as u32) ) } as u64;
	// 82680BA0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82680BA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680BA8 size=112
    let mut pc: u32 = 0x82680BA8;
    'dispatch: loop {
        match pc {
            0x82680BA8 => {
    //   block [0x82680BA8..0x82680C18)
	// 82680BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680BB4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82680BB8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680BBC: 392A4A48  addi r9, r10, 0x4a48
	ctx.r[9].s64 = ctx.r[10].s64 + 19016;
	// 82680BC0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680BC4: 390BF5C0  addi r8, r11, -0xa40
	ctx.r[8].s64 = ctx.r[11].s64 + -2624;
	// 82680BC8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82680BCC: 388A4488  addi r4, r10, 0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + 17544;
	// 82680BD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680BD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680BD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680BDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680BE0: 386A8958  addi r3, r10, -0x76a8
	ctx.r[3].s64 = ctx.r[10].s64 + -30376;
	// 82680BE4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82680BE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82680BEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680BF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680BF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680BF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82680C00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680C04: 4BDE621D  bl 0x82466e20
	ctx.lr = 0x82680C08;
	sub_82466E20(ctx, base);
	// 82680C08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680C0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680C10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680C18 size=108
    let mut pc: u32 = 0x82680C18;
    'dispatch: loop {
        match pc {
            0x82680C18 => {
    //   block [0x82680C18..0x82680C84)
	// 82680C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680C24: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680C28: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680C2C: 38EBBB44  addi r7, r11, -0x44bc
	ctx.r[7].s64 = ctx.r[11].s64 + -17596;
	// 82680C30: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82680C34: 388A4494  addi r4, r10, 0x4494
	ctx.r[4].s64 = ctx.r[10].s64 + 17556;
	// 82680C38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680C3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680C40: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82680C44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680C48: 386A8988  addi r3, r10, -0x7678
	ctx.r[3].s64 = ctx.r[10].s64 + -30328;
	// 82680C4C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82680C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680C54: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680C58: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680C5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680C60: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680C64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680C68: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680C6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82680C70: 4BDE61B1  bl 0x82466e20
	ctx.lr = 0x82680C74;
	sub_82466E20(ctx, base);
	// 82680C74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680C80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680C88 size=108
    let mut pc: u32 = 0x82680C88;
    'dispatch: loop {
        match pc {
            0x82680C88 => {
    //   block [0x82680C88..0x82680CF4)
	// 82680C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680C8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680C90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680C94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680C98: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680C9C: 38EBBB60  addi r7, r11, -0x44a0
	ctx.r[7].s64 = ctx.r[11].s64 + -17568;
	// 82680CA0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82680CA4: 388A44A4  addi r4, r10, 0x44a4
	ctx.r[4].s64 = ctx.r[10].s64 + 17572;
	// 82680CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680CAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680CB0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82680CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680CB8: 386A89B8  addi r3, r10, -0x7648
	ctx.r[3].s64 = ctx.r[10].s64 + -30280;
	// 82680CBC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82680CC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680CC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680CC8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680CCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680CD0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680CD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680CD8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680CDC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82680CE0: 4BDE6141  bl 0x82466e20
	ctx.lr = 0x82680CE4;
	sub_82466E20(ctx, base);
	// 82680CE4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680CF8 size=116
    let mut pc: u32 = 0x82680CF8;
    'dispatch: loop {
        match pc {
            0x82680CF8 => {
    //   block [0x82680CF8..0x82680D6C)
	// 82680CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680D00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680D04: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680D08: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82680D0C: 390BBBA8  addi r8, r11, -0x4458
	ctx.r[8].s64 = ctx.r[11].s64 + -17496;
	// 82680D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680D14: 392A4B00  addi r9, r10, 0x4b00
	ctx.r[9].s64 = ctx.r[10].s64 + 19200;
	// 82680D18: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82680D1C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82680D20: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82680D24: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680D28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680D2C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680D30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680D34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680D38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680D3C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82680D40: 388A44B0  addi r4, r10, 0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17584;
	// 82680D44: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82680D48: 386B89E8  addi r3, r11, -0x7618
	ctx.r[3].s64 = ctx.r[11].s64 + -30232;
	// 82680D4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82680D50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680D54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680D58: 4BDE60C9  bl 0x82466e20
	ctx.lr = 0x82680D5C;
	sub_82466E20(ctx, base);
	// 82680D5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680D68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82680D70 size=24
    let mut pc: u32 = 0x82680D70;
    'dispatch: loop {
        match pc {
            0x82680D70 => {
    //   block [0x82680D70..0x82680D88)
	// 82680D70: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680D74: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82680D78: 394AF608  addi r10, r10, -0x9f8
	ctx.r[10].s64 = ctx.r[10].s64 + -2552;
	// 82680D7C: 816BBBC0  lwz r11, -0x4440(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17472 as u32) ) } as u64;
	// 82680D80: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82680D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680D88 size=116
    let mut pc: u32 = 0x82680D88;
    'dispatch: loop {
        match pc {
            0x82680D88 => {
    //   block [0x82680D88..0x82680DFC)
	// 82680D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680D90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680D94: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680D98: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82680D9C: 390BF608  addi r8, r11, -0x9f8
	ctx.r[8].s64 = ctx.r[11].s64 + -2552;
	// 82680DA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680DA4: 392A4B5C  addi r9, r10, 0x4b5c
	ctx.r[9].s64 = ctx.r[10].s64 + 19292;
	// 82680DA8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82680DAC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82680DB0: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82680DB4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680DB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680DBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680DC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680DC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680DC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680DCC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82680DD0: 388A44DC  addi r4, r10, 0x44dc
	ctx.r[4].s64 = ctx.r[10].s64 + 17628;
	// 82680DD4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82680DD8: 386B8A18  addi r3, r11, -0x75e8
	ctx.r[3].s64 = ctx.r[11].s64 + -30184;
	// 82680DDC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82680DE0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680DE8: 4BDE6039  bl 0x82466e20
	ctx.lr = 0x82680DEC;
	sub_82466E20(ctx, base);
	// 82680DEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680DF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680DF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680DF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680E00 size=108
    let mut pc: u32 = 0x82680E00;
    'dispatch: loop {
        match pc {
            0x82680E00 => {
    //   block [0x82680E00..0x82680E6C)
	// 82680E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680E04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680E08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680E0C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680E10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680E14: 38EBBBCC  addi r7, r11, -0x4434
	ctx.r[7].s64 = ctx.r[11].s64 + -17460;
	// 82680E18: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82680E1C: 388A44F4  addi r4, r10, 0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + 17652;
	// 82680E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680E24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680E28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82680E2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680E30: 386A8A48  addi r3, r10, -0x75b8
	ctx.r[3].s64 = ctx.r[10].s64 + -30136;
	// 82680E34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82680E38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680E3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680E40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680E44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680E48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680E4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680E50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680E54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82680E58: 4BDE5FC9  bl 0x82466e20
	ctx.lr = 0x82680E5C;
	sub_82466E20(ctx, base);
	// 82680E5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680E60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680E64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680E70 size=112
    let mut pc: u32 = 0x82680E70;
    'dispatch: loop {
        match pc {
            0x82680E70 => {
    //   block [0x82680E70..0x82680EE0)
	// 82680E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680E74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680E7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680E80: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680E84: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82680E88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680E8C: 390BBBFC  addi r8, r11, -0x4404
	ctx.r[8].s64 = ctx.r[11].s64 + -17412;
	// 82680E90: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82680E94: 388A4518  addi r4, r10, 0x4518
	ctx.r[4].s64 = ctx.r[10].s64 + 17688;
	// 82680E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680E9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680EA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680EA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680EA8: 386A8A78  addi r3, r10, -0x7588
	ctx.r[3].s64 = ctx.r[10].s64 + -30088;
	// 82680EAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680EB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680EB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680EB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680EC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680EC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680EC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680ECC: 4BDE5F55  bl 0x82466e20
	ctx.lr = 0x82680ED0;
	sub_82466E20(ctx, base);
	// 82680ED0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680ED4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680ED8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680EDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680EE0 size=112
    let mut pc: u32 = 0x82680EE0;
    'dispatch: loop {
        match pc {
            0x82680EE0 => {
    //   block [0x82680EE0..0x82680F50)
	// 82680EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680EE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680EE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680EEC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82680EF0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680EF4: 392A4BA0  addi r9, r10, 0x4ba0
	ctx.r[9].s64 = ctx.r[10].s64 + 19360;
	// 82680EF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680EFC: 390BBC18  addi r8, r11, -0x43e8
	ctx.r[8].s64 = ctx.r[11].s64 + -17384;
	// 82680F00: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82680F04: 388A4538  addi r4, r10, 0x4538
	ctx.r[4].s64 = ctx.r[10].s64 + 17720;
	// 82680F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680F0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680F10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680F14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680F18: 386A8AA8  addi r3, r10, -0x7558
	ctx.r[3].s64 = ctx.r[10].s64 + -30040;
	// 82680F1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82680F20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82680F24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680F28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680F30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680F34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82680F38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680F3C: 4BDE5EE5  bl 0x82466e20
	ctx.lr = 0x82680F40;
	sub_82466E20(ctx, base);
	// 82680F40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680F50 size=112
    let mut pc: u32 = 0x82680F50;
    'dispatch: loop {
        match pc {
            0x82680F50 => {
    //   block [0x82680F50..0x82680FC0)
	// 82680F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680F58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680F5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680F60: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680F64: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82680F68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680F6C: 390BBC60  addi r8, r11, -0x43a0
	ctx.r[8].s64 = ctx.r[11].s64 + -17312;
	// 82680F70: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82680F74: 388A4554  addi r4, r10, 0x4554
	ctx.r[4].s64 = ctx.r[10].s64 + 17748;
	// 82680F78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680F7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680F80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82680F88: 386A8AD8  addi r3, r10, -0x7528
	ctx.r[3].s64 = ctx.r[10].s64 + -29992;
	// 82680F8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82680F90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82680F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82680F98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82680F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82680FA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82680FA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680FA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82680FAC: 4BDE5E75  bl 0x82466e20
	ctx.lr = 0x82680FB0;
	sub_82466E20(ctx, base);
	// 82680FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82680FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82680FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82680FBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82680FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82680FC0 size=112
    let mut pc: u32 = 0x82680FC0;
    'dispatch: loop {
        match pc {
            0x82680FC0 => {
    //   block [0x82680FC0..0x82681030)
	// 82680FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82680FC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82680FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82680FCC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82680FD0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82680FD4: 392A4BCC  addi r9, r10, 0x4bcc
	ctx.r[9].s64 = ctx.r[10].s64 + 19404;
	// 82680FD8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82680FDC: 390BBC80  addi r8, r11, -0x4380
	ctx.r[8].s64 = ctx.r[11].s64 + -17280;
	// 82680FE0: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82680FE4: 388A456C  addi r4, r10, 0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + 17772;
	// 82680FE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82680FEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82680FF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82680FF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82680FF8: 386A8B08  addi r3, r10, -0x74f8
	ctx.r[3].s64 = ctx.r[10].s64 + -29944;
	// 82680FFC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82681000: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82681004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268100C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681014: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268101C: 4BDE5E05  bl 0x82466e20
	ctx.lr = 0x82681020;
	sub_82466E20(ctx, base);
	// 82681020: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268102C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681030 size=112
    let mut pc: u32 = 0x82681030;
    'dispatch: loop {
        match pc {
            0x82681030 => {
    //   block [0x82681030..0x826810A0)
	// 82681030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268103C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681040: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681044: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82681048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268104C: 390BBD10  addi r8, r11, -0x42f0
	ctx.r[8].s64 = ctx.r[11].s64 + -17136;
	// 82681050: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82681054: 388A4590  addi r4, r10, 0x4590
	ctx.r[4].s64 = ctx.r[10].s64 + 17808;
	// 82681058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268105C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681064: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681068: 386A8B38  addi r3, r10, -0x74c8
	ctx.r[3].s64 = ctx.r[10].s64 + -29896;
	// 8268106C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681070: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268107C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268108C: 4BDE5D95  bl 0x82466e20
	ctx.lr = 0x82681090;
	sub_82466E20(ctx, base);
	// 82681090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268109C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826810A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826810A0 size=112
    let mut pc: u32 = 0x826810A0;
    'dispatch: loop {
        match pc {
            0x826810A0 => {
    //   block [0x826810A0..0x82681110)
	// 826810A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826810A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826810A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826810AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826810B0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826810B4: 38AA8B98  addi r5, r10, -0x7468
	ctx.r[5].s64 = ctx.r[10].s64 + -29800;
	// 826810B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826810BC: 390BBD28  addi r8, r11, -0x42d8
	ctx.r[8].s64 = ctx.r[11].s64 + -17112;
	// 826810C0: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826810C4: 388A45B0  addi r4, r10, 0x45b0
	ctx.r[4].s64 = ctx.r[10].s64 + 17840;
	// 826810C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826810CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826810D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826810D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826810D8: 386A8B68  addi r3, r10, -0x7498
	ctx.r[3].s64 = ctx.r[10].s64 + -29848;
	// 826810DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826810E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826810E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826810E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826810EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826810F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826810F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826810F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826810FC: 4BDE5D25  bl 0x82466e20
	ctx.lr = 0x82681100;
	sub_82466E20(ctx, base);
	// 82681100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268110C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681110 size=100
    let mut pc: u32 = 0x82681110;
    'dispatch: loop {
        match pc {
            0x82681110 => {
    //   block [0x82681110..0x82681174)
	// 82681110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268111C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82681120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681124: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82681128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268112C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681130: 388A45C8  addi r4, r10, 0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + 17864;
	// 82681134: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268113C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681144: 386A8B98  addi r3, r10, -0x7468
	ctx.r[3].s64 = ctx.r[10].s64 + -29800;
	// 82681148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268114C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681150: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82681154: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681158: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268115C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681160: 4BDE5CC1  bl 0x82466e20
	ctx.lr = 0x82681164;
	sub_82466E20(ctx, base);
	// 82681164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268116C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681170: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82681178 size=24
    let mut pc: u32 = 0x82681178;
    'dispatch: loop {
        match pc {
            0x82681178 => {
    //   block [0x82681178..0x82681190)
	// 82681178: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268117C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82681180: 394AF6E0  addi r10, r10, -0x920
	ctx.r[10].s64 = ctx.r[10].s64 + -2336;
	// 82681184: 816BBC7C  lwz r11, -0x4384(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17284 as u32) ) } as u64;
	// 82681188: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8268118C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681190 size=116
    let mut pc: u32 = 0x82681190;
    'dispatch: loop {
        match pc {
            0x82681190 => {
    //   block [0x82681190..0x82681204)
	// 82681190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268119C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826811A0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826811A4: 390BF6E0  addi r8, r11, -0x920
	ctx.r[8].s64 = ctx.r[11].s64 + -2336;
	// 826811A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826811AC: 392A4C08  addi r9, r10, 0x4c08
	ctx.r[9].s64 = ctx.r[10].s64 + 19464;
	// 826811B0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826811B4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826811B8: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 826811BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826811C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826811C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826811C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826811CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826811D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826811D4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826811D8: 388A45EC  addi r4, r10, 0x45ec
	ctx.r[4].s64 = ctx.r[10].s64 + 17900;
	// 826811DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826811E0: 386B8BC8  addi r3, r11, -0x7438
	ctx.r[3].s64 = ctx.r[11].s64 + -29752;
	// 826811E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826811E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826811EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826811F0: 4BDE5C31  bl 0x82466e20
	ctx.lr = 0x826811F4;
	sub_82466E20(ctx, base);
	// 826811F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826811F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826811FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681208 size=108
    let mut pc: u32 = 0x82681208;
    'dispatch: loop {
        match pc {
            0x82681208 => {
    //   block [0x82681208..0x82681274)
	// 82681208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268120C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681214: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681218: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268121C: 38EBBDA0  addi r7, r11, -0x4260
	ctx.r[7].s64 = ctx.r[11].s64 + -16992;
	// 82681220: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82681224: 388A4608  addi r4, r10, 0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + 17928;
	// 82681228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268122C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681230: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82681234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681238: 386A8BF8  addi r3, r10, -0x7408
	ctx.r[3].s64 = ctx.r[10].s64 + -29704;
	// 8268123C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82681240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681244: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268124C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268125C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681260: 4BDE5BC1  bl 0x82466e20
	ctx.lr = 0x82681264;
	sub_82466E20(ctx, base);
	// 82681264: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268126C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681278 size=112
    let mut pc: u32 = 0x82681278;
    'dispatch: loop {
        match pc {
            0x82681278 => {
    //   block [0x82681278..0x826812E8)
	// 82681278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268127C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681280: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681284: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681288: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268128C: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82681290: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681294: 390BBDD0  addi r8, r11, -0x4230
	ctx.r[8].s64 = ctx.r[11].s64 + -16944;
	// 82681298: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268129C: 388A462C  addi r4, r10, 0x462c
	ctx.r[4].s64 = ctx.r[10].s64 + 17964;
	// 826812A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826812A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826812A8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826812AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826812B0: 386A8C28  addi r3, r10, -0x73d8
	ctx.r[3].s64 = ctx.r[10].s64 + -29656;
	// 826812B4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826812B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826812BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826812C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826812C4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826812C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826812CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826812D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826812D4: 4BDE5B4D  bl 0x82466e20
	ctx.lr = 0x826812D8;
	sub_82466E20(ctx, base);
	// 826812D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826812DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826812E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826812E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826812E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826812E8 size=112
    let mut pc: u32 = 0x826812E8;
    'dispatch: loop {
        match pc {
            0x826812E8 => {
    //   block [0x826812E8..0x82681358)
	// 826812E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826812EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826812F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826812F4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826812F8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826812FC: 392A4C2C  addi r9, r10, 0x4c2c
	ctx.r[9].s64 = ctx.r[10].s64 + 19500;
	// 82681300: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681304: 390BBDF0  addi r8, r11, -0x4210
	ctx.r[8].s64 = ctx.r[11].s64 + -16912;
	// 82681308: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8268130C: 388A464C  addi r4, r10, 0x464c
	ctx.r[4].s64 = ctx.r[10].s64 + 17996;
	// 82681310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681314: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681318: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268131C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681320: 386A8C58  addi r3, r10, -0x73a8
	ctx.r[3].s64 = ctx.r[10].s64 + -29608;
	// 82681324: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82681328: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8268132C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681334: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268133C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681344: 4BDE5ADD  bl 0x82466e20
	ctx.lr = 0x82681348;
	sub_82466E20(ctx, base);
	// 82681348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268134C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681358 size=112
    let mut pc: u32 = 0x82681358;
    'dispatch: loop {
        match pc {
            0x82681358 => {
    //   block [0x82681358..0x826813C8)
	// 82681358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268135C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681364: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681368: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268136C: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82681370: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681374: 390BBE98  addi r8, r11, -0x4168
	ctx.r[8].s64 = ctx.r[11].s64 + -16744;
	// 82681378: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268137C: 388A466C  addi r4, r10, 0x466c
	ctx.r[4].s64 = ctx.r[10].s64 + 18028;
	// 82681380: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681384: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681388: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268138C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681390: 386A8C88  addi r3, r10, -0x7378
	ctx.r[3].s64 = ctx.r[10].s64 + -29560;
	// 82681394: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681398: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268139C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826813A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826813A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826813A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826813AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826813B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826813B4: 4BDE5A6D  bl 0x82466e20
	ctx.lr = 0x826813B8;
	sub_82466E20(ctx, base);
	// 826813B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826813BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826813C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826813C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826813C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826813C8 size=108
    let mut pc: u32 = 0x826813C8;
    'dispatch: loop {
        match pc {
            0x826813C8 => {
    //   block [0x826813C8..0x82681434)
	// 826813C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826813CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826813D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826813D4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826813D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826813DC: 38EBBEB0  addi r7, r11, -0x4150
	ctx.r[7].s64 = ctx.r[11].s64 + -16720;
	// 826813E0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826813E4: 388A4688  addi r4, r10, 0x4688
	ctx.r[4].s64 = ctx.r[10].s64 + 18056;
	// 826813E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826813EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826813F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826813F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826813F8: 386A8CB8  addi r3, r10, -0x7348
	ctx.r[3].s64 = ctx.r[10].s64 + -29512;
	// 826813FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82681400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681404: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268140C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268141C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681420: 4BDE5A01  bl 0x82466e20
	ctx.lr = 0x82681424;
	sub_82466E20(ctx, base);
	// 82681424: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681428: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268142C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681430: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681438 size=112
    let mut pc: u32 = 0x82681438;
    'dispatch: loop {
        match pc {
            0x82681438 => {
    //   block [0x82681438..0x826814A8)
	// 82681438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268143C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681440: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681448: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268144C: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82681450: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681454: 390BBEE0  addi r8, r11, -0x4120
	ctx.r[8].s64 = ctx.r[11].s64 + -16672;
	// 82681458: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8268145C: 388A46AC  addi r4, r10, 0x46ac
	ctx.r[4].s64 = ctx.r[10].s64 + 18092;
	// 82681460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681464: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681468: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268146C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681470: 386A8CE8  addi r3, r10, -0x7318
	ctx.r[3].s64 = ctx.r[10].s64 + -29464;
	// 82681474: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681478: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268147C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268148C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681494: 4BDE598D  bl 0x82466e20
	ctx.lr = 0x82681498;
	sub_82466E20(ctx, base);
	// 82681498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268149C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826814A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826814A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826814A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826814A8 size=112
    let mut pc: u32 = 0x826814A8;
    'dispatch: loop {
        match pc {
            0x826814A8 => {
    //   block [0x826814A8..0x82681518)
	// 826814A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826814AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826814B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826814B4: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 826814B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826814BC: 392A4C60  addi r9, r10, 0x4c60
	ctx.r[9].s64 = ctx.r[10].s64 + 19552;
	// 826814C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826814C4: 390BBEF8  addi r8, r11, -0x4108
	ctx.r[8].s64 = ctx.r[11].s64 + -16648;
	// 826814C8: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826814CC: 388A46C8  addi r4, r10, 0x46c8
	ctx.r[4].s64 = ctx.r[10].s64 + 18120;
	// 826814D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826814D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826814D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826814DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826814E0: 386A8D18  addi r3, r10, -0x72e8
	ctx.r[3].s64 = ctx.r[10].s64 + -29416;
	// 826814E4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826814E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826814EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826814F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826814F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826814F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826814FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681504: 4BDE591D  bl 0x82466e20
	ctx.lr = 0x82681508;
	sub_82466E20(ctx, base);
	// 82681508: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268150C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681510: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681518 size=112
    let mut pc: u32 = 0x82681518;
    'dispatch: loop {
        match pc {
            0x82681518 => {
    //   block [0x82681518..0x82681588)
	// 82681518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268151C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681520: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681524: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681528: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268152C: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82681530: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681534: 390BBFA0  addi r8, r11, -0x4060
	ctx.r[8].s64 = ctx.r[11].s64 + -16480;
	// 82681538: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8268153C: 388A46E4  addi r4, r10, 0x46e4
	ctx.r[4].s64 = ctx.r[10].s64 + 18148;
	// 82681540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681544: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681548: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268154C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681550: 386A8D48  addi r3, r10, -0x72b8
	ctx.r[3].s64 = ctx.r[10].s64 + -29368;
	// 82681554: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681558: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268155C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681560: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681568: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268156C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681570: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681574: 4BDE58AD  bl 0x82466e20
	ctx.lr = 0x82681578;
	sub_82466E20(ctx, base);
	// 82681578: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268157C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681584: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681588 size=112
    let mut pc: u32 = 0x82681588;
    'dispatch: loop {
        match pc {
            0x82681588 => {
    //   block [0x82681588..0x826815F8)
	// 82681588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268158C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681590: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681598: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268159C: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 826815A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826815A4: 390BBFE8  addi r8, r11, -0x4018
	ctx.r[8].s64 = ctx.r[11].s64 + -16408;
	// 826815A8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826815AC: 388A46FC  addi r4, r10, 0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18172;
	// 826815B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826815B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826815B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826815BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826815C0: 386A8D78  addi r3, r10, -0x7288
	ctx.r[3].s64 = ctx.r[10].s64 + -29320;
	// 826815C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826815C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826815CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826815D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826815D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826815D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826815DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826815E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826815E4: 4BDE583D  bl 0x82466e20
	ctx.lr = 0x826815E8;
	sub_82466E20(ctx, base);
	// 826815E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826815EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826815F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826815F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826815F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826815F8 size=100
    let mut pc: u32 = 0x826815F8;
    'dispatch: loop {
        match pc {
            0x826815F8 => {
    //   block [0x826815F8..0x8268165C)
	// 826815F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826815FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681600: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268160C: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82681610: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681614: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681618: 388A4718  addi r4, r10, 0x4718
	ctx.r[4].s64 = ctx.r[10].s64 + 18200;
	// 8268161C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681624: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268162C: 386A8DA8  addi r3, r10, -0x7258
	ctx.r[3].s64 = ctx.r[10].s64 + -29272;
	// 82681630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681634: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681638: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268163C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681640: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82681644: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681648: 4BDE57D9  bl 0x82466e20
	ctx.lr = 0x8268164C;
	sub_82466E20(ctx, base);
	// 8268164C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681660 size=112
    let mut pc: u32 = 0x82681660;
    'dispatch: loop {
        match pc {
            0x82681660 => {
    //   block [0x82681660..0x826816D0)
	// 82681660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268166C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681670: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681674: 38AA8A18  addi r5, r10, -0x75e8
	ctx.r[5].s64 = ctx.r[10].s64 + -30184;
	// 82681678: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268167C: 390BC0A8  addi r8, r11, -0x3f58
	ctx.r[8].s64 = ctx.r[11].s64 + -16216;
	// 82681680: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82681684: 388A4730  addi r4, r10, 0x4730
	ctx.r[4].s64 = ctx.r[10].s64 + 18224;
	// 82681688: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268168C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681690: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681698: 386A8DD8  addi r3, r10, -0x7228
	ctx.r[3].s64 = ctx.r[10].s64 + -29224;
	// 8268169C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826816A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826816A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826816A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826816AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826816B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826816B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826816B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826816BC: 4BDE5765  bl 0x82466e20
	ctx.lr = 0x826816C0;
	sub_82466E20(ctx, base);
	// 826816C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826816C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826816C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826816CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826816D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826816D0 size=112
    let mut pc: u32 = 0x826816D0;
    'dispatch: loop {
        match pc {
            0x826816D0 => {
    //   block [0x826816D0..0x82681740)
	// 826816D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826816D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826816D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826816DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826816E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826816E4: 38AA8898  addi r5, r10, -0x7768
	ctx.r[5].s64 = ctx.r[10].s64 + -30568;
	// 826816E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826816EC: 390BC0D8  addi r8, r11, -0x3f28
	ctx.r[8].s64 = ctx.r[11].s64 + -16168;
	// 826816F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826816F4: 388A474C  addi r4, r10, 0x474c
	ctx.r[4].s64 = ctx.r[10].s64 + 18252;
	// 826816F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826816FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681700: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681708: 386A8E08  addi r3, r10, -0x71f8
	ctx.r[3].s64 = ctx.r[10].s64 + -29176;
	// 8268170C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681710: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681714: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681718: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268171C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681720: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681728: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268172C: 4BDE56F5  bl 0x82466e20
	ctx.lr = 0x82681730;
	sub_82466E20(ctx, base);
	// 82681730: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268173C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681740 size=108
    let mut pc: u32 = 0x82681740;
    'dispatch: loop {
        match pc {
            0x82681740 => {
    //   block [0x82681740..0x826817AC)
	// 82681740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268174C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681750: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681754: 38EBC0F0  addi r7, r11, -0x3f10
	ctx.r[7].s64 = ctx.r[11].s64 + -16144;
	// 82681758: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268175C: 388A476C  addi r4, r10, 0x476c
	ctx.r[4].s64 = ctx.r[10].s64 + 18284;
	// 82681760: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681764: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681768: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268176C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681770: 386A8E38  addi r3, r10, -0x71c8
	ctx.r[3].s64 = ctx.r[10].s64 + -29128;
	// 82681774: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82681778: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268177C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681780: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681788: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268178C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681790: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681794: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681798: 4BDE5689  bl 0x82466e20
	ctx.lr = 0x8268179C;
	sub_82466E20(ctx, base);
	// 8268179C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826817A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826817A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826817A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826817B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826817B0 size=112
    let mut pc: u32 = 0x826817B0;
    'dispatch: loop {
        match pc {
            0x826817B0 => {
    //   block [0x826817B0..0x82681820)
	// 826817B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826817B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826817B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826817BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826817C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826817C4: 38AA8DA8  addi r5, r10, -0x7258
	ctx.r[5].s64 = ctx.r[10].s64 + -29272;
	// 826817C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826817CC: 390BC120  addi r8, r11, -0x3ee0
	ctx.r[8].s64 = ctx.r[11].s64 + -16096;
	// 826817D0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 826817D4: 388A4790  addi r4, r10, 0x4790
	ctx.r[4].s64 = ctx.r[10].s64 + 18320;
	// 826817D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826817DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826817E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826817E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826817E8: 386A8E68  addi r3, r10, -0x7198
	ctx.r[3].s64 = ctx.r[10].s64 + -29080;
	// 826817EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826817F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826817F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826817F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826817FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681800: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681804: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681808: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268180C: 4BDE5615  bl 0x82466e20
	ctx.lr = 0x82681810;
	sub_82466E20(ctx, base);
	// 82681810: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268181C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681820 size=112
    let mut pc: u32 = 0x82681820;
    'dispatch: loop {
        match pc {
            0x82681820 => {
    //   block [0x82681820..0x82681890)
	// 82681820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681828: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268182C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82681830: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681834: 392A4C8C  addi r9, r10, 0x4c8c
	ctx.r[9].s64 = ctx.r[10].s64 + 19596;
	// 82681838: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268183C: 390BC1B8  addi r8, r11, -0x3e48
	ctx.r[8].s64 = ctx.r[11].s64 + -15944;
	// 82681840: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82681844: 388A47A8  addi r4, r10, 0x47a8
	ctx.r[4].s64 = ctx.r[10].s64 + 18344;
	// 82681848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268184C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681850: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681854: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681858: 386A8E98  addi r3, r10, -0x7168
	ctx.r[3].s64 = ctx.r[10].s64 + -29032;
	// 8268185C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82681860: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82681864: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681868: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268186C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681870: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681874: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681878: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268187C: 4BDE55A5  bl 0x82466e20
	ctx.lr = 0x82681880;
	sub_82466E20(ctx, base);
	// 82681880: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681884: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681888: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268188C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681890 size=112
    let mut pc: u32 = 0x82681890;
    'dispatch: loop {
        match pc {
            0x82681890 => {
    //   block [0x82681890..0x82681900)
	// 82681890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268189C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826818A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826818A4: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 826818A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826818AC: 390BC200  addi r8, r11, -0x3e00
	ctx.r[8].s64 = ctx.r[11].s64 + -15872;
	// 826818B0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826818B4: 388A47C0  addi r4, r10, 0x47c0
	ctx.r[4].s64 = ctx.r[10].s64 + 18368;
	// 826818B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826818BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826818C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826818C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826818C8: 386A8EC8  addi r3, r10, -0x7138
	ctx.r[3].s64 = ctx.r[10].s64 + -28984;
	// 826818CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826818D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826818D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826818D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826818DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826818E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826818E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826818E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826818EC: 4BDE5535  bl 0x82466e20
	ctx.lr = 0x826818F0;
	sub_82466E20(ctx, base);
	// 826818F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826818F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826818F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826818FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681900 size=108
    let mut pc: u32 = 0x82681900;
    'dispatch: loop {
        match pc {
            0x82681900 => {
    //   block [0x82681900..0x8268196C)
	// 82681900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268190C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681910: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681914: 38EBC218  addi r7, r11, -0x3de8
	ctx.r[7].s64 = ctx.r[11].s64 + -15848;
	// 82681918: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8268191C: 388A47D4  addi r4, r10, 0x47d4
	ctx.r[4].s64 = ctx.r[10].s64 + 18388;
	// 82681920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681924: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681928: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268192C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681930: 386A8EF8  addi r3, r10, -0x7108
	ctx.r[3].s64 = ctx.r[10].s64 + -28936;
	// 82681934: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82681938: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268193C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268194C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681954: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681958: 4BDE54C9  bl 0x82466e20
	ctx.lr = 0x8268195C;
	sub_82466E20(ctx, base);
	// 8268195C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681970 size=116
    let mut pc: u32 = 0x82681970;
    'dispatch: loop {
        match pc {
            0x82681970 => {
    //   block [0x82681970..0x826819E4)
	// 82681970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681978: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268197C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82681980: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82681984: 390AC2A8  addi r8, r10, -0x3d58
	ctx.r[8].s64 = ctx.r[10].s64 + -15704;
	// 82681988: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268198C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82681990: 38AA8DA8  addi r5, r10, -0x7258
	ctx.r[5].s64 = ctx.r[10].s64 + -29272;
	// 82681994: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681998: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8268199C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826819A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826819A4: 388A47F8  addi r4, r10, 0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + 18424;
	// 826819A8: 396B4CA0  addi r11, r11, 0x4ca0
	ctx.r[11].s64 = ctx.r[11].s64 + 19616;
	// 826819AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826819B0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826819B4: 386A8F28  addi r3, r10, -0x70d8
	ctx.r[3].s64 = ctx.r[10].s64 + -28888;
	// 826819B8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826819BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826819C0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826819C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826819C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826819CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826819D0: 4BDE5451  bl 0x82466e20
	ctx.lr = 0x826819D4;
	sub_82466E20(ctx, base);
	// 826819D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826819D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826819DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826819E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826819E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826819E8 size=108
    let mut pc: u32 = 0x826819E8;
    'dispatch: loop {
        match pc {
            0x826819E8 => {
    //   block [0x826819E8..0x82681A54)
	// 826819E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826819EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826819F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826819F4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826819F8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826819FC: 38EBC380  addi r7, r11, -0x3c80
	ctx.r[7].s64 = ctx.r[11].s64 + -15488;
	// 82681A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82681A04: 388A480C  addi r4, r10, 0x480c
	ctx.r[4].s64 = ctx.r[10].s64 + 18444;
	// 82681A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681A0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82681A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681A18: 386A8F58  addi r3, r10, -0x70a8
	ctx.r[3].s64 = ctx.r[10].s64 + -28840;
	// 82681A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82681A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681A40: 4BDE53E1  bl 0x82466e20
	ctx.lr = 0x82681A44;
	sub_82466E20(ctx, base);
	// 82681A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681A58 size=112
    let mut pc: u32 = 0x82681A58;
    'dispatch: loop {
        match pc {
            0x82681A58 => {
    //   block [0x82681A58..0x82681AC8)
	// 82681A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681A64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681A68: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681A6C: 38AA8DA8  addi r5, r10, -0x7258
	ctx.r[5].s64 = ctx.r[10].s64 + -29272;
	// 82681A70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681A74: 390BC3C8  addi r8, r11, -0x3c38
	ctx.r[8].s64 = ctx.r[11].s64 + -15416;
	// 82681A78: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82681A7C: 388A4834  addi r4, r10, 0x4834
	ctx.r[4].s64 = ctx.r[10].s64 + 18484;
	// 82681A80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681A84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681A8C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681A90: 386A8F88  addi r3, r10, -0x7078
	ctx.r[3].s64 = ctx.r[10].s64 + -28792;
	// 82681A94: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681A98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681A9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681AA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681AA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681AB4: 4BDE536D  bl 0x82466e20
	ctx.lr = 0x82681AB8;
	sub_82466E20(ctx, base);
	// 82681AB8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681ABC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681AC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681AC8 size=112
    let mut pc: u32 = 0x82681AC8;
    'dispatch: loop {
        match pc {
            0x82681AC8 => {
    //   block [0x82681AC8..0x82681B38)
	// 82681AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681AD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681AD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681AD8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681ADC: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82681AE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681AE4: 390BC440  addi r8, r11, -0x3bc0
	ctx.r[8].s64 = ctx.r[11].s64 + -15296;
	// 82681AE8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82681AEC: 388A484C  addi r4, r10, 0x484c
	ctx.r[4].s64 = ctx.r[10].s64 + 18508;
	// 82681AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681AF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681AF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681B00: 386A8FB8  addi r3, r10, -0x7048
	ctx.r[3].s64 = ctx.r[10].s64 + -28744;
	// 82681B04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681B08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681B0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681B10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681B14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681B18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681B20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681B24: 4BDE52FD  bl 0x82466e20
	ctx.lr = 0x82681B28;
	sub_82466E20(ctx, base);
	// 82681B28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681B34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681B38 size=108
    let mut pc: u32 = 0x82681B38;
    'dispatch: loop {
        match pc {
            0x82681B38 => {
    //   block [0x82681B38..0x82681BA4)
	// 82681B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681B3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681B40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681B44: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681B48: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681B4C: 38EBC470  addi r7, r11, -0x3b90
	ctx.r[7].s64 = ctx.r[11].s64 + -15248;
	// 82681B50: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82681B54: 388A4864  addi r4, r10, 0x4864
	ctx.r[4].s64 = ctx.r[10].s64 + 18532;
	// 82681B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681B5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681B60: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82681B64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681B68: 386A8FE8  addi r3, r10, -0x7018
	ctx.r[3].s64 = ctx.r[10].s64 + -28696;
	// 82681B6C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82681B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681B74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681B8C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681B90: 4BDE5291  bl 0x82466e20
	ctx.lr = 0x82681B94;
	sub_82466E20(ctx, base);
	// 82681B94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681B98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681B9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681BA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681BA8 size=108
    let mut pc: u32 = 0x82681BA8;
    'dispatch: loop {
        match pc {
            0x82681BA8 => {
    //   block [0x82681BA8..0x82681C14)
	// 82681BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681BB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681BB4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681BB8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681BBC: 38EBC4D0  addi r7, r11, -0x3b30
	ctx.r[7].s64 = ctx.r[11].s64 + -15152;
	// 82681BC0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82681BC4: 388A4890  addi r4, r10, 0x4890
	ctx.r[4].s64 = ctx.r[10].s64 + 18576;
	// 82681BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681BCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681BD0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82681BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681BD8: 386A9018  addi r3, r10, -0x6fe8
	ctx.r[3].s64 = ctx.r[10].s64 + -28648;
	// 82681BDC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82681BE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681BE4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681BE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681BF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681BF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681BF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681BFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681C00: 4BDE5221  bl 0x82466e20
	ctx.lr = 0x82681C04;
	sub_82466E20(ctx, base);
	// 82681C04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681C08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681C0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681C18 size=112
    let mut pc: u32 = 0x82681C18;
    'dispatch: loop {
        match pc {
            0x82681C18 => {
    //   block [0x82681C18..0x82681C88)
	// 82681C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681C20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681C24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681C28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681C2C: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82681C30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681C34: 390BC548  addi r8, r11, -0x3ab8
	ctx.r[8].s64 = ctx.r[11].s64 + -15032;
	// 82681C38: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82681C3C: 388A48B0  addi r4, r10, 0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + 18608;
	// 82681C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681C44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681C48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681C4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681C50: 386A9048  addi r3, r10, -0x6fb8
	ctx.r[3].s64 = ctx.r[10].s64 + -28600;
	// 82681C54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681C58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681C5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681C60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681C68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681C6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681C70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681C74: 4BDE51AD  bl 0x82466e20
	ctx.lr = 0x82681C78;
	sub_82466E20(ctx, base);
	// 82681C78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82681C88 size=24
    let mut pc: u32 = 0x82681C88;
    'dispatch: loop {
        match pc {
            0x82681C88 => {
    //   block [0x82681C88..0x82681CA0)
	// 82681C88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681C8C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82681C90: 394AF758  addi r10, r10, -0x8a8
	ctx.r[10].s64 = ctx.r[10].s64 + -2216;
	// 82681C94: 816BC1B4  lwz r11, -0x3e4c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15948 as u32) ) } as u64;
	// 82681C98: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82681C9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681CA0 size=116
    let mut pc: u32 = 0x82681CA0;
    'dispatch: loop {
        match pc {
            0x82681CA0 => {
    //   block [0x82681CA0..0x82681D14)
	// 82681CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681CA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681CAC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681CB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82681CB4: 390BF758  addi r8, r11, -0x8a8
	ctx.r[8].s64 = ctx.r[11].s64 + -2216;
	// 82681CB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681CBC: 392A4D04  addi r9, r10, 0x4d04
	ctx.r[9].s64 = ctx.r[10].s64 + 19716;
	// 82681CC0: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82681CC4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82681CC8: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82681CCC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681CD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681CD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681CD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681CDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681CE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681CE4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82681CE8: 388A48CC  addi r4, r10, 0x48cc
	ctx.r[4].s64 = ctx.r[10].s64 + 18636;
	// 82681CEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82681CF0: 386B9078  addi r3, r11, -0x6f88
	ctx.r[3].s64 = ctx.r[11].s64 + -28552;
	// 82681CF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82681CF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681CFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681D00: 4BDE5121  bl 0x82466e20
	ctx.lr = 0x82681D04;
	sub_82466E20(ctx, base);
	// 82681D04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681D08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681D0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681D18 size=112
    let mut pc: u32 = 0x82681D18;
    'dispatch: loop {
        match pc {
            0x82681D18 => {
    //   block [0x82681D18..0x82681D88)
	// 82681D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681D1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681D24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681D28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681D2C: 38AA9078  addi r5, r10, -0x6f88
	ctx.r[5].s64 = ctx.r[10].s64 + -28552;
	// 82681D30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681D34: 390BC590  addi r8, r11, -0x3a70
	ctx.r[8].s64 = ctx.r[11].s64 + -14960;
	// 82681D38: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82681D3C: 388A48E0  addi r4, r10, 0x48e0
	ctx.r[4].s64 = ctx.r[10].s64 + 18656;
	// 82681D40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681D44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681D48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681D4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681D50: 386A90A8  addi r3, r10, -0x6f58
	ctx.r[3].s64 = ctx.r[10].s64 + -28504;
	// 82681D54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681D58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681D60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681D64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681D68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681D6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681D70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681D74: 4BDE50AD  bl 0x82466e20
	ctx.lr = 0x82681D78;
	sub_82466E20(ctx, base);
	// 82681D78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681D7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681D80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82681D88 size=24
    let mut pc: u32 = 0x82681D88;
    'dispatch: loop {
        match pc {
            0x82681D88 => {
    //   block [0x82681D88..0x82681DA0)
	// 82681D88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681D8C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82681D90: 394AF770  addi r10, r10, -0x890
	ctx.r[10].s64 = ctx.r[10].s64 + -2192;
	// 82681D94: 816BC5C0  lwz r11, -0x3a40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14912 as u32) ) } as u64;
	// 82681D98: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82681D9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681DA0 size=116
    let mut pc: u32 = 0x82681DA0;
    'dispatch: loop {
        match pc {
            0x82681DA0 => {
    //   block [0x82681DA0..0x82681E14)
	// 82681DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681DAC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681DB0: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82681DB4: 390BF770  addi r8, r11, -0x890
	ctx.r[8].s64 = ctx.r[11].s64 + -2192;
	// 82681DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681DBC: 392A4D40  addi r9, r10, 0x4d40
	ctx.r[9].s64 = ctx.r[10].s64 + 19776;
	// 82681DC0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681DC4: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82681DC8: 38AA90A8  addi r5, r10, -0x6f58
	ctx.r[5].s64 = ctx.r[10].s64 + -28504;
	// 82681DCC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681DD0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681DD4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681DD8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681DE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681DE4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82681DE8: 388A490C  addi r4, r10, 0x490c
	ctx.r[4].s64 = ctx.r[10].s64 + 18700;
	// 82681DEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82681DF0: 386B90D8  addi r3, r11, -0x6f28
	ctx.r[3].s64 = ctx.r[11].s64 + -28456;
	// 82681DF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82681DF8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681DFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681E00: 4BDE5021  bl 0x82466e20
	ctx.lr = 0x82681E04;
	sub_82466E20(ctx, base);
	// 82681E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681E10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681E18 size=112
    let mut pc: u32 = 0x82681E18;
    'dispatch: loop {
        match pc {
            0x82681E18 => {
    //   block [0x82681E18..0x82681E88)
	// 82681E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681E24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681E28: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681E2C: 38AA90A8  addi r5, r10, -0x6f58
	ctx.r[5].s64 = ctx.r[10].s64 + -28504;
	// 82681E30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681E34: 390BC5C8  addi r8, r11, -0x3a38
	ctx.r[8].s64 = ctx.r[11].s64 + -14904;
	// 82681E38: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82681E3C: 388A4928  addi r4, r10, 0x4928
	ctx.r[4].s64 = ctx.r[10].s64 + 18728;
	// 82681E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681E44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681E48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681E50: 386A9108  addi r3, r10, -0x6ef8
	ctx.r[3].s64 = ctx.r[10].s64 + -28408;
	// 82681E54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681E58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681E5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681E60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681E68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681E70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681E74: 4BDE4FAD  bl 0x82466e20
	ctx.lr = 0x82681E78;
	sub_82466E20(ctx, base);
	// 82681E78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681E88 size=112
    let mut pc: u32 = 0x82681E88;
    'dispatch: loop {
        match pc {
            0x82681E88 => {
    //   block [0x82681E88..0x82681EF8)
	// 82681E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681E90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681E94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681E98: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681E9C: 38AA90A8  addi r5, r10, -0x6f58
	ctx.r[5].s64 = ctx.r[10].s64 + -28504;
	// 82681EA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681EA4: 390BC628  addi r8, r11, -0x39d8
	ctx.r[8].s64 = ctx.r[11].s64 + -14808;
	// 82681EA8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82681EAC: 388A4944  addi r4, r10, 0x4944
	ctx.r[4].s64 = ctx.r[10].s64 + 18756;
	// 82681EB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681EB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681EB8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681EBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681EC0: 386A9138  addi r3, r10, -0x6ec8
	ctx.r[3].s64 = ctx.r[10].s64 + -28360;
	// 82681EC4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681EC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681ECC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681ED0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681ED8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681EDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681EE0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681EE4: 4BDE4F3D  bl 0x82466e20
	ctx.lr = 0x82681EE8;
	sub_82466E20(ctx, base);
	// 82681EE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681EF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681EF8 size=112
    let mut pc: u32 = 0x82681EF8;
    'dispatch: loop {
        match pc {
            0x82681EF8 => {
    //   block [0x82681EF8..0x82681F68)
	// 82681EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681EFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681F04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681F08: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681F0C: 38AA90A8  addi r5, r10, -0x6f58
	ctx.r[5].s64 = ctx.r[10].s64 + -28504;
	// 82681F10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681F14: 390BC658  addi r8, r11, -0x39a8
	ctx.r[8].s64 = ctx.r[11].s64 + -14760;
	// 82681F18: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82681F1C: 388A4964  addi r4, r10, 0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + 18788;
	// 82681F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681F24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681F28: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82681F2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681F30: 386A9168  addi r3, r10, -0x6e98
	ctx.r[3].s64 = ctx.r[10].s64 + -28312;
	// 82681F34: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82681F38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681F3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681F40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681F48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681F4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681F50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681F54: 4BDE4ECD  bl 0x82466e20
	ctx.lr = 0x82681F58;
	sub_82466E20(ctx, base);
	// 82681F58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681F68 size=108
    let mut pc: u32 = 0x82681F68;
    'dispatch: loop {
        match pc {
            0x82681F68 => {
    //   block [0x82681F68..0x82681FD4)
	// 82681F68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681F6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681F70: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681F74: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681F78: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681F7C: 38EBC6A0  addi r7, r11, -0x3960
	ctx.r[7].s64 = ctx.r[11].s64 + -14688;
	// 82681F80: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82681F84: 388A4980  addi r4, r10, 0x4980
	ctx.r[4].s64 = ctx.r[10].s64 + 18816;
	// 82681F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82681F8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681F90: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82681F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82681F98: 386A9198  addi r3, r10, -0x6e68
	ctx.r[3].s64 = ctx.r[10].s64 + -28264;
	// 82681F9C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82681FA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82681FA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82681FA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82681FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82681FB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82681FB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82681FB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82681FBC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82681FC0: 4BDE4E61  bl 0x82466e20
	ctx.lr = 0x82681FC4;
	sub_82466E20(ctx, base);
	// 82681FC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82681FC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82681FCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82681FD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82681FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82681FD8 size=112
    let mut pc: u32 = 0x82681FD8;
    'dispatch: loop {
        match pc {
            0x82681FD8 => {
    //   block [0x82681FD8..0x82682048)
	// 82681FD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82681FDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82681FE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82681FE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82681FE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82681FEC: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82681FF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82681FF4: 390BC6D0  addi r8, r11, -0x3930
	ctx.r[8].s64 = ctx.r[11].s64 + -14640;
	// 82681FF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82681FFC: 388A499C  addi r4, r10, 0x499c
	ctx.r[4].s64 = ctx.r[10].s64 + 18844;
	// 82682000: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682004: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682008: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8268200C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682010: 386A91C8  addi r3, r10, -0x6e38
	ctx.r[3].s64 = ctx.r[10].s64 + -28216;
	// 82682014: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82682018: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268201C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682020: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682028: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268202C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682030: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682034: 4BDE4DED  bl 0x82466e20
	ctx.lr = 0x82682038;
	sub_82466E20(ctx, base);
	// 82682038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268203C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682044: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682048 size=108
    let mut pc: u32 = 0x82682048;
    'dispatch: loop {
        match pc {
            0x82682048 => {
    //   block [0x82682048..0x826820B4)
	// 82682048: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268204C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682050: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682054: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682058: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268205C: 38EBC6E8  addi r7, r11, -0x3918
	ctx.r[7].s64 = ctx.r[11].s64 + -14616;
	// 82682060: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82682064: 388A49B4  addi r4, r10, 0x49b4
	ctx.r[4].s64 = ctx.r[10].s64 + 18868;
	// 82682068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268206C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682070: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82682074: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682078: 386A91F8  addi r3, r10, -0x6e08
	ctx.r[3].s64 = ctx.r[10].s64 + -28168;
	// 8268207C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82682080: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682088: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268208C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682090: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682094: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682098: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268209C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826820A0: 4BDE4D81  bl 0x82466e20
	ctx.lr = 0x826820A4;
	sub_82466E20(ctx, base);
	// 826820A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826820A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826820AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826820B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826820B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826820B8 size=24
    let mut pc: u32 = 0x826820B8;
    'dispatch: loop {
        match pc {
            0x826820B8 => {
    //   block [0x826820B8..0x826820D0)
	// 826820B8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826820BC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 826820C0: 394AF7E8  addi r10, r10, -0x818
	ctx.r[10].s64 = ctx.r[10].s64 + -2072;
	// 826820C4: 816BC5C4  lwz r11, -0x3a3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14908 as u32) ) } as u64;
	// 826820C8: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826820CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826820D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826820D0 size=108
    let mut pc: u32 = 0x826820D0;
    'dispatch: loop {
        match pc {
            0x826820D0 => {
    //   block [0x826820D0..0x8268213C)
	// 826820D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826820D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826820D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826820DC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826820E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826820E4: 38EBF7E8  addi r7, r11, -0x818
	ctx.r[7].s64 = ctx.r[11].s64 + -2072;
	// 826820E8: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 826820EC: 388A49DC  addi r4, r10, 0x49dc
	ctx.r[4].s64 = ctx.r[10].s64 + 18908;
	// 826820F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826820F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826820F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826820FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682100: 386A9228  addi r3, r10, -0x6dd8
	ctx.r[3].s64 = ctx.r[10].s64 + -28120;
	// 82682104: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82682108: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268210C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682110: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682118: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268211C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682120: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682124: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82682128: 4BDE4CF9  bl 0x82466e20
	ctx.lr = 0x8268212C;
	sub_82466E20(ctx, base);
	// 8268212C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682140 size=116
    let mut pc: u32 = 0x82682140;
    'dispatch: loop {
        match pc {
            0x82682140 => {
    //   block [0x82682140..0x826821B4)
	// 82682140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682148: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268214C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82682150: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682154: 392B4D6C  addi r9, r11, 0x4d6c
	ctx.r[9].s64 = ctx.r[11].s64 + 19820;
	// 82682158: 38AA96A8  addi r5, r10, -0x6958
	ctx.r[5].s64 = ctx.r[10].s64 + -26968;
	// 8268215C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682160: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82682164: 38C00011  li r6, 0x11
	ctx.r[6].s64 = 17;
	// 82682168: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268216C: 388A49FC  addi r4, r10, 0x49fc
	ctx.r[4].s64 = ctx.r[10].s64 + 18940;
	// 82682170: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682174: 396BC730  addi r11, r11, -0x38d0
	ctx.r[11].s64 = ctx.r[11].s64 + -14544;
	// 82682178: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8268217C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682180: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82682184: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682188: 386A9258  addi r3, r10, -0x6da8
	ctx.r[3].s64 = ctx.r[10].s64 + -28072;
	// 8268218C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82682190: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82682194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682198: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8268219C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826821A0: 4BDE4C81  bl 0x82466e20
	ctx.lr = 0x826821A4;
	sub_82466E20(ctx, base);
	// 826821A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826821A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826821AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826821B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826821B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826821B8 size=100
    let mut pc: u32 = 0x826821B8;
    'dispatch: loop {
        match pc {
            0x826821B8 => {
    //   block [0x826821B8..0x8268221C)
	// 826821B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826821BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826821C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826821C4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826821C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826821CC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826821D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826821D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826821D8: 388A4A08  addi r4, r10, 0x4a08
	ctx.r[4].s64 = ctx.r[10].s64 + 18952;
	// 826821DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826821E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826821E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826821E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826821EC: 386A9288  addi r3, r10, -0x6d78
	ctx.r[3].s64 = ctx.r[10].s64 + -28024;
	// 826821F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826821F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826821F8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826821FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682200: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82682204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682208: 4BDE4C19  bl 0x82466e20
	ctx.lr = 0x8268220C;
	sub_82466E20(ctx, base);
	// 8268220C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682220 size=100
    let mut pc: u32 = 0x82682220;
    'dispatch: loop {
        match pc {
            0x82682220 => {
    //   block [0x82682220..0x82682284)
	// 82682220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682228: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268222C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682230: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682234: 38AA9318  addi r5, r10, -0x6ce8
	ctx.r[5].s64 = ctx.r[10].s64 + -27880;
	// 82682238: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268223C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682240: 388A4A1C  addi r4, r10, 0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + 18972;
	// 82682244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268224C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682254: 386A92B8  addi r3, r10, -0x6d48
	ctx.r[3].s64 = ctx.r[10].s64 + -27976;
	// 82682258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268225C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682260: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82682264: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682268: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268226C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682270: 4BDE4BB1  bl 0x82466e20
	ctx.lr = 0x82682274;
	sub_82466E20(ctx, base);
	// 82682274: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268227C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682280: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682288 size=100
    let mut pc: u32 = 0x82682288;
    'dispatch: loop {
        match pc {
            0x82682288 => {
    //   block [0x82682288..0x826822EC)
	// 82682288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268228C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268229C: 38AA9258  addi r5, r10, -0x6da8
	ctx.r[5].s64 = ctx.r[10].s64 + -28072;
	// 826822A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826822A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826822A8: 388A4A38  addi r4, r10, 0x4a38
	ctx.r[4].s64 = ctx.r[10].s64 + 19000;
	// 826822AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826822B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826822B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826822B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826822BC: 386A92E8  addi r3, r10, -0x6d18
	ctx.r[3].s64 = ctx.r[10].s64 + -27928;
	// 826822C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826822C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826822C8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826822CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826822D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826822D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826822D8: 4BDE4B49  bl 0x82466e20
	ctx.lr = 0x826822DC;
	sub_82466E20(ctx, base);
	// 826822DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826822E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826822E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826822E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826822F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826822F0 size=104
    let mut pc: u32 = 0x826822F0;
    'dispatch: loop {
        match pc {
            0x826822F0 => {
    //   block [0x826822F0..0x82682358)
	// 826822F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826822F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826822F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826822FC: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82682300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682304: 392A4DE8  addi r9, r10, 0x4de8
	ctx.r[9].s64 = ctx.r[10].s64 + 19944;
	// 82682308: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8268230C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682310: 38AA9288  addi r5, r10, -0x6d78
	ctx.r[5].s64 = ctx.r[10].s64 + -28024;
	// 82682314: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268231C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682324: 388A4A44  addi r4, r10, 0x4a44
	ctx.r[4].s64 = ctx.r[10].s64 + 19012;
	// 82682328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268232C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682330: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82682334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682338: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268233C: 386A9318  addi r3, r10, -0x6ce8
	ctx.r[3].s64 = ctx.r[10].s64 + -27880;
	// 82682340: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82682344: 4BDE4ADD  bl 0x82466e20
	ctx.lr = 0x82682348;
	sub_82466E20(ctx, base);
	// 82682348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268234C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682350: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682358 size=108
    let mut pc: u32 = 0x82682358;
    'dispatch: loop {
        match pc {
            0x82682358 => {
    //   block [0x82682358..0x826823C4)
	// 82682358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268235C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682360: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682364: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268236C: 38EBC8CC  addi r7, r11, -0x3734
	ctx.r[7].s64 = ctx.r[11].s64 + -14132;
	// 82682370: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82682374: 388A4A5C  addi r4, r10, 0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + 19036;
	// 82682378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268237C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82682384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682388: 386A9348  addi r3, r10, -0x6cb8
	ctx.r[3].s64 = ctx.r[10].s64 + -27832;
	// 8268238C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82682390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268239C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826823A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826823A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826823A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826823AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826823B0: 4BDE4A71  bl 0x82466e20
	ctx.lr = 0x826823B4;
	sub_82466E20(ctx, base);
	// 826823B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826823B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826823BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826823C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826823C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826823C8 size=112
    let mut pc: u32 = 0x826823C8;
    'dispatch: loop {
        match pc {
            0x826823C8 => {
    //   block [0x826823C8..0x82682438)
	// 826823C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826823CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826823D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826823D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826823D8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826823DC: 38AA9318  addi r5, r10, -0x6ce8
	ctx.r[5].s64 = ctx.r[10].s64 + -27880;
	// 826823E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826823E4: 390BC900  addi r8, r11, -0x3700
	ctx.r[8].s64 = ctx.r[11].s64 + -14080;
	// 826823E8: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826823EC: 388A4A80  addi r4, r10, 0x4a80
	ctx.r[4].s64 = ctx.r[10].s64 + 19072;
	// 826823F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826823F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826823F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826823FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682400: 386A9378  addi r3, r10, -0x6c88
	ctx.r[3].s64 = ctx.r[10].s64 + -27784;
	// 82682404: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82682408: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268240C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682410: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682414: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682418: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268241C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682420: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682424: 4BDE49FD  bl 0x82466e20
	ctx.lr = 0x82682428;
	sub_82466E20(ctx, base);
	// 82682428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8268242C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82682438 size=24
    let mut pc: u32 = 0x82682438;
    'dispatch: loop {
        match pc {
            0x82682438 => {
    //   block [0x82682438..0x82682450)
	// 82682438: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 8268243C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82682440: 394AF848  addi r10, r10, -0x7b8
	ctx.r[10].s64 = ctx.r[10].s64 + -1976;
	// 82682444: 816BC8FC  lwz r11, -0x3704(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14084 as u32) ) } as u64;
	// 82682448: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8268244C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682450 size=116
    let mut pc: u32 = 0x82682450;
    'dispatch: loop {
        match pc {
            0x82682450 => {
    //   block [0x82682450..0x826824C4)
	// 82682450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682458: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268245C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682460: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82682464: 390BF848  addi r8, r11, -0x7b8
	ctx.r[8].s64 = ctx.r[11].s64 + -1976;
	// 82682468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268246C: 392A4E50  addi r9, r10, 0x4e50
	ctx.r[9].s64 = ctx.r[10].s64 + 20048;
	// 82682470: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82682474: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82682478: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 8268247C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682480: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682484: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682488: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268248C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682490: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682494: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82682498: 388A4B0C  addi r4, r10, 0x4b0c
	ctx.r[4].s64 = ctx.r[10].s64 + 19212;
	// 8268249C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826824A0: 386B93A8  addi r3, r11, -0x6c58
	ctx.r[3].s64 = ctx.r[11].s64 + -27736;
	// 826824A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826824A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826824AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826824B0: 4BDE4971  bl 0x82466e20
	ctx.lr = 0x826824B4;
	sub_82466E20(ctx, base);
	// 826824B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826824B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826824BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826824C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826824C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826824C8 size=100
    let mut pc: u32 = 0x826824C8;
    'dispatch: loop {
        match pc {
            0x826824C8 => {
    //   block [0x826824C8..0x8268252C)
	// 826824C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826824CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826824D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826824D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826824D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826824DC: 38AA93A8  addi r5, r10, -0x6c58
	ctx.r[5].s64 = ctx.r[10].s64 + -27736;
	// 826824E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826824E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826824E8: 388A4B18  addi r4, r10, 0x4b18
	ctx.r[4].s64 = ctx.r[10].s64 + 19224;
	// 826824EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826824F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826824F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826824F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826824FC: 386A93D8  addi r3, r10, -0x6c28
	ctx.r[3].s64 = ctx.r[10].s64 + -27688;
	// 82682500: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682504: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682508: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268250C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682510: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82682514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682518: 4BDE4909  bl 0x82466e20
	ctx.lr = 0x8268251C;
	sub_82466E20(ctx, base);
	// 8268251C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682520: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682524: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682530 size=100
    let mut pc: u32 = 0x82682530;
    'dispatch: loop {
        match pc {
            0x82682530 => {
    //   block [0x82682530..0x82682594)
	// 82682530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682538: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268253C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682544: 38AA9438  addi r5, r10, -0x6bc8
	ctx.r[5].s64 = ctx.r[10].s64 + -27592;
	// 82682548: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268254C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682550: 388A4B24  addi r4, r10, 0x4b24
	ctx.r[4].s64 = ctx.r[10].s64 + 19236;
	// 82682554: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268255C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682564: 386A9408  addi r3, r10, -0x6bf8
	ctx.r[3].s64 = ctx.r[10].s64 + -27640;
	// 82682568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268256C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682570: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82682574: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682578: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268257C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682580: 4BDE48A1  bl 0x82466e20
	ctx.lr = 0x82682584;
	sub_82466E20(ctx, base);
	// 82682584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268258C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682598 size=112
    let mut pc: u32 = 0x82682598;
    'dispatch: loop {
        match pc {
            0x82682598 => {
    //   block [0x82682598..0x82682608)
	// 82682598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268259C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826825A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826825A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826825A8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826825AC: 38AA93A8  addi r5, r10, -0x6c58
	ctx.r[5].s64 = ctx.r[10].s64 + -27736;
	// 826825B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826825B4: 390BC9A8  addi r8, r11, -0x3658
	ctx.r[8].s64 = ctx.r[11].s64 + -13912;
	// 826825B8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826825BC: 388A4B38  addi r4, r10, 0x4b38
	ctx.r[4].s64 = ctx.r[10].s64 + 19256;
	// 826825C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826825C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826825C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826825CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826825D0: 386A9438  addi r3, r10, -0x6bc8
	ctx.r[3].s64 = ctx.r[10].s64 + -27592;
	// 826825D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826825D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826825DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826825E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826825E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826825E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826825EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826825F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826825F4: 4BDE482D  bl 0x82466e20
	ctx.lr = 0x826825F8;
	sub_82466E20(ctx, base);
	// 826825F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826825FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682608 size=100
    let mut pc: u32 = 0x82682608;
    'dispatch: loop {
        match pc {
            0x82682608 => {
    //   block [0x82682608..0x8268266C)
	// 82682608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8268260C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682614: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682618: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268261C: 38AA9438  addi r5, r10, -0x6bc8
	ctx.r[5].s64 = ctx.r[10].s64 + -27592;
	// 82682620: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682624: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682628: 388A4B50  addi r4, r10, 0x4b50
	ctx.r[4].s64 = ctx.r[10].s64 + 19280;
	// 8268262C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682634: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268263C: 386A9468  addi r3, r10, -0x6b98
	ctx.r[3].s64 = ctx.r[10].s64 + -27544;
	// 82682640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682644: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682648: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268264C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682650: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82682654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682658: 4BDE47C9  bl 0x82466e20
	ctx.lr = 0x8268265C;
	sub_82466E20(ctx, base);
	// 8268265C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682670 size=100
    let mut pc: u32 = 0x82682670;
    'dispatch: loop {
        match pc {
            0x82682670 => {
    //   block [0x82682670..0x826826D4)
	// 82682670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682678: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268267C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682684: 38AA93A8  addi r5, r10, -0x6c58
	ctx.r[5].s64 = ctx.r[10].s64 + -27736;
	// 82682688: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268268C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682690: 388A4B60  addi r4, r10, 0x4b60
	ctx.r[4].s64 = ctx.r[10].s64 + 19296;
	// 82682694: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268269C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826826A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826826A4: 386A9498  addi r3, r10, -0x6b68
	ctx.r[3].s64 = ctx.r[10].s64 + -27496;
	// 826826A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826826AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826826B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826826B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826826B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826826BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826826C0: 4BDE4761  bl 0x82466e20
	ctx.lr = 0x826826C4;
	sub_82466E20(ctx, base);
	// 826826C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826826C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826826CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826826D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826826D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826826D8 size=100
    let mut pc: u32 = 0x826826D8;
    'dispatch: loop {
        match pc {
            0x826826D8 => {
    //   block [0x826826D8..0x8268273C)
	// 826826D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826826DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826826E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826826E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826826E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826826EC: 38AA93D8  addi r5, r10, -0x6c28
	ctx.r[5].s64 = ctx.r[10].s64 + -27688;
	// 826826F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826826F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826826F8: 388A4B70  addi r4, r10, 0x4b70
	ctx.r[4].s64 = ctx.r[10].s64 + 19312;
	// 826826FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268270C: 386A94C8  addi r3, r10, -0x6b38
	ctx.r[3].s64 = ctx.r[10].s64 + -27448;
	// 82682710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682714: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682718: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8268271C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682720: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82682724: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682728: 4BDE46F9  bl 0x82466e20
	ctx.lr = 0x8268272C;
	sub_82466E20(ctx, base);
	// 8268272C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682740 size=100
    let mut pc: u32 = 0x82682740;
    'dispatch: loop {
        match pc {
            0x82682740 => {
    //   block [0x82682740..0x826827A4)
	// 82682740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268274C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682754: 38AA9498  addi r5, r10, -0x6b68
	ctx.r[5].s64 = ctx.r[10].s64 + -27496;
	// 82682758: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268275C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682760: 388A4B88  addi r4, r10, 0x4b88
	ctx.r[4].s64 = ctx.r[10].s64 + 19336;
	// 82682764: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682768: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268276C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682770: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682774: 386A94F8  addi r3, r10, -0x6b08
	ctx.r[3].s64 = ctx.r[10].s64 + -27400;
	// 82682778: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268277C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682780: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82682784: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682788: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8268278C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682790: 4BDE4691  bl 0x82466e20
	ctx.lr = 0x82682794;
	sub_82466E20(ctx, base);
	// 82682794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8268279C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826827A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826827A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826827A8 size=100
    let mut pc: u32 = 0x826827A8;
    'dispatch: loop {
        match pc {
            0x826827A8 => {
    //   block [0x826827A8..0x8268280C)
	// 826827A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826827AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826827B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826827B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826827B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826827BC: 38AA93D8  addi r5, r10, -0x6c28
	ctx.r[5].s64 = ctx.r[10].s64 + -27688;
	// 826827C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826827C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826827C8: 388A4BA4  addi r4, r10, 0x4ba4
	ctx.r[4].s64 = ctx.r[10].s64 + 19364;
	// 826827CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826827D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826827D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826827D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826827DC: 386A9528  addi r3, r10, -0x6ad8
	ctx.r[3].s64 = ctx.r[10].s64 + -27352;
	// 826827E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826827E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826827E8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 826827EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826827F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826827F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826827F8: 4BDE4629  bl 0x82466e20
	ctx.lr = 0x826827FC;
	sub_82466E20(ctx, base);
	// 826827FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682810 size=112
    let mut pc: u32 = 0x82682810;
    'dispatch: loop {
        match pc {
            0x82682810 => {
    //   block [0x82682810..0x82682880)
	// 82682810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268281C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682820: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682824: 38AA95B8  addi r5, r10, -0x6a48
	ctx.r[5].s64 = ctx.r[10].s64 + -27208;
	// 82682828: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268282C: 390BC9D8  addi r8, r11, -0x3628
	ctx.r[8].s64 = ctx.r[11].s64 + -13864;
	// 82682830: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82682834: 388A4BB4  addi r4, r10, 0x4bb4
	ctx.r[4].s64 = ctx.r[10].s64 + 19380;
	// 82682838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268283C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682840: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682844: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682848: 386A9558  addi r3, r10, -0x6aa8
	ctx.r[3].s64 = ctx.r[10].s64 + -27304;
	// 8268284C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82682850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682854: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268285C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268286C: 4BDE45B5  bl 0x82466e20
	ctx.lr = 0x82682870;
	sub_82466E20(ctx, base);
	// 82682870: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268287C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682880 size=112
    let mut pc: u32 = 0x82682880;
    'dispatch: loop {
        match pc {
            0x82682880 => {
    //   block [0x82682880..0x826828F0)
	// 82682880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268288C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682890: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682894: 38AA95E8  addi r5, r10, -0x6a18
	ctx.r[5].s64 = ctx.r[10].s64 + -27160;
	// 82682898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268289C: 390BCA08  addi r8, r11, -0x35f8
	ctx.r[8].s64 = ctx.r[11].s64 + -13816;
	// 826828A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826828A4: 388A4BC4  addi r4, r10, 0x4bc4
	ctx.r[4].s64 = ctx.r[10].s64 + 19396;
	// 826828A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826828AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826828B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826828B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826828B8: 386A9588  addi r3, r10, -0x6a78
	ctx.r[3].s64 = ctx.r[10].s64 + -27256;
	// 826828BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826828C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826828C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826828C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826828CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826828D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826828D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826828D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826828DC: 4BDE4545  bl 0x82466e20
	ctx.lr = 0x826828E0;
	sub_82466E20(ctx, base);
	// 826828E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826828E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826828E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826828EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826828F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826828F0 size=112
    let mut pc: u32 = 0x826828F0;
    'dispatch: loop {
        match pc {
            0x826828F0 => {
    //   block [0x826828F0..0x82682960)
	// 826828F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826828F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826828F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826828FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682900: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682904: 38AA96A8  addi r5, r10, -0x6958
	ctx.r[5].s64 = ctx.r[10].s64 + -26968;
	// 82682908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268290C: 390BCA20  addi r8, r11, -0x35e0
	ctx.r[8].s64 = ctx.r[11].s64 + -13792;
	// 82682910: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82682914: 388A4BDC  addi r4, r10, 0x4bdc
	ctx.r[4].s64 = ctx.r[10].s64 + 19420;
	// 82682918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268291C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682928: 386A95B8  addi r3, r10, -0x6a48
	ctx.r[3].s64 = ctx.r[10].s64 + -27208;
	// 8268292C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82682930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268293C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268294C: 4BDE44D5  bl 0x82466e20
	ctx.lr = 0x82682950;
	sub_82466E20(ctx, base);
	// 82682950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268295C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682960 size=112
    let mut pc: u32 = 0x82682960;
    'dispatch: loop {
        match pc {
            0x82682960 => {
    //   block [0x82682960..0x826829D0)
	// 82682960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268296C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682970: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682974: 38AA95B8  addi r5, r10, -0x6a48
	ctx.r[5].s64 = ctx.r[10].s64 + -27208;
	// 82682978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268297C: 390BCA50  addi r8, r11, -0x35b0
	ctx.r[8].s64 = ctx.r[11].s64 + -13744;
	// 82682980: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82682984: 388A4BE8  addi r4, r10, 0x4be8
	ctx.r[4].s64 = ctx.r[10].s64 + 19432;
	// 82682988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268298C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682998: 386A95E8  addi r3, r10, -0x6a18
	ctx.r[3].s64 = ctx.r[10].s64 + -27160;
	// 8268299C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826829A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826829A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826829A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826829AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826829B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826829B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826829B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826829BC: 4BDE4465  bl 0x82466e20
	ctx.lr = 0x826829C0;
	sub_82466E20(ctx, base);
	// 826829C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826829C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826829C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826829CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826829D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826829D0 size=112
    let mut pc: u32 = 0x826829D0;
    'dispatch: loop {
        match pc {
            0x826829D0 => {
    //   block [0x826829D0..0x82682A40)
	// 826829D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826829D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826829D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826829DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826829E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826829E4: 38AA95E8  addi r5, r10, -0x6a18
	ctx.r[5].s64 = ctx.r[10].s64 + -27160;
	// 826829E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826829EC: 390BCA68  addi r8, r11, -0x3598
	ctx.r[8].s64 = ctx.r[11].s64 + -13720;
	// 826829F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826829F4: 388A4BF8  addi r4, r10, 0x4bf8
	ctx.r[4].s64 = ctx.r[10].s64 + 19448;
	// 826829F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826829FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682A08: 386A9618  addi r3, r10, -0x69e8
	ctx.r[3].s64 = ctx.r[10].s64 + -27112;
	// 82682A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82682A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682A2C: 4BDE43F5  bl 0x82466e20
	ctx.lr = 0x82682A30;
	sub_82466E20(ctx, base);
	// 82682A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682A40 size=116
    let mut pc: u32 = 0x82682A40;
    'dispatch: loop {
        match pc {
            0x82682A40 => {
    //   block [0x82682A40..0x82682AB4)
	// 82682A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682A4C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82682A50: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82682A54: 390ACA80  addi r8, r10, -0x3580
	ctx.r[8].s64 = ctx.r[10].s64 + -13696;
	// 82682A58: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82682A5C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82682A60: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82682A64: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682A68: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82682A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682A70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682A74: 388A4C10  addi r4, r10, 0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + 19472;
	// 82682A78: 396B4E64  addi r11, r11, 0x4e64
	ctx.r[11].s64 = ctx.r[11].s64 + 20068;
	// 82682A7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682A80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682A84: 386A9648  addi r3, r10, -0x69b8
	ctx.r[3].s64 = ctx.r[10].s64 + -27064;
	// 82682A88: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82682A8C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682A90: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82682A94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682A98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682A9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682AA0: 4BDE4381  bl 0x82466e20
	ctx.lr = 0x82682AA4;
	sub_82466E20(ctx, base);
	// 82682AA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682AA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682AAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82682AB8 size=48
    let mut pc: u32 = 0x82682AB8;
    'dispatch: loop {
        match pc {
            0x82682AB8 => {
    //   block [0x82682AB8..0x82682AE8)
	// 82682AB8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682ABC: 814BCB34  lwz r10, -0x34cc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13516 as u32) ) } as u64;
	// 82682AC0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682AC4: 396BF908  addi r11, r11, -0x6f8
	ctx.r[11].s64 = ctx.r[11].s64 + -1784;
	// 82682AC8: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82682ACC: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82682AD0: 814ACB30  lwz r10, -0x34d0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13520 as u32) ) } as u64;
	// 82682AD4: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82682AD8: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82682ADC: 814ACB2C  lwz r10, -0x34d4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13524 as u32) ) } as u64;
	// 82682AE0: 914B02F0  stw r10, 0x2f0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(752 as u32), ctx.r[10].u32 ) };
	// 82682AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682AE8 size=116
    let mut pc: u32 = 0x82682AE8;
    'dispatch: loop {
        match pc {
            0x82682AE8 => {
    //   block [0x82682AE8..0x82682B5C)
	// 82682AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682AF4: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82682AF8: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82682AFC: 392B4F38  addi r9, r11, 0x4f38
	ctx.r[9].s64 = ctx.r[11].s64 + 20280;
	// 82682B00: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82682B04: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682B08: 38E90050  addi r7, r9, 0x50
	ctx.r[7].s64 = ctx.r[9].s64 + 80;
	// 82682B0C: 38C00024  li r6, 0x24
	ctx.r[6].s64 = 36;
	// 82682B10: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682B14: 388A4FC8  addi r4, r10, 0x4fc8
	ctx.r[4].s64 = ctx.r[10].s64 + 20424;
	// 82682B18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682B1C: 396BF908  addi r11, r11, -0x6f8
	ctx.r[11].s64 = ctx.r[11].s64 + -1784;
	// 82682B20: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82682B24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682B28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82682B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682B30: 386A9678  addi r3, r10, -0x6988
	ctx.r[3].s64 = ctx.r[10].s64 + -27016;
	// 82682B34: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82682B38: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82682B3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682B40: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82682B44: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82682B48: 4BDE42D9  bl 0x82466e20
	ctx.lr = 0x82682B4C;
	sub_82466E20(ctx, base);
	// 82682B4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682B60 size=116
    let mut pc: u32 = 0x82682B60;
    'dispatch: loop {
        match pc {
            0x82682B60 => {
    //   block [0x82682B60..0x82682BD4)
	// 82682B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682B68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682B6C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682B70: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82682B74: 390BCB40  addi r8, r11, -0x34c0
	ctx.r[8].s64 = ctx.r[11].s64 + -13504;
	// 82682B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682B7C: 392A50A4  addi r9, r10, 0x50a4
	ctx.r[9].s64 = ctx.r[10].s64 + 20644;
	// 82682B80: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82682B84: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82682B88: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82682B8C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682B90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682B94: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682B98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682B9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682BA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682BA4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82682BA8: 388A4FD8  addi r4, r10, 0x4fd8
	ctx.r[4].s64 = ctx.r[10].s64 + 20440;
	// 82682BAC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82682BB0: 386B96A8  addi r3, r11, -0x6958
	ctx.r[3].s64 = ctx.r[11].s64 + -26968;
	// 82682BB4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82682BB8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682BBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682BC0: 4BDE4261  bl 0x82466e20
	ctx.lr = 0x82682BC4;
	sub_82466E20(ctx, base);
	// 82682BC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682BC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682BCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682BD8 size=112
    let mut pc: u32 = 0x82682BD8;
    'dispatch: loop {
        match pc {
            0x82682BD8 => {
    //   block [0x82682BD8..0x82682C48)
	// 82682BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682BE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682BE4: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82682BE8: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682BEC: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82682BF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682BF4: 390BCBD0  addi r8, r11, -0x3430
	ctx.r[8].s64 = ctx.r[11].s64 + -13360;
	// 82682BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82682BFC: 388A4FE8  addi r4, r10, 0x4fe8
	ctx.r[4].s64 = ctx.r[10].s64 + 20456;
	// 82682C00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682C04: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682C08: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682C0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682C10: 386A96D8  addi r3, r10, -0x6928
	ctx.r[3].s64 = ctx.r[10].s64 + -26920;
	// 82682C14: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82682C18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682C1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682C20: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682C28: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682C2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682C30: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682C34: 4BDE41ED  bl 0x82466e20
	ctx.lr = 0x82682C38;
	sub_82466E20(ctx, base);
	// 82682C38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682C48 size=112
    let mut pc: u32 = 0x82682C48;
    'dispatch: loop {
        match pc {
            0x82682C48 => {
    //   block [0x82682C48..0x82682CB8)
	// 82682C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682C50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682C54: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82682C58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682C5C: 38AA76F8  addi r5, r10, 0x76f8
	ctx.r[5].s64 = ctx.r[10].s64 + 30456;
	// 82682C60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682C64: 390BCBE8  addi r8, r11, -0x3418
	ctx.r[8].s64 = ctx.r[11].s64 + -13336;
	// 82682C68: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82682C6C: 388A5000  addi r4, r10, 0x5000
	ctx.r[4].s64 = ctx.r[10].s64 + 20480;
	// 82682C70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682C74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682C78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682C7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682C80: 386A9708  addi r3, r10, -0x68f8
	ctx.r[3].s64 = ctx.r[10].s64 + -26872;
	// 82682C84: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82682C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682C90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682C98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682C9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682CA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682CA4: 4BDE417D  bl 0x82466e20
	ctx.lr = 0x82682CA8;
	sub_82466E20(ctx, base);
	// 82682CA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682CAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682CB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682CB8 size=108
    let mut pc: u32 = 0x82682CB8;
    'dispatch: loop {
        match pc {
            0x82682CB8 => {
    //   block [0x82682CB8..0x82682D24)
	// 82682CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682CC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682CC4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682CC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682CCC: 38EBCC00  addi r7, r11, -0x3400
	ctx.r[7].s64 = ctx.r[11].s64 + -13312;
	// 82682CD0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82682CD4: 388A5014  addi r4, r10, 0x5014
	ctx.r[4].s64 = ctx.r[10].s64 + 20500;
	// 82682CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682CDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682CE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82682CE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682CE8: 386A9738  addi r3, r10, -0x68c8
	ctx.r[3].s64 = ctx.r[10].s64 + -26824;
	// 82682CEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82682CF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682D04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682D0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82682D10: 4BDE4111  bl 0x82466e20
	ctx.lr = 0x82682D14;
	sub_82466E20(ctx, base);
	// 82682D14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682D18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682D1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682D28 size=112
    let mut pc: u32 = 0x82682D28;
    'dispatch: loop {
        match pc {
            0x82682D28 => {
    //   block [0x82682D28..0x82682D98)
	// 82682D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682D34: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82682D38: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682D3C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82682D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682D44: 390BCC18  addi r8, r11, -0x33e8
	ctx.r[8].s64 = ctx.r[11].s64 + -13288;
	// 82682D48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82682D4C: 388A5028  addi r4, r10, 0x5028
	ctx.r[4].s64 = ctx.r[10].s64 + 20520;
	// 82682D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682D58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682D5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682D60: 386A9768  addi r3, r10, -0x6898
	ctx.r[3].s64 = ctx.r[10].s64 + -26776;
	// 82682D64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82682D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682D6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682D74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682D84: 4BDE409D  bl 0x82466e20
	ctx.lr = 0x82682D88;
	sub_82466E20(ctx, base);
	// 82682D88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682D8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682D90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682D98 size=108
    let mut pc: u32 = 0x82682D98;
    'dispatch: loop {
        match pc {
            0x82682D98 => {
    //   block [0x82682D98..0x82682E04)
	// 82682D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682DA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682DA4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682DA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682DAC: 38EBCC60  addi r7, r11, -0x33a0
	ctx.r[7].s64 = ctx.r[11].s64 + -13216;
	// 82682DB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82682DB4: 388A5040  addi r4, r10, 0x5040
	ctx.r[4].s64 = ctx.r[10].s64 + 20544;
	// 82682DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682DBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682DC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82682DC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682DC8: 386A9798  addi r3, r10, -0x6868
	ctx.r[3].s64 = ctx.r[10].s64 + -26728;
	// 82682DCC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82682DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682DD4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682DD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682DDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682DE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682DE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682DE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682DEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82682DF0: 4BDE4031  bl 0x82466e20
	ctx.lr = 0x82682DF4;
	sub_82466E20(ctx, base);
	// 82682DF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682E08 size=108
    let mut pc: u32 = 0x82682E08;
    'dispatch: loop {
        match pc {
            0x82682E08 => {
    //   block [0x82682E08..0x82682E74)
	// 82682E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682E10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682E14: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682E18: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682E1C: 38EBCC90  addi r7, r11, -0x3370
	ctx.r[7].s64 = ctx.r[11].s64 + -13168;
	// 82682E20: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82682E24: 388A505C  addi r4, r10, 0x505c
	ctx.r[4].s64 = ctx.r[10].s64 + 20572;
	// 82682E28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682E2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682E30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82682E34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682E38: 386A97C8  addi r3, r10, -0x6838
	ctx.r[3].s64 = ctx.r[10].s64 + -26680;
	// 82682E3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82682E40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682E44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682E48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682E4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682E50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682E54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682E58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682E5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82682E60: 4BDE3FC1  bl 0x82466e20
	ctx.lr = 0x82682E64;
	sub_82466E20(ctx, base);
	// 82682E64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682E78 size=112
    let mut pc: u32 = 0x82682E78;
    'dispatch: loop {
        match pc {
            0x82682E78 => {
    //   block [0x82682E78..0x82682EE8)
	// 82682E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682E80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682E84: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82682E88: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682E8C: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82682E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682E94: 390BCCA8  addi r8, r11, -0x3358
	ctx.r[8].s64 = ctx.r[11].s64 + -13144;
	// 82682E98: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82682E9C: 388A5070  addi r4, r10, 0x5070
	ctx.r[4].s64 = ctx.r[10].s64 + 20592;
	// 82682EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682EA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682EA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682EAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682EB0: 386A97F8  addi r3, r10, -0x6808
	ctx.r[3].s64 = ctx.r[10].s64 + -26632;
	// 82682EB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82682EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682EBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682EC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682ED4: 4BDE3F4D  bl 0x82466e20
	ctx.lr = 0x82682ED8;
	sub_82466E20(ctx, base);
	// 82682ED8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682EE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682EE8 size=96
    let mut pc: u32 = 0x82682EE8;
    'dispatch: loop {
        match pc {
            0x82682EE8 => {
    //   block [0x82682EE8..0x82682F48)
	// 82682EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682EF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682EF4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682EF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682EFC: 388A507C  addi r4, r10, 0x507c
	ctx.r[4].s64 = ctx.r[10].s64 + 20604;
	// 82682F00: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682F04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82682F08: 386A9828  addi r3, r10, -0x67d8
	ctx.r[3].s64 = ctx.r[10].s64 + -26584;
	// 82682F0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82682F10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682F14: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82682F18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682F1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682F20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682F24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682F28: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82682F2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82682F30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82682F34: 4BDE3EED  bl 0x82466e20
	ctx.lr = 0x82682F38;
	sub_82466E20(ctx, base);
	// 82682F38: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682F3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682F40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682F48 size=112
    let mut pc: u32 = 0x82682F48;
    'dispatch: loop {
        match pc {
            0x82682F48 => {
    //   block [0x82682F48..0x82682FB8)
	// 82682F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682F50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682F54: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82682F58: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682F5C: 392A50FC  addi r9, r10, 0x50fc
	ctx.r[9].s64 = ctx.r[10].s64 + 20732;
	// 82682F60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682F64: 390BCCE0  addi r8, r11, -0x3320
	ctx.r[8].s64 = ctx.r[11].s64 + -13088;
	// 82682F68: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82682F6C: 388A508C  addi r4, r10, 0x508c
	ctx.r[4].s64 = ctx.r[10].s64 + 20620;
	// 82682F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682F74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682F78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682F7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82682F80: 386A9858  addi r3, r10, -0x67a8
	ctx.r[3].s64 = ctx.r[10].s64 + -26536;
	// 82682F84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82682F88: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82682F8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682F90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682F94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82682F98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682F9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82682FA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682FA4: 4BDE3E7D  bl 0x82466e20
	ctx.lr = 0x82682FA8;
	sub_82466E20(ctx, base);
	// 82682FA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82682FAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82682FB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82682FB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82682FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82682FB8 size=116
    let mut pc: u32 = 0x82682FB8;
    'dispatch: loop {
        match pc {
            0x82682FB8 => {
    //   block [0x82682FB8..0x8268302C)
	// 82682FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82682FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82682FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82682FC4: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82682FC8: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82682FCC: 390BCD88  addi r8, r11, -0x3278
	ctx.r[8].s64 = ctx.r[11].s64 + -12920;
	// 82682FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82682FD4: 392A50D0  addi r9, r10, 0x50d0
	ctx.r[9].s64 = ctx.r[10].s64 + 20688;
	// 82682FD8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82682FDC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82682FE0: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 82682FE4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82682FE8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82682FEC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82682FF0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82682FF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82682FF8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82682FFC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82683000: 388A50AC  addi r4, r10, 0x50ac
	ctx.r[4].s64 = ctx.r[10].s64 + 20652;
	// 82683004: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82683008: 386B9888  addi r3, r11, -0x6778
	ctx.r[3].s64 = ctx.r[11].s64 + -26488;
	// 8268300C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82683010: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683018: 4BDE3E09  bl 0x82466e20
	ctx.lr = 0x8268301C;
	sub_82466E20(ctx, base);
	// 8268301C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683030 size=112
    let mut pc: u32 = 0x82683030;
    'dispatch: loop {
        match pc {
            0x82683030 => {
    //   block [0x82683030..0x826830A0)
	// 82683030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268303C: 3D408204  lis r10, -0x7dfc
	ctx.r[10].s64 = -2113667072;
	// 82683040: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683044: 392A5128  addi r9, r10, 0x5128
	ctx.r[9].s64 = ctx.r[10].s64 + 20776;
	// 82683048: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268304C: 390BCDA0  addi r8, r11, -0x3260
	ctx.r[8].s64 = ctx.r[11].s64 + -12896;
	// 82683050: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82683054: 388A50C4  addi r4, r10, 0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + 20676;
	// 82683058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268305C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683060: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683068: 386A98B8  addi r3, r10, -0x6748
	ctx.r[3].s64 = ctx.r[10].s64 + -26440;
	// 8268306C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82683070: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82683074: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268307C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683084: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82683088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268308C: 4BDE3D95  bl 0x82466e20
	ctx.lr = 0x82683090;
	sub_82466E20(ctx, base);
	// 82683090: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268309C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826830A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826830A0 size=112
    let mut pc: u32 = 0x826830A0;
    'dispatch: loop {
        match pc {
            0x826830A0 => {
    //   block [0x826830A0..0x82683110)
	// 826830A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826830A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826830A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826830AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826830B0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826830B4: 38AA89E8  addi r5, r10, -0x7618
	ctx.r[5].s64 = ctx.r[10].s64 + -30232;
	// 826830B8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826830BC: 390BCE00  addi r8, r11, -0x3200
	ctx.r[8].s64 = ctx.r[11].s64 + -12800;
	// 826830C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826830C4: 388A50E0  addi r4, r10, 0x50e0
	ctx.r[4].s64 = ctx.r[10].s64 + 20704;
	// 826830C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826830CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826830D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826830D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826830D8: 386A98E8  addi r3, r10, -0x6718
	ctx.r[3].s64 = ctx.r[10].s64 + -26392;
	// 826830DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826830E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826830E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826830E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826830EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826830F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826830F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826830F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826830FC: 4BDE3D25  bl 0x82466e20
	ctx.lr = 0x82683100;
	sub_82466E20(ctx, base);
	// 82683100: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268310C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683110 size=112
    let mut pc: u32 = 0x82683110;
    'dispatch: loop {
        match pc {
            0x82683110 => {
    //   block [0x82683110..0x82683180)
	// 82683110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268311C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683120: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683124: 38AA88F8  addi r5, r10, -0x7708
	ctx.r[5].s64 = ctx.r[10].s64 + -30472;
	// 82683128: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268312C: 390BCE18  addi r8, r11, -0x31e8
	ctx.r[8].s64 = ctx.r[11].s64 + -12776;
	// 82683130: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82683134: 388A50F4  addi r4, r10, 0x50f4
	ctx.r[4].s64 = ctx.r[10].s64 + 20724;
	// 82683138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268313C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683140: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683148: 386A9918  addi r3, r10, -0x66e8
	ctx.r[3].s64 = ctx.r[10].s64 + -26344;
	// 8268314C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683154: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268315C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268316C: 4BDE3CB5  bl 0x82466e20
	ctx.lr = 0x82683170;
	sub_82466E20(ctx, base);
	// 82683170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268317C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683180 size=112
    let mut pc: u32 = 0x82683180;
    'dispatch: loop {
        match pc {
            0x82683180 => {
    //   block [0x82683180..0x826831F0)
	// 82683180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683188: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268318C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683190: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683194: 38AA88F8  addi r5, r10, -0x7708
	ctx.r[5].s64 = ctx.r[10].s64 + -30472;
	// 82683198: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268319C: 390BCE60  addi r8, r11, -0x31a0
	ctx.r[8].s64 = ctx.r[11].s64 + -12704;
	// 826831A0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 826831A4: 388A510C  addi r4, r10, 0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + 20748;
	// 826831A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826831AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826831B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826831B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826831B8: 386A9948  addi r3, r10, -0x66b8
	ctx.r[3].s64 = ctx.r[10].s64 + -26296;
	// 826831BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826831C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826831C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826831C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826831CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826831D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826831D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826831D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826831DC: 4BDE3C45  bl 0x82466e20
	ctx.lr = 0x826831E0;
	sub_82466E20(ctx, base);
	// 826831E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826831E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826831E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826831EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826831F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826831F0 size=112
    let mut pc: u32 = 0x826831F0;
    'dispatch: loop {
        match pc {
            0x826831F0 => {
    //   block [0x826831F0..0x82683260)
	// 826831F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826831F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826831F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826831FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683200: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683204: 38AA8928  addi r5, r10, -0x76d8
	ctx.r[5].s64 = ctx.r[10].s64 + -30424;
	// 82683208: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268320C: 390BCEC0  addi r8, r11, -0x3140
	ctx.r[8].s64 = ctx.r[11].s64 + -12608;
	// 82683210: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82683214: 388A511C  addi r4, r10, 0x511c
	ctx.r[4].s64 = ctx.r[10].s64 + 20764;
	// 82683218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268321C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683228: 386A9978  addi r3, r10, -0x6688
	ctx.r[3].s64 = ctx.r[10].s64 + -26248;
	// 8268322C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268323C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268324C: 4BDE3BD5  bl 0x82466e20
	ctx.lr = 0x82683250;
	sub_82466E20(ctx, base);
	// 82683250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268325C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683260 size=112
    let mut pc: u32 = 0x82683260;
    'dispatch: loop {
        match pc {
            0x82683260 => {
    //   block [0x82683260..0x826832D0)
	// 82683260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268326C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683270: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683274: 38AA8928  addi r5, r10, -0x76d8
	ctx.r[5].s64 = ctx.r[10].s64 + -30424;
	// 82683278: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268327C: 390BCF20  addi r8, r11, -0x30e0
	ctx.r[8].s64 = ctx.r[11].s64 + -12512;
	// 82683280: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82683284: 388A512C  addi r4, r10, 0x512c
	ctx.r[4].s64 = ctx.r[10].s64 + 20780;
	// 82683288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268328C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683298: 386A99A8  addi r3, r10, -0x6658
	ctx.r[3].s64 = ctx.r[10].s64 + -26200;
	// 8268329C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826832A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826832A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826832A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826832AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826832B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826832B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826832B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826832BC: 4BDE3B65  bl 0x82466e20
	ctx.lr = 0x826832C0;
	sub_82466E20(ctx, base);
	// 826832C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826832C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826832C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826832CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826832D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826832D0 size=112
    let mut pc: u32 = 0x826832D0;
    'dispatch: loop {
        match pc {
            0x826832D0 => {
    //   block [0x826832D0..0x82683340)
	// 826832D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826832D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826832D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826832DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826832E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826832E4: 38AA88F8  addi r5, r10, -0x7708
	ctx.r[5].s64 = ctx.r[10].s64 + -30472;
	// 826832E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826832EC: 390BCF80  addi r8, r11, -0x3080
	ctx.r[8].s64 = ctx.r[11].s64 + -12416;
	// 826832F0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 826832F4: 388A5140  addi r4, r10, 0x5140
	ctx.r[4].s64 = ctx.r[10].s64 + 20800;
	// 826832F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826832FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683300: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683304: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683308: 386A99D8  addi r3, r10, -0x6628
	ctx.r[3].s64 = ctx.r[10].s64 + -26152;
	// 8268330C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683314: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268331C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683324: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268332C: 4BDE3AF5  bl 0x82466e20
	ctx.lr = 0x82683330;
	sub_82466E20(ctx, base);
	// 82683330: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683334: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683338: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268333C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683340 size=112
    let mut pc: u32 = 0x82683340;
    'dispatch: loop {
        match pc {
            0x82683340 => {
    //   block [0x82683340..0x826833B0)
	// 82683340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268334C: 3D40827B  lis r10, -0x7d85
	ctx.r[10].s64 = -2105868288;
	// 82683350: 39000013  li r8, 0x13
	ctx.r[8].s64 = 19;
	// 82683354: 38EAD040  addi r7, r10, -0x2fc0
	ctx.r[7].s64 = ctx.r[10].s64 + -12224;
	// 82683358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268335C: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 82683360: 388A5150  addi r4, r10, 0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + 20816;
	// 82683364: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683368: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8268336C: 396B5140  addi r11, r11, 0x5140
	ctx.r[11].s64 = ctx.r[11].s64 + 20800;
	// 82683370: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82683374: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683378: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8268337C: 386A9A08  addi r3, r10, -0x65f8
	ctx.r[3].s64 = ctx.r[10].s64 + -26104;
	// 82683380: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683384: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82683388: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8268338C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82683390: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683394: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683398: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8268339C: 4BDE3A85  bl 0x82466e20
	ctx.lr = 0x826833A0;
	sub_82466E20(ctx, base);
	// 826833A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826833A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826833A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826833AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826833B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826833B0 size=112
    let mut pc: u32 = 0x826833B0;
    'dispatch: loop {
        match pc {
            0x826833B0 => {
    //   block [0x826833B0..0x82683420)
	// 826833B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826833B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826833B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826833BC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826833C0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826833C4: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 826833C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826833CC: 390BD208  addi r8, r11, -0x2df8
	ctx.r[8].s64 = ctx.r[11].s64 + -11768;
	// 826833D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826833D4: 388A5168  addi r4, r10, 0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + 20840;
	// 826833D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826833DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826833E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826833E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826833E8: 386A9A38  addi r3, r10, -0x65c8
	ctx.r[3].s64 = ctx.r[10].s64 + -26056;
	// 826833EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826833F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826833F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826833F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826833FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82683400: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683404: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683408: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268340C: 4BDE3A15  bl 0x82466e20
	ctx.lr = 0x82683410;
	sub_82466E20(ctx, base);
	// 82683410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268341C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683420 size=112
    let mut pc: u32 = 0x82683420;
    'dispatch: loop {
        match pc {
            0x82683420 => {
    //   block [0x82683420..0x82683490)
	// 82683420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268342C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683430: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683434: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 82683438: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268343C: 390BD220  addi r8, r11, -0x2de0
	ctx.r[8].s64 = ctx.r[11].s64 + -11744;
	// 82683440: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82683444: 388A5184  addi r4, r10, 0x5184
	ctx.r[4].s64 = ctx.r[10].s64 + 20868;
	// 82683448: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268344C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683450: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683454: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683458: 386A9A68  addi r3, r10, -0x6598
	ctx.r[3].s64 = ctx.r[10].s64 + -26008;
	// 8268345C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683460: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683464: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683468: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268346C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82683470: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683474: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683478: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268347C: 4BDE39A5  bl 0x82466e20
	ctx.lr = 0x82683480;
	sub_82466E20(ctx, base);
	// 82683480: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268348C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683490 size=112
    let mut pc: u32 = 0x82683490;
    'dispatch: loop {
        match pc {
            0x82683490 => {
    //   block [0x82683490..0x82683500)
	// 82683490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268349C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826834A0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826834A4: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 826834A8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826834AC: 390BD238  addi r8, r11, -0x2dc8
	ctx.r[8].s64 = ctx.r[11].s64 + -11720;
	// 826834B0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 826834B4: 388A51A4  addi r4, r10, 0x51a4
	ctx.r[4].s64 = ctx.r[10].s64 + 20900;
	// 826834B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826834BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826834C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826834C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826834C8: 386A9A98  addi r3, r10, -0x6568
	ctx.r[3].s64 = ctx.r[10].s64 + -25960;
	// 826834CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826834D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826834D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826834D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826834DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826834E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826834E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826834E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826834EC: 4BDE3935  bl 0x82466e20
	ctx.lr = 0x826834F0;
	sub_82466E20(ctx, base);
	// 826834F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826834F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826834F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826834FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683500 size=108
    let mut pc: u32 = 0x82683500;
    'dispatch: loop {
        match pc {
            0x82683500 => {
    //   block [0x82683500..0x8268356C)
	// 82683500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268350C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683510: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683514: 38EBD268  addi r7, r11, -0x2d98
	ctx.r[7].s64 = ctx.r[11].s64 + -11672;
	// 82683518: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268351C: 388A51BC  addi r4, r10, 0x51bc
	ctx.r[4].s64 = ctx.r[10].s64 + 20924;
	// 82683520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683524: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683528: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268352C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683530: 386A9AC8  addi r3, r10, -0x6538
	ctx.r[3].s64 = ctx.r[10].s64 + -25912;
	// 82683534: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82683538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268353C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683540: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683548: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268354C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683550: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683554: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82683558: 4BDE38C9  bl 0x82466e20
	ctx.lr = 0x8268355C;
	sub_82466E20(ctx, base);
	// 8268355C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683570 size=112
    let mut pc: u32 = 0x82683570;
    'dispatch: loop {
        match pc {
            0x82683570 => {
    //   block [0x82683570..0x826835E0)
	// 82683570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268357C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683580: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683584: 38AA77B8  addi r5, r10, 0x77b8
	ctx.r[5].s64 = ctx.r[10].s64 + 30648;
	// 82683588: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268358C: 390BD298  addi r8, r11, -0x2d68
	ctx.r[8].s64 = ctx.r[11].s64 + -11624;
	// 82683590: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82683594: 388A51E4  addi r4, r10, 0x51e4
	ctx.r[4].s64 = ctx.r[10].s64 + 20964;
	// 82683598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268359C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826835A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826835A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826835A8: 386A9AF8  addi r3, r10, -0x6508
	ctx.r[3].s64 = ctx.r[10].s64 + -25864;
	// 826835AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826835B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826835B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826835B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826835BC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826835C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826835C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826835C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826835CC: 4BDE3855  bl 0x82466e20
	ctx.lr = 0x826835D0;
	sub_82466E20(ctx, base);
	// 826835D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826835D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826835D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826835DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826835E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826835E0 size=108
    let mut pc: u32 = 0x826835E0;
    'dispatch: loop {
        match pc {
            0x826835E0 => {
    //   block [0x826835E0..0x8268364C)
	// 826835E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826835E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826835E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826835EC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826835F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826835F4: 38EBD2B0  addi r7, r11, -0x2d50
	ctx.r[7].s64 = ctx.r[11].s64 + -11600;
	// 826835F8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826835FC: 388A5200  addi r4, r10, 0x5200
	ctx.r[4].s64 = ctx.r[10].s64 + 20992;
	// 82683600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268360C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683610: 386A9B28  addi r3, r10, -0x64d8
	ctx.r[3].s64 = ctx.r[10].s64 + -25816;
	// 82683614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82683618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268361C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268362C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82683638: 4BDE37E9  bl 0x82466e20
	ctx.lr = 0x8268363C;
	sub_82466E20(ctx, base);
	// 8268363C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683650 size=108
    let mut pc: u32 = 0x82683650;
    'dispatch: loop {
        match pc {
            0x82683650 => {
    //   block [0x82683650..0x826836BC)
	// 82683650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268365C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683664: 38EBD2E0  addi r7, r11, -0x2d20
	ctx.r[7].s64 = ctx.r[11].s64 + -11552;
	// 82683668: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8268366C: 388A521C  addi r4, r10, 0x521c
	ctx.r[4].s64 = ctx.r[10].s64 + 21020;
	// 82683670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683674: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268367C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683680: 386A9B58  addi r3, r10, -0x64a8
	ctx.r[3].s64 = ctx.r[10].s64 + -25768;
	// 82683684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82683688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268368C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268369C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826836A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826836A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826836A8: 4BDE3779  bl 0x82466e20
	ctx.lr = 0x826836AC;
	sub_82466E20(ctx, base);
	// 826836AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826836B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826836B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826836B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826836C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826836C0 size=112
    let mut pc: u32 = 0x826836C0;
    'dispatch: loop {
        match pc {
            0x826836C0 => {
    //   block [0x826836C0..0x82683730)
	// 826836C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826836C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826836C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826836CC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826836D0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826836D4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826836D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826836DC: 390BD328  addi r8, r11, -0x2cd8
	ctx.r[8].s64 = ctx.r[11].s64 + -11480;
	// 826836E0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 826836E4: 388A523C  addi r4, r10, 0x523c
	ctx.r[4].s64 = ctx.r[10].s64 + 21052;
	// 826836E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826836EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826836F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826836F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826836F8: 386A9B88  addi r3, r10, -0x6478
	ctx.r[3].s64 = ctx.r[10].s64 + -25720;
	// 826836FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683704: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268370C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268371C: 4BDE3705  bl 0x82466e20
	ctx.lr = 0x82683720;
	sub_82466E20(ctx, base);
	// 82683720: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268372C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683730 size=112
    let mut pc: u32 = 0x82683730;
    'dispatch: loop {
        match pc {
            0x82683730 => {
    //   block [0x82683730..0x826837A0)
	// 82683730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268373C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683740: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683744: 38AA8928  addi r5, r10, -0x76d8
	ctx.r[5].s64 = ctx.r[10].s64 + -30424;
	// 82683748: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268374C: 390BD370  addi r8, r11, -0x2c90
	ctx.r[8].s64 = ctx.r[11].s64 + -11408;
	// 82683750: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82683754: 388A5254  addi r4, r10, 0x5254
	ctx.r[4].s64 = ctx.r[10].s64 + 21076;
	// 82683758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268375C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683760: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683764: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683768: 386A9BB8  addi r3, r10, -0x6448
	ctx.r[3].s64 = ctx.r[10].s64 + -25672;
	// 8268376C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683774: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268377C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268378C: 4BDE3695  bl 0x82466e20
	ctx.lr = 0x82683790;
	sub_82466E20(ctx, base);
	// 82683790: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683794: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683798: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268379C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826837A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826837A0 size=108
    let mut pc: u32 = 0x826837A0;
    'dispatch: loop {
        match pc {
            0x826837A0 => {
    //   block [0x826837A0..0x8268380C)
	// 826837A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826837A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826837A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826837AC: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826837B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826837B4: 38EBD400  addi r7, r11, -0x2c00
	ctx.r[7].s64 = ctx.r[11].s64 + -11264;
	// 826837B8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826837BC: 388A5268  addi r4, r10, 0x5268
	ctx.r[4].s64 = ctx.r[10].s64 + 21096;
	// 826837C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826837C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826837C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826837CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826837D0: 386A9BE8  addi r3, r10, -0x6418
	ctx.r[3].s64 = ctx.r[10].s64 + -25624;
	// 826837D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826837D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826837DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826837E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826837E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826837E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826837EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826837F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826837F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826837F8: 4BDE3629  bl 0x82466e20
	ctx.lr = 0x826837FC;
	sub_82466E20(ctx, base);
	// 826837FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683810 size=108
    let mut pc: u32 = 0x82683810;
    'dispatch: loop {
        match pc {
            0x82683810 => {
    //   block [0x82683810..0x8268387C)
	// 82683810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268381C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683824: 38EBD448  addi r7, r11, -0x2bb8
	ctx.r[7].s64 = ctx.r[11].s64 + -11192;
	// 82683828: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268382C: 388A5284  addi r4, r10, 0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + 21124;
	// 82683830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683834: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683838: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8268383C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683840: 386A9C18  addi r3, r10, -0x63e8
	ctx.r[3].s64 = ctx.r[10].s64 + -25576;
	// 82683844: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82683848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8268384C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8268385C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82683868: 4BDE35B9  bl 0x82466e20
	ctx.lr = 0x8268386C;
	sub_82466E20(ctx, base);
	// 8268386C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683880 size=108
    let mut pc: u32 = 0x82683880;
    'dispatch: loop {
        match pc {
            0x82683880 => {
    //   block [0x82683880..0x826838EC)
	// 82683880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268388C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683890: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683894: 38EBD478  addi r7, r11, -0x2b88
	ctx.r[7].s64 = ctx.r[11].s64 + -11144;
	// 82683898: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8268389C: 388A52A4  addi r4, r10, 0x52a4
	ctx.r[4].s64 = ctx.r[10].s64 + 21156;
	// 826838A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826838A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826838A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826838AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826838B0: 386A9C48  addi r3, r10, -0x63b8
	ctx.r[3].s64 = ctx.r[10].s64 + -25528;
	// 826838B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826838B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826838BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826838C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826838C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826838C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826838CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826838D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826838D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826838D8: 4BDE3549  bl 0x82466e20
	ctx.lr = 0x826838DC;
	sub_82466E20(ctx, base);
	// 826838DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826838E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826838E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826838E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826838F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826838F0 size=112
    let mut pc: u32 = 0x826838F0;
    'dispatch: loop {
        match pc {
            0x826838F0 => {
    //   block [0x826838F0..0x82683960)
	// 826838F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826838F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826838F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826838FC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683900: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683904: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268390C: 390BD4A8  addi r8, r11, -0x2b58
	ctx.r[8].s64 = ctx.r[11].s64 + -11096;
	// 82683910: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82683914: 388A52BC  addi r4, r10, 0x52bc
	ctx.r[4].s64 = ctx.r[10].s64 + 21180;
	// 82683918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268391C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683920: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683924: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683928: 386A9C78  addi r3, r10, -0x6388
	ctx.r[3].s64 = ctx.r[10].s64 + -25480;
	// 8268392C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683934: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8268393C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8268394C: 4BDE34D5  bl 0x82466e20
	ctx.lr = 0x82683950;
	sub_82466E20(ctx, base);
	// 82683950: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683954: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683958: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8268395C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683960 size=112
    let mut pc: u32 = 0x82683960;
    'dispatch: loop {
        match pc {
            0x82683960 => {
    //   block [0x82683960..0x826839D0)
	// 82683960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683964: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8268396C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683970: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683974: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683978: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8268397C: 390BD4D8  addi r8, r11, -0x2b28
	ctx.r[8].s64 = ctx.r[11].s64 + -11048;
	// 82683980: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82683984: 388A52CC  addi r4, r10, 0x52cc
	ctx.r[4].s64 = ctx.r[10].s64 + 21196;
	// 82683988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8268398C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683990: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683994: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683998: 386A9CA8  addi r3, r10, -0x6358
	ctx.r[3].s64 = ctx.r[10].s64 + -25432;
	// 8268399C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826839A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826839A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826839A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826839AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826839B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826839B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826839B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826839BC: 4BDE3465  bl 0x82466e20
	ctx.lr = 0x826839C0;
	sub_82466E20(ctx, base);
	// 826839C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826839C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826839C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826839CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826839D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826839D0 size=112
    let mut pc: u32 = 0x826839D0;
    'dispatch: loop {
        match pc {
            0x826839D0 => {
    //   block [0x826839D0..0x82683A40)
	// 826839D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826839D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826839D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826839DC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 826839E0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 826839E4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 826839E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826839EC: 390BD4F0  addi r8, r11, -0x2b10
	ctx.r[8].s64 = ctx.r[11].s64 + -11024;
	// 826839F0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 826839F4: 388A52E8  addi r4, r10, 0x52e8
	ctx.r[4].s64 = ctx.r[10].s64 + 21224;
	// 826839F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826839FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683A00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683A04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683A08: 386A9CD8  addi r3, r10, -0x6328
	ctx.r[3].s64 = ctx.r[10].s64 + -25384;
	// 82683A0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683A1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683A2C: 4BDE33F5  bl 0x82466e20
	ctx.lr = 0x82683A30;
	sub_82466E20(ctx, base);
	// 82683A30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683A34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683A38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683A3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683A40 size=108
    let mut pc: u32 = 0x82683A40;
    'dispatch: loop {
        match pc {
            0x82683A40 => {
    //   block [0x82683A40..0x82683AAC)
	// 82683A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683A48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683A4C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683A50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683A54: 38EBD508  addi r7, r11, -0x2af8
	ctx.r[7].s64 = ctx.r[11].s64 + -11000;
	// 82683A58: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82683A5C: 388A5308  addi r4, r10, 0x5308
	ctx.r[4].s64 = ctx.r[10].s64 + 21256;
	// 82683A60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683A64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683A68: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82683A6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683A70: 386A9D08  addi r3, r10, -0x62f8
	ctx.r[3].s64 = ctx.r[10].s64 + -25336;
	// 82683A74: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82683A78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683A7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683A80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683A84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683A88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683A8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683A90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683A94: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82683A98: 4BDE3389  bl 0x82466e20
	ctx.lr = 0x82683A9C;
	sub_82466E20(ctx, base);
	// 82683A9C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683AB0 size=112
    let mut pc: u32 = 0x82683AB0;
    'dispatch: loop {
        match pc {
            0x82683AB0 => {
    //   block [0x82683AB0..0x82683B20)
	// 82683AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683AB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683ABC: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683AC0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683AC4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683AC8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683ACC: 390BD538  addi r8, r11, -0x2ac8
	ctx.r[8].s64 = ctx.r[11].s64 + -10952;
	// 82683AD0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82683AD4: 388A533C  addi r4, r10, 0x533c
	ctx.r[4].s64 = ctx.r[10].s64 + 21308;
	// 82683AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683ADC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683AE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683AE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683AE8: 386A9D38  addi r3, r10, -0x62c8
	ctx.r[3].s64 = ctx.r[10].s64 + -25288;
	// 82683AEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683AF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683AF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683AFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683B04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683B0C: 4BDE3315  bl 0x82466e20
	ctx.lr = 0x82683B10;
	sub_82466E20(ctx, base);
	// 82683B10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683B1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683B20 size=108
    let mut pc: u32 = 0x82683B20;
    'dispatch: loop {
        match pc {
            0x82683B20 => {
    //   block [0x82683B20..0x82683B8C)
	// 82683B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683B2C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683B30: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683B34: 38EBD550  addi r7, r11, -0x2ab0
	ctx.r[7].s64 = ctx.r[11].s64 + -10928;
	// 82683B38: 3900000A  li r8, 0xa
	ctx.r[8].s64 = 10;
	// 82683B3C: 388A535C  addi r4, r10, 0x535c
	ctx.r[4].s64 = ctx.r[10].s64 + 21340;
	// 82683B40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683B44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683B48: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82683B4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683B50: 386A9D68  addi r3, r10, -0x6298
	ctx.r[3].s64 = ctx.r[10].s64 + -25240;
	// 82683B54: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82683B58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683B5C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683B60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683B68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683B6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683B70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683B74: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82683B78: 4BDE32A9  bl 0x82466e20
	ctx.lr = 0x82683B7C;
	sub_82466E20(ctx, base);
	// 82683B7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683B90 size=112
    let mut pc: u32 = 0x82683B90;
    'dispatch: loop {
        match pc {
            0x82683B90 => {
    //   block [0x82683B90..0x82683C00)
	// 82683B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683B98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683B9C: 3D408296  lis r10, -0x7d6a
	ctx.r[10].s64 = -2104098816;
	// 82683BA0: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683BA4: 38AA6A98  addi r5, r10, 0x6a98
	ctx.r[5].s64 = ctx.r[10].s64 + 27288;
	// 82683BA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683BAC: 390BD640  addi r8, r11, -0x29c0
	ctx.r[8].s64 = ctx.r[11].s64 + -10688;
	// 82683BB0: 39200012  li r9, 0x12
	ctx.r[9].s64 = 18;
	// 82683BB4: 388A5380  addi r4, r10, 0x5380
	ctx.r[4].s64 = ctx.r[10].s64 + 21376;
	// 82683BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683BBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683BC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683BC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683BC8: 386A9D98  addi r3, r10, -0x6268
	ctx.r[3].s64 = ctx.r[10].s64 + -25192;
	// 82683BCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683BD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683BD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683BD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683BDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683BE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683BE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683BE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683BEC: 4BDE3235  bl 0x82466e20
	ctx.lr = 0x82683BF0;
	sub_82466E20(ctx, base);
	// 82683BF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683C00 size=108
    let mut pc: u32 = 0x82683C00;
    'dispatch: loop {
        match pc {
            0x82683C00 => {
    //   block [0x82683C00..0x82683C6C)
	// 82683C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683C08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683C0C: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683C10: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683C14: 38EBD7F0  addi r7, r11, -0x2810
	ctx.r[7].s64 = ctx.r[11].s64 + -10256;
	// 82683C18: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82683C1C: 388A5390  addi r4, r10, 0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + 21392;
	// 82683C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683C24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683C28: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82683C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683C30: 386A9DC8  addi r3, r10, -0x6238
	ctx.r[3].s64 = ctx.r[10].s64 + -25144;
	// 82683C34: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82683C38: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683C3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683C40: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683C44: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683C48: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683C4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683C50: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683C54: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82683C58: 4BDE31C9  bl 0x82466e20
	ctx.lr = 0x82683C5C;
	sub_82466E20(ctx, base);
	// 82683C5C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82683C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82683C70 size=112
    let mut pc: u32 = 0x82683C70;
    'dispatch: loop {
        match pc {
            0x82683C70 => {
    //   block [0x82683C70..0x82683CE0)
	// 82683C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82683C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82683C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82683C7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683C80: 3D60827B  lis r11, -0x7d85
	ctx.r[11].s64 = -2105868288;
	// 82683C84: 38AA8928  addi r5, r10, -0x76d8
	ctx.r[5].s64 = ctx.r[10].s64 + -30424;
	// 82683C88: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82683C8C: 390BD988  addi r8, r11, -0x2678
	ctx.r[8].s64 = ctx.r[11].s64 + -9848;
	// 82683C90: 39200019  li r9, 0x19
	ctx.r[9].s64 = 25;
	// 82683C94: 388A53AC  addi r4, r10, 0x53ac
	ctx.r[4].s64 = ctx.r[10].s64 + 21420;
	// 82683C98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82683C9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82683CA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82683CA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82683CA8: 386A9DF8  addi r3, r10, -0x6208
	ctx.r[3].s64 = ctx.r[10].s64 + -25096;
	// 82683CAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82683CB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82683CB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82683CB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82683CBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82683CC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82683CC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82683CC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82683CCC: 4BDE3155  bl 0x82466e20
	ctx.lr = 0x82683CD0;
	sub_82466E20(ctx, base);
	// 82683CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82683CD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82683CD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82683CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


