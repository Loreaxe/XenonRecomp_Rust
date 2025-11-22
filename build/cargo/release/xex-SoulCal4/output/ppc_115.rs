pub fn sub_826973F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826973F0 size=108
    let mut pc: u32 = 0x826973F0;
    'dispatch: loop {
        match pc {
            0x826973F0 => {
    //   block [0x826973F0..0x8269745C)
	// 826973F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826973F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826973F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826973FC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697400: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697404: 38EBD8D8  addi r7, r11, -0x2728
	ctx.r[7].s64 = ctx.r[11].s64 + -10024;
	// 82697408: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269740C: 388A3788  addi r4, r10, 0x3788
	ctx.r[4].s64 = ctx.r[10].s64 + 14216;
	// 82697410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697414: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697418: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269741C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697420: 386A21E0  addi r3, r10, 0x21e0
	ctx.r[3].s64 = ctx.r[10].s64 + 8672;
	// 82697424: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697428: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269742C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697430: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697434: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697438: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269743C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697440: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697444: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697448: 4BDCF9D9  bl 0x82466e20
	ctx.lr = 0x8269744C;
	sub_82466E20(ctx, base);
	// 8269744C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697460 size=116
    let mut pc: u32 = 0x82697460;
    'dispatch: loop {
        match pc {
            0x82697460 => {
    //   block [0x82697460..0x826974D4)
	// 82697460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697468: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269746C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697470: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82697474: 390BD908  addi r8, r11, -0x26f8
	ctx.r[8].s64 = ctx.r[11].s64 + -9976;
	// 82697478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269747C: 392A9044  addi r9, r10, -0x6fbc
	ctx.r[9].s64 = ctx.r[10].s64 + -28604;
	// 82697480: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697484: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82697488: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269748C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697490: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697494: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697498: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269749C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826974A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826974A4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826974A8: 388A37A0  addi r4, r10, 0x37a0
	ctx.r[4].s64 = ctx.r[10].s64 + 14240;
	// 826974AC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826974B0: 386B2210  addi r3, r11, 0x2210
	ctx.r[3].s64 = ctx.r[11].s64 + 8720;
	// 826974B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826974B8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826974BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826974C0: 4BDCF961  bl 0x82466e20
	ctx.lr = 0x826974C4;
	sub_82466E20(ctx, base);
	// 826974C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826974C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826974CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826974D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826974D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826974D8 size=108
    let mut pc: u32 = 0x826974D8;
    'dispatch: loop {
        match pc {
            0x826974D8 => {
    //   block [0x826974D8..0x82697544)
	// 826974D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826974DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826974E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826974E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826974E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826974EC: 38EBD920  addi r7, r11, -0x26e0
	ctx.r[7].s64 = ctx.r[11].s64 + -9952;
	// 826974F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826974F4: 388A3714  addi r4, r10, 0x3714
	ctx.r[4].s64 = ctx.r[10].s64 + 14100;
	// 826974F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826974FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697500: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697504: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697508: 386A2240  addi r3, r10, 0x2240
	ctx.r[3].s64 = ctx.r[10].s64 + 8768;
	// 8269750C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697510: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697518: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269751C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697520: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697524: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697528: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269752C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697530: 4BDCF8F1  bl 0x82466e20
	ctx.lr = 0x82697534;
	sub_82466E20(ctx, base);
	// 82697534: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269753C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697548 size=112
    let mut pc: u32 = 0x82697548;
    'dispatch: loop {
        match pc {
            0x82697548 => {
    //   block [0x82697548..0x826975B8)
	// 82697548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269754C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697550: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697554: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697558: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269755C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 82697560: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82697564: 390BD938  addi r8, r11, -0x26c8
	ctx.r[8].s64 = ctx.r[11].s64 + -9928;
	// 82697568: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 8269756C: 388A36AC  addi r4, r10, 0x36ac
	ctx.r[4].s64 = ctx.r[10].s64 + 13996;
	// 82697570: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697574: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697578: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269757C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697580: 386A2270  addi r3, r10, 0x2270
	ctx.r[3].s64 = ctx.r[10].s64 + 8816;
	// 82697584: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269758C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697590: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697594: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697598: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269759C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826975A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826975A4: 4BDCF87D  bl 0x82466e20
	ctx.lr = 0x826975A8;
	sub_82466E20(ctx, base);
	// 826975A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826975AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826975B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826975B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826975B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826975B8 size=108
    let mut pc: u32 = 0x826975B8;
    'dispatch: loop {
        match pc {
            0x826975B8 => {
    //   block [0x826975B8..0x82697624)
	// 826975B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826975BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826975C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826975C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826975C8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826975CC: 38EBDA10  addi r7, r11, -0x25f0
	ctx.r[7].s64 = ctx.r[11].s64 + -9712;
	// 826975D0: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 826975D4: 388A36C8  addi r4, r10, 0x36c8
	ctx.r[4].s64 = ctx.r[10].s64 + 14024;
	// 826975D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826975DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826975E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826975E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826975E8: 386A22A0  addi r3, r10, 0x22a0
	ctx.r[3].s64 = ctx.r[10].s64 + 8864;
	// 826975EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826975F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826975F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826975F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826975FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697600: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697604: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697608: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269760C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697610: 4BDCF811  bl 0x82466e20
	ctx.lr = 0x82697614;
	sub_82466E20(ctx, base);
	// 82697614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697618: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269761C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697628 size=108
    let mut pc: u32 = 0x82697628;
    'dispatch: loop {
        match pc {
            0x82697628 => {
    //   block [0x82697628..0x82697694)
	// 82697628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269762C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697634: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697638: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269763C: 38EBDA88  addi r7, r11, -0x2578
	ctx.r[7].s64 = ctx.r[11].s64 + -9592;
	// 82697640: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82697644: 388A36E4  addi r4, r10, 0x36e4
	ctx.r[4].s64 = ctx.r[10].s64 + 14052;
	// 82697648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269764C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697650: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697654: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697658: 386A22D0  addi r3, r10, 0x22d0
	ctx.r[3].s64 = ctx.r[10].s64 + 8912;
	// 8269765C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697660: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697664: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697668: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269766C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697670: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697674: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697678: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269767C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697680: 4BDCF7A1  bl 0x82466e20
	ctx.lr = 0x82697684;
	sub_82466E20(ctx, base);
	// 82697684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697688: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269768C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697698 size=116
    let mut pc: u32 = 0x82697698;
    'dispatch: loop {
        match pc {
            0x82697698 => {
    //   block [0x82697698..0x8269770C)
	// 82697698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269769C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826976A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826976A4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826976A8: 38E00017  li r7, 0x17
	ctx.r[7].s64 = 23;
	// 826976AC: 390ADAD0  addi r8, r10, -0x2530
	ctx.r[8].s64 = ctx.r[10].s64 + -9520;
	// 826976B0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826976B4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 826976B8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826976BC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826976C0: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826976C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826976C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826976CC: 388A3704  addi r4, r10, 0x3704
	ctx.r[4].s64 = ctx.r[10].s64 + 14084;
	// 826976D0: 396B9058  addi r11, r11, -0x6fa8
	ctx.r[11].s64 = ctx.r[11].s64 + -28584;
	// 826976D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826976D8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826976DC: 386A2300  addi r3, r10, 0x2300
	ctx.r[3].s64 = ctx.r[10].s64 + 8960;
	// 826976E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826976E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826976E8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826976EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826976F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826976F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826976F8: 4BDCF729  bl 0x82466e20
	ctx.lr = 0x826976FC;
	sub_82466E20(ctx, base);
	// 826976FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697710 size=112
    let mut pc: u32 = 0x82697710;
    'dispatch: loop {
        match pc {
            0x82697710 => {
    //   block [0x82697710..0x82697780)
	// 82697710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269771C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697720: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697724: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82697728: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269772C: 390BDCF8  addi r8, r11, -0x2308
	ctx.r[8].s64 = ctx.r[11].s64 + -8968;
	// 82697730: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82697734: 388AB374  addi r4, r10, -0x4c8c
	ctx.r[4].s64 = ctx.r[10].s64 + -19596;
	// 82697738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269773C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697744: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697748: 386A2330  addi r3, r10, 0x2330
	ctx.r[3].s64 = ctx.r[10].s64 + 9008;
	// 8269774C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697750: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697754: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269775C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269776C: 4BDCF6B5  bl 0x82466e20
	ctx.lr = 0x82697770;
	sub_82466E20(ctx, base);
	// 82697770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269777C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697780 size=76
    let mut pc: u32 = 0x82697780;
    'dispatch: loop {
        match pc {
            0x82697780 => {
    //   block [0x82697780..0x826977CC)
	// 82697780: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697784: 814BD874  lwz r10, -0x278c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10124 as u32) ) } as u64;
	// 82697788: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269778C: 396B5988  addi r11, r11, 0x5988
	ctx.r[11].s64 = ctx.r[11].s64 + 22920;
	// 82697790: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 82697794: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82697798: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 8269779C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 826977A0: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 826977A4: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 826977A8: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826977AC: 814ADD10  lwz r10, -0x22f0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8944 as u32) ) } as u64;
	// 826977B0: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 826977B4: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 826977B8: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 826977BC: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 826977C0: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 826977C4: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 826977C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826977D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826977D0 size=108
    let mut pc: u32 = 0x826977D0;
    'dispatch: loop {
        match pc {
            0x826977D0 => {
    //   block [0x826977D0..0x8269783C)
	// 826977D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826977D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826977D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826977DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826977E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826977E4: 38EB5988  addi r7, r11, 0x5988
	ctx.r[7].s64 = ctx.r[11].s64 + 22920;
	// 826977E8: 3900001A  li r8, 0x1a
	ctx.r[8].s64 = 26;
	// 826977EC: 388AABE8  addi r4, r10, -0x5418
	ctx.r[4].s64 = ctx.r[10].s64 + -21528;
	// 826977F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826977F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826977F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826977FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697800: 386A2360  addi r3, r10, 0x2360
	ctx.r[3].s64 = ctx.r[10].s64 + 9056;
	// 82697804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269780C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269781C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697828: 4BDCF5F9  bl 0x82466e20
	ctx.lr = 0x8269782C;
	sub_82466E20(ctx, base);
	// 8269782C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697840 size=76
    let mut pc: u32 = 0x82697840;
    'dispatch: loop {
        match pc {
            0x82697840 => {
    //   block [0x82697840..0x8269788C)
	// 82697840: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697844: 814BD874  lwz r10, -0x278c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10124 as u32) ) } as u64;
	// 82697848: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269784C: 396B5BF8  addi r11, r11, 0x5bf8
	ctx.r[11].s64 = ctx.r[11].s64 + 23544;
	// 82697850: 914B00F8  stw r10, 0xf8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(248 as u32), ctx.r[10].u32 ) };
	// 82697854: 914B0110  stw r10, 0x110(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(272 as u32), ctx.r[10].u32 ) };
	// 82697858: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 8269785C: 914B0140  stw r10, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[10].u32 ) };
	// 82697860: 914B0158  stw r10, 0x158(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 82697864: 914B0170  stw r10, 0x170(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(368 as u32), ctx.r[10].u32 ) };
	// 82697868: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269786C: 814ADD10  lwz r10, -0x22f0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8944 as u32) ) } as u64;
	// 82697870: 914B0188  stw r10, 0x188(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(392 as u32), ctx.r[10].u32 ) };
	// 82697874: 914B01A0  stw r10, 0x1a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(416 as u32), ctx.r[10].u32 ) };
	// 82697878: 914B01B8  stw r10, 0x1b8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(440 as u32), ctx.r[10].u32 ) };
	// 8269787C: 914B01D0  stw r10, 0x1d0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(464 as u32), ctx.r[10].u32 ) };
	// 82697880: 914B01E8  stw r10, 0x1e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(488 as u32), ctx.r[10].u32 ) };
	// 82697884: 914B0200  stw r10, 0x200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(512 as u32), ctx.r[10].u32 ) };
	// 82697888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697890 size=116
    let mut pc: u32 = 0x82697890;
    'dispatch: loop {
        match pc {
            0x82697890 => {
    //   block [0x82697890..0x82697904)
	// 82697890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697898: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269789C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826978A0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826978A4: 390B5BF8  addi r8, r11, 0x5bf8
	ctx.r[8].s64 = ctx.r[11].s64 + 23544;
	// 826978A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826978AC: 392A90F4  addi r9, r10, -0x6f0c
	ctx.r[9].s64 = ctx.r[10].s64 + -28428;
	// 826978B0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826978B4: 38E00028  li r7, 0x28
	ctx.r[7].s64 = 40;
	// 826978B8: 38AA1610  addi r5, r10, 0x1610
	ctx.r[5].s64 = ctx.r[10].s64 + 5648;
	// 826978BC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826978C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826978C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826978C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826978CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826978D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826978D4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826978D8: 388AACDC  addi r4, r10, -0x5324
	ctx.r[4].s64 = ctx.r[10].s64 + -21284;
	// 826978DC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826978E0: 386B2390  addi r3, r11, 0x2390
	ctx.r[3].s64 = ctx.r[11].s64 + 9104;
	// 826978E4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826978E8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826978EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826978F0: 4BDCF531  bl 0x82466e20
	ctx.lr = 0x826978F4;
	sub_82466E20(ctx, base);
	// 826978F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826978F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826978FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697908 size=112
    let mut pc: u32 = 0x82697908;
    'dispatch: loop {
        match pc {
            0x82697908 => {
    //   block [0x82697908..0x82697978)
	// 82697908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269790C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697914: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697918: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269791C: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82697920: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697924: 390BDD18  addi r8, r11, -0x22e8
	ctx.r[8].s64 = ctx.r[11].s64 + -8936;
	// 82697928: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269792C: 388AACF4  addi r4, r10, -0x530c
	ctx.r[4].s64 = ctx.r[10].s64 + -21260;
	// 82697930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269793C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697940: 386A23C0  addi r3, r10, 0x23c0
	ctx.r[3].s64 = ctx.r[10].s64 + 9152;
	// 82697944: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697948: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269794C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697954: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269795C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697964: 4BDCF4BD  bl 0x82466e20
	ctx.lr = 0x82697968;
	sub_82466E20(ctx, base);
	// 82697968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269796C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697978 size=108
    let mut pc: u32 = 0x82697978;
    'dispatch: loop {
        match pc {
            0x82697978 => {
    //   block [0x82697978..0x826979E4)
	// 82697978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269797C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697984: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697988: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269798C: 38EBDD60  addi r7, r11, -0x22a0
	ctx.r[7].s64 = ctx.r[11].s64 + -8864;
	// 82697990: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82697994: 388AAD10  addi r4, r10, -0x52f0
	ctx.r[4].s64 = ctx.r[10].s64 + -21232;
	// 82697998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269799C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826979A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826979A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826979A8: 386A23F0  addi r3, r10, 0x23f0
	ctx.r[3].s64 = ctx.r[10].s64 + 9200;
	// 826979AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826979B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826979B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826979B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826979BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826979C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826979C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826979C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826979CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826979D0: 4BDCF451  bl 0x82466e20
	ctx.lr = 0x826979D4;
	sub_82466E20(ctx, base);
	// 826979D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826979D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826979DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826979E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826979E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826979E8 size=108
    let mut pc: u32 = 0x826979E8;
    'dispatch: loop {
        match pc {
            0x826979E8 => {
    //   block [0x826979E8..0x82697A54)
	// 826979E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826979EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826979F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826979F4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826979F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826979FC: 38EBDDA8  addi r7, r11, -0x2258
	ctx.r[7].s64 = ctx.r[11].s64 + -8792;
	// 82697A00: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82697A04: 388AAD38  addi r4, r10, -0x52c8
	ctx.r[4].s64 = ctx.r[10].s64 + -21192;
	// 82697A08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697A0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697A10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697A14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697A18: 386A2420  addi r3, r10, 0x2420
	ctx.r[3].s64 = ctx.r[10].s64 + 9248;
	// 82697A1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697A20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697A28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697A2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697A30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697A34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697A38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697A3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697A40: 4BDCF3E1  bl 0x82466e20
	ctx.lr = 0x82697A44;
	sub_82466E20(ctx, base);
	// 82697A44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697A48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697A4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697A58 size=116
    let mut pc: u32 = 0x82697A58;
    'dispatch: loop {
        match pc {
            0x82697A58 => {
    //   block [0x82697A58..0x82697ACC)
	// 82697A58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697A5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697A60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697A64: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697A68: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 82697A6C: 390ADDF0  addi r8, r10, -0x2210
	ctx.r[8].s64 = ctx.r[10].s64 + -8720;
	// 82697A70: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697A74: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697A78: 38AA12B0  addi r5, r10, 0x12b0
	ctx.r[5].s64 = ctx.r[10].s64 + 4784;
	// 82697A7C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697A80: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697A84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697A88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697A8C: 388AAD60  addi r4, r10, -0x52a0
	ctx.r[4].s64 = ctx.r[10].s64 + -21152;
	// 82697A90: 396B911C  addi r11, r11, -0x6ee4
	ctx.r[11].s64 = ctx.r[11].s64 + -28388;
	// 82697A94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697A98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697A9C: 386A2450  addi r3, r10, 0x2450
	ctx.r[3].s64 = ctx.r[10].s64 + 9296;
	// 82697AA0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697AA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697AA8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697AB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697AB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697AB8: 4BDCF369  bl 0x82466e20
	ctx.lr = 0x82697ABC;
	sub_82466E20(ctx, base);
	// 82697ABC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697AC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697AD0 size=116
    let mut pc: u32 = 0x82697AD0;
    'dispatch: loop {
        match pc {
            0x82697AD0 => {
    //   block [0x82697AD0..0x82697B44)
	// 82697AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697ADC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697AE0: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82697AE4: 390ADE98  addi r8, r10, -0x2168
	ctx.r[8].s64 = ctx.r[10].s64 + -8552;
	// 82697AE8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697AEC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697AF0: 38AA2450  addi r5, r10, 0x2450
	ctx.r[5].s64 = ctx.r[10].s64 + 9296;
	// 82697AF4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697AF8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697AFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697B00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697B04: 388AAD7C  addi r4, r10, -0x5284
	ctx.r[4].s64 = ctx.r[10].s64 + -21124;
	// 82697B08: 396B9140  addi r11, r11, -0x6ec0
	ctx.r[11].s64 = ctx.r[11].s64 + -28352;
	// 82697B0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697B10: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697B14: 386A2480  addi r3, r10, 0x2480
	ctx.r[3].s64 = ctx.r[10].s64 + 9344;
	// 82697B18: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697B1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697B20: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697B28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697B2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697B30: 4BDCF2F1  bl 0x82466e20
	ctx.lr = 0x82697B34;
	sub_82466E20(ctx, base);
	// 82697B34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697B38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697B3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82697B48 size=36
    let mut pc: u32 = 0x82697B48;
    'dispatch: loop {
        match pc {
            0x82697B48 => {
    //   block [0x82697B48..0x82697B6C)
	// 82697B48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697B4C: 814BDD14  lwz r10, -0x22ec(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8940 as u32) ) } as u64;
	// 82697B50: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697B54: 396B5FB8  addi r11, r11, 0x5fb8
	ctx.r[11].s64 = ctx.r[11].s64 + 24504;
	// 82697B58: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82697B5C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697B60: 814ADEE0  lwz r10, -0x2120(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8480 as u32) ) } as u64;
	// 82697B64: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82697B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697B70 size=116
    let mut pc: u32 = 0x82697B70;
    'dispatch: loop {
        match pc {
            0x82697B70 => {
    //   block [0x82697B70..0x82697BE4)
	// 82697B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697B7C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697B80: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82697B84: 390B5FB8  addi r8, r11, 0x5fb8
	ctx.r[8].s64 = ctx.r[11].s64 + 24504;
	// 82697B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697B8C: 392A9180  addi r9, r10, -0x6e80
	ctx.r[9].s64 = ctx.r[10].s64 + -28288;
	// 82697B90: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697B94: 38E0000B  li r7, 0xb
	ctx.r[7].s64 = 11;
	// 82697B98: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697B9C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697BA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697BA4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697BA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697BAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697BB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697BB4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82697BB8: 388AAE18  addi r4, r10, -0x51e8
	ctx.r[4].s64 = ctx.r[10].s64 + -20968;
	// 82697BBC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697BC0: 386B24B0  addi r3, r11, 0x24b0
	ctx.r[3].s64 = ctx.r[11].s64 + 9392;
	// 82697BC4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82697BC8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697BCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697BD0: 4BDCF251  bl 0x82466e20
	ctx.lr = 0x82697BD4;
	sub_82466E20(ctx, base);
	// 82697BD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697BE8 size=112
    let mut pc: u32 = 0x82697BE8;
    'dispatch: loop {
        match pc {
            0x82697BE8 => {
    //   block [0x82697BE8..0x82697C58)
	// 82697BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697BF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697BF8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697BFC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697C00: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697C04: 390BDEE8  addi r8, r11, -0x2118
	ctx.r[8].s64 = ctx.r[11].s64 + -8472;
	// 82697C08: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82697C0C: 388AAE30  addi r4, r10, -0x51d0
	ctx.r[4].s64 = ctx.r[10].s64 + -20944;
	// 82697C10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697C14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697C18: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697C1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697C20: 386A24E0  addi r3, r10, 0x24e0
	ctx.r[3].s64 = ctx.r[10].s64 + 9440;
	// 82697C24: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697C28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697C2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697C30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697C34: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697C38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697C3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697C40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697C44: 4BDCF1DD  bl 0x82466e20
	ctx.lr = 0x82697C48;
	sub_82466E20(ctx, base);
	// 82697C48: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697C4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697C50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697C54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697C58 size=108
    let mut pc: u32 = 0x82697C58;
    'dispatch: loop {
        match pc {
            0x82697C58 => {
    //   block [0x82697C58..0x82697CC4)
	// 82697C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697C60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697C64: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697C68: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697C6C: 38EBDFA8  addi r7, r11, -0x2058
	ctx.r[7].s64 = ctx.r[11].s64 + -8280;
	// 82697C70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82697C74: 388AAE50  addi r4, r10, -0x51b0
	ctx.r[4].s64 = ctx.r[10].s64 + -20912;
	// 82697C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697C7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697C80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697C84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697C88: 386A2510  addi r3, r10, 0x2510
	ctx.r[3].s64 = ctx.r[10].s64 + 9488;
	// 82697C8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697C94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697C9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697CAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697CB0: 4BDCF171  bl 0x82466e20
	ctx.lr = 0x82697CB4;
	sub_82466E20(ctx, base);
	// 82697CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697CC8 size=112
    let mut pc: u32 = 0x82697CC8;
    'dispatch: loop {
        match pc {
            0x82697CC8 => {
    //   block [0x82697CC8..0x82697D38)
	// 82697CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697CD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697CD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697CD8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697CDC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697CE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697CE4: 390BDFD8  addi r8, r11, -0x2028
	ctx.r[8].s64 = ctx.r[11].s64 + -8232;
	// 82697CE8: 3920000B  li r9, 0xb
	ctx.r[9].s64 = 11;
	// 82697CEC: 388AAE6C  addi r4, r10, -0x5194
	ctx.r[4].s64 = ctx.r[10].s64 + -20884;
	// 82697CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697CF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697CF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697D00: 386A2540  addi r3, r10, 0x2540
	ctx.r[3].s64 = ctx.r[10].s64 + 9536;
	// 82697D04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697D08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697D0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697D14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697D18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697D1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697D20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697D24: 4BDCF0FD  bl 0x82466e20
	ctx.lr = 0x82697D28;
	sub_82466E20(ctx, base);
	// 82697D28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697D2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697D30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697D34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697D38 size=112
    let mut pc: u32 = 0x82697D38;
    'dispatch: loop {
        match pc {
            0x82697D38 => {
    //   block [0x82697D38..0x82697DA8)
	// 82697D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697D44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697D48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697D4C: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697D50: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697D54: 390BE0E0  addi r8, r11, -0x1f20
	ctx.r[8].s64 = ctx.r[11].s64 + -7968;
	// 82697D58: 3920000D  li r9, 0xd
	ctx.r[9].s64 = 13;
	// 82697D5C: 388AAE80  addi r4, r10, -0x5180
	ctx.r[4].s64 = ctx.r[10].s64 + -20864;
	// 82697D60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697D64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697D68: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697D6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697D70: 386A2570  addi r3, r10, 0x2570
	ctx.r[3].s64 = ctx.r[10].s64 + 9584;
	// 82697D74: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697D78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697D7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697D80: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697D84: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697D88: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697D8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697D90: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697D94: 4BDCF08D  bl 0x82466e20
	ctx.lr = 0x82697D98;
	sub_82466E20(ctx, base);
	// 82697D98: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697DA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697DA8 size=112
    let mut pc: u32 = 0x82697DA8;
    'dispatch: loop {
        match pc {
            0x82697DA8 => {
    //   block [0x82697DA8..0x82697E18)
	// 82697DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697DAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697DB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697DB4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697DB8: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82697DBC: 38EAE218  addi r7, r10, -0x1de8
	ctx.r[7].s64 = ctx.r[10].s64 + -7656;
	// 82697DC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697DC4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697DC8: 388A7AE8  addi r4, r10, 0x7ae8
	ctx.r[4].s64 = ctx.r[10].s64 + 31464;
	// 82697DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697DD0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82697DD4: 396B91CC  addi r11, r11, -0x6e34
	ctx.r[11].s64 = ctx.r[11].s64 + -28212;
	// 82697DD8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82697DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697DE0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697DE4: 386A25A0  addi r3, r10, 0x25a0
	ctx.r[3].s64 = ctx.r[10].s64 + 9632;
	// 82697DE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697DEC: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697DF0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697DF4: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697DF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697DFC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697E00: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82697E04: 4BDCF01D  bl 0x82466e20
	ctx.lr = 0x82697E08;
	sub_82466E20(ctx, base);
	// 82697E08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697E18 size=116
    let mut pc: u32 = 0x82697E18;
    'dispatch: loop {
        match pc {
            0x82697E18 => {
    //   block [0x82697E18..0x82697E8C)
	// 82697E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697E20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697E24: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697E28: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697E2C: 392B91B8  addi r9, r11, -0x6e48
	ctx.r[9].s64 = ctx.r[11].s64 + -28232;
	// 82697E30: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697E34: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697E38: 38E90038  addi r7, r9, 0x38
	ctx.r[7].s64 = ctx.r[9].s64 + 56;
	// 82697E3C: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82697E40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697E44: 388AAE98  addi r4, r10, -0x5168
	ctx.r[4].s64 = ctx.r[10].s64 + -20840;
	// 82697E48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697E4C: 396BE290  addi r11, r11, -0x1d70
	ctx.r[11].s64 = ctx.r[11].s64 + -7536;
	// 82697E50: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82697E54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697E58: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 82697E5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697E60: 386A25D0  addi r3, r10, 0x25d0
	ctx.r[3].s64 = ctx.r[10].s64 + 9680;
	// 82697E64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82697E68: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82697E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697E70: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 82697E74: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82697E78: 4BDCEFA9  bl 0x82466e20
	ctx.lr = 0x82697E7C;
	sub_82466E20(ctx, base);
	// 82697E7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697E80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697E84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697E90 size=112
    let mut pc: u32 = 0x82697E90;
    'dispatch: loop {
        match pc {
            0x82697E90 => {
    //   block [0x82697E90..0x82697F00)
	// 82697E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697E98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697E9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697EA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697EA4: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697EA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697EAC: 390BE578  addi r8, r11, -0x1a88
	ctx.r[8].s64 = ctx.r[11].s64 + -6792;
	// 82697EB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 82697EB4: 388AAEB0  addi r4, r10, -0x5150
	ctx.r[4].s64 = ctx.r[10].s64 + -20816;
	// 82697EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697EC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697EC8: 386A2600  addi r3, r10, 0x2600
	ctx.r[3].s64 = ctx.r[10].s64 + 9728;
	// 82697ECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697ED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697EDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697EEC: 4BDCEF35  bl 0x82466e20
	ctx.lr = 0x82697EF0;
	sub_82466E20(ctx, base);
	// 82697EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697F00 size=116
    let mut pc: u32 = 0x82697F00;
    'dispatch: loop {
        match pc {
            0x82697F00 => {
    //   block [0x82697F00..0x82697F74)
	// 82697F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697F08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697F0C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82697F10: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82697F14: 390AE5C0  addi r8, r10, -0x1a40
	ctx.r[8].s64 = ctx.r[10].s64 + -6720;
	// 82697F18: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697F1C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82697F20: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697F24: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697F28: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82697F2C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697F30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697F34: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 82697F38: 396B92D0  addi r11, r11, -0x6d30
	ctx.r[11].s64 = ctx.r[11].s64 + -27952;
	// 82697F3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697F40: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697F44: 386A2630  addi r3, r10, 0x2630
	ctx.r[3].s64 = ctx.r[10].s64 + 9776;
	// 82697F48: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82697F4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697F50: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82697F54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697F5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697F60: 4BDCEEC1  bl 0x82466e20
	ctx.lr = 0x82697F64;
	sub_82466E20(ctx, base);
	// 82697F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697F78 size=112
    let mut pc: u32 = 0x82697F78;
    'dispatch: loop {
        match pc {
            0x82697F78 => {
    //   block [0x82697F78..0x82697FE8)
	// 82697F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697F88: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697F8C: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82697F90: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82697F94: 390BE608  addi r8, r11, -0x19f8
	ctx.r[8].s64 = ctx.r[11].s64 + -6648;
	// 82697F98: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82697F9C: 388AAEF4  addi r4, r10, -0x510c
	ctx.r[4].s64 = ctx.r[10].s64 + -20748;
	// 82697FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82697FA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697FA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82697FAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82697FB0: 386A2660  addi r3, r10, 0x2660
	ctx.r[3].s64 = ctx.r[10].s64 + 9824;
	// 82697FB4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82697FB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82697FBC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82697FC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82697FC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82697FC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82697FCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82697FD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82697FD4: 4BDCEE4D  bl 0x82466e20
	ctx.lr = 0x82697FD8;
	sub_82466E20(ctx, base);
	// 82697FD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82697FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82697FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82697FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82697FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82697FE8 size=112
    let mut pc: u32 = 0x82697FE8;
    'dispatch: loop {
        match pc {
            0x82697FE8 => {
    //   block [0x82697FE8..0x82698058)
	// 82697FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82697FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82697FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82697FF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82697FF8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82697FFC: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698000: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698004: 390BE698  addi r8, r11, -0x1968
	ctx.r[8].s64 = ctx.r[11].s64 + -6504;
	// 82698008: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269800C: 388AAF08  addi r4, r10, -0x50f8
	ctx.r[4].s64 = ctx.r[10].s64 + -20728;
	// 82698010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698014: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698018: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269801C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698020: 386A2690  addi r3, r10, 0x2690
	ctx.r[3].s64 = ctx.r[10].s64 + 9872;
	// 82698024: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698028: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269802C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698030: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698034: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698038: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269803C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698040: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698044: 4BDCEDDD  bl 0x82466e20
	ctx.lr = 0x82698048;
	sub_82466E20(ctx, base);
	// 82698048: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269804C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698058 size=28
    let mut pc: u32 = 0x82698058;
    'dispatch: loop {
        match pc {
            0x82698058 => {
    //   block [0x82698058..0x82698074)
	// 82698058: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269805C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698060: 394A60C0  addi r10, r10, 0x60c0
	ctx.r[10].s64 = ctx.r[10].s64 + 24768;
	// 82698064: 816BE710  lwz r11, -0x18f0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6384 as u32) ) } as u64;
	// 82698068: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8269806C: 916A0098  stw r11, 0x98(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82698070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698078 size=116
    let mut pc: u32 = 0x82698078;
    'dispatch: loop {
        match pc {
            0x82698078 => {
    //   block [0x82698078..0x826980EC)
	// 82698078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269807C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698080: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698084: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698088: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269808C: 392B92FC  addi r9, r11, -0x6d04
	ctx.r[9].s64 = ctx.r[11].s64 + -27908;
	// 82698090: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698094: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698098: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269809C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 826980A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826980A4: 388AAF24  addi r4, r10, -0x50dc
	ctx.r[4].s64 = ctx.r[10].s64 + -20700;
	// 826980A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826980AC: 396B60C0  addi r11, r11, 0x60c0
	ctx.r[11].s64 = ctx.r[11].s64 + 24768;
	// 826980B0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 826980B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826980B8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826980BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826980C0: 386A26C0  addi r3, r10, 0x26c0
	ctx.r[3].s64 = ctx.r[10].s64 + 9920;
	// 826980C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826980C8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826980CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826980D0: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826980D4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826980D8: 4BDCED49  bl 0x82466e20
	ctx.lr = 0x826980DC;
	sub_82466E20(ctx, base);
	// 826980DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826980E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826980E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826980E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826980F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826980F0 size=112
    let mut pc: u32 = 0x826980F0;
    'dispatch: loop {
        match pc {
            0x826980F0 => {
    //   block [0x826980F0..0x82698160)
	// 826980F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826980F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826980F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826980FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698100: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698104: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698108: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269810C: 390BE718  addi r8, r11, -0x18e8
	ctx.r[8].s64 = ctx.r[11].s64 + -6376;
	// 82698110: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 82698114: 388AAF40  addi r4, r10, -0x50c0
	ctx.r[4].s64 = ctx.r[10].s64 + -20672;
	// 82698118: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269811C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698120: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698124: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698128: 386A26F0  addi r3, r10, 0x26f0
	ctx.r[3].s64 = ctx.r[10].s64 + 9968;
	// 8269812C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698130: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698134: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698138: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269813C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698140: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698144: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698148: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269814C: 4BDCECD5  bl 0x82466e20
	ctx.lr = 0x82698150;
	sub_82466E20(ctx, base);
	// 82698150: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269815C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698160 size=24
    let mut pc: u32 = 0x82698160;
    'dispatch: loop {
        match pc {
            0x82698160 => {
    //   block [0x82698160..0x82698178)
	// 82698160: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698164: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698168: 394A6180  addi r10, r10, 0x6180
	ctx.r[10].s64 = ctx.r[10].s64 + 24960;
	// 8269816C: 816BE714  lwz r11, -0x18ec(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-6380 as u32) ) } as u64;
	// 82698170: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82698174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698178 size=116
    let mut pc: u32 = 0x82698178;
    'dispatch: loop {
        match pc {
            0x82698178 => {
    //   block [0x82698178..0x826981EC)
	// 82698178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269817C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698184: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698188: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269818C: 390B6180  addi r8, r11, 0x6180
	ctx.r[8].s64 = ctx.r[11].s64 + 24960;
	// 82698190: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698194: 392A9368  addi r9, r10, -0x6c98
	ctx.r[9].s64 = ctx.r[10].s64 + -27800;
	// 82698198: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269819C: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 826981A0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826981A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826981A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826981AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826981B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826981B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826981B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826981BC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826981C0: 388AAF78  addi r4, r10, -0x5088
	ctx.r[4].s64 = ctx.r[10].s64 + -20616;
	// 826981C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826981C8: 386B2720  addi r3, r11, 0x2720
	ctx.r[3].s64 = ctx.r[11].s64 + 10016;
	// 826981CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826981D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826981D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826981D8: 4BDCEC49  bl 0x82466e20
	ctx.lr = 0x826981DC;
	sub_82466E20(ctx, base);
	// 826981DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826981E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826981E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826981E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826981F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826981F0 size=112
    let mut pc: u32 = 0x826981F0;
    'dispatch: loop {
        match pc {
            0x826981F0 => {
    //   block [0x826981F0..0x82698260)
	// 826981F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826981F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826981F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826981FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698200: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698204: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698208: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269820C: 390BE790  addi r8, r11, -0x1870
	ctx.r[8].s64 = ctx.r[11].s64 + -6256;
	// 82698210: 3920000A  li r9, 0xa
	ctx.r[9].s64 = 10;
	// 82698214: 388AAF98  addi r4, r10, -0x5068
	ctx.r[4].s64 = ctx.r[10].s64 + -20584;
	// 82698218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269821C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698220: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698224: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698228: 386A2750  addi r3, r10, 0x2750
	ctx.r[3].s64 = ctx.r[10].s64 + 10064;
	// 8269822C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698230: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698238: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269823C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698240: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698248: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269824C: 4BDCEBD5  bl 0x82466e20
	ctx.lr = 0x82698250;
	sub_82466E20(ctx, base);
	// 82698250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269825C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698260 size=112
    let mut pc: u32 = 0x82698260;
    'dispatch: loop {
        match pc {
            0x82698260 => {
    //   block [0x82698260..0x826982D0)
	// 82698260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698268: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269826C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698270: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698274: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698278: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269827C: 390BE880  addi r8, r11, -0x1780
	ctx.r[8].s64 = ctx.r[11].s64 + -6016;
	// 82698280: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82698284: 388AAFB4  addi r4, r10, -0x504c
	ctx.r[4].s64 = ctx.r[10].s64 + -20556;
	// 82698288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269828C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698290: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698294: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698298: 386A2780  addi r3, r10, 0x2780
	ctx.r[3].s64 = ctx.r[10].s64 + 10112;
	// 8269829C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826982A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826982A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826982A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826982AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826982B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826982B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826982B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826982BC: 4BDCEB65  bl 0x82466e20
	ctx.lr = 0x826982C0;
	sub_82466E20(ctx, base);
	// 826982C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826982C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826982C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826982CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826982D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826982D0 size=24
    let mut pc: u32 = 0x826982D0;
    'dispatch: loop {
        match pc {
            0x826982D0 => {
    //   block [0x826982D0..0x826982E8)
	// 826982D0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826982D4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826982D8: 394A6228  addi r10, r10, 0x6228
	ctx.r[10].s64 = ctx.r[10].s64 + 25128;
	// 826982DC: 816BE8E0  lwz r11, -0x1720(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5920 as u32) ) } as u64;
	// 826982E0: 916A0038  stw r11, 0x38(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 826982E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826982E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826982E8 size=116
    let mut pc: u32 = 0x826982E8;
    'dispatch: loop {
        match pc {
            0x826982E8 => {
    //   block [0x826982E8..0x8269835C)
	// 826982E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826982EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826982F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826982F4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826982F8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826982FC: 390B6228  addi r8, r11, 0x6228
	ctx.r[8].s64 = ctx.r[11].s64 + 25128;
	// 82698300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698304: 392A93B4  addi r9, r10, -0x6c4c
	ctx.r[9].s64 = ctx.r[10].s64 + -27724;
	// 82698308: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269830C: 38E00019  li r7, 0x19
	ctx.r[7].s64 = 25;
	// 82698310: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698314: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698318: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269831C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698320: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698324: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698328: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269832C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82698330: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82698334: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698338: 386B27B0  addi r3, r11, 0x27b0
	ctx.r[3].s64 = ctx.r[11].s64 + 10160;
	// 8269833C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82698340: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698348: 4BDCEAD9  bl 0x82466e20
	ctx.lr = 0x8269834C;
	sub_82466E20(ctx, base);
	// 8269834C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698360 size=116
    let mut pc: u32 = 0x82698360;
    'dispatch: loop {
        match pc {
            0x82698360 => {
    //   block [0x82698360..0x826983D4)
	// 82698360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698368: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269836C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698370: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698374: 392B93EC  addi r9, r11, -0x6c14
	ctx.r[9].s64 = ctx.r[11].s64 + -27668;
	// 82698378: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 8269837C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698380: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 82698384: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82698388: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269838C: 388AB168  addi r4, r10, -0x4e98
	ctx.r[4].s64 = ctx.r[10].s64 + -20120;
	// 82698390: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698394: 396BE8F0  addi r11, r11, -0x1710
	ctx.r[11].s64 = ctx.r[11].s64 + -5904;
	// 82698398: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269839C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826983A0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 826983A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826983A8: 386A27E0  addi r3, r10, 0x27e0
	ctx.r[3].s64 = ctx.r[10].s64 + 10208;
	// 826983AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 826983B0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 826983B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826983B8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 826983BC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 826983C0: 4BDCEA61  bl 0x82466e20
	ctx.lr = 0x826983C4;
	sub_82466E20(ctx, base);
	// 826983C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826983C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826983CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826983D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826983D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826983D8 size=112
    let mut pc: u32 = 0x826983D8;
    'dispatch: loop {
        match pc {
            0x826983D8 => {
    //   block [0x826983D8..0x82698448)
	// 826983D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826983DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826983E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826983E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826983E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826983EC: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 826983F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826983F4: 390BEAB8  addi r8, r11, -0x1548
	ctx.r[8].s64 = ctx.r[11].s64 + -5448;
	// 826983F8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 826983FC: 388AB188  addi r4, r10, -0x4e78
	ctx.r[4].s64 = ctx.r[10].s64 + -20088;
	// 82698400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698404: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269840C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698410: 386A2810  addi r3, r10, 0x2810
	ctx.r[3].s64 = ctx.r[10].s64 + 10256;
	// 82698414: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698418: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269841C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698424: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269842C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698434: 4BDCE9ED  bl 0x82466e20
	ctx.lr = 0x82698438;
	sub_82466E20(ctx, base);
	// 82698438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269843C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698448 size=48
    let mut pc: u32 = 0x82698448;
    'dispatch: loop {
        match pc {
            0x82698448 => {
    //   block [0x82698448..0x82698478)
	// 82698448: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269844C: 814BEB30  lwz r10, -0x14d0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-5328 as u32) ) } as u64;
	// 82698450: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698454: 396B6480  addi r11, r11, 0x6480
	ctx.r[11].s64 = ctx.r[11].s64 + 25728;
	// 82698458: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8269845C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698460: 814AEB34  lwz r10, -0x14cc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-5324 as u32) ) } as u64;
	// 82698464: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82698468: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269846C: 814AE8EC  lwz r10, -0x1714(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-5908 as u32) ) } as u64;
	// 82698470: 914B0128  stw r10, 0x128(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(296 as u32), ctx.r[10].u32 ) };
	// 82698474: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698478 size=116
    let mut pc: u32 = 0x82698478;
    'dispatch: loop {
        match pc {
            0x82698478 => {
    //   block [0x82698478..0x826984EC)
	// 82698478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269847C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698480: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698484: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698488: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269848C: 390B6480  addi r8, r11, 0x6480
	ctx.r[8].s64 = ctx.r[11].s64 + 25728;
	// 82698490: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698494: 392A94A8  addi r9, r10, -0x6b58
	ctx.r[9].s64 = ctx.r[10].s64 + -27480;
	// 82698498: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269849C: 38E00016  li r7, 0x16
	ctx.r[7].s64 = 22;
	// 826984A0: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 826984A4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826984A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826984AC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826984B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826984B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826984B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826984BC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 826984C0: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826984C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826984C8: 386B2840  addi r3, r11, 0x2840
	ctx.r[3].s64 = ctx.r[11].s64 + 10304;
	// 826984CC: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 826984D0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826984D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826984D8: 4BDCE949  bl 0x82466e20
	ctx.lr = 0x826984DC;
	sub_82466E20(ctx, base);
	// 826984DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826984E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826984E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826984E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826984F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826984F0 size=116
    let mut pc: u32 = 0x826984F0;
    'dispatch: loop {
        match pc {
            0x826984F0 => {
    //   block [0x826984F0..0x82698564)
	// 826984F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826984F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826984F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826984FC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698500: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82698504: 390AEB38  addi r8, r10, -0x14c8
	ctx.r[8].s64 = ctx.r[10].s64 + -5320;
	// 82698508: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269850C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698510: 38AA1640  addi r5, r10, 0x1640
	ctx.r[5].s64 = ctx.r[10].s64 + 5696;
	// 82698514: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698518: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269851C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698520: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698524: 388AB2C4  addi r4, r10, -0x4d3c
	ctx.r[4].s64 = ctx.r[10].s64 + -19772;
	// 82698528: 396B94E4  addi r11, r11, -0x6b1c
	ctx.r[11].s64 = ctx.r[11].s64 + -27420;
	// 8269852C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698530: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698534: 386A2870  addi r3, r10, 0x2870
	ctx.r[3].s64 = ctx.r[10].s64 + 10352;
	// 82698538: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269853C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698540: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82698544: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698548: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269854C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698550: 4BDCE8D1  bl 0x82466e20
	ctx.lr = 0x82698554;
	sub_82466E20(ctx, base);
	// 82698554: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269855C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698560: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698568 size=116
    let mut pc: u32 = 0x82698568;
    'dispatch: loop {
        match pc {
            0x82698568 => {
    //   block [0x82698568..0x826985DC)
	// 82698568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269856C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698570: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698574: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698578: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269857C: 390AEB80  addi r8, r10, -0x1480
	ctx.r[8].s64 = ctx.r[10].s64 + -5248;
	// 82698580: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698584: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698588: 38AA1CA0  addi r5, r10, 0x1ca0
	ctx.r[5].s64 = ctx.r[10].s64 + 7328;
	// 8269858C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698590: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698594: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698598: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269859C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 826985A0: 396B94F4  addi r11, r11, -0x6b0c
	ctx.r[11].s64 = ctx.r[11].s64 + -27404;
	// 826985A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826985A8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826985AC: 386A28A0  addi r3, r10, 0x28a0
	ctx.r[3].s64 = ctx.r[10].s64 + 10400;
	// 826985B0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 826985B4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826985B8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 826985BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826985C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826985C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826985C8: 4BDCE859  bl 0x82466e20
	ctx.lr = 0x826985CC;
	sub_82466E20(ctx, base);
	// 826985CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826985D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826985D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826985D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826985E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826985E0 size=116
    let mut pc: u32 = 0x826985E0;
    'dispatch: loop {
        match pc {
            0x826985E0 => {
    //   block [0x826985E0..0x82698654)
	// 826985E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826985E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826985E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826985EC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826985F0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 826985F4: 390AEBF8  addi r8, r10, -0x1408
	ctx.r[8].s64 = ctx.r[10].s64 + -5128;
	// 826985F8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826985FC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 82698600: 38AA20C0  addi r5, r10, 0x20c0
	ctx.r[5].s64 = ctx.r[10].s64 + 8384;
	// 82698604: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 82698608: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269860C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698610: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698614: 388AB2F4  addi r4, r10, -0x4d0c
	ctx.r[4].s64 = ctx.r[10].s64 + -19724;
	// 82698618: 396B950C  addi r11, r11, -0x6af4
	ctx.r[11].s64 = ctx.r[11].s64 + -27380;
	// 8269861C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698620: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698624: 386A28D0  addi r3, r10, 0x28d0
	ctx.r[3].s64 = ctx.r[10].s64 + 10448;
	// 82698628: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269862C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698630: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 82698634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698638: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269863C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698640: 4BDCE7E1  bl 0x82466e20
	ctx.lr = 0x82698644;
	sub_82466E20(ctx, base);
	// 82698644: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698648: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269864C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698658 size=108
    let mut pc: u32 = 0x82698658;
    'dispatch: loop {
        match pc {
            0x82698658 => {
    //   block [0x82698658..0x826986C4)
	// 82698658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269865C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698660: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698664: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698668: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269866C: 38EBEC70  addi r7, r11, -0x1390
	ctx.r[7].s64 = ctx.r[11].s64 + -5008;
	// 82698670: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82698674: 388AB318  addi r4, r10, -0x4ce8
	ctx.r[4].s64 = ctx.r[10].s64 + -19688;
	// 82698678: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269867C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698680: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698684: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698688: 386A2900  addi r3, r10, 0x2900
	ctx.r[3].s64 = ctx.r[10].s64 + 10496;
	// 8269868C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698690: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698694: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698698: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269869C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826986A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826986A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826986A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826986AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826986B0: 4BDCE771  bl 0x82466e20
	ctx.lr = 0x826986B4;
	sub_82466E20(ctx, base);
	// 826986B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826986B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826986BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826986C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826986C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826986C8 size=108
    let mut pc: u32 = 0x826986C8;
    'dispatch: loop {
        match pc {
            0x826986C8 => {
    //   block [0x826986C8..0x82698734)
	// 826986C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826986CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826986D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826986D4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826986D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826986DC: 38EBECB8  addi r7, r11, -0x1348
	ctx.r[7].s64 = ctx.r[11].s64 + -4936;
	// 826986E0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 826986E4: 388AB344  addi r4, r10, -0x4cbc
	ctx.r[4].s64 = ctx.r[10].s64 + -19644;
	// 826986E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826986EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826986F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826986F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826986F8: 386A2930  addi r3, r10, 0x2930
	ctx.r[3].s64 = ctx.r[10].s64 + 10544;
	// 826986FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698700: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698704: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698708: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269870C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698710: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698714: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698718: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269871C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698720: 4BDCE701  bl 0x82466e20
	ctx.lr = 0x82698724;
	sub_82466E20(ctx, base);
	// 82698724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269872C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698738 size=108
    let mut pc: u32 = 0x82698738;
    'dispatch: loop {
        match pc {
            0x82698738 => {
    //   block [0x82698738..0x826987A4)
	// 82698738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269873C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698744: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698748: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269874C: 38EBED00  addi r7, r11, -0x1300
	ctx.r[7].s64 = ctx.r[11].s64 + -4864;
	// 82698750: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 82698754: 388AB36C  addi r4, r10, -0x4c94
	ctx.r[4].s64 = ctx.r[10].s64 + -19604;
	// 82698758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269875C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698760: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698764: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698768: 386A2960  addi r3, r10, 0x2960
	ctx.r[3].s64 = ctx.r[10].s64 + 10592;
	// 8269876C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698770: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698774: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698778: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269877C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698780: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698784: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698788: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269878C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698790: 4BDCE691  bl 0x82466e20
	ctx.lr = 0x82698794;
	sub_82466E20(ctx, base);
	// 82698794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269879C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826987A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826987A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826987A8 size=108
    let mut pc: u32 = 0x826987A8;
    'dispatch: loop {
        match pc {
            0x826987A8 => {
    //   block [0x826987A8..0x82698814)
	// 826987A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826987AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826987B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826987B4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826987B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 826987BC: 38EBED48  addi r7, r11, -0x12b8
	ctx.r[7].s64 = ctx.r[11].s64 + -4792;
	// 826987C0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826987C4: 388AB398  addi r4, r10, -0x4c68
	ctx.r[4].s64 = ctx.r[10].s64 + -19560;
	// 826987C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826987CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826987D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826987D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826987D8: 386A2990  addi r3, r10, 0x2990
	ctx.r[3].s64 = ctx.r[10].s64 + 10640;
	// 826987DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826987E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826987E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826987E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826987EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826987F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826987F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826987F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826987FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698800: 4BDCE621  bl 0x82466e20
	ctx.lr = 0x82698804;
	sub_82466E20(ctx, base);
	// 82698804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269880C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698818 size=108
    let mut pc: u32 = 0x82698818;
    'dispatch: loop {
        match pc {
            0x82698818 => {
    //   block [0x82698818..0x82698884)
	// 82698818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269881C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698820: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698824: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698828: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8269882C: 38EBEDF0  addi r7, r11, -0x1210
	ctx.r[7].s64 = ctx.r[11].s64 + -4624;
	// 82698830: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698834: 388A7914  addi r4, r10, 0x7914
	ctx.r[4].s64 = ctx.r[10].s64 + 30996;
	// 82698838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269883C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698840: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698848: 386A29C0  addi r3, r10, 0x29c0
	ctx.r[3].s64 = ctx.r[10].s64 + 10688;
	// 8269884C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698850: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698854: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698858: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269885C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698860: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698868: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269886C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698870: 4BDCE5B1  bl 0x82466e20
	ctx.lr = 0x82698874;
	sub_82466E20(ctx, base);
	// 82698874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269887C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698880: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698888 size=112
    let mut pc: u32 = 0x82698888;
    'dispatch: loop {
        match pc {
            0x82698888 => {
    //   block [0x82698888..0x826988F8)
	// 82698888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269888C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698894: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698898: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269889C: 392A954C  addi r9, r10, -0x6ab4
	ctx.r[9].s64 = ctx.r[10].s64 + -27316;
	// 826988A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826988A4: 390BEE28  addi r8, r11, -0x11d8
	ctx.r[8].s64 = ctx.r[11].s64 + -4568;
	// 826988A8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 826988AC: 388A8198  addi r4, r10, -0x7e68
	ctx.r[4].s64 = ctx.r[10].s64 + -32360;
	// 826988B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826988B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826988B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826988BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826988C0: 386A29F0  addi r3, r10, 0x29f0
	ctx.r[3].s64 = ctx.r[10].s64 + 10736;
	// 826988C4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 826988C8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 826988CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826988D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826988D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826988D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826988DC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826988E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826988E4: 4BDCE53D  bl 0x82466e20
	ctx.lr = 0x826988E8;
	sub_82466E20(ctx, base);
	// 826988E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826988EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826988F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826988F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826988F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826988F8 size=108
    let mut pc: u32 = 0x826988F8;
    'dispatch: loop {
        match pc {
            0x826988F8 => {
    //   block [0x826988F8..0x82698964)
	// 826988F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826988FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698904: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269890C: 38EBEE70  addi r7, r11, -0x1190
	ctx.r[7].s64 = ctx.r[11].s64 + -4496;
	// 82698910: 39000005  li r8, 5
	ctx.r[8].s64 = 5;
	// 82698914: 388A8060  addi r4, r10, -0x7fa0
	ctx.r[4].s64 = ctx.r[10].s64 + -32672;
	// 82698918: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269891C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698920: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698924: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698928: 386A2A20  addi r3, r10, 0x2a20
	ctx.r[3].s64 = ctx.r[10].s64 + 10784;
	// 8269892C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698930: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698938: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269893C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698940: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698944: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698948: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269894C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698950: 4BDCE4D1  bl 0x82466e20
	ctx.lr = 0x82698954;
	sub_82466E20(ctx, base);
	// 82698954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269895C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698960: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698968 size=108
    let mut pc: u32 = 0x82698968;
    'dispatch: loop {
        match pc {
            0x82698968 => {
    //   block [0x82698968..0x826989D4)
	// 82698968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269896C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698970: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698974: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698978: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 8269897C: 38EBEEE8  addi r7, r11, -0x1118
	ctx.r[7].s64 = ctx.r[11].s64 + -4376;
	// 82698980: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698984: 388A7788  addi r4, r10, 0x7788
	ctx.r[4].s64 = ctx.r[10].s64 + 30600;
	// 82698988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269898C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698990: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698994: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698998: 386A2A50  addi r3, r10, 0x2a50
	ctx.r[3].s64 = ctx.r[10].s64 + 10832;
	// 8269899C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826989A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826989A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826989A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826989AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826989B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826989B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826989B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826989BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826989C0: 4BDCE461  bl 0x82466e20
	ctx.lr = 0x826989C4;
	sub_82466E20(ctx, base);
	// 826989C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826989C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826989CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826989D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826989D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826989D8 size=108
    let mut pc: u32 = 0x826989D8;
    'dispatch: loop {
        match pc {
            0x826989D8 => {
    //   block [0x826989D8..0x82698A44)
	// 826989D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826989DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826989E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826989E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826989E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826989EC: 38EBEF18  addi r7, r11, -0x10e8
	ctx.r[7].s64 = ctx.r[11].s64 + -4328;
	// 826989F0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826989F4: 388A77AC  addi r4, r10, 0x77ac
	ctx.r[4].s64 = ctx.r[10].s64 + 30636;
	// 826989F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826989FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698A00: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698A04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698A08: 386A2A80  addi r3, r10, 0x2a80
	ctx.r[3].s64 = ctx.r[10].s64 + 10880;
	// 82698A0C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698A10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698A14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698A1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698A24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698A2C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698A30: 4BDCE3F1  bl 0x82466e20
	ctx.lr = 0x82698A34;
	sub_82466E20(ctx, base);
	// 82698A34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698A38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698A3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698A40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698A48 size=24
    let mut pc: u32 = 0x82698A48;
    'dispatch: loop {
        match pc {
            0x82698A48 => {
    //   block [0x82698A48..0x82698A60)
	// 82698A48: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698A4C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698A50: 394A6690  addi r10, r10, 0x6690
	ctx.r[10].s64 = ctx.r[10].s64 + 26256;
	// 82698A54: 816BEF30  lwz r11, -0x10d0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4304 as u32) ) } as u64;
	// 82698A58: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82698A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698A60 size=112
    let mut pc: u32 = 0x82698A60;
    'dispatch: loop {
        match pc {
            0x82698A60 => {
    //   block [0x82698A60..0x82698AD0)
	// 82698A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698A68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698A6C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698A70: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698A74: 392A958C  addi r9, r10, -0x6a74
	ctx.r[9].s64 = ctx.r[10].s64 + -27252;
	// 82698A78: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698A7C: 390B6690  addi r8, r11, 0x6690
	ctx.r[8].s64 = ctx.r[11].s64 + 26256;
	// 82698A80: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82698A84: 388A77C8  addi r4, r10, 0x77c8
	ctx.r[4].s64 = ctx.r[10].s64 + 30664;
	// 82698A88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698A8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698A90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698A98: 386A2AB0  addi r3, r10, 0x2ab0
	ctx.r[3].s64 = ctx.r[10].s64 + 10928;
	// 82698A9C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698AA0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82698AA4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698AA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698AAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698AB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698AB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698AB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698ABC: 4BDCE365  bl 0x82466e20
	ctx.lr = 0x82698AC0;
	sub_82466E20(ctx, base);
	// 82698AC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698AC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698AC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698ACC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698AD0 size=96
    let mut pc: u32 = 0x82698AD0;
    'dispatch: loop {
        match pc {
            0x82698AD0 => {
    //   block [0x82698AD0..0x82698B30)
	// 82698AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698AD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698AD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698ADC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698AE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698AE4: 388A6DD4  addi r4, r10, 0x6dd4
	ctx.r[4].s64 = ctx.r[10].s64 + 28116;
	// 82698AE8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698AEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698AF0: 386A2AE0  addi r3, r10, 0x2ae0
	ctx.r[3].s64 = ctx.r[10].s64 + 10976;
	// 82698AF4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698AF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698AFC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82698B00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698B04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698B08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698B0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698B10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82698B14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698B18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82698B1C: 4BDCE305  bl 0x82466e20
	ctx.lr = 0x82698B20;
	sub_82466E20(ctx, base);
	// 82698B20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698B24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698B28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698B30 size=112
    let mut pc: u32 = 0x82698B30;
    'dispatch: loop {
        match pc {
            0x82698B30 => {
    //   block [0x82698B30..0x82698BA0)
	// 82698B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698B3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698B40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698B44: 38AA2AE0  addi r5, r10, 0x2ae0
	ctx.r[5].s64 = ctx.r[10].s64 + 10976;
	// 82698B48: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698B4C: 390BEF34  addi r8, r11, -0x10cc
	ctx.r[8].s64 = ctx.r[11].s64 + -4300;
	// 82698B50: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82698B54: 388A7380  addi r4, r10, 0x7380
	ctx.r[4].s64 = ctx.r[10].s64 + 29568;
	// 82698B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698B5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698B60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698B68: 386A2B10  addi r3, r10, 0x2b10
	ctx.r[3].s64 = ctx.r[10].s64 + 11024;
	// 82698B6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82698B70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698B74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698B78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698B7C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698B80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698B88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698B8C: 4BDCE295  bl 0x82466e20
	ctx.lr = 0x82698B90;
	sub_82466E20(ctx, base);
	// 82698B90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698B94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698B98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698BA0 size=24
    let mut pc: u32 = 0x82698BA0;
    'dispatch: loop {
        match pc {
            0x82698BA0 => {
    //   block [0x82698BA0..0x82698BB8)
	// 82698BA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698BA4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698BA8: 394A6768  addi r10, r10, 0x6768
	ctx.r[10].s64 = ctx.r[10].s64 + 26472;
	// 82698BAC: 816BEF68  lwz r11, -0x1098(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4248 as u32) ) } as u64;
	// 82698BB0: 916A00C8  stw r11, 0xc8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82698BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698BB8 size=112
    let mut pc: u32 = 0x82698BB8;
    'dispatch: loop {
        match pc {
            0x82698BB8 => {
    //   block [0x82698BB8..0x82698C28)
	// 82698BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698BBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698BC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698BC4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698BC8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698BCC: 392A95B8  addi r9, r10, -0x6a48
	ctx.r[9].s64 = ctx.r[10].s64 + -27208;
	// 82698BD0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698BD4: 390B6768  addi r8, r11, 0x6768
	ctx.r[8].s64 = ctx.r[11].s64 + 26472;
	// 82698BD8: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 82698BDC: 388A7468  addi r4, r10, 0x7468
	ctx.r[4].s64 = ctx.r[10].s64 + 29800;
	// 82698BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698BE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698BE8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698BEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698BF0: 386A2B40  addi r3, r10, 0x2b40
	ctx.r[3].s64 = ctx.r[10].s64 + 11072;
	// 82698BF4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698BF8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82698BFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698C00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698C04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698C08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698C0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698C10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698C14: 4BDCE20D  bl 0x82466e20
	ctx.lr = 0x82698C18;
	sub_82466E20(ctx, base);
	// 82698C18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698C28 size=108
    let mut pc: u32 = 0x82698C28;
    'dispatch: loop {
        match pc {
            0x82698C28 => {
    //   block [0x82698C28..0x82698C94)
	// 82698C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698C30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698C34: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698C38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698C3C: 38EBEF70  addi r7, r11, -0x1090
	ctx.r[7].s64 = ctx.r[11].s64 + -4240;
	// 82698C40: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698C44: 388A7888  addi r4, r10, 0x7888
	ctx.r[4].s64 = ctx.r[10].s64 + 30856;
	// 82698C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698C4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698C50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698C58: 386A2B70  addi r3, r10, 0x2b70
	ctx.r[3].s64 = ctx.r[10].s64 + 11120;
	// 82698C5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698C60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698C64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698C68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698C6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698C70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698C74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698C78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698C7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698C80: 4BDCE1A1  bl 0x82466e20
	ctx.lr = 0x82698C84;
	sub_82466E20(ctx, base);
	// 82698C84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698C98 size=24
    let mut pc: u32 = 0x82698C98;
    'dispatch: loop {
        match pc {
            0x82698C98 => {
    //   block [0x82698C98..0x82698CB0)
	// 82698C98: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698C9C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698CA0: 394A6858  addi r10, r10, 0x6858
	ctx.r[10].s64 = ctx.r[10].s64 + 26712;
	// 82698CA4: 816BEF6C  lwz r11, -0x1094(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4244 as u32) ) } as u64;
	// 82698CA8: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82698CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698CB0 size=112
    let mut pc: u32 = 0x82698CB0;
    'dispatch: loop {
        match pc {
            0x82698CB0 => {
    //   block [0x82698CB0..0x82698D20)
	// 82698CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698CB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698CBC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698CC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698CC4: 392A95E8  addi r9, r10, -0x6a18
	ctx.r[9].s64 = ctx.r[10].s64 + -27160;
	// 82698CC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698CCC: 390B6858  addi r8, r11, 0x6858
	ctx.r[8].s64 = ctx.r[11].s64 + 26712;
	// 82698CD0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 82698CD4: 388A7898  addi r4, r10, 0x7898
	ctx.r[4].s64 = ctx.r[10].s64 + 30872;
	// 82698CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698CDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698CE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698CE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698CE8: 386A2BA0  addi r3, r10, 0x2ba0
	ctx.r[3].s64 = ctx.r[10].s64 + 11168;
	// 82698CEC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698CF0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82698CF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698CF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698CFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698D00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698D04: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698D08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698D0C: 4BDCE115  bl 0x82466e20
	ctx.lr = 0x82698D10;
	sub_82466E20(ctx, base);
	// 82698D10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698D1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82698D20 size=40
    let mut pc: u32 = 0x82698D20;
    'dispatch: loop {
        match pc {
            0x82698D20 => {
    //   block [0x82698D20..0x82698D48)
	// 82698D20: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698D24: 814BEFA0  lwz r10, -0x1060(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4192 as u32) ) } as u64;
	// 82698D28: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698D2C: 396B68B8  addi r11, r11, 0x68b8
	ctx.r[11].s64 = ctx.r[11].s64 + 26808;
	// 82698D30: 914B0050  stw r10, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82698D34: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82698D38: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82698D3C: 814AEFA4  lwz r10, -0x105c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4188 as u32) ) } as u64;
	// 82698D40: 914B0098  stw r10, 0x98(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(152 as u32), ctx.r[10].u32 ) };
	// 82698D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698D48 size=112
    let mut pc: u32 = 0x82698D48;
    'dispatch: loop {
        match pc {
            0x82698D48 => {
    //   block [0x82698D48..0x82698DB8)
	// 82698D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698D50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698D54: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82698D58: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698D5C: 392A9760  addi r9, r10, -0x68a0
	ctx.r[9].s64 = ctx.r[10].s64 + -26784;
	// 82698D60: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698D64: 390B68B8  addi r8, r11, 0x68b8
	ctx.r[8].s64 = ctx.r[11].s64 + 26808;
	// 82698D68: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82698D6C: 388A72F4  addi r4, r10, 0x72f4
	ctx.r[4].s64 = ctx.r[10].s64 + 29428;
	// 82698D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698D74: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698D78: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82698D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698D80: 386A2BD0  addi r3, r10, 0x2bd0
	ctx.r[3].s64 = ctx.r[10].s64 + 11216;
	// 82698D84: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82698D88: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82698D8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698D90: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698D94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698D98: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698D9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698DA0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698DA4: 4BDCE07D  bl 0x82466e20
	ctx.lr = 0x82698DA8;
	sub_82466E20(ctx, base);
	// 82698DA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698DB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698DB8 size=108
    let mut pc: u32 = 0x82698DB8;
    'dispatch: loop {
        match pc {
            0x82698DB8 => {
    //   block [0x82698DB8..0x82698E24)
	// 82698DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698DC4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698DC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698DCC: 38EBEFAC  addi r7, r11, -0x1054
	ctx.r[7].s64 = ctx.r[11].s64 + -4180;
	// 82698DD0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698DD4: 388A6E48  addi r4, r10, 0x6e48
	ctx.r[4].s64 = ctx.r[10].s64 + 28232;
	// 82698DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698DDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698DE0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698DE4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698DE8: 386A2C00  addi r3, r10, 0x2c00
	ctx.r[3].s64 = ctx.r[10].s64 + 11264;
	// 82698DEC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698DF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698DF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698DFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698E00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698E04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698E08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698E0C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698E10: 4BDCE011  bl 0x82466e20
	ctx.lr = 0x82698E14;
	sub_82466E20(ctx, base);
	// 82698E14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698E18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698E1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698E28 size=108
    let mut pc: u32 = 0x82698E28;
    'dispatch: loop {
        match pc {
            0x82698E28 => {
    //   block [0x82698E28..0x82698E94)
	// 82698E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698E30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698E34: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698E38: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698E3C: 38EBEFDC  addi r7, r11, -0x1024
	ctx.r[7].s64 = ctx.r[11].s64 + -4132;
	// 82698E40: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82698E44: 388A6E64  addi r4, r10, 0x6e64
	ctx.r[4].s64 = ctx.r[10].s64 + 28260;
	// 82698E48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698E4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698E50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698E54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698E58: 386A2C30  addi r3, r10, 0x2c30
	ctx.r[3].s64 = ctx.r[10].s64 + 11312;
	// 82698E5C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698E60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698E64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698E68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698E6C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698E70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698E74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698E78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698E7C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698E80: 4BDCDFA1  bl 0x82466e20
	ctx.lr = 0x82698E84;
	sub_82466E20(ctx, base);
	// 82698E84: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698E90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698E98 size=108
    let mut pc: u32 = 0x82698E98;
    'dispatch: loop {
        match pc {
            0x82698E98 => {
    //   block [0x82698E98..0x82698F04)
	// 82698E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698E9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698EA4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698EA8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698EAC: 38EBEFF4  addi r7, r11, -0x100c
	ctx.r[7].s64 = ctx.r[11].s64 + -4108;
	// 82698EB0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82698EB4: 388A7BB0  addi r4, r10, 0x7bb0
	ctx.r[4].s64 = ctx.r[10].s64 + 31664;
	// 82698EB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698EBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698EC0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698EC4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698EC8: 386A2C60  addi r3, r10, 0x2c60
	ctx.r[3].s64 = ctx.r[10].s64 + 11360;
	// 82698ECC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698ED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698ED4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698ED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698EDC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698EE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698EE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698EE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698EEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698EF0: 4BDCDF31  bl 0x82466e20
	ctx.lr = 0x82698EF4;
	sub_82466E20(ctx, base);
	// 82698EF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698F08 size=108
    let mut pc: u32 = 0x82698F08;
    'dispatch: loop {
        match pc {
            0x82698F08 => {
    //   block [0x82698F08..0x82698F74)
	// 82698F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698F0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698F10: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698F14: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698F18: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698F1C: 38EBF028  addi r7, r11, -0xfd8
	ctx.r[7].s64 = ctx.r[11].s64 + -4056;
	// 82698F20: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 82698F24: 388A7BB8  addi r4, r10, 0x7bb8
	ctx.r[4].s64 = ctx.r[10].s64 + 31672;
	// 82698F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698F2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698F30: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698F34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698F38: 386A2C90  addi r3, r10, 0x2c90
	ctx.r[3].s64 = ctx.r[10].s64 + 11408;
	// 82698F3C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698F44: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698F48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698F4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698F50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698F54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698F58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698F5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698F60: 4BDCDEC1  bl 0x82466e20
	ctx.lr = 0x82698F64;
	sub_82466E20(ctx, base);
	// 82698F64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698F68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698F6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698F70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698F78 size=108
    let mut pc: u32 = 0x82698F78;
    'dispatch: loop {
        match pc {
            0x82698F78 => {
    //   block [0x82698F78..0x82698FE4)
	// 82698F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698F80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698F84: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698F88: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698F8C: 38EBF0B8  addi r7, r11, -0xf48
	ctx.r[7].s64 = ctx.r[11].s64 + -3912;
	// 82698F90: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82698F94: 388A78C0  addi r4, r10, 0x78c0
	ctx.r[4].s64 = ctx.r[10].s64 + 30912;
	// 82698F98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82698F9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82698FA0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82698FA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82698FA8: 386A2CC0  addi r3, r10, 0x2cc0
	ctx.r[3].s64 = ctx.r[10].s64 + 11456;
	// 82698FAC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82698FB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82698FB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82698FB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82698FBC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82698FC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82698FC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82698FC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82698FCC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82698FD0: 4BDCDE51  bl 0x82466e20
	ctx.lr = 0x82698FD4;
	sub_82466E20(ctx, base);
	// 82698FD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82698FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82698FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82698FE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82698FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82698FE8 size=108
    let mut pc: u32 = 0x82698FE8;
    'dispatch: loop {
        match pc {
            0x82698FE8 => {
    //   block [0x82698FE8..0x82699054)
	// 82698FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82698FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82698FF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82698FF4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82698FF8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82698FFC: 38EBF0D0  addi r7, r11, -0xf30
	ctx.r[7].s64 = ctx.r[11].s64 + -3888;
	// 82699000: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699004: 388A7F88  addi r4, r10, 0x7f88
	ctx.r[4].s64 = ctx.r[10].s64 + 32648;
	// 82699008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269900C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699010: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699014: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699018: 386A2CF0  addi r3, r10, 0x2cf0
	ctx.r[3].s64 = ctx.r[10].s64 + 11504;
	// 8269901C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699024: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269902C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269903C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699040: 4BDCDDE1  bl 0x82466e20
	ctx.lr = 0x82699044;
	sub_82466E20(ctx, base);
	// 82699044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699048: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269904C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699058 size=112
    let mut pc: u32 = 0x82699058;
    'dispatch: loop {
        match pc {
            0x82699058 => {
    //   block [0x82699058..0x826990C8)
	// 82699058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269905C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699060: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699064: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699068: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269906C: 392A97B4  addi r9, r10, -0x684c
	ctx.r[9].s64 = ctx.r[10].s64 + -26700;
	// 82699070: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699074: 390BF100  addi r8, r11, -0xf00
	ctx.r[8].s64 = ctx.r[11].s64 + -3840;
	// 82699078: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269907C: 388A82E8  addi r4, r10, -0x7d18
	ctx.r[4].s64 = ctx.r[10].s64 + -32024;
	// 82699080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699084: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699088: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269908C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699090: 386A2D20  addi r3, r10, 0x2d20
	ctx.r[3].s64 = ctx.r[10].s64 + 11552;
	// 82699094: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699098: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269909C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826990A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826990A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826990A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826990AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826990B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826990B4: 4BDCDD6D  bl 0x82466e20
	ctx.lr = 0x826990B8;
	sub_82466E20(ctx, base);
	// 826990B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826990BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826990C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826990C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826990C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826990C8 size=108
    let mut pc: u32 = 0x826990C8;
    'dispatch: loop {
        match pc {
            0x826990C8 => {
    //   block [0x826990C8..0x82699134)
	// 826990C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826990CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826990D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826990D4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826990D8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 826990DC: 38EBF178  addi r7, r11, -0xe88
	ctx.r[7].s64 = ctx.r[11].s64 + -3720;
	// 826990E0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 826990E4: 388A7F28  addi r4, r10, 0x7f28
	ctx.r[4].s64 = ctx.r[10].s64 + 32552;
	// 826990E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826990EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826990F0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826990F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826990F8: 386A2D50  addi r3, r10, 0x2d50
	ctx.r[3].s64 = ctx.r[10].s64 + 11600;
	// 826990FC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699100: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699108: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269910C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699110: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699114: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699118: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269911C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699120: 4BDCDD01  bl 0x82466e20
	ctx.lr = 0x82699124;
	sub_82466E20(ctx, base);
	// 82699124: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269912C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82699138 size=24
    let mut pc: u32 = 0x82699138;
    'dispatch: loop {
        match pc {
            0x82699138 => {
    //   block [0x82699138..0x82699150)
	// 82699138: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269913C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82699140: 394A6990  addi r10, r10, 0x6990
	ctx.r[10].s64 = ctx.r[10].s64 + 27024;
	// 82699144: 816BF268  lwz r11, -0xd98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3480 as u32) ) } as u64;
	// 82699148: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269914C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699150 size=108
    let mut pc: u32 = 0x82699150;
    'dispatch: loop {
        match pc {
            0x82699150 => {
    //   block [0x82699150..0x826991BC)
	// 82699150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269915C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699160: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699164: 38EB6990  addi r7, r11, 0x6990
	ctx.r[7].s64 = ctx.r[11].s64 + 27024;
	// 82699168: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269916C: 388AB020  addi r4, r10, -0x4fe0
	ctx.r[4].s64 = ctx.r[10].s64 + -20448;
	// 82699170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699174: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269917C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699180: 386A2D80  addi r3, r10, 0x2d80
	ctx.r[3].s64 = ctx.r[10].s64 + 11648;
	// 82699184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269918C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269919C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826991A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826991A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826991A8: 4BDCDC79  bl 0x82466e20
	ctx.lr = 0x826991AC;
	sub_82466E20(ctx, base);
	// 826991AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826991B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826991B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826991B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826991C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826991C0 size=24
    let mut pc: u32 = 0x826991C0;
    'dispatch: loop {
        match pc {
            0x826991C0 => {
    //   block [0x826991C0..0x826991D8)
	// 826991C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826991C4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826991C8: 394A69C0  addi r10, r10, 0x69c0
	ctx.r[10].s64 = ctx.r[10].s64 + 27072;
	// 826991CC: 816BF268  lwz r11, -0xd98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3480 as u32) ) } as u64;
	// 826991D0: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826991D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826991D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826991D8 size=108
    let mut pc: u32 = 0x826991D8;
    'dispatch: loop {
        match pc {
            0x826991D8 => {
    //   block [0x826991D8..0x82699244)
	// 826991D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826991DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826991E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826991E4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826991E8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826991EC: 38EB69C0  addi r7, r11, 0x69c0
	ctx.r[7].s64 = ctx.r[11].s64 + 27072;
	// 826991F0: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826991F4: 388A9C48  addi r4, r10, -0x63b8
	ctx.r[4].s64 = ctx.r[10].s64 + -25528;
	// 826991F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826991FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699200: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699208: 386A2DB0  addi r3, r10, 0x2db0
	ctx.r[3].s64 = ctx.r[10].s64 + 11696;
	// 8269920C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699210: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699214: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699218: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269921C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699220: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699224: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699228: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269922C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699230: 4BDCDBF1  bl 0x82466e20
	ctx.lr = 0x82699234;
	sub_82466E20(ctx, base);
	// 82699234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269923C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699248 size=108
    let mut pc: u32 = 0x82699248;
    'dispatch: loop {
        match pc {
            0x82699248 => {
    //   block [0x82699248..0x826992B4)
	// 82699248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269924C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699254: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699258: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269925C: 38EBF250  addi r7, r11, -0xdb0
	ctx.r[7].s64 = ctx.r[11].s64 + -3504;
	// 82699260: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699264: 388AA080  addi r4, r10, -0x5f80
	ctx.r[4].s64 = ctx.r[10].s64 + -24448;
	// 82699268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269926C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699270: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699278: 386A2DE0  addi r3, r10, 0x2de0
	ctx.r[3].s64 = ctx.r[10].s64 + 11744;
	// 8269927C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699280: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699284: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699288: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269928C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699290: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699298: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269929C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826992A0: 4BDCDB81  bl 0x82466e20
	ctx.lr = 0x826992A4;
	sub_82466E20(ctx, base);
	// 826992A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826992A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826992AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826992B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826992B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826992B8 size=24
    let mut pc: u32 = 0x826992B8;
    'dispatch: loop {
        match pc {
            0x826992B8 => {
    //   block [0x826992B8..0x826992D0)
	// 826992B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826992BC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826992C0: 394A69F0  addi r10, r10, 0x69f0
	ctx.r[10].s64 = ctx.r[10].s64 + 27120;
	// 826992C4: 816BF268  lwz r11, -0xd98(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3480 as u32) ) } as u64;
	// 826992C8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 826992CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826992D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826992D0 size=108
    let mut pc: u32 = 0x826992D0;
    'dispatch: loop {
        match pc {
            0x826992D0 => {
    //   block [0x826992D0..0x8269933C)
	// 826992D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826992D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826992D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826992DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826992E0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826992E4: 38EB69F0  addi r7, r11, 0x69f0
	ctx.r[7].s64 = ctx.r[11].s64 + 27120;
	// 826992E8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826992EC: 388A9BE8  addi r4, r10, -0x6418
	ctx.r[4].s64 = ctx.r[10].s64 + -25624;
	// 826992F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826992F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826992F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826992FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699300: 386A2E10  addi r3, r10, 0x2e10
	ctx.r[3].s64 = ctx.r[10].s64 + 11792;
	// 82699304: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699308: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269930C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699310: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699318: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269931C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699320: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699324: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699328: 4BDCDAF9  bl 0x82466e20
	ctx.lr = 0x8269932C;
	sub_82466E20(ctx, base);
	// 8269932C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699340 size=112
    let mut pc: u32 = 0x82699340;
    'dispatch: loop {
        match pc {
            0x82699340 => {
    //   block [0x82699340..0x826993B0)
	// 82699340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269934C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699350: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699354: 392A97F8  addi r9, r10, -0x6808
	ctx.r[9].s64 = ctx.r[10].s64 + -26632;
	// 82699358: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269935C: 390BF26C  addi r8, r11, -0xd94
	ctx.r[8].s64 = ctx.r[11].s64 + -3476;
	// 82699360: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 82699364: 388AA778  addi r4, r10, -0x5888
	ctx.r[4].s64 = ctx.r[10].s64 + -22664;
	// 82699368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269936C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699370: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699378: 386A2E40  addi r3, r10, 0x2e40
	ctx.r[3].s64 = ctx.r[10].s64 + 11840;
	// 8269937C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699380: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82699384: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699388: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269938C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699390: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699394: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699398: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269939C: 4BDCDA85  bl 0x82466e20
	ctx.lr = 0x826993A0;
	sub_82466E20(ctx, base);
	// 826993A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826993A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826993A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826993AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826993B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826993B0 size=108
    let mut pc: u32 = 0x826993B0;
    'dispatch: loop {
        match pc {
            0x826993B0 => {
    //   block [0x826993B0..0x8269941C)
	// 826993B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826993B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826993B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826993BC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826993C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826993C4: 38EBF29C  addi r7, r11, -0xd64
	ctx.r[7].s64 = ctx.r[11].s64 + -3428;
	// 826993C8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826993CC: 388AA350  addi r4, r10, -0x5cb0
	ctx.r[4].s64 = ctx.r[10].s64 + -23728;
	// 826993D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826993D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826993D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826993DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826993E0: 386A2E70  addi r3, r10, 0x2e70
	ctx.r[3].s64 = ctx.r[10].s64 + 11888;
	// 826993E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826993E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826993EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826993F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826993F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826993F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826993FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699400: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699404: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699408: 4BDCDA19  bl 0x82466e20
	ctx.lr = 0x8269940C;
	sub_82466E20(ctx, base);
	// 8269940C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699410: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699414: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699418: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699420 size=108
    let mut pc: u32 = 0x82699420;
    'dispatch: loop {
        match pc {
            0x82699420 => {
    //   block [0x82699420..0x8269948C)
	// 82699420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699428: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269942C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699430: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699434: 38EBF2CC  addi r7, r11, -0xd34
	ctx.r[7].s64 = ctx.r[11].s64 + -3380;
	// 82699438: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269943C: 388AB2D8  addi r4, r10, -0x4d28
	ctx.r[4].s64 = ctx.r[10].s64 + -19752;
	// 82699440: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699444: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699448: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269944C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699450: 386A2EA0  addi r3, r10, 0x2ea0
	ctx.r[3].s64 = ctx.r[10].s64 + 11936;
	// 82699454: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699458: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269945C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699460: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699468: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269946C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699470: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699474: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699478: 4BDCD9A9  bl 0x82466e20
	ctx.lr = 0x8269947C;
	sub_82466E20(ctx, base);
	// 8269947C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699480: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699484: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699490 size=108
    let mut pc: u32 = 0x82699490;
    'dispatch: loop {
        match pc {
            0x82699490 => {
    //   block [0x82699490..0x826994FC)
	// 82699490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699498: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269949C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826994A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826994A4: 38EBF2E4  addi r7, r11, -0xd1c
	ctx.r[7].s64 = ctx.r[11].s64 + -3356;
	// 826994A8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 826994AC: 388AAA44  addi r4, r10, -0x55bc
	ctx.r[4].s64 = ctx.r[10].s64 + -21948;
	// 826994B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826994B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826994B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826994BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826994C0: 386A2ED0  addi r3, r10, 0x2ed0
	ctx.r[3].s64 = ctx.r[10].s64 + 11984;
	// 826994C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826994C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826994CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826994D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826994D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826994D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826994DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826994E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826994E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826994E8: 4BDCD939  bl 0x82466e20
	ctx.lr = 0x826994EC;
	sub_82466E20(ctx, base);
	// 826994EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826994F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826994F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826994F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699500 size=112
    let mut pc: u32 = 0x82699500;
    'dispatch: loop {
        match pc {
            0x82699500 => {
    //   block [0x82699500..0x82699570)
	// 82699500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699508: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269950C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699510: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699514: 38AA2F30  addi r5, r10, 0x2f30
	ctx.r[5].s64 = ctx.r[10].s64 + 12080;
	// 82699518: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269951C: 390BF314  addi r8, r11, -0xcec
	ctx.r[8].s64 = ctx.r[11].s64 + -3308;
	// 82699520: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82699524: 388AA9A0  addi r4, r10, -0x5660
	ctx.r[4].s64 = ctx.r[10].s64 + -22112;
	// 82699528: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269952C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699530: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699534: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699538: 386A2F00  addi r3, r10, 0x2f00
	ctx.r[3].s64 = ctx.r[10].s64 + 12032;
	// 8269953C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82699540: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699544: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699548: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269954C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699550: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699554: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699558: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269955C: 4BDCD8C5  bl 0x82466e20
	ctx.lr = 0x82699560;
	sub_82466E20(ctx, base);
	// 82699560: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699564: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699568: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269956C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699570 size=108
    let mut pc: u32 = 0x82699570;
    'dispatch: loop {
        match pc {
            0x82699570 => {
    //   block [0x82699570..0x826995DC)
	// 82699570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699578: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269957C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699580: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699584: 38EBF32C  addi r7, r11, -0xcd4
	ctx.r[7].s64 = ctx.r[11].s64 + -3284;
	// 82699588: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269958C: 388AA640  addi r4, r10, -0x59c0
	ctx.r[4].s64 = ctx.r[10].s64 + -22976;
	// 82699590: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699594: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699598: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269959C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826995A0: 386A2F30  addi r3, r10, 0x2f30
	ctx.r[3].s64 = ctx.r[10].s64 + 12080;
	// 826995A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826995A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826995AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826995B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826995B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826995B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826995BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826995C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826995C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826995C8: 4BDCD859  bl 0x82466e20
	ctx.lr = 0x826995CC;
	sub_82466E20(ctx, base);
	// 826995CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826995D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826995D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826995D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826995E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826995E0 size=108
    let mut pc: u32 = 0x826995E0;
    'dispatch: loop {
        match pc {
            0x826995E0 => {
    //   block [0x826995E0..0x8269964C)
	// 826995E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826995E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826995E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826995EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826995F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826995F4: 38EBF35C  addi r7, r11, -0xca4
	ctx.r[7].s64 = ctx.r[11].s64 + -3236;
	// 826995F8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826995FC: 388AA2E0  addi r4, r10, -0x5d20
	ctx.r[4].s64 = ctx.r[10].s64 + -23840;
	// 82699600: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699604: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699608: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269960C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699610: 386A2F60  addi r3, r10, 0x2f60
	ctx.r[3].s64 = ctx.r[10].s64 + 12128;
	// 82699614: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269961C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699620: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699624: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699628: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269962C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699630: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699638: 4BDCD7E9  bl 0x82466e20
	ctx.lr = 0x8269963C;
	sub_82466E20(ctx, base);
	// 8269963C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699650 size=108
    let mut pc: u32 = 0x82699650;
    'dispatch: loop {
        match pc {
            0x82699650 => {
    //   block [0x82699650..0x826996BC)
	// 82699650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699658: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269965C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699660: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699664: 38EBF374  addi r7, r11, -0xc8c
	ctx.r[7].s64 = ctx.r[11].s64 + -3212;
	// 82699668: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269966C: 388AA304  addi r4, r10, -0x5cfc
	ctx.r[4].s64 = ctx.r[10].s64 + -23804;
	// 82699670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699674: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699678: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269967C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699680: 386A2F90  addi r3, r10, 0x2f90
	ctx.r[3].s64 = ctx.r[10].s64 + 12176;
	// 82699684: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699688: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269968C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699690: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699694: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699698: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269969C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826996A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826996A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826996A8: 4BDCD779  bl 0x82466e20
	ctx.lr = 0x826996AC;
	sub_82466E20(ctx, base);
	// 826996AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826996B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826996B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826996B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826996C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826996C0 size=108
    let mut pc: u32 = 0x826996C0;
    'dispatch: loop {
        match pc {
            0x826996C0 => {
    //   block [0x826996C0..0x8269972C)
	// 826996C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826996C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826996C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826996CC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826996D0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826996D4: 38EBF3A8  addi r7, r11, -0xc58
	ctx.r[7].s64 = ctx.r[11].s64 + -3160;
	// 826996D8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 826996DC: 388AAED0  addi r4, r10, -0x5130
	ctx.r[4].s64 = ctx.r[10].s64 + -20784;
	// 826996E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826996E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826996E8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826996EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826996F0: 386A2FC0  addi r3, r10, 0x2fc0
	ctx.r[3].s64 = ctx.r[10].s64 + 12224;
	// 826996F4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826996F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826996FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699700: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699704: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699708: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269970C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699710: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699714: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699718: 4BDCD709  bl 0x82466e20
	ctx.lr = 0x8269971C;
	sub_82466E20(ctx, base);
	// 8269971C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699730 size=108
    let mut pc: u32 = 0x82699730;
    'dispatch: loop {
        match pc {
            0x82699730 => {
    //   block [0x82699730..0x8269979C)
	// 82699730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269973C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699740: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699744: 38EBF450  addi r7, r11, -0xbb0
	ctx.r[7].s64 = ctx.r[11].s64 + -2992;
	// 82699748: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269974C: 388AA130  addi r4, r10, -0x5ed0
	ctx.r[4].s64 = ctx.r[10].s64 + -24272;
	// 82699750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699754: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699758: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269975C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699760: 386A2FF0  addi r3, r10, 0x2ff0
	ctx.r[3].s64 = ctx.r[10].s64 + 12272;
	// 82699764: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269976C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699774: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269977C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699784: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699788: 4BDCD699  bl 0x82466e20
	ctx.lr = 0x8269978C;
	sub_82466E20(ctx, base);
	// 8269978C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699790: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699794: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826997A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826997A0 size=108
    let mut pc: u32 = 0x826997A0;
    'dispatch: loop {
        match pc {
            0x826997A0 => {
    //   block [0x826997A0..0x8269980C)
	// 826997A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826997A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826997A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826997AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826997B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 826997B4: 38EBF480  addi r7, r11, -0xb80
	ctx.r[7].s64 = ctx.r[11].s64 + -2944;
	// 826997B8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 826997BC: 388AA148  addi r4, r10, -0x5eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -24248;
	// 826997C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826997C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826997C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826997CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826997D0: 386A3020  addi r3, r10, 0x3020
	ctx.r[3].s64 = ctx.r[10].s64 + 12320;
	// 826997D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826997D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826997DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826997E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826997E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826997E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826997EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826997F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826997F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826997F8: 4BDCD629  bl 0x82466e20
	ctx.lr = 0x826997FC;
	sub_82466E20(ctx, base);
	// 826997FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699800: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699804: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699810 size=108
    let mut pc: u32 = 0x82699810;
    'dispatch: loop {
        match pc {
            0x82699810 => {
    //   block [0x82699810..0x8269987C)
	// 82699810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699814: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269981C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699820: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699824: 38EBF498  addi r7, r11, -0xb68
	ctx.r[7].s64 = ctx.r[11].s64 + -2920;
	// 82699828: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269982C: 388AB290  addi r4, r10, -0x4d70
	ctx.r[4].s64 = ctx.r[10].s64 + -19824;
	// 82699830: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699834: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699838: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269983C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699840: 386A3050  addi r3, r10, 0x3050
	ctx.r[3].s64 = ctx.r[10].s64 + 12368;
	// 82699844: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699848: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269984C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699854: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269985C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699864: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699868: 4BDCD5B9  bl 0x82466e20
	ctx.lr = 0x8269986C;
	sub_82466E20(ctx, base);
	// 8269986C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699880 size=112
    let mut pc: u32 = 0x82699880;
    'dispatch: loop {
        match pc {
            0x82699880 => {
    //   block [0x82699880..0x826998F0)
	// 82699880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269988C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699890: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699894: 38AA2EA0  addi r5, r10, 0x2ea0
	ctx.r[5].s64 = ctx.r[10].s64 + 11936;
	// 82699898: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269989C: 390BF4C8  addi r8, r11, -0xb38
	ctx.r[8].s64 = ctx.r[11].s64 + -2872;
	// 826998A0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 826998A4: 388AB2A8  addi r4, r10, -0x4d58
	ctx.r[4].s64 = ctx.r[10].s64 + -19800;
	// 826998A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 826998AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826998B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 826998B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826998B8: 386A3080  addi r3, r10, 0x3080
	ctx.r[3].s64 = ctx.r[10].s64 + 12416;
	// 826998BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 826998C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826998C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826998C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826998CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826998D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826998D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826998D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826998DC: 4BDCD545  bl 0x82466e20
	ctx.lr = 0x826998E0;
	sub_82466E20(ctx, base);
	// 826998E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826998E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826998E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826998EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826998F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x826998F0 size=24
    let mut pc: u32 = 0x826998F0;
    'dispatch: loop {
        match pc {
            0x826998F0 => {
    //   block [0x826998F0..0x82699908)
	// 826998F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826998F4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 826998F8: 394A6A20  addi r10, r10, 0x6a20
	ctx.r[10].s64 = ctx.r[10].s64 + 27168;
	// 826998FC: 816BF3A4  lwz r11, -0xc5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-3164 as u32) ) } as u64;
	// 82699900: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82699904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699908 size=112
    let mut pc: u32 = 0x82699908;
    'dispatch: loop {
        match pc {
            0x82699908 => {
    //   block [0x82699908..0x82699978)
	// 82699908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269990C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699910: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699914: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699918: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269991C: 392A9824  addi r9, r10, -0x67dc
	ctx.r[9].s64 = ctx.r[10].s64 + -26588;
	// 82699920: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699924: 390B6A20  addi r8, r11, 0x6a20
	ctx.r[8].s64 = ctx.r[11].s64 + 27168;
	// 82699928: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269992C: 388AB098  addi r4, r10, -0x4f68
	ctx.r[4].s64 = ctx.r[10].s64 + -20328;
	// 82699930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699938: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269993C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699940: 386A30B0  addi r3, r10, 0x30b0
	ctx.r[3].s64 = ctx.r[10].s64 + 12464;
	// 82699944: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699948: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269994C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699950: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699958: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269995C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699960: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699964: 4BDCD4BD  bl 0x82466e20
	ctx.lr = 0x82699968;
	sub_82466E20(ctx, base);
	// 82699968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269996C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699970: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699974: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699978 size=108
    let mut pc: u32 = 0x82699978;
    'dispatch: loop {
        match pc {
            0x82699978 => {
    //   block [0x82699978..0x826999E4)
	// 82699978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269997C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699980: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699984: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699988: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269998C: 38EBF574  addi r7, r11, -0xa8c
	ctx.r[7].s64 = ctx.r[11].s64 + -2700;
	// 82699990: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699994: 388AAC70  addi r4, r10, -0x5390
	ctx.r[4].s64 = ctx.r[10].s64 + -21392;
	// 82699998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269999C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 826999A0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 826999A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 826999A8: 386A30E0  addi r3, r10, 0x30e0
	ctx.r[3].s64 = ctx.r[10].s64 + 12512;
	// 826999AC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 826999B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 826999B4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 826999B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 826999BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 826999C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 826999C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 826999C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 826999CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 826999D0: 4BDCD451  bl 0x82466e20
	ctx.lr = 0x826999D4;
	sub_82466E20(ctx, base);
	// 826999D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 826999D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 826999DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 826999E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_826999E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x826999E8 size=116
    let mut pc: u32 = 0x826999E8;
    'dispatch: loop {
        match pc {
            0x826999E8 => {
    //   block [0x826999E8..0x82699A5C)
	// 826999E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 826999EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 826999F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 826999F4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 826999F8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 826999FC: 390BF5A8  addi r8, r11, -0xa58
	ctx.r[8].s64 = ctx.r[11].s64 + -2648;
	// 82699A00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699A04: 392A9868  addi r9, r10, -0x6798
	ctx.r[9].s64 = ctx.r[10].s64 + -26520;
	// 82699A08: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699A0C: 38E00008  li r7, 8
	ctx.r[7].s64 = 8;
	// 82699A10: 38AA2EA0  addi r5, r10, 0x2ea0
	ctx.r[5].s64 = ctx.r[10].s64 + 11936;
	// 82699A14: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699A18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699A1C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699A20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699A24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699A28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699A2C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 82699A30: 388AAC88  addi r4, r10, -0x5378
	ctx.r[4].s64 = ctx.r[10].s64 + -21368;
	// 82699A34: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699A38: 386B3110  addi r3, r11, 0x3110
	ctx.r[3].s64 = ctx.r[11].s64 + 12560;
	// 82699A3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82699A40: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699A44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699A48: 4BDCD3D9  bl 0x82466e20
	ctx.lr = 0x82699A4C;
	sub_82466E20(ctx, base);
	// 82699A4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699A50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699A54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82699A60 size=24
    let mut pc: u32 = 0x82699A60;
    'dispatch: loop {
        match pc {
            0x82699A60 => {
    //   block [0x82699A60..0x82699A78)
	// 82699A60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699A64: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82699A68: 394A6A98  addi r10, r10, 0x6a98
	ctx.r[10].s64 = ctx.r[10].s64 + 27288;
	// 82699A6C: 816BF5A4  lwz r11, -0xa5c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2652 as u32) ) } as u64;
	// 82699A70: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82699A74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699A78 size=112
    let mut pc: u32 = 0x82699A78;
    'dispatch: loop {
        match pc {
            0x82699A78 => {
    //   block [0x82699A78..0x82699AE8)
	// 82699A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699A7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699A84: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699A88: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699A8C: 392A98A4  addi r9, r10, -0x675c
	ctx.r[9].s64 = ctx.r[10].s64 + -26460;
	// 82699A90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699A94: 390B6A98  addi r8, r11, 0x6a98
	ctx.r[8].s64 = ctx.r[11].s64 + 27288;
	// 82699A98: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 82699A9C: 388AB004  addi r4, r10, -0x4ffc
	ctx.r[4].s64 = ctx.r[10].s64 + -20476;
	// 82699AA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699AA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699AA8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699AAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699AB0: 386A3140  addi r3, r10, 0x3140
	ctx.r[3].s64 = ctx.r[10].s64 + 12608;
	// 82699AB4: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699AB8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82699ABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699AC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699AC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699AC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699ACC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699AD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699AD4: 4BDCD34D  bl 0x82466e20
	ctx.lr = 0x82699AD8;
	sub_82466E20(ctx, base);
	// 82699AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699AE8 size=108
    let mut pc: u32 = 0x82699AE8;
    'dispatch: loop {
        match pc {
            0x82699AE8 => {
    //   block [0x82699AE8..0x82699B54)
	// 82699AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699AEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699AF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699AF4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699AF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699AFC: 38EBF668  addi r7, r11, -0x998
	ctx.r[7].s64 = ctx.r[11].s64 + -2456;
	// 82699B00: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699B04: 388AAF00  addi r4, r10, -0x5100
	ctx.r[4].s64 = ctx.r[10].s64 + -20736;
	// 82699B08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699B0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699B10: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699B14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699B18: 386A3170  addi r3, r10, 0x3170
	ctx.r[3].s64 = ctx.r[10].s64 + 12656;
	// 82699B1C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699B20: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699B24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699B28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699B2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699B30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699B34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699B38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699B3C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699B40: 4BDCD2E1  bl 0x82466e20
	ctx.lr = 0x82699B44;
	sub_82466E20(ctx, base);
	// 82699B44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699B58 size=108
    let mut pc: u32 = 0x82699B58;
    'dispatch: loop {
        match pc {
            0x82699B58 => {
    //   block [0x82699B58..0x82699BC4)
	// 82699B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699B64: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699B68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699B6C: 38EBF680  addi r7, r11, -0x980
	ctx.r[7].s64 = ctx.r[11].s64 + -2432;
	// 82699B70: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699B74: 388A9D38  addi r4, r10, -0x62c8
	ctx.r[4].s64 = ctx.r[10].s64 + -25288;
	// 82699B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699B7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699B80: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699B84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699B88: 386A31A0  addi r3, r10, 0x31a0
	ctx.r[3].s64 = ctx.r[10].s64 + 12704;
	// 82699B8C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699B90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699B94: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699B98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699B9C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699BA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699BA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699BA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699BAC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699BB0: 4BDCD271  bl 0x82466e20
	ctx.lr = 0x82699BB4;
	sub_82466E20(ctx, base);
	// 82699BB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82699BC8 size=24
    let mut pc: u32 = 0x82699BC8;
    'dispatch: loop {
        match pc {
            0x82699BC8 => {
    //   block [0x82699BC8..0x82699BE0)
	// 82699BC8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699BCC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 82699BD0: 394A6AE0  addi r10, r10, 0x6ae0
	ctx.r[10].s64 = ctx.r[10].s64 + 27360;
	// 82699BD4: 816BF6B0  lwz r11, -0x950(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-2384 as u32) ) } as u64;
	// 82699BD8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82699BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699BE0 size=112
    let mut pc: u32 = 0x82699BE0;
    'dispatch: loop {
        match pc {
            0x82699BE0 => {
    //   block [0x82699BE0..0x82699C50)
	// 82699BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699BE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699BEC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 82699BF0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699BF4: 392A98E0  addi r9, r10, -0x6720
	ctx.r[9].s64 = ctx.r[10].s64 + -26400;
	// 82699BF8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699BFC: 390B6AE0  addi r8, r11, 0x6ae0
	ctx.r[8].s64 = ctx.r[11].s64 + 27360;
	// 82699C00: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 82699C04: 388AB450  addi r4, r10, -0x4bb0
	ctx.r[4].s64 = ctx.r[10].s64 + -19376;
	// 82699C08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699C0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699C10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699C18: 386A31D0  addi r3, r10, 0x31d0
	ctx.r[3].s64 = ctx.r[10].s64 + 12752;
	// 82699C1C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 82699C20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82699C24: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699C28: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699C2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699C30: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699C34: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699C38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699C3C: 4BDCD1E5  bl 0x82466e20
	ctx.lr = 0x82699C40;
	sub_82466E20(ctx, base);
	// 82699C40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699C50 size=112
    let mut pc: u32 = 0x82699C50;
    'dispatch: loop {
        match pc {
            0x82699C50 => {
    //   block [0x82699C50..0x82699CC0)
	// 82699C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699C58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699C5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699C60: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699C64: 38AA2EA0  addi r5, r10, 0x2ea0
	ctx.r[5].s64 = ctx.r[10].s64 + 11936;
	// 82699C68: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699C6C: 390BF6B4  addi r8, r11, -0x94c
	ctx.r[8].s64 = ctx.r[11].s64 + -2380;
	// 82699C70: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 82699C74: 388A9CDC  addi r4, r10, -0x6324
	ctx.r[4].s64 = ctx.r[10].s64 + -25380;
	// 82699C78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699C7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699C80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82699C84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699C88: 386A3200  addi r3, r10, 0x3200
	ctx.r[3].s64 = ctx.r[10].s64 + 12800;
	// 82699C8C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82699C90: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699C94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699C98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699C9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699CA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699CA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699CA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699CAC: 4BDCD175  bl 0x82466e20
	ctx.lr = 0x82699CB0;
	sub_82466E20(ctx, base);
	// 82699CB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699CB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699CB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699CBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699CC0 size=108
    let mut pc: u32 = 0x82699CC0;
    'dispatch: loop {
        match pc {
            0x82699CC0 => {
    //   block [0x82699CC0..0x82699D2C)
	// 82699CC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699CC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699CC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699CCC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699CD0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699CD4: 38EBF6E4  addi r7, r11, -0x91c
	ctx.r[7].s64 = ctx.r[11].s64 + -2332;
	// 82699CD8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699CDC: 388A9CF4  addi r4, r10, -0x630c
	ctx.r[4].s64 = ctx.r[10].s64 + -25356;
	// 82699CE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699CE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699CE8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699CEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699CF0: 386A3230  addi r3, r10, 0x3230
	ctx.r[3].s64 = ctx.r[10].s64 + 12848;
	// 82699CF4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699CF8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699CFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699D00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699D04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699D08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699D0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699D10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699D14: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699D18: 4BDCD109  bl 0x82466e20
	ctx.lr = 0x82699D1C;
	sub_82466E20(ctx, base);
	// 82699D1C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699D30 size=108
    let mut pc: u32 = 0x82699D30;
    'dispatch: loop {
        match pc {
            0x82699D30 => {
    //   block [0x82699D30..0x82699D9C)
	// 82699D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699D38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699D3C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699D40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699D44: 38EBF718  addi r7, r11, -0x8e8
	ctx.r[7].s64 = ctx.r[11].s64 + -2280;
	// 82699D48: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82699D4C: 388AB148  addi r4, r10, -0x4eb8
	ctx.r[4].s64 = ctx.r[10].s64 + -20152;
	// 82699D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699D54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699D58: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699D5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699D60: 386A3260  addi r3, r10, 0x3260
	ctx.r[3].s64 = ctx.r[10].s64 + 12896;
	// 82699D64: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699D68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699D6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699D70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699D74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699D78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699D7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699D80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699D84: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699D88: 4BDCD099  bl 0x82466e20
	ctx.lr = 0x82699D8C;
	sub_82466E20(ctx, base);
	// 82699D8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699D90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699D94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699DA0 size=108
    let mut pc: u32 = 0x82699DA0;
    'dispatch: loop {
        match pc {
            0x82699DA0 => {
    //   block [0x82699DA0..0x82699E0C)
	// 82699DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699DA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699DAC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699DB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699DB4: 38EBF778  addi r7, r11, -0x888
	ctx.r[7].s64 = ctx.r[11].s64 + -2184;
	// 82699DB8: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 82699DBC: 388AA9FC  addi r4, r10, -0x5604
	ctx.r[4].s64 = ctx.r[10].s64 + -22020;
	// 82699DC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699DC4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699DC8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699DCC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699DD0: 386A3290  addi r3, r10, 0x3290
	ctx.r[3].s64 = ctx.r[10].s64 + 12944;
	// 82699DD4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699DD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699DDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699DE0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699DE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699DE8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699DEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699DF0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699DF4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699DF8: 4BDCD029  bl 0x82466e20
	ctx.lr = 0x82699DFC;
	sub_82466E20(ctx, base);
	// 82699DFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699E00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699E04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699E10 size=108
    let mut pc: u32 = 0x82699E10;
    'dispatch: loop {
        match pc {
            0x82699E10 => {
    //   block [0x82699E10..0x82699E7C)
	// 82699E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699E18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699E1C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699E20: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699E24: 38EBF7A8  addi r7, r11, -0x858
	ctx.r[7].s64 = ctx.r[11].s64 + -2136;
	// 82699E28: 3900000C  li r8, 0xc
	ctx.r[8].s64 = 12;
	// 82699E2C: 388AA978  addi r4, r10, -0x5688
	ctx.r[4].s64 = ctx.r[10].s64 + -22152;
	// 82699E30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699E34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699E38: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699E3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699E40: 386A32C0  addi r3, r10, 0x32c0
	ctx.r[3].s64 = ctx.r[10].s64 + 12992;
	// 82699E44: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699E48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699E4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699E50: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699E54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699E58: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699E5C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699E60: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699E64: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699E68: 4BDCCFB9  bl 0x82466e20
	ctx.lr = 0x82699E6C;
	sub_82466E20(ctx, base);
	// 82699E6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699E80 size=108
    let mut pc: u32 = 0x82699E80;
    'dispatch: loop {
        match pc {
            0x82699E80 => {
    //   block [0x82699E80..0x82699EEC)
	// 82699E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699E88: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699E8C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699E90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699E94: 38EBF8C8  addi r7, r11, -0x738
	ctx.r[7].s64 = ctx.r[11].s64 + -1848;
	// 82699E98: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699E9C: 388AA044  addi r4, r10, -0x5fbc
	ctx.r[4].s64 = ctx.r[10].s64 + -24508;
	// 82699EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699EA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699EA8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699EAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699EB0: 386A32F0  addi r3, r10, 0x32f0
	ctx.r[3].s64 = ctx.r[10].s64 + 13040;
	// 82699EB4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699EBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699EC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699EC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699EC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699ECC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699ED0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699ED4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699ED8: 4BDCCF49  bl 0x82466e20
	ctx.lr = 0x82699EDC;
	sub_82466E20(ctx, base);
	// 82699EDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699EF0 size=108
    let mut pc: u32 = 0x82699EF0;
    'dispatch: loop {
        match pc {
            0x82699EF0 => {
    //   block [0x82699EF0..0x82699F5C)
	// 82699EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699EFC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699F00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699F04: 38EBF8E0  addi r7, r11, -0x720
	ctx.r[7].s64 = ctx.r[11].s64 + -1824;
	// 82699F08: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699F0C: 388AAF3C  addi r4, r10, -0x50c4
	ctx.r[4].s64 = ctx.r[10].s64 + -20676;
	// 82699F10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699F14: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699F18: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699F1C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699F20: 386A3320  addi r3, r10, 0x3320
	ctx.r[3].s64 = ctx.r[10].s64 + 13088;
	// 82699F24: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699F28: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699F2C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699F30: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699F38: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699F3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699F40: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699F44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699F48: 4BDCCED9  bl 0x82466e20
	ctx.lr = 0x82699F4C;
	sub_82466E20(ctx, base);
	// 82699F4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699F50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699F54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699F60 size=108
    let mut pc: u32 = 0x82699F60;
    'dispatch: loop {
        match pc {
            0x82699F60 => {
    //   block [0x82699F60..0x82699FCC)
	// 82699F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699F6C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699F70: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699F74: 38EBF8F8  addi r7, r11, -0x708
	ctx.r[7].s64 = ctx.r[11].s64 + -1800;
	// 82699F78: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699F7C: 388AA0C0  addi r4, r10, -0x5f40
	ctx.r[4].s64 = ctx.r[10].s64 + -24384;
	// 82699F80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699F84: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699F88: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699F8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82699F90: 386A3350  addi r3, r10, 0x3350
	ctx.r[3].s64 = ctx.r[10].s64 + 13136;
	// 82699F94: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82699F98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82699F9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82699FA0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82699FA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82699FA8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82699FAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82699FB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82699FB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82699FB8: 4BDCCE69  bl 0x82466e20
	ctx.lr = 0x82699FBC;
	sub_82466E20(ctx, base);
	// 82699FBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82699FC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82699FC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82699FC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82699FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82699FD0 size=108
    let mut pc: u32 = 0x82699FD0;
    'dispatch: loop {
        match pc {
            0x82699FD0 => {
    //   block [0x82699FD0..0x8269A03C)
	// 82699FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82699FD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82699FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82699FDC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82699FE0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82699FE4: 38EBF910  addi r7, r11, -0x6f0
	ctx.r[7].s64 = ctx.r[11].s64 + -1776;
	// 82699FE8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82699FEC: 388A9C24  addi r4, r10, -0x63dc
	ctx.r[4].s64 = ctx.r[10].s64 + -25564;
	// 82699FF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82699FF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 82699FF8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 82699FFC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A000: 386A3380  addi r3, r10, 0x3380
	ctx.r[3].s64 = ctx.r[10].s64 + 13184;
	// 8269A004: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A008: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A00C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A010: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A018: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A01C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A020: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A024: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A028: 4BDCCDF9  bl 0x82466e20
	ctx.lr = 0x8269A02C;
	sub_82466E20(ctx, base);
	// 8269A02C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A038: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A040 size=108
    let mut pc: u32 = 0x8269A040;
    'dispatch: loop {
        match pc {
            0x8269A040 => {
    //   block [0x8269A040..0x8269A0AC)
	// 8269A040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A04C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A050: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A054: 38EBF928  addi r7, r11, -0x6d8
	ctx.r[7].s64 = ctx.r[11].s64 + -1752;
	// 8269A058: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A05C: 388AA684  addi r4, r10, -0x597c
	ctx.r[4].s64 = ctx.r[10].s64 + -22908;
	// 8269A060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A068: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A070: 386A33B0  addi r3, r10, 0x33b0
	ctx.r[3].s64 = ctx.r[10].s64 + 13232;
	// 8269A074: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A084: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A094: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A098: 4BDCCD89  bl 0x82466e20
	ctx.lr = 0x8269A09C;
	sub_82466E20(ctx, base);
	// 8269A09C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A0A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A0A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A0A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A0B0 size=108
    let mut pc: u32 = 0x8269A0B0;
    'dispatch: loop {
        match pc {
            0x8269A0B0 => {
    //   block [0x8269A0B0..0x8269A11C)
	// 8269A0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A0B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A0BC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A0C0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A0C4: 38EBF940  addi r7, r11, -0x6c0
	ctx.r[7].s64 = ctx.r[11].s64 + -1728;
	// 8269A0C8: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A0CC: 388AA26C  addi r4, r10, -0x5d94
	ctx.r[4].s64 = ctx.r[10].s64 + -23956;
	// 8269A0D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A0D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A0D8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A0DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A0E0: 386A33E0  addi r3, r10, 0x33e0
	ctx.r[3].s64 = ctx.r[10].s64 + 13280;
	// 8269A0E4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A0E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A0EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A0F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A0F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A0F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A0FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A100: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A104: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A108: 4BDCCD19  bl 0x82466e20
	ctx.lr = 0x8269A10C;
	sub_82466E20(ctx, base);
	// 8269A10C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A110: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A114: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A120 size=108
    let mut pc: u32 = 0x8269A120;
    'dispatch: loop {
        match pc {
            0x8269A120 => {
    //   block [0x8269A120..0x8269A18C)
	// 8269A120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A12C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A130: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A134: 38EBF958  addi r7, r11, -0x6a8
	ctx.r[7].s64 = ctx.r[11].s64 + -1704;
	// 8269A138: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8269A13C: 388AB380  addi r4, r10, -0x4c80
	ctx.r[4].s64 = ctx.r[10].s64 + -19584;
	// 8269A140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A144: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A148: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A14C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A150: 386A3410  addi r3, r10, 0x3410
	ctx.r[3].s64 = ctx.r[10].s64 + 13328;
	// 8269A154: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A158: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A15C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A160: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A164: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A168: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A16C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A170: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A174: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A178: 4BDCCCA9  bl 0x82466e20
	ctx.lr = 0x8269A17C;
	sub_82466E20(ctx, base);
	// 8269A17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A190 size=108
    let mut pc: u32 = 0x8269A190;
    'dispatch: loop {
        match pc {
            0x8269A190 => {
    //   block [0x8269A190..0x8269A1FC)
	// 8269A190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A19C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A1A0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A1A4: 38EBF9E8  addi r7, r11, -0x618
	ctx.r[7].s64 = ctx.r[11].s64 + -1560;
	// 8269A1A8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8269A1AC: 388AA228  addi r4, r10, -0x5dd8
	ctx.r[4].s64 = ctx.r[10].s64 + -24024;
	// 8269A1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A1B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A1B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A1C0: 386A3440  addi r3, r10, 0x3440
	ctx.r[3].s64 = ctx.r[10].s64 + 13376;
	// 8269A1C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A1E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A1E8: 4BDCCC39  bl 0x82466e20
	ctx.lr = 0x8269A1EC;
	sub_82466E20(ctx, base);
	// 8269A1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A200 size=108
    let mut pc: u32 = 0x8269A200;
    'dispatch: loop {
        match pc {
            0x8269A200 => {
    //   block [0x8269A200..0x8269A26C)
	// 8269A200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A20C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A210: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A214: 38EBFAA8  addi r7, r11, -0x558
	ctx.r[7].s64 = ctx.r[11].s64 + -1368;
	// 8269A218: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8269A21C: 388AA000  addi r4, r10, -0x6000
	ctx.r[4].s64 = ctx.r[10].s64 + -24576;
	// 8269A220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A228: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A22C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A230: 386A3470  addi r3, r10, 0x3470
	ctx.r[3].s64 = ctx.r[10].s64 + 13424;
	// 8269A234: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A238: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A23C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A240: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A244: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A248: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A24C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A250: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A254: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A258: 4BDCCBC9  bl 0x82466e20
	ctx.lr = 0x8269A25C;
	sub_82466E20(ctx, base);
	// 8269A25C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A270 size=108
    let mut pc: u32 = 0x8269A270;
    'dispatch: loop {
        match pc {
            0x8269A270 => {
    //   block [0x8269A270..0x8269A2DC)
	// 8269A270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A27C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A280: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A284: 38EBFB80  addi r7, r11, -0x480
	ctx.r[7].s64 = ctx.r[11].s64 + -1152;
	// 8269A288: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8269A28C: 388A9F00  addi r4, r10, -0x6100
	ctx.r[4].s64 = ctx.r[10].s64 + -24832;
	// 8269A290: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A298: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A29C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A2A0: 386A34A0  addi r3, r10, 0x34a0
	ctx.r[3].s64 = ctx.r[10].s64 + 13472;
	// 8269A2A4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A2A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A2AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A2B0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A2B8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A2BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A2C0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A2C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A2C8: 4BDCCB59  bl 0x82466e20
	ctx.lr = 0x8269A2CC;
	sub_82466E20(ctx, base);
	// 8269A2CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A2D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A2D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A2D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A2E0 size=108
    let mut pc: u32 = 0x8269A2E0;
    'dispatch: loop {
        match pc {
            0x8269A2E0 => {
    //   block [0x8269A2E0..0x8269A34C)
	// 8269A2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A2EC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A2F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A2F4: 38EBFC40  addi r7, r11, -0x3c0
	ctx.r[7].s64 = ctx.r[11].s64 + -960;
	// 8269A2F8: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8269A2FC: 388AADE0  addi r4, r10, -0x5220
	ctx.r[4].s64 = ctx.r[10].s64 + -21024;
	// 8269A300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A304: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A308: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A30C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A310: 386A34D0  addi r3, r10, 0x34d0
	ctx.r[3].s64 = ctx.r[10].s64 + 13520;
	// 8269A314: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A318: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A31C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A320: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A324: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A328: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A32C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A330: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A334: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A338: 4BDCCAE9  bl 0x82466e20
	ctx.lr = 0x8269A33C;
	sub_82466E20(ctx, base);
	// 8269A33C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A350 size=112
    let mut pc: u32 = 0x8269A350;
    'dispatch: loop {
        match pc {
            0x8269A350 => {
    //   block [0x8269A350..0x8269A3C0)
	// 8269A350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A35C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269A360: 3900000D  li r8, 0xd
	ctx.r[8].s64 = 13;
	// 8269A364: 38EAFCE8  addi r7, r10, -0x318
	ctx.r[7].s64 = ctx.r[10].s64 + -792;
	// 8269A368: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A36C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269A370: 388AA5FC  addi r4, r10, -0x5a04
	ctx.r[4].s64 = ctx.r[10].s64 + -23044;
	// 8269A374: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A378: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A37C: 396B98F8  addi r11, r11, -0x6708
	ctx.r[11].s64 = ctx.r[11].s64 + -26376;
	// 8269A380: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A384: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A388: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A38C: 386A3500  addi r3, r10, 0x3500
	ctx.r[3].s64 = ctx.r[10].s64 + 13568;
	// 8269A390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A394: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269A398: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A39C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269A3A0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A3A4: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A3A8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A3AC: 4BDCCA75  bl 0x82466e20
	ctx.lr = 0x8269A3B0;
	sub_82466E20(ctx, base);
	// 8269A3B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A3C0 size=112
    let mut pc: u32 = 0x8269A3C0;
    'dispatch: loop {
        match pc {
            0x8269A3C0 => {
    //   block [0x8269A3C0..0x8269A430)
	// 8269A3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A3CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A3D0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A3D4: 38AA2EA0  addi r5, r10, 0x2ea0
	ctx.r[5].s64 = ctx.r[10].s64 + 11936;
	// 8269A3D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A3DC: 390BFE20  addi r8, r11, -0x1e0
	ctx.r[8].s64 = ctx.r[11].s64 + -480;
	// 8269A3E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269A3E4: 388AA3A4  addi r4, r10, -0x5c5c
	ctx.r[4].s64 = ctx.r[10].s64 + -23644;
	// 8269A3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A3EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A3F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269A3F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A3F8: 386A3530  addi r3, r10, 0x3530
	ctx.r[3].s64 = ctx.r[10].s64 + 13616;
	// 8269A3FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269A400: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A404: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A408: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A40C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A410: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A414: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A418: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A41C: 4BDCCA05  bl 0x82466e20
	ctx.lr = 0x8269A420;
	sub_82466E20(ctx, base);
	// 8269A420: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A42C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A430 size=108
    let mut pc: u32 = 0x8269A430;
    'dispatch: loop {
        match pc {
            0x8269A430 => {
    //   block [0x8269A430..0x8269A49C)
	// 8269A430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A43C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A440: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A444: 38EBFE50  addi r7, r11, -0x1b0
	ctx.r[7].s64 = ctx.r[11].s64 + -432;
	// 8269A448: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269A44C: 388AAD28  addi r4, r10, -0x52d8
	ctx.r[4].s64 = ctx.r[10].s64 + -21208;
	// 8269A450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A454: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A458: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A45C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A460: 386A3560  addi r3, r10, 0x3560
	ctx.r[3].s64 = ctx.r[10].s64 + 13664;
	// 8269A464: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A468: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A46C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A470: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A478: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A47C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A480: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A484: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A488: 4BDCC999  bl 0x82466e20
	ctx.lr = 0x8269A48C;
	sub_82466E20(ctx, base);
	// 8269A48C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A498: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A4A0 size=108
    let mut pc: u32 = 0x8269A4A0;
    'dispatch: loop {
        match pc {
            0x8269A4A0 => {
    //   block [0x8269A4A0..0x8269A50C)
	// 8269A4A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A4A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A4A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A4AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A4B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A4B4: 38EBFEB0  addi r7, r11, -0x150
	ctx.r[7].s64 = ctx.r[11].s64 + -336;
	// 8269A4B8: 3900000B  li r8, 0xb
	ctx.r[8].s64 = 11;
	// 8269A4BC: 388A9070  addi r4, r10, -0x6f90
	ctx.r[4].s64 = ctx.r[10].s64 + -28560;
	// 8269A4C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A4C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A4C8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A4CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A4D0: 386A3590  addi r3, r10, 0x3590
	ctx.r[3].s64 = ctx.r[10].s64 + 13712;
	// 8269A4D4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A4D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A4DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A4E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A4E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A4EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A4F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A4F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A4F8: 4BDCC929  bl 0x82466e20
	ctx.lr = 0x8269A4FC;
	sub_82466E20(ctx, base);
	// 8269A4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A510 size=108
    let mut pc: u32 = 0x8269A510;
    'dispatch: loop {
        match pc {
            0x8269A510 => {
    //   block [0x8269A510..0x8269A57C)
	// 8269A510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A518: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A51C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A520: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A524: 38EBFFB8  addi r7, r11, -0x48
	ctx.r[7].s64 = ctx.r[11].s64 + -72;
	// 8269A528: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 8269A52C: 388A8E98  addi r4, r10, -0x7168
	ctx.r[4].s64 = ctx.r[10].s64 + -29032;
	// 8269A530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A534: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A538: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A53C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A540: 386A35C0  addi r3, r10, 0x35c0
	ctx.r[3].s64 = ctx.r[10].s64 + 13760;
	// 8269A544: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A548: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A54C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A554: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A564: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A568: 4BDCC8B9  bl 0x82466e20
	ctx.lr = 0x8269A56C;
	sub_82466E20(ctx, base);
	// 8269A56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A578: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A580 size=108
    let mut pc: u32 = 0x8269A580;
    'dispatch: loop {
        match pc {
            0x8269A580 => {
    //   block [0x8269A580..0x8269A5EC)
	// 8269A580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A58C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A590: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A594: 38EB0090  addi r7, r11, 0x90
	ctx.r[7].s64 = ctx.r[11].s64 + 144;
	// 8269A598: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269A59C: 388A8CF0  addi r4, r10, -0x7310
	ctx.r[4].s64 = ctx.r[10].s64 + -29456;
	// 8269A5A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A5A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A5A8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A5AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A5B0: 386A35F0  addi r3, r10, 0x35f0
	ctx.r[3].s64 = ctx.r[10].s64 + 13808;
	// 8269A5B4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A5B8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A5BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A5C0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A5C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A5C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A5CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A5D0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A5D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A5D8: 4BDCC849  bl 0x82466e20
	ctx.lr = 0x8269A5DC;
	sub_82466E20(ctx, base);
	// 8269A5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A5E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A5E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A5E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A5F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A5F0 size=108
    let mut pc: u32 = 0x8269A5F0;
    'dispatch: loop {
        match pc {
            0x8269A5F0 => {
    //   block [0x8269A5F0..0x8269A65C)
	// 8269A5F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A5F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A5F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A5FC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A600: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 8269A604: 38EB00D8  addi r7, r11, 0xd8
	ctx.r[7].s64 = ctx.r[11].s64 + 216;
	// 8269A608: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A60C: 388A8D14  addi r4, r10, -0x72ec
	ctx.r[4].s64 = ctx.r[10].s64 + -29420;
	// 8269A610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A614: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A618: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A61C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A620: 386A3620  addi r3, r10, 0x3620
	ctx.r[3].s64 = ctx.r[10].s64 + 13856;
	// 8269A624: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A628: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A62C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A630: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A634: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A638: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A63C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A640: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A644: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A648: 4BDCC7D9  bl 0x82466e20
	ctx.lr = 0x8269A64C;
	sub_82466E20(ctx, base);
	// 8269A64C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A650: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A654: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A660 size=108
    let mut pc: u32 = 0x8269A660;
    'dispatch: loop {
        match pc {
            0x8269A660 => {
    //   block [0x8269A660..0x8269A6CC)
	// 8269A660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A66C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A670: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A674: 38EB00F0  addi r7, r11, 0xf0
	ctx.r[7].s64 = ctx.r[11].s64 + 240;
	// 8269A678: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269A67C: 388AB3B4  addi r4, r10, -0x4c4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19532;
	// 8269A680: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A684: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A688: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A68C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A690: 386A3650  addi r3, r10, 0x3650
	ctx.r[3].s64 = ctx.r[10].s64 + 13904;
	// 8269A694: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A698: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A69C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A6A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A6A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A6A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A6B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A6B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A6B8: 4BDCC769  bl 0x82466e20
	ctx.lr = 0x8269A6BC;
	sub_82466E20(ctx, base);
	// 8269A6BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A6C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A6C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A6C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A6D0 size=108
    let mut pc: u32 = 0x8269A6D0;
    'dispatch: loop {
        match pc {
            0x8269A6D0 => {
    //   block [0x8269A6D0..0x8269A73C)
	// 8269A6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A6D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A6DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A6E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A6E4: 38EB0150  addi r7, r11, 0x150
	ctx.r[7].s64 = ctx.r[11].s64 + 336;
	// 8269A6E8: 39000008  li r8, 8
	ctx.r[8].s64 = 8;
	// 8269A6EC: 388AB3C0  addi r4, r10, -0x4c40
	ctx.r[4].s64 = ctx.r[10].s64 + -19520;
	// 8269A6F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A6F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A6F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269A6FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A700: 386A3680  addi r3, r10, 0x3680
	ctx.r[3].s64 = ctx.r[10].s64 + 13952;
	// 8269A704: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269A708: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A70C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A710: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A714: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A718: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A71C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A720: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A724: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A728: 4BDCC6F9  bl 0x82466e20
	ctx.lr = 0x8269A72C;
	sub_82466E20(ctx, base);
	// 8269A72C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A730: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A734: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A740 size=116
    let mut pc: u32 = 0x8269A740;
    'dispatch: loop {
        match pc {
            0x8269A740 => {
    //   block [0x8269A740..0x8269A7B4)
	// 8269A740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A744: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A748: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A74C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A750: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269A754: 390B0210  addi r8, r11, 0x210
	ctx.r[8].s64 = ctx.r[11].s64 + 528;
	// 8269A758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A75C: 392A9974  addi r9, r10, -0x668c
	ctx.r[9].s64 = ctx.r[10].s64 + -26252;
	// 8269A760: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A764: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8269A768: 38AA3650  addi r5, r10, 0x3650
	ctx.r[5].s64 = ctx.r[10].s64 + 13904;
	// 8269A76C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269A770: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A774: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A778: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A77C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A780: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A784: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269A788: 388AB3E0  addi r4, r10, -0x4c20
	ctx.r[4].s64 = ctx.r[10].s64 + -19488;
	// 8269A78C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269A790: 386B36B0  addi r3, r11, 0x36b0
	ctx.r[3].s64 = ctx.r[11].s64 + 14000;
	// 8269A794: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269A798: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A79C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A7A0: 4BDCC681  bl 0x82466e20
	ctx.lr = 0x8269A7A4;
	sub_82466E20(ctx, base);
	// 8269A7A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A7A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A7AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A7B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A7B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A7B8 size=112
    let mut pc: u32 = 0x8269A7B8;
    'dispatch: loop {
        match pc {
            0x8269A7B8 => {
    //   block [0x8269A7B8..0x8269A828)
	// 8269A7B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A7BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A7C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A7C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A7C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A7CC: 38AA5990  addi r5, r10, 0x5990
	ctx.r[5].s64 = ctx.r[10].s64 + 22928;
	// 8269A7D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A7D4: 390B02A0  addi r8, r11, 0x2a0
	ctx.r[8].s64 = ctx.r[11].s64 + 672;
	// 8269A7D8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269A7DC: 388AB3F0  addi r4, r10, -0x4c10
	ctx.r[4].s64 = ctx.r[10].s64 + -19472;
	// 8269A7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A7E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A7E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269A7EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A7F0: 386A36E0  addi r3, r10, 0x36e0
	ctx.r[3].s64 = ctx.r[10].s64 + 14048;
	// 8269A7F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269A7F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A800: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A804: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A808: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A810: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A814: 4BDCC60D  bl 0x82466e20
	ctx.lr = 0x8269A818;
	sub_82466E20(ctx, base);
	// 8269A818: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A81C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A820: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A828 size=96
    let mut pc: u32 = 0x8269A828;
    'dispatch: loop {
        match pc {
            0x8269A828 => {
    //   block [0x8269A828..0x8269A888)
	// 8269A828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A82C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A830: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A834: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A83C: 388AB40C  addi r4, r10, -0x4bf4
	ctx.r[4].s64 = ctx.r[10].s64 + -19444;
	// 8269A840: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A844: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A848: 386A3710  addi r3, r10, 0x3710
	ctx.r[3].s64 = ctx.r[10].s64 + 14096;
	// 8269A84C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A850: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A854: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A858: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A85C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A860: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A864: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A868: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269A86C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A870: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269A874: 4BDCC5AD  bl 0x82466e20
	ctx.lr = 0x8269A878;
	sub_82466E20(ctx, base);
	// 8269A878: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A87C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A880: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269A888 size=24
    let mut pc: u32 = 0x8269A888;
    'dispatch: loop {
        match pc {
            0x8269A888 => {
    //   block [0x8269A888..0x8269A8A0)
	// 8269A888: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A88C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269A890: 394A6B58  addi r10, r10, 0x6b58
	ctx.r[10].s64 = ctx.r[10].s64 + 27480;
	// 8269A894: 816B0300  lwz r11, 0x300(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(768 as u32) ) } as u64;
	// 8269A898: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269A89C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A8A0 size=116
    let mut pc: u32 = 0x8269A8A0;
    'dispatch: loop {
        match pc {
            0x8269A8A0 => {
    //   block [0x8269A8A0..0x8269A914)
	// 8269A8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A8A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A8AC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269A8B0: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269A8B4: 390B6B58  addi r8, r11, 0x6b58
	ctx.r[8].s64 = ctx.r[11].s64 + 27480;
	// 8269A8B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A8BC: 392A99C0  addi r9, r10, -0x6640
	ctx.r[9].s64 = ctx.r[10].s64 + -26176;
	// 8269A8C0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A8C4: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269A8C8: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269A8CC: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269A8D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A8D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A8D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A8DC: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269A8E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A8E4: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269A8E8: 388AB42C  addi r4, r10, -0x4bd4
	ctx.r[4].s64 = ctx.r[10].s64 + -19412;
	// 8269A8EC: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269A8F0: 386B3740  addi r3, r11, 0x3740
	ctx.r[3].s64 = ctx.r[11].s64 + 14144;
	// 8269A8F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269A8F8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A8FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A900: 4BDCC521  bl 0x82466e20
	ctx.lr = 0x8269A904;
	sub_82466E20(ctx, base);
	// 8269A904: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A908: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A90C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A918 size=104
    let mut pc: u32 = 0x8269A918;
    'dispatch: loop {
        match pc {
            0x8269A918 => {
    //   block [0x8269A918..0x8269A980)
	// 8269A918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A91C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A920: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A924: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269A928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A92C: 392A99EC  addi r9, r10, -0x6614
	ctx.r[9].s64 = ctx.r[10].s64 + -26132;
	// 8269A930: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A934: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269A938: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269A93C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A940: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A944: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A948: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A94C: 388AB440  addi r4, r10, -0x4bc0
	ctx.r[4].s64 = ctx.r[10].s64 + -19392;
	// 8269A950: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A958: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269A95C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A960: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269A964: 386A3770  addi r3, r10, 0x3770
	ctx.r[3].s64 = ctx.r[10].s64 + 14192;
	// 8269A968: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269A96C: 4BDCC4B5  bl 0x82466e20
	ctx.lr = 0x8269A970;
	sub_82466E20(ctx, base);
	// 8269A970: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A97C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A980 size=96
    let mut pc: u32 = 0x8269A980;
    'dispatch: loop {
        match pc {
            0x8269A980 => {
    //   block [0x8269A980..0x8269A9E0)
	// 8269A980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A988: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A98C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A994: 388AB454  addi r4, r10, -0x4bac
	ctx.r[4].s64 = ctx.r[10].s64 + -19372;
	// 8269A998: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A99C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269A9A0: 386A37A0  addi r3, r10, 0x37a0
	ctx.r[3].s64 = ctx.r[10].s64 + 14240;
	// 8269A9A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269A9A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269A9AC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269A9B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269A9B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269A9B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269A9BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269A9C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269A9C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269A9C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269A9CC: 4BDCC455  bl 0x82466e20
	ctx.lr = 0x8269A9D0;
	sub_82466E20(ctx, base);
	// 8269A9D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269A9D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269A9D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269A9DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269A9E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269A9E0 size=96
    let mut pc: u32 = 0x8269A9E0;
    'dispatch: loop {
        match pc {
            0x8269A9E0 => {
    //   block [0x8269A9E0..0x8269AA40)
	// 8269A9E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269A9E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269A9E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269A9EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269A9F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269A9F4: 388AB46C  addi r4, r10, -0x4b94
	ctx.r[4].s64 = ctx.r[10].s64 + -19348;
	// 8269A9F8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269A9FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AA00: 386A37D0  addi r3, r10, 0x37d0
	ctx.r[3].s64 = ctx.r[10].s64 + 14288;
	// 8269AA04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AA08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AA0C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269AA10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AA14: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AA18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AA1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AA20: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269AA24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269AA28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269AA2C: 4BDCC3F5  bl 0x82466e20
	ctx.lr = 0x8269AA30;
	sub_82466E20(ctx, base);
	// 8269AA30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AA34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AA38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AA40 size=100
    let mut pc: u32 = 0x8269AA40;
    'dispatch: loop {
        match pc {
            0x8269AA40 => {
    //   block [0x8269AA40..0x8269AAA4)
	// 8269AA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AA4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AA54: 38AA3770  addi r5, r10, 0x3770
	ctx.r[5].s64 = ctx.r[10].s64 + 14192;
	// 8269AA58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AA5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AA60: 388AB488  addi r4, r10, -0x4b78
	ctx.r[4].s64 = ctx.r[10].s64 + -19320;
	// 8269AA64: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AA68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AA6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AA70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AA74: 386A3800  addi r3, r10, 0x3800
	ctx.r[3].s64 = ctx.r[10].s64 + 14336;
	// 8269AA78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AA7C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AA80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269AA84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AA88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269AA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AA90: 4BDCC391  bl 0x82466e20
	ctx.lr = 0x8269AA94;
	sub_82466E20(ctx, base);
	// 8269AA94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AA98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AA9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AAA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AAA8 size=112
    let mut pc: u32 = 0x8269AAA8;
    'dispatch: loop {
        match pc {
            0x8269AAA8 => {
    //   block [0x8269AAA8..0x8269AB18)
	// 8269AAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AAB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AAB4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AAB8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AABC: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 8269AAC0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AAC4: 390B0308  addi r8, r11, 0x308
	ctx.r[8].s64 = ctx.r[11].s64 + 776;
	// 8269AAC8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269AACC: 388AB4A4  addi r4, r10, -0x4b5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19292;
	// 8269AAD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AAD4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AAD8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AAE0: 386A3830  addi r3, r10, 0x3830
	ctx.r[3].s64 = ctx.r[10].s64 + 14384;
	// 8269AAE4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AAE8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AAEC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AAF0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AAF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AAF8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AB00: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AB04: 4BDCC31D  bl 0x82466e20
	ctx.lr = 0x8269AB08;
	sub_82466E20(ctx, base);
	// 8269AB08: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AB0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AB10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AB18 size=112
    let mut pc: u32 = 0x8269AB18;
    'dispatch: loop {
        match pc {
            0x8269AB18 => {
    //   block [0x8269AB18..0x8269AB88)
	// 8269AB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AB20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AB24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AB28: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AB2C: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 8269AB30: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AB34: 390B0350  addi r8, r11, 0x350
	ctx.r[8].s64 = ctx.r[11].s64 + 848;
	// 8269AB38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269AB3C: 388AB4B4  addi r4, r10, -0x4b4c
	ctx.r[4].s64 = ctx.r[10].s64 + -19276;
	// 8269AB40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AB44: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AB48: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AB4C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AB50: 386A3860  addi r3, r10, 0x3860
	ctx.r[3].s64 = ctx.r[10].s64 + 14432;
	// 8269AB54: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AB58: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AB5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AB60: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AB64: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AB68: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AB70: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AB74: 4BDCC2AD  bl 0x82466e20
	ctx.lr = 0x8269AB78;
	sub_82466E20(ctx, base);
	// 8269AB78: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AB84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AB88 size=100
    let mut pc: u32 = 0x8269AB88;
    'dispatch: loop {
        match pc {
            0x8269AB88 => {
    //   block [0x8269AB88..0x8269ABEC)
	// 8269AB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AB8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AB90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AB94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AB9C: 38AA3740  addi r5, r10, 0x3740
	ctx.r[5].s64 = ctx.r[10].s64 + 14144;
	// 8269ABA0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ABA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269ABA8: 388AB4CC  addi r4, r10, -0x4b34
	ctx.r[4].s64 = ctx.r[10].s64 + -19252;
	// 8269ABAC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ABB0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269ABB4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269ABB8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269ABBC: 386A3890  addi r3, r10, 0x3890
	ctx.r[3].s64 = ctx.r[10].s64 + 14480;
	// 8269ABC0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269ABC4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269ABC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269ABCC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ABD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ABD4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269ABD8: 4BDCC249  bl 0x82466e20
	ctx.lr = 0x8269ABDC;
	sub_82466E20(ctx, base);
	// 8269ABDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269ABE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269ABE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269ABE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ABF0 size=112
    let mut pc: u32 = 0x8269ABF0;
    'dispatch: loop {
        match pc {
            0x8269ABF0 => {
    //   block [0x8269ABF0..0x8269AC60)
	// 8269ABF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ABF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ABF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ABFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AC00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AC04: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269AC08: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AC0C: 390B0368  addi r8, r11, 0x368
	ctx.r[8].s64 = ctx.r[11].s64 + 872;
	// 8269AC10: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269AC14: 388AB4E4  addi r4, r10, -0x4b1c
	ctx.r[4].s64 = ctx.r[10].s64 + -19228;
	// 8269AC18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AC1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AC20: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AC24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AC28: 386A38C0  addi r3, r10, 0x38c0
	ctx.r[3].s64 = ctx.r[10].s64 + 14528;
	// 8269AC2C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AC30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AC34: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AC38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AC3C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AC40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AC44: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AC48: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AC4C: 4BDCC1D5  bl 0x82466e20
	ctx.lr = 0x8269AC50;
	sub_82466E20(ctx, base);
	// 8269AC50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AC54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AC58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AC60 size=96
    let mut pc: u32 = 0x8269AC60;
    'dispatch: loop {
        match pc {
            0x8269AC60 => {
    //   block [0x8269AC60..0x8269ACC0)
	// 8269AC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AC6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AC70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AC74: 388AB4F0  addi r4, r10, -0x4b10
	ctx.r[4].s64 = ctx.r[10].s64 + -19216;
	// 8269AC78: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AC7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AC80: 386A38F0  addi r3, r10, 0x38f0
	ctx.r[3].s64 = ctx.r[10].s64 + 14576;
	// 8269AC84: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AC88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AC8C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269AC90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AC98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AC9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269ACA0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269ACA4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269ACA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269ACAC: 4BDCC175  bl 0x82466e20
	ctx.lr = 0x8269ACB0;
	sub_82466E20(ctx, base);
	// 8269ACB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269ACB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269ACB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269ACBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ACC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ACC0 size=112
    let mut pc: u32 = 0x8269ACC0;
    'dispatch: loop {
        match pc {
            0x8269ACC0 => {
    //   block [0x8269ACC0..0x8269AD30)
	// 8269ACC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ACC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ACC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ACCC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ACD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269ACD4: 38AA38F0  addi r5, r10, 0x38f0
	ctx.r[5].s64 = ctx.r[10].s64 + 14576;
	// 8269ACD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ACDC: 390B0398  addi r8, r11, 0x398
	ctx.r[8].s64 = ctx.r[11].s64 + 920;
	// 8269ACE0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269ACE4: 388AB504  addi r4, r10, -0x4afc
	ctx.r[4].s64 = ctx.r[10].s64 + -19196;
	// 8269ACE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269ACEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ACF0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269ACF4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269ACF8: 386A3920  addi r3, r10, 0x3920
	ctx.r[3].s64 = ctx.r[10].s64 + 14624;
	// 8269ACFC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AD00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AD04: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AD08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AD0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AD10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AD14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AD18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AD1C: 4BDCC105  bl 0x82466e20
	ctx.lr = 0x8269AD20;
	sub_82466E20(ctx, base);
	// 8269AD20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AD24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AD28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AD2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AD30 size=112
    let mut pc: u32 = 0x8269AD30;
    'dispatch: loop {
        match pc {
            0x8269AD30 => {
    //   block [0x8269AD30..0x8269ADA0)
	// 8269AD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AD34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AD3C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AD40: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AD44: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269AD48: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AD4C: 390B03B0  addi r8, r11, 0x3b0
	ctx.r[8].s64 = ctx.r[11].s64 + 944;
	// 8269AD50: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269AD54: 388AB51C  addi r4, r10, -0x4ae4
	ctx.r[4].s64 = ctx.r[10].s64 + -19172;
	// 8269AD58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AD5C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AD60: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AD64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AD68: 386A3950  addi r3, r10, 0x3950
	ctx.r[3].s64 = ctx.r[10].s64 + 14672;
	// 8269AD6C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AD70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AD74: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AD78: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AD7C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269AD80: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AD84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AD88: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AD8C: 4BDCC095  bl 0x82466e20
	ctx.lr = 0x8269AD90;
	sub_82466E20(ctx, base);
	// 8269AD90: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AD94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AD98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AD9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ADA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269ADA0 size=36
    let mut pc: u32 = 0x8269ADA0;
    'dispatch: loop {
        match pc {
            0x8269ADA0 => {
    //   block [0x8269ADA0..0x8269ADC4)
	// 8269ADA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269ADA4: 814B03D0  lwz r10, 0x3d0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(976 as u32) ) } as u64;
	// 8269ADA8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269ADAC: 396B6BA0  addi r11, r11, 0x6ba0
	ctx.r[11].s64 = ctx.r[11].s64 + 27552;
	// 8269ADB0: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8269ADB4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269ADB8: 814A03C8  lwz r10, 0x3c8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(968 as u32) ) } as u64;
	// 8269ADBC: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8269ADC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269ADC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269ADC8 size=108
    let mut pc: u32 = 0x8269ADC8;
    'dispatch: loop {
        match pc {
            0x8269ADC8 => {
    //   block [0x8269ADC8..0x8269AE34)
	// 8269ADC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269ADCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269ADD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269ADD4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269ADD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269ADDC: 38EB6BA0  addi r7, r11, 0x6ba0
	ctx.r[7].s64 = ctx.r[11].s64 + 27552;
	// 8269ADE0: 39000007  li r8, 7
	ctx.r[8].s64 = 7;
	// 8269ADE4: 388AB548  addi r4, r10, -0x4ab8
	ctx.r[4].s64 = ctx.r[10].s64 + -19128;
	// 8269ADE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269ADEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269ADF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269ADF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269ADF8: 386A3980  addi r3, r10, 0x3980
	ctx.r[3].s64 = ctx.r[10].s64 + 14720;
	// 8269ADFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269AE00: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AE08: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AE0C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AE10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AE14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AE18: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AE1C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269AE20: 4BDCC001  bl 0x82466e20
	ctx.lr = 0x8269AE24;
	sub_82466E20(ctx, base);
	// 8269AE24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AE28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AE2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269AE38 size=24
    let mut pc: u32 = 0x8269AE38;
    'dispatch: loop {
        match pc {
            0x8269AE38 => {
    //   block [0x8269AE38..0x8269AE50)
	// 8269AE38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AE3C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269AE40: 394A6C48  addi r10, r10, 0x6c48
	ctx.r[10].s64 = ctx.r[10].s64 + 27720;
	// 8269AE44: 816B03C8  lwz r11, 0x3c8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(968 as u32) ) } as u64;
	// 8269AE48: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8269AE4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AE50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AE50 size=116
    let mut pc: u32 = 0x8269AE50;
    'dispatch: loop {
        match pc {
            0x8269AE50 => {
    //   block [0x8269AE50..0x8269AEC4)
	// 8269AE50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AE54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AE58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AE5C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269AE60: 38E0000A  li r7, 0xa
	ctx.r[7].s64 = 10;
	// 8269AE64: 390A6C48  addi r8, r10, 0x6c48
	ctx.r[8].s64 = ctx.r[10].s64 + 27720;
	// 8269AE68: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AE6C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269AE70: 38AA3980  addi r5, r10, 0x3980
	ctx.r[5].s64 = ctx.r[10].s64 + 14720;
	// 8269AE74: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AE78: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269AE7C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AE80: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AE84: 388AB57C  addi r4, r10, -0x4a84
	ctx.r[4].s64 = ctx.r[10].s64 + -19076;
	// 8269AE88: 396B9A8C  addi r11, r11, -0x6574
	ctx.r[11].s64 = ctx.r[11].s64 + -25972;
	// 8269AE8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AE90: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AE94: 386A39B0  addi r3, r10, 0x39b0
	ctx.r[3].s64 = ctx.r[10].s64 + 14768;
	// 8269AE98: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269AE9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AEA0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269AEA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AEA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AEB0: 4BDCBF71  bl 0x82466e20
	ctx.lr = 0x8269AEB4;
	sub_82466E20(ctx, base);
	// 8269AEB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AEB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AEBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AEC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AEC8 size=112
    let mut pc: u32 = 0x8269AEC8;
    'dispatch: loop {
        match pc {
            0x8269AEC8 => {
    //   block [0x8269AEC8..0x8269AF38)
	// 8269AEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AED0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AED4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AED8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AEDC: 38AA3980  addi r5, r10, 0x3980
	ctx.r[5].s64 = ctx.r[10].s64 + 14720;
	// 8269AEE0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AEE4: 390B03D8  addi r8, r11, 0x3d8
	ctx.r[8].s64 = ctx.r[11].s64 + 984;
	// 8269AEE8: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269AEEC: 388AB5A4  addi r4, r10, -0x4a5c
	ctx.r[4].s64 = ctx.r[10].s64 + -19036;
	// 8269AEF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AEF4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AEF8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269AEFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AF00: 386A39E0  addi r3, r10, 0x39e0
	ctx.r[3].s64 = ctx.r[10].s64 + 14816;
	// 8269AF04: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269AF08: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269AF0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AF10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269AF14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AF18: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269AF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AF20: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269AF24: 4BDCBEFD  bl 0x82466e20
	ctx.lr = 0x8269AF28;
	sub_82466E20(ctx, base);
	// 8269AF28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AF2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AF30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AF34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AF38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269AF38 size=24
    let mut pc: u32 = 0x8269AF38;
    'dispatch: loop {
        match pc {
            0x8269AF38 => {
    //   block [0x8269AF38..0x8269AF50)
	// 8269AF38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AF3C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269AF40: 394A6D38  addi r10, r10, 0x6d38
	ctx.r[10].s64 = ctx.r[10].s64 + 27960;
	// 8269AF44: 816B0EB0  lwz r11, 0xeb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3760 as u32) ) } as u64;
	// 8269AF48: 916A00B0  stw r11, 0xb0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8269AF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AF50 size=116
    let mut pc: u32 = 0x8269AF50;
    'dispatch: loop {
        match pc {
            0x8269AF50 => {
    //   block [0x8269AF50..0x8269AFC4)
	// 8269AF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AF58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AF5C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269AF60: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AF64: 392B9A50  addi r9, r11, -0x65b0
	ctx.r[9].s64 = ctx.r[11].s64 + -26032;
	// 8269AF68: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269AF6C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AF70: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 8269AF74: 38C0000B  li r6, 0xb
	ctx.r[6].s64 = 11;
	// 8269AF78: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AF7C: 388AB5E4  addi r4, r10, -0x4a1c
	ctx.r[4].s64 = ctx.r[10].s64 + -18972;
	// 8269AF80: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269AF84: 396B6D38  addi r11, r11, 0x6d38
	ctx.r[11].s64 = ctx.r[11].s64 + 27960;
	// 8269AF88: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269AF8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AF90: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269AF94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269AF98: 386A3A10  addi r3, r10, 0x3a10
	ctx.r[3].s64 = ctx.r[10].s64 + 14864;
	// 8269AF9C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8269AFA0: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269AFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269AFA8: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269AFAC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269AFB0: 4BDCBE71  bl 0x82466e20
	ctx.lr = 0x8269AFB4;
	sub_82466E20(ctx, base);
	// 8269AFB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269AFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269AFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269AFC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269AFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269AFC8 size=108
    let mut pc: u32 = 0x8269AFC8;
    'dispatch: loop {
        match pc {
            0x8269AFC8 => {
    //   block [0x8269AFC8..0x8269B034)
	// 8269AFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269AFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269AFD0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269AFD4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269AFD8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269AFDC: 38EB0450  addi r7, r11, 0x450
	ctx.r[7].s64 = ctx.r[11].s64 + 1104;
	// 8269AFE0: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269AFE4: 388AB610  addi r4, r10, -0x49f0
	ctx.r[4].s64 = ctx.r[10].s64 + -18928;
	// 8269AFE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269AFEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269AFF0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269AFF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269AFF8: 386A3A40  addi r3, r10, 0x3a40
	ctx.r[3].s64 = ctx.r[10].s64 + 14912;
	// 8269AFFC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269B000: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B004: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B008: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B00C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B010: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B018: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B01C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269B020: 4BDCBE01  bl 0x82466e20
	ctx.lr = 0x8269B024;
	sub_82466E20(ctx, base);
	// 8269B024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B02C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B038 size=112
    let mut pc: u32 = 0x8269B038;
    'dispatch: loop {
        match pc {
            0x8269B038 => {
    //   block [0x8269B038..0x8269B0A8)
	// 8269B038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B03C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B040: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B044: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B048: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B04C: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269B050: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B054: 390B04B0  addi r8, r11, 0x4b0
	ctx.r[8].s64 = ctx.r[11].s64 + 1200;
	// 8269B058: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269B05C: 388AB628  addi r4, r10, -0x49d8
	ctx.r[4].s64 = ctx.r[10].s64 + -18904;
	// 8269B060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B064: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B068: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B06C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B070: 386A3A70  addi r3, r10, 0x3a70
	ctx.r[3].s64 = ctx.r[10].s64 + 14960;
	// 8269B074: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B078: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B07C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B080: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B084: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B088: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B08C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B090: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B094: 4BDCBD8D  bl 0x82466e20
	ctx.lr = 0x8269B098;
	sub_82466E20(ctx, base);
	// 8269B098: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B09C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B0A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B0A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B0A8 size=108
    let mut pc: u32 = 0x8269B0A8;
    'dispatch: loop {
        match pc {
            0x8269B0A8 => {
    //   block [0x8269B0A8..0x8269B114)
	// 8269B0A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B0AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B0B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B0B4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B0B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B0BC: 38EB0510  addi r7, r11, 0x510
	ctx.r[7].s64 = ctx.r[11].s64 + 1296;
	// 8269B0C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269B0C4: 388AB638  addi r4, r10, -0x49c8
	ctx.r[4].s64 = ctx.r[10].s64 + -18888;
	// 8269B0C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B0CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B0D0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269B0D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B0D8: 386A3AA0  addi r3, r10, 0x3aa0
	ctx.r[3].s64 = ctx.r[10].s64 + 15008;
	// 8269B0DC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269B0E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B0E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B0E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B0F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B0F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B0FC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269B100: 4BDCBD21  bl 0x82466e20
	ctx.lr = 0x8269B104;
	sub_82466E20(ctx, base);
	// 8269B104: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B110: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B118 size=108
    let mut pc: u32 = 0x8269B118;
    'dispatch: loop {
        match pc {
            0x8269B118 => {
    //   block [0x8269B118..0x8269B184)
	// 8269B118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B11C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B120: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B124: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B128: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B12C: 38EB0528  addi r7, r11, 0x528
	ctx.r[7].s64 = ctx.r[11].s64 + 1320;
	// 8269B130: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269B134: 388AB66C  addi r4, r10, -0x4994
	ctx.r[4].s64 = ctx.r[10].s64 + -18836;
	// 8269B138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B13C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B140: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269B144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B148: 386A3AD0  addi r3, r10, 0x3ad0
	ctx.r[3].s64 = ctx.r[10].s64 + 15056;
	// 8269B14C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269B150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B158: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B15C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B160: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B168: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B16C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269B170: 4BDCBCB1  bl 0x82466e20
	ctx.lr = 0x8269B174;
	sub_82466E20(ctx, base);
	// 8269B174: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B178: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B17C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B180: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269B188 size=24
    let mut pc: u32 = 0x8269B188;
    'dispatch: loop {
        match pc {
            0x8269B188 => {
    //   block [0x8269B188..0x8269B1A0)
	// 8269B188: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B18C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269B190: 394A6E40  addi r10, r10, 0x6e40
	ctx.r[10].s64 = ctx.r[10].s64 + 28224;
	// 8269B194: 816B0EB0  lwz r11, 0xeb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3760 as u32) ) } as u64;
	// 8269B198: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269B19C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B1A0 size=116
    let mut pc: u32 = 0x8269B1A0;
    'dispatch: loop {
        match pc {
            0x8269B1A0 => {
    //   block [0x8269B1A0..0x8269B214)
	// 8269B1A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B1A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B1A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B1AC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269B1B0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269B1B4: 390A6E40  addi r8, r10, 0x6e40
	ctx.r[8].s64 = ctx.r[10].s64 + 28224;
	// 8269B1B8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B1BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269B1C0: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269B1C4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B1C8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269B1CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B1D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B1D4: 388AB688  addi r4, r10, -0x4978
	ctx.r[4].s64 = ctx.r[10].s64 + -18808;
	// 8269B1D8: 396B9AE8  addi r11, r11, -0x6518
	ctx.r[11].s64 = ctx.r[11].s64 + -25880;
	// 8269B1DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B1E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B1E4: 386A3B00  addi r3, r10, 0x3b00
	ctx.r[3].s64 = ctx.r[10].s64 + 15104;
	// 8269B1E8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269B1EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B1F0: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269B1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B1F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B1FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B200: 4BDCBC21  bl 0x82466e20
	ctx.lr = 0x8269B204;
	sub_82466E20(ctx, base);
	// 8269B204: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B208: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B20C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B210: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B218 size=112
    let mut pc: u32 = 0x8269B218;
    'dispatch: loop {
        match pc {
            0x8269B218 => {
    //   block [0x8269B218..0x8269B288)
	// 8269B218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B21C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B220: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B224: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B228: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B22C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269B230: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B234: 390B0588  addi r8, r11, 0x588
	ctx.r[8].s64 = ctx.r[11].s64 + 1416;
	// 8269B238: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269B23C: 388AB69C  addi r4, r10, -0x4964
	ctx.r[4].s64 = ctx.r[10].s64 + -18788;
	// 8269B240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B244: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B248: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B24C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B250: 386A3B30  addi r3, r10, 0x3b30
	ctx.r[3].s64 = ctx.r[10].s64 + 15152;
	// 8269B254: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B258: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B25C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B260: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B264: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B268: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B26C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B270: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B274: 4BDCBBAD  bl 0x82466e20
	ctx.lr = 0x8269B278;
	sub_82466E20(ctx, base);
	// 8269B278: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B27C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B280: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B284: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B288 size=112
    let mut pc: u32 = 0x8269B288;
    'dispatch: loop {
        match pc {
            0x8269B288 => {
    //   block [0x8269B288..0x8269B2F8)
	// 8269B288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B28C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B290: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B294: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B298: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B29C: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269B2A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B2A4: 390B0618  addi r8, r11, 0x618
	ctx.r[8].s64 = ctx.r[11].s64 + 1560;
	// 8269B2A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269B2AC: 388AB6CC  addi r4, r10, -0x4934
	ctx.r[4].s64 = ctx.r[10].s64 + -18740;
	// 8269B2B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B2B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B2B8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B2BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B2C0: 386A3B60  addi r3, r10, 0x3b60
	ctx.r[3].s64 = ctx.r[10].s64 + 15200;
	// 8269B2C4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B2C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B2CC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B2D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B2D4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B2D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B2DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B2E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B2E4: 4BDCBB3D  bl 0x82466e20
	ctx.lr = 0x8269B2E8;
	sub_82466E20(ctx, base);
	// 8269B2E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B2EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B2F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B2F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B2F8 size=112
    let mut pc: u32 = 0x8269B2F8;
    'dispatch: loop {
        match pc {
            0x8269B2F8 => {
    //   block [0x8269B2F8..0x8269B368)
	// 8269B2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B300: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B304: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B308: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B30C: 38AA3A10  addi r5, r10, 0x3a10
	ctx.r[5].s64 = ctx.r[10].s64 + 14864;
	// 8269B310: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B314: 390B0678  addi r8, r11, 0x678
	ctx.r[8].s64 = ctx.r[11].s64 + 1656;
	// 8269B318: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269B31C: 388AB6FC  addi r4, r10, -0x4904
	ctx.r[4].s64 = ctx.r[10].s64 + -18692;
	// 8269B320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B324: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B328: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B32C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B330: 386A3B90  addi r3, r10, 0x3b90
	ctx.r[3].s64 = ctx.r[10].s64 + 15248;
	// 8269B334: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B338: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B33C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B340: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B344: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B348: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B34C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B350: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B354: 4BDCBACD  bl 0x82466e20
	ctx.lr = 0x8269B358;
	sub_82466E20(ctx, base);
	// 8269B358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B368 size=100
    let mut pc: u32 = 0x8269B368;
    'dispatch: loop {
        match pc {
            0x8269B368 => {
    //   block [0x8269B368..0x8269B3CC)
	// 8269B368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B374: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B37C: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269B380: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B384: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B388: 388AB750  addi r4, r10, -0x48b0
	ctx.r[4].s64 = ctx.r[10].s64 + -18608;
	// 8269B38C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B390: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B394: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B398: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B39C: 386A3BC0  addi r3, r10, 0x3bc0
	ctx.r[3].s64 = ctx.r[10].s64 + 15296;
	// 8269B3A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B3A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B3A8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269B3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B3B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269B3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B3B8: 4BDCBA69  bl 0x82466e20
	ctx.lr = 0x8269B3BC;
	sub_82466E20(ctx, base);
	// 8269B3BC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B3C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B3C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B3C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B3D0 size=112
    let mut pc: u32 = 0x8269B3D0;
    'dispatch: loop {
        match pc {
            0x8269B3D0 => {
    //   block [0x8269B3D0..0x8269B440)
	// 8269B3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B3D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B3D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B3DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B3E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B3E4: 38AA3BC0  addi r5, r10, 0x3bc0
	ctx.r[5].s64 = ctx.r[10].s64 + 15296;
	// 8269B3E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B3EC: 390B06A8  addi r8, r11, 0x6a8
	ctx.r[8].s64 = ctx.r[11].s64 + 1704;
	// 8269B3F0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269B3F4: 388AB760  addi r4, r10, -0x48a0
	ctx.r[4].s64 = ctx.r[10].s64 + -18592;
	// 8269B3F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B3FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B400: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B408: 386A3BF0  addi r3, r10, 0x3bf0
	ctx.r[3].s64 = ctx.r[10].s64 + 15344;
	// 8269B40C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B410: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B414: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B418: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B420: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B428: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B42C: 4BDCB9F5  bl 0x82466e20
	ctx.lr = 0x8269B430;
	sub_82466E20(ctx, base);
	// 8269B430: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B440 size=112
    let mut pc: u32 = 0x8269B440;
    'dispatch: loop {
        match pc {
            0x8269B440 => {
    //   block [0x8269B440..0x8269B4B0)
	// 8269B440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B448: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B44C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B450: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B454: 38AA3BF0  addi r5, r10, 0x3bf0
	ctx.r[5].s64 = ctx.r[10].s64 + 15344;
	// 8269B458: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B45C: 390B0708  addi r8, r11, 0x708
	ctx.r[8].s64 = ctx.r[11].s64 + 1800;
	// 8269B460: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269B464: 388AB778  addi r4, r10, -0x4888
	ctx.r[4].s64 = ctx.r[10].s64 + -18568;
	// 8269B468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B46C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B470: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B474: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B478: 386A3C20  addi r3, r10, 0x3c20
	ctx.r[3].s64 = ctx.r[10].s64 + 15392;
	// 8269B47C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B484: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B48C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B49C: 4BDCB985  bl 0x82466e20
	ctx.lr = 0x8269B4A0;
	sub_82466E20(ctx, base);
	// 8269B4A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B4A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B4A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B4AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B4B0 size=112
    let mut pc: u32 = 0x8269B4B0;
    'dispatch: loop {
        match pc {
            0x8269B4B0 => {
    //   block [0x8269B4B0..0x8269B520)
	// 8269B4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B4B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B4B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B4BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B4C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B4C4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269B4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B4CC: 390B0738  addi r8, r11, 0x738
	ctx.r[8].s64 = ctx.r[11].s64 + 1848;
	// 8269B4D0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269B4D4: 388AB7A4  addi r4, r10, -0x485c
	ctx.r[4].s64 = ctx.r[10].s64 + -18524;
	// 8269B4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B4DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B4E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B4E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B4E8: 386A3C50  addi r3, r10, 0x3c50
	ctx.r[3].s64 = ctx.r[10].s64 + 15440;
	// 8269B4EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B4F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B4FC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269B500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B50C: 4BDCB915  bl 0x82466e20
	ctx.lr = 0x8269B510;
	sub_82466E20(ctx, base);
	// 8269B510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B51C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B520 size=116
    let mut pc: u32 = 0x8269B520;
    'dispatch: loop {
        match pc {
            0x8269B520 => {
    //   block [0x8269B520..0x8269B594)
	// 8269B520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B52C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B530: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269B534: 390B0768  addi r8, r11, 0x768
	ctx.r[8].s64 = ctx.r[11].s64 + 1896;
	// 8269B538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B53C: 392A9B18  addi r9, r10, -0x64e8
	ctx.r[9].s64 = ctx.r[10].s64 + -25832;
	// 8269B540: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B544: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269B548: 38AA4010  addi r5, r10, 0x4010
	ctx.r[5].s64 = ctx.r[10].s64 + 16400;
	// 8269B54C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B550: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B554: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B558: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B55C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B560: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B564: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269B568: 388AB7C4  addi r4, r10, -0x483c
	ctx.r[4].s64 = ctx.r[10].s64 + -18492;
	// 8269B56C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269B570: 386B3C80  addi r3, r11, 0x3c80
	ctx.r[3].s64 = ctx.r[11].s64 + 15488;
	// 8269B574: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269B578: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B57C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B580: 4BDCB8A1  bl 0x82466e20
	ctx.lr = 0x8269B584;
	sub_82466E20(ctx, base);
	// 8269B584: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B58C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B598 size=112
    let mut pc: u32 = 0x8269B598;
    'dispatch: loop {
        match pc {
            0x8269B598 => {
    //   block [0x8269B598..0x8269B608)
	// 8269B598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B59C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B5A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B5A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B5A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B5AC: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B5B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B5B4: 390B0780  addi r8, r11, 0x780
	ctx.r[8].s64 = ctx.r[11].s64 + 1920;
	// 8269B5B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269B5BC: 388AB7D4  addi r4, r10, -0x482c
	ctx.r[4].s64 = ctx.r[10].s64 + -18476;
	// 8269B5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B5C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B5C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B5CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B5D0: 386A3CB0  addi r3, r10, 0x3cb0
	ctx.r[3].s64 = ctx.r[10].s64 + 15536;
	// 8269B5D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B5D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B5DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B5E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B5E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B5E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B5EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B5F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B5F4: 4BDCB82D  bl 0x82466e20
	ctx.lr = 0x8269B5F8;
	sub_82466E20(ctx, base);
	// 8269B5F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B5FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B608 size=116
    let mut pc: u32 = 0x8269B608;
    'dispatch: loop {
        match pc {
            0x8269B608 => {
    //   block [0x8269B608..0x8269B67C)
	// 8269B608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B60C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B610: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B614: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B618: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269B61C: 390B079C  addi r8, r11, 0x79c
	ctx.r[8].s64 = ctx.r[11].s64 + 1948;
	// 8269B620: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B624: 392A9B44  addi r9, r10, -0x64bc
	ctx.r[9].s64 = ctx.r[10].s64 + -25788;
	// 8269B628: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B62C: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8269B630: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B634: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B638: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B63C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B640: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B648: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B64C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269B650: 388AB7E0  addi r4, r10, -0x4820
	ctx.r[4].s64 = ctx.r[10].s64 + -18464;
	// 8269B654: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269B658: 386B3CE0  addi r3, r11, 0x3ce0
	ctx.r[3].s64 = ctx.r[11].s64 + 15584;
	// 8269B65C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269B660: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B668: 4BDCB7B9  bl 0x82466e20
	ctx.lr = 0x8269B66C;
	sub_82466E20(ctx, base);
	// 8269B66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B680 size=112
    let mut pc: u32 = 0x8269B680;
    'dispatch: loop {
        match pc {
            0x8269B680 => {
    //   block [0x8269B680..0x8269B6F0)
	// 8269B680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B68C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B690: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B694: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B69C: 390B07D0  addi r8, r11, 0x7d0
	ctx.r[8].s64 = ctx.r[11].s64 + 2000;
	// 8269B6A0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269B6A4: 388AB7F0  addi r4, r10, -0x4810
	ctx.r[4].s64 = ctx.r[10].s64 + -18448;
	// 8269B6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B6AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B6B8: 386A3D10  addi r3, r10, 0x3d10
	ctx.r[3].s64 = ctx.r[10].s64 + 15632;
	// 8269B6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B6DC: 4BDCB745  bl 0x82466e20
	ctx.lr = 0x8269B6E0;
	sub_82466E20(ctx, base);
	// 8269B6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B6F0 size=112
    let mut pc: u32 = 0x8269B6F0;
    'dispatch: loop {
        match pc {
            0x8269B6F0 => {
    //   block [0x8269B6F0..0x8269B760)
	// 8269B6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B6FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B700: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B704: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B70C: 390B0818  addi r8, r11, 0x818
	ctx.r[8].s64 = ctx.r[11].s64 + 2072;
	// 8269B710: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269B714: 388AB808  addi r4, r10, -0x47f8
	ctx.r[4].s64 = ctx.r[10].s64 + -18424;
	// 8269B718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B71C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B728: 386A3D40  addi r3, r10, 0x3d40
	ctx.r[3].s64 = ctx.r[10].s64 + 15680;
	// 8269B72C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B74C: 4BDCB6D5  bl 0x82466e20
	ctx.lr = 0x8269B750;
	sub_82466E20(ctx, base);
	// 8269B750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B760 size=112
    let mut pc: u32 = 0x8269B760;
    'dispatch: loop {
        match pc {
            0x8269B760 => {
    //   block [0x8269B760..0x8269B7D0)
	// 8269B760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B76C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B770: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B774: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269B778: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269B77C: 390B0860  addi r8, r11, 0x860
	ctx.r[8].s64 = ctx.r[11].s64 + 2144;
	// 8269B780: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269B784: 388AB39C  addi r4, r10, -0x4c64
	ctx.r[4].s64 = ctx.r[10].s64 + -19556;
	// 8269B788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B78C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B798: 386A3D70  addi r3, r10, 0x3d70
	ctx.r[3].s64 = ctx.r[10].s64 + 15728;
	// 8269B79C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B7BC: 4BDCB665  bl 0x82466e20
	ctx.lr = 0x8269B7C0;
	sub_82466E20(ctx, base);
	// 8269B7C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B7D0 size=108
    let mut pc: u32 = 0x8269B7D0;
    'dispatch: loop {
        match pc {
            0x8269B7D0 => {
    //   block [0x8269B7D0..0x8269B83C)
	// 8269B7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B7DC: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B7E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B7E4: 38EB0890  addi r7, r11, 0x890
	ctx.r[7].s64 = ctx.r[11].s64 + 2192;
	// 8269B7E8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269B7EC: 388AB820  addi r4, r10, -0x47e0
	ctx.r[4].s64 = ctx.r[10].s64 + -18400;
	// 8269B7F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B7F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B7F8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269B7FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B800: 386A3DA0  addi r3, r10, 0x3da0
	ctx.r[3].s64 = ctx.r[10].s64 + 15776;
	// 8269B804: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269B808: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B80C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B810: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B814: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B818: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B81C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B820: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B824: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269B828: 4BDCB5F9  bl 0x82466e20
	ctx.lr = 0x8269B82C;
	sub_82466E20(ctx, base);
	// 8269B82C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B838: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B840 size=112
    let mut pc: u32 = 0x8269B840;
    'dispatch: loop {
        match pc {
            0x8269B840 => {
    //   block [0x8269B840..0x8269B8B0)
	// 8269B840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B84C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B854: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B85C: 390B08D8  addi r8, r11, 0x8d8
	ctx.r[8].s64 = ctx.r[11].s64 + 2264;
	// 8269B860: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269B864: 388AB844  addi r4, r10, -0x47bc
	ctx.r[4].s64 = ctx.r[10].s64 + -18364;
	// 8269B868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B86C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B878: 386A3DD0  addi r3, r10, 0x3dd0
	ctx.r[3].s64 = ctx.r[10].s64 + 15824;
	// 8269B87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B89C: 4BDCB585  bl 0x82466e20
	ctx.lr = 0x8269B8A0;
	sub_82466E20(ctx, base);
	// 8269B8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B8B0 size=116
    let mut pc: u32 = 0x8269B8B0;
    'dispatch: loop {
        match pc {
            0x8269B8B0 => {
    //   block [0x8269B8B0..0x8269B924)
	// 8269B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B8BC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269B8C0: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B8C4: 392B9B80  addi r9, r11, -0x6480
	ctx.r[9].s64 = ctx.r[11].s64 + -25728;
	// 8269B8C8: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B8CC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B8D0: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269B8D4: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8269B8D8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B8DC: 388AB85C  addi r4, r10, -0x47a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18340;
	// 8269B8E0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B8E4: 396B0968  addi r11, r11, 0x968
	ctx.r[11].s64 = ctx.r[11].s64 + 2408;
	// 8269B8E8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269B8EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B8F0: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269B8F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B8F8: 386A3E00  addi r3, r10, 0x3e00
	ctx.r[3].s64 = ctx.r[10].s64 + 15872;
	// 8269B8FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269B900: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269B904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B908: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269B90C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269B910: 4BDCB511  bl 0x82466e20
	ctx.lr = 0x8269B914;
	sub_82466E20(ctx, base);
	// 8269B914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B920: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B928 size=112
    let mut pc: u32 = 0x8269B928;
    'dispatch: loop {
        match pc {
            0x8269B928 => {
    //   block [0x8269B928..0x8269B998)
	// 8269B928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B934: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B938: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B93C: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B940: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B944: 390B09F8  addi r8, r11, 0x9f8
	ctx.r[8].s64 = ctx.r[11].s64 + 2552;
	// 8269B948: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269B94C: 388AB8E0  addi r4, r10, -0x4720
	ctx.r[4].s64 = ctx.r[10].s64 + -18208;
	// 8269B950: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269B954: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B958: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B95C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269B960: 386A3E30  addi r3, r10, 0x3e30
	ctx.r[3].s64 = ctx.r[10].s64 + 15920;
	// 8269B964: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269B968: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269B96C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B970: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269B974: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B978: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269B97C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269B980: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269B984: 4BDCB49D  bl 0x82466e20
	ctx.lr = 0x8269B988;
	sub_82466E20(ctx, base);
	// 8269B988: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269B98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269B990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269B994: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B998(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269B998 size=24
    let mut pc: u32 = 0x8269B998;
    'dispatch: loop {
        match pc {
            0x8269B998 => {
    //   block [0x8269B998..0x8269B9B0)
	// 8269B998: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269B99C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269B9A0: 394A6EB8  addi r10, r10, 0x6eb8
	ctx.r[10].s64 = ctx.r[10].s64 + 28344;
	// 8269B9A4: 816B0EB0  lwz r11, 0xeb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3760 as u32) ) } as u64;
	// 8269B9A8: 916A0020  stw r11, 0x20(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8269B9AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269B9B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269B9B0 size=116
    let mut pc: u32 = 0x8269B9B0;
    'dispatch: loop {
        match pc {
            0x8269B9B0 => {
    //   block [0x8269B9B0..0x8269BA24)
	// 8269B9B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269B9B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269B9B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269B9BC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269B9C0: 38E00007  li r7, 7
	ctx.r[7].s64 = 7;
	// 8269B9C4: 390A6EB8  addi r8, r10, 0x6eb8
	ctx.r[8].s64 = ctx.r[10].s64 + 28344;
	// 8269B9C8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B9CC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269B9D0: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269B9D4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269B9D8: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269B9DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269B9E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269B9E4: 388AB8F0  addi r4, r10, -0x4710
	ctx.r[4].s64 = ctx.r[10].s64 + -18192;
	// 8269B9E8: 396B9BB0  addi r11, r11, -0x6450
	ctx.r[11].s64 = ctx.r[11].s64 + -25680;
	// 8269B9EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269B9F0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269B9F4: 386A3E60  addi r3, r10, 0x3e60
	ctx.r[3].s64 = ctx.r[10].s64 + 15968;
	// 8269B9F8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269B9FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BA00: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269BA04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BA08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BA0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BA10: 4BDCB411  bl 0x82466e20
	ctx.lr = 0x8269BA14;
	sub_82466E20(ctx, base);
	// 8269BA14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BA18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BA1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BA28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BA28 size=112
    let mut pc: u32 = 0x8269BA28;
    'dispatch: loop {
        match pc {
            0x8269BA28 => {
    //   block [0x8269BA28..0x8269BA98)
	// 8269BA28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BA2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BA30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BA34: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BA38: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BA3C: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269BA40: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BA44: 390B0A10  addi r8, r11, 0xa10
	ctx.r[8].s64 = ctx.r[11].s64 + 2576;
	// 8269BA48: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269BA4C: 388AB530  addi r4, r10, -0x4ad0
	ctx.r[4].s64 = ctx.r[10].s64 + -19152;
	// 8269BA50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BA54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BA58: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BA5C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BA60: 386A3E90  addi r3, r10, 0x3e90
	ctx.r[3].s64 = ctx.r[10].s64 + 16016;
	// 8269BA64: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BA68: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BA6C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BA70: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BA74: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BA78: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BA7C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BA80: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BA84: 4BDCB39D  bl 0x82466e20
	ctx.lr = 0x8269BA88;
	sub_82466E20(ctx, base);
	// 8269BA88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BA94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BA98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BA98 size=100
    let mut pc: u32 = 0x8269BA98;
    'dispatch: loop {
        match pc {
            0x8269BA98 => {
    //   block [0x8269BA98..0x8269BAFC)
	// 8269BA98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BA9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BAA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BAA4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BAAC: 38AA3F20  addi r5, r10, 0x3f20
	ctx.r[5].s64 = ctx.r[10].s64 + 16160;
	// 8269BAB0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BAB8: 388AB5FC  addi r4, r10, -0x4a04
	ctx.r[4].s64 = ctx.r[10].s64 + -18948;
	// 8269BABC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BAC0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BAC4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BAC8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BACC: 386A3EC0  addi r3, r10, 0x3ec0
	ctx.r[3].s64 = ctx.r[10].s64 + 16064;
	// 8269BAD0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BAD4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BAD8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269BADC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BAE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269BAE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BAE8: 4BDCB339  bl 0x82466e20
	ctx.lr = 0x8269BAEC;
	sub_82466E20(ctx, base);
	// 8269BAEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BAF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BAF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269BB00 size=28
    let mut pc: u32 = 0x8269BB00;
    'dispatch: loop {
        match pc {
            0x8269BB00 => {
    //   block [0x8269BB00..0x8269BB1C)
	// 8269BB00: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BB04: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269BB08: 394A6F60  addi r10, r10, 0x6f60
	ctx.r[10].s64 = ctx.r[10].s64 + 28512;
	// 8269BB0C: 816B0A58  lwz r11, 0xa58(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2648 as u32) ) } as u64;
	// 8269BB10: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269BB14: 916A0080  stw r11, 0x80(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8269BB18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BB20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BB20 size=112
    let mut pc: u32 = 0x8269BB20;
    'dispatch: loop {
        match pc {
            0x8269BB20 => {
    //   block [0x8269BB20..0x8269BB90)
	// 8269BB20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BB24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BB28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BB2C: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269BB30: 3900000F  li r8, 0xf
	ctx.r[8].s64 = 15;
	// 8269BB34: 38EA6F60  addi r7, r10, 0x6f60
	ctx.r[7].s64 = ctx.r[10].s64 + 28512;
	// 8269BB38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BB3C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269BB40: 388AB648  addi r4, r10, -0x49b8
	ctx.r[4].s64 = ctx.r[10].s64 + -18872;
	// 8269BB44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BB48: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269BB4C: 396B9C38  addi r11, r11, -0x63c8
	ctx.r[11].s64 = ctx.r[11].s64 + -25544;
	// 8269BB50: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269BB54: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BB58: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BB5C: 386A3EF0  addi r3, r10, 0x3ef0
	ctx.r[3].s64 = ctx.r[10].s64 + 16112;
	// 8269BB60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BB64: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269BB68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BB6C: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269BB70: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BB74: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BB78: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269BB7C: 4BDCB2A5  bl 0x82466e20
	ctx.lr = 0x8269BB80;
	sub_82466E20(ctx, base);
	// 8269BB80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BB84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BB88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269BB90 size=24
    let mut pc: u32 = 0x8269BB90;
    'dispatch: loop {
        match pc {
            0x8269BB90 => {
    //   block [0x8269BB90..0x8269BBA8)
	// 8269BB90: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BB94: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269BB98: 394A70C8  addi r10, r10, 0x70c8
	ctx.r[10].s64 = ctx.r[10].s64 + 28872;
	// 8269BB9C: 816B0EB0  lwz r11, 0xeb0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3760 as u32) ) } as u64;
	// 8269BBA0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269BBA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BBA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BBA8 size=116
    let mut pc: u32 = 0x8269BBA8;
    'dispatch: loop {
        match pc {
            0x8269BBA8 => {
    //   block [0x8269BBA8..0x8269BC1C)
	// 8269BBA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BBAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BBB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BBB4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269BBB8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BBBC: 392B9C10  addi r9, r11, -0x63f0
	ctx.r[9].s64 = ctx.r[11].s64 + -25584;
	// 8269BBC0: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269BBC4: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BBC8: 38E90068  addi r7, r9, 0x68
	ctx.r[7].s64 = ctx.r[9].s64 + 104;
	// 8269BBCC: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 8269BBD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BBD4: 388AB65C  addi r4, r10, -0x49a4
	ctx.r[4].s64 = ctx.r[10].s64 + -18852;
	// 8269BBD8: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BBDC: 396B70C8  addi r11, r11, 0x70c8
	ctx.r[11].s64 = ctx.r[11].s64 + 28872;
	// 8269BBE0: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269BBE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BBE8: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269BBEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BBF0: 386A3F20  addi r3, r10, 0x3f20
	ctx.r[3].s64 = ctx.r[10].s64 + 16160;
	// 8269BBF4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8269BBF8: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269BBFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BC00: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269BC04: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269BC08: 4BDCB219  bl 0x82466e20
	ctx.lr = 0x8269BC0C;
	sub_82466E20(ctx, base);
	// 8269BC0C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BC10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BC14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BC18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BC20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BC20 size=112
    let mut pc: u32 = 0x8269BC20;
    'dispatch: loop {
        match pc {
            0x8269BC20 => {
    //   block [0x8269BC20..0x8269BC90)
	// 8269BC20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BC24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BC28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BC2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BC30: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BC34: 38AA4010  addi r5, r10, 0x4010
	ctx.r[5].s64 = ctx.r[10].s64 + 16400;
	// 8269BC38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BC3C: 390B0A60  addi r8, r11, 0xa60
	ctx.r[8].s64 = ctx.r[11].s64 + 2656;
	// 8269BC40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269BC44: 388ABA0C  addi r4, r10, -0x45f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17908;
	// 8269BC48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BC4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BC50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BC54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BC58: 386A3F50  addi r3, r10, 0x3f50
	ctx.r[3].s64 = ctx.r[10].s64 + 16208;
	// 8269BC5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BC60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BC64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BC68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BC6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BC70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BC74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BC78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BC7C: 4BDCB1A5  bl 0x82466e20
	ctx.lr = 0x8269BC80;
	sub_82466E20(ctx, base);
	// 8269BC80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BC84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BC88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BC8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BC90 size=112
    let mut pc: u32 = 0x8269BC90;
    'dispatch: loop {
        match pc {
            0x8269BC90 => {
    //   block [0x8269BC90..0x8269BD00)
	// 8269BC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BC98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BC9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BCA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BCA4: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269BCA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BCAC: 390B0A90  addi r8, r11, 0xa90
	ctx.r[8].s64 = ctx.r[11].s64 + 2704;
	// 8269BCB0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269BCB4: 388AB718  addi r4, r10, -0x48e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18664;
	// 8269BCB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BCBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BCC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BCC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BCC8: 386A3F80  addi r3, r10, 0x3f80
	ctx.r[3].s64 = ctx.r[10].s64 + 16256;
	// 8269BCCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BCD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BCD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BCD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BCDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BCE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BCE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BCE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BCEC: 4BDCB135  bl 0x82466e20
	ctx.lr = 0x8269BCF0;
	sub_82466E20(ctx, base);
	// 8269BCF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BCF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BCF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BCFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BD00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BD00 size=112
    let mut pc: u32 = 0x8269BD00;
    'dispatch: loop {
        match pc {
            0x8269BD00 => {
    //   block [0x8269BD00..0x8269BD70)
	// 8269BD00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BD04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BD08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BD0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BD10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BD14: 38AA3F20  addi r5, r10, 0x3f20
	ctx.r[5].s64 = ctx.r[10].s64 + 16160;
	// 8269BD18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BD1C: 390B0B20  addi r8, r11, 0xb20
	ctx.r[8].s64 = ctx.r[11].s64 + 2848;
	// 8269BD20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269BD24: 388AB73C  addi r4, r10, -0x48c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18628;
	// 8269BD28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BD2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BD30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BD34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BD38: 386A3FB0  addi r3, r10, 0x3fb0
	ctx.r[3].s64 = ctx.r[10].s64 + 16304;
	// 8269BD3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BD40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BD44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BD48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BD4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BD50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BD54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BD58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BD5C: 4BDCB0C5  bl 0x82466e20
	ctx.lr = 0x8269BD60;
	sub_82466E20(ctx, base);
	// 8269BD60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BD64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BD68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BD6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BD70 size=100
    let mut pc: u32 = 0x8269BD70;
    'dispatch: loop {
        match pc {
            0x8269BD70 => {
    //   block [0x8269BD70..0x8269BDD4)
	// 8269BD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BD78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BD7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BD80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BD84: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269BD88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BD8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BD90: 388AB904  addi r4, r10, -0x46fc
	ctx.r[4].s64 = ctx.r[10].s64 + -18172;
	// 8269BD94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BD98: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BD9C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BDA0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BDA4: 386A3FE0  addi r3, r10, 0x3fe0
	ctx.r[3].s64 = ctx.r[10].s64 + 16352;
	// 8269BDA8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BDAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BDB0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269BDB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BDB8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269BDBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BDC0: 4BDCB061  bl 0x82466e20
	ctx.lr = 0x8269BDC4;
	sub_82466E20(ctx, base);
	// 8269BDC4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BDC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BDCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BDD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BDD8 size=100
    let mut pc: u32 = 0x8269BDD8;
    'dispatch: loop {
        match pc {
            0x8269BDD8 => {
    //   block [0x8269BDD8..0x8269BE3C)
	// 8269BDD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BDDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BDE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BDE4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BDE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BDEC: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269BDF0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BDF8: 388AB918  addi r4, r10, -0x46e8
	ctx.r[4].s64 = ctx.r[10].s64 + -18152;
	// 8269BDFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BE00: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BE04: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BE08: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BE0C: 386A4010  addi r3, r10, 0x4010
	ctx.r[3].s64 = ctx.r[10].s64 + 16400;
	// 8269BE10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BE14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BE18: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269BE1C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BE20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269BE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BE28: 4BDCAFF9  bl 0x82466e20
	ctx.lr = 0x8269BE2C;
	sub_82466E20(ctx, base);
	// 8269BE2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BE30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BE34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BE38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BE40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BE40 size=112
    let mut pc: u32 = 0x8269BE40;
    'dispatch: loop {
        match pc {
            0x8269BE40 => {
    //   block [0x8269BE40..0x8269BEB0)
	// 8269BE40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BE44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BE48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BE4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BE50: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BE54: 38AA3FE0  addi r5, r10, 0x3fe0
	ctx.r[5].s64 = ctx.r[10].s64 + 16352;
	// 8269BE58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BE5C: 390B0B38  addi r8, r11, 0xb38
	ctx.r[8].s64 = ctx.r[11].s64 + 2872;
	// 8269BE60: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269BE64: 388AB92C  addi r4, r10, -0x46d4
	ctx.r[4].s64 = ctx.r[10].s64 + -18132;
	// 8269BE68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BE6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BE70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BE74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BE78: 386A4040  addi r3, r10, 0x4040
	ctx.r[3].s64 = ctx.r[10].s64 + 16448;
	// 8269BE7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BE80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BE84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BE88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BE8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BE90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BE98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BE9C: 4BDCAF85  bl 0x82466e20
	ctx.lr = 0x8269BEA0;
	sub_82466E20(ctx, base);
	// 8269BEA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BEA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BEA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BEAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BEB0 size=112
    let mut pc: u32 = 0x8269BEB0;
    'dispatch: loop {
        match pc {
            0x8269BEB0 => {
    //   block [0x8269BEB0..0x8269BF20)
	// 8269BEB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BEB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BEB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BEBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BEC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BEC4: 38AA3FE0  addi r5, r10, 0x3fe0
	ctx.r[5].s64 = ctx.r[10].s64 + 16352;
	// 8269BEC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BECC: 390B0B80  addi r8, r11, 0xb80
	ctx.r[8].s64 = ctx.r[11].s64 + 2944;
	// 8269BED0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269BED4: 388AB93C  addi r4, r10, -0x46c4
	ctx.r[4].s64 = ctx.r[10].s64 + -18116;
	// 8269BED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BEDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BEE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BEE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BEE8: 386A4070  addi r3, r10, 0x4070
	ctx.r[3].s64 = ctx.r[10].s64 + 16496;
	// 8269BEEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BEF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BEF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BEF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BEFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BF00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BF04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BF08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BF0C: 4BDCAF15  bl 0x82466e20
	ctx.lr = 0x8269BF10;
	sub_82466E20(ctx, base);
	// 8269BF10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BF1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BF20 size=112
    let mut pc: u32 = 0x8269BF20;
    'dispatch: loop {
        match pc {
            0x8269BF20 => {
    //   block [0x8269BF20..0x8269BF90)
	// 8269BF20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BF24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BF28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BF2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BF30: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BF34: 38AA4070  addi r5, r10, 0x4070
	ctx.r[5].s64 = ctx.r[10].s64 + 16496;
	// 8269BF38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BF3C: 390B0C40  addi r8, r11, 0xc40
	ctx.r[8].s64 = ctx.r[11].s64 + 3136;
	// 8269BF40: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269BF44: 388AB958  addi r4, r10, -0x46a8
	ctx.r[4].s64 = ctx.r[10].s64 + -18088;
	// 8269BF48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BF4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BF50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BF54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BF58: 386A40A0  addi r3, r10, 0x40a0
	ctx.r[3].s64 = ctx.r[10].s64 + 16544;
	// 8269BF5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BF60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BF64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BF68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BF6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BF70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BF74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BF78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BF7C: 4BDCAEA5  bl 0x82466e20
	ctx.lr = 0x8269BF80;
	sub_82466E20(ctx, base);
	// 8269BF80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BF84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BF88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269BF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269BF90 size=112
    let mut pc: u32 = 0x8269BF90;
    'dispatch: loop {
        match pc {
            0x8269BF90 => {
    //   block [0x8269BF90..0x8269C000)
	// 8269BF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269BF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269BF98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269BF9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BFA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269BFA4: 38AA3BC0  addi r5, r10, 0x3bc0
	ctx.r[5].s64 = ctx.r[10].s64 + 15296;
	// 8269BFA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269BFAC: 390B0C70  addi r8, r11, 0xc70
	ctx.r[8].s64 = ctx.r[11].s64 + 3184;
	// 8269BFB0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269BFB4: 388AB97C  addi r4, r10, -0x4684
	ctx.r[4].s64 = ctx.r[10].s64 + -18052;
	// 8269BFB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269BFBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269BFC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269BFC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269BFC8: 386A40D0  addi r3, r10, 0x40d0
	ctx.r[3].s64 = ctx.r[10].s64 + 16592;
	// 8269BFCC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269BFD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269BFD4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269BFD8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269BFDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269BFE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269BFE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269BFE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269BFEC: 4BDCAE35  bl 0x82466e20
	ctx.lr = 0x8269BFF0;
	sub_82466E20(ctx, base);
	// 8269BFF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269BFF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269BFF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269BFFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C000 size=112
    let mut pc: u32 = 0x8269C000;
    'dispatch: loop {
        match pc {
            0x8269C000 => {
    //   block [0x8269C000..0x8269C070)
	// 8269C000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C008: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C00C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C010: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C014: 38AA3950  addi r5, r10, 0x3950
	ctx.r[5].s64 = ctx.r[10].s64 + 14672;
	// 8269C018: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C01C: 390B0CB8  addi r8, r11, 0xcb8
	ctx.r[8].s64 = ctx.r[11].s64 + 3256;
	// 8269C020: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269C024: 388AB9A0  addi r4, r10, -0x4660
	ctx.r[4].s64 = ctx.r[10].s64 + -18016;
	// 8269C028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C02C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C030: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C034: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C038: 386A4100  addi r3, r10, 0x4100
	ctx.r[3].s64 = ctx.r[10].s64 + 16640;
	// 8269C03C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C040: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C044: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C048: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C04C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C050: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C054: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C058: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C05C: 4BDCADC5  bl 0x82466e20
	ctx.lr = 0x8269C060;
	sub_82466E20(ctx, base);
	// 8269C060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C064: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C068: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C070 size=112
    let mut pc: u32 = 0x8269C070;
    'dispatch: loop {
        match pc {
            0x8269C070 => {
    //   block [0x8269C070..0x8269C0E0)
	// 8269C070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C074: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C07C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C080: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C084: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269C088: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C08C: 390B0D00  addi r8, r11, 0xd00
	ctx.r[8].s64 = ctx.r[11].s64 + 3328;
	// 8269C090: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C094: 388AB9C4  addi r4, r10, -0x463c
	ctx.r[4].s64 = ctx.r[10].s64 + -17980;
	// 8269C098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C09C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C0A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C0A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C0A8: 386A4130  addi r3, r10, 0x4130
	ctx.r[3].s64 = ctx.r[10].s64 + 16688;
	// 8269C0AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C0B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C0B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C0B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C0BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C0C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C0CC: 4BDCAD55  bl 0x82466e20
	ctx.lr = 0x8269C0D0;
	sub_82466E20(ctx, base);
	// 8269C0D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C0D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C0D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C0DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C0E0 size=112
    let mut pc: u32 = 0x8269C0E0;
    'dispatch: loop {
        match pc {
            0x8269C0E0 => {
    //   block [0x8269C0E0..0x8269C150)
	// 8269C0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C0E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C0EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C0F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C0F4: 38AA3C80  addi r5, r10, 0x3c80
	ctx.r[5].s64 = ctx.r[10].s64 + 15488;
	// 8269C0F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C0FC: 390B0D30  addi r8, r11, 0xd30
	ctx.r[8].s64 = ctx.r[11].s64 + 3376;
	// 8269C100: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269C104: 388AB9D0  addi r4, r10, -0x4630
	ctx.r[4].s64 = ctx.r[10].s64 + -17968;
	// 8269C108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C10C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C110: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C114: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C118: 386A4160  addi r3, r10, 0x4160
	ctx.r[3].s64 = ctx.r[10].s64 + 16736;
	// 8269C11C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C120: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C124: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C128: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C12C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269C130: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C134: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C138: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C13C: 4BDCACE5  bl 0x82466e20
	ctx.lr = 0x8269C140;
	sub_82466E20(ctx, base);
	// 8269C140: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C144: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C148: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C14C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C150 size=108
    let mut pc: u32 = 0x8269C150;
    'dispatch: loop {
        match pc {
            0x8269C150 => {
    //   block [0x8269C150..0x8269C1BC)
	// 8269C150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C158: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C15C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C160: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C164: 38EB0DA8  addi r7, r11, 0xda8
	ctx.r[7].s64 = ctx.r[11].s64 + 3496;
	// 8269C168: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269C16C: 388AB9E4  addi r4, r10, -0x461c
	ctx.r[4].s64 = ctx.r[10].s64 + -17948;
	// 8269C170: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C174: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C178: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269C17C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C180: 386A4190  addi r3, r10, 0x4190
	ctx.r[3].s64 = ctx.r[10].s64 + 16784;
	// 8269C184: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269C188: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C18C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C190: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C194: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C198: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C19C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C1A0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C1A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C1A8: 4BDCAC79  bl 0x82466e20
	ctx.lr = 0x8269C1AC;
	sub_82466E20(ctx, base);
	// 8269C1AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C1C0 size=112
    let mut pc: u32 = 0x8269C1C0;
    'dispatch: loop {
        match pc {
            0x8269C1C0 => {
    //   block [0x8269C1C0..0x8269C230)
	// 8269C1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C1C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C1CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C1D0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C1D4: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269C1D8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C1DC: 390B0DD8  addi r8, r11, 0xdd8
	ctx.r[8].s64 = ctx.r[11].s64 + 3544;
	// 8269C1E0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C1E4: 388AB9F8  addi r4, r10, -0x4608
	ctx.r[4].s64 = ctx.r[10].s64 + -17928;
	// 8269C1E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C1EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C1F0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C1F4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C1F8: 386A41C0  addi r3, r10, 0x41c0
	ctx.r[3].s64 = ctx.r[10].s64 + 16832;
	// 8269C1FC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C200: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C204: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C208: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C20C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C210: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C214: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C218: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C21C: 4BDCAC05  bl 0x82466e20
	ctx.lr = 0x8269C220;
	sub_82466E20(ctx, base);
	// 8269C220: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C224: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C228: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C22C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C230 size=100
    let mut pc: u32 = 0x8269C230;
    'dispatch: loop {
        match pc {
            0x8269C230 => {
    //   block [0x8269C230..0x8269C294)
	// 8269C230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C238: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C23C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C244: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269C248: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C24C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C250: 388ABA20  addi r4, r10, -0x45e0
	ctx.r[4].s64 = ctx.r[10].s64 + -17888;
	// 8269C254: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C258: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C25C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C260: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C264: 386A41F0  addi r3, r10, 0x41f0
	ctx.r[3].s64 = ctx.r[10].s64 + 16880;
	// 8269C268: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C26C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C270: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269C274: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C278: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269C27C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C280: 4BDCABA1  bl 0x82466e20
	ctx.lr = 0x8269C284;
	sub_82466E20(ctx, base);
	// 8269C284: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C290: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C298 size=112
    let mut pc: u32 = 0x8269C298;
    'dispatch: loop {
        match pc {
            0x8269C298 => {
    //   block [0x8269C298..0x8269C308)
	// 8269C298: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C29C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C2A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C2A4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C2A8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C2AC: 38AA38C0  addi r5, r10, 0x38c0
	ctx.r[5].s64 = ctx.r[10].s64 + 14528;
	// 8269C2B0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C2B4: 390B0E08  addi r8, r11, 0xe08
	ctx.r[8].s64 = ctx.r[11].s64 + 3592;
	// 8269C2B8: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269C2BC: 388ABA38  addi r4, r10, -0x45c8
	ctx.r[4].s64 = ctx.r[10].s64 + -17864;
	// 8269C2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C2C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C2C8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C2CC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C2D0: 386A4220  addi r3, r10, 0x4220
	ctx.r[3].s64 = ctx.r[10].s64 + 16928;
	// 8269C2D4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C2D8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C2DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C2E0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C2E4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C2E8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C2EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C2F0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C2F4: 4BDCAB2D  bl 0x82466e20
	ctx.lr = 0x8269C2F8;
	sub_82466E20(ctx, base);
	// 8269C2F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C308 size=96
    let mut pc: u32 = 0x8269C308;
    'dispatch: loop {
        match pc {
            0x8269C308 => {
    //   block [0x8269C308..0x8269C368)
	// 8269C308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C310: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C314: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C318: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C31C: 388ABA4C  addi r4, r10, -0x45b4
	ctx.r[4].s64 = ctx.r[10].s64 + -17844;
	// 8269C320: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C328: 386A4250  addi r3, r10, 0x4250
	ctx.r[3].s64 = ctx.r[10].s64 + 16976;
	// 8269C32C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C330: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C334: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269C338: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C33C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C340: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C344: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C348: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269C34C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C350: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269C354: 4BDCAACD  bl 0x82466e20
	ctx.lr = 0x8269C358;
	sub_82466E20(ctx, base);
	// 8269C358: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C35C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C368 size=108
    let mut pc: u32 = 0x8269C368;
    'dispatch: loop {
        match pc {
            0x8269C368 => {
    //   block [0x8269C368..0x8269C3D4)
	// 8269C368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C36C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C374: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C378: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C37C: 38EB0E50  addi r7, r11, 0xe50
	ctx.r[7].s64 = ctx.r[11].s64 + 3664;
	// 8269C380: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8269C384: 388ABA68  addi r4, r10, -0x4598
	ctx.r[4].s64 = ctx.r[10].s64 + -17816;
	// 8269C388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C38C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C390: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269C394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C398: 386A4280  addi r3, r10, 0x4280
	ctx.r[3].s64 = ctx.r[10].s64 + 17024;
	// 8269C39C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269C3A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C3A4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C3A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C3AC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C3B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C3B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C3B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C3BC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C3C0: 4BDCAA61  bl 0x82466e20
	ctx.lr = 0x8269C3C4;
	sub_82466E20(ctx, base);
	// 8269C3C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C3C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C3CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C3D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C3D8 size=100
    let mut pc: u32 = 0x8269C3D8;
    'dispatch: loop {
        match pc {
            0x8269C3D8 => {
    //   block [0x8269C3D8..0x8269C43C)
	// 8269C3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C3E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269C3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C3EC: 392A9CE8  addi r9, r10, -0x6318
	ctx.r[9].s64 = ctx.r[10].s64 + -25368;
	// 8269C3F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C3F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C3F8: 388ABA80  addi r4, r10, -0x4580
	ctx.r[4].s64 = ctx.r[10].s64 + -17792;
	// 8269C3FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C400: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C404: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C408: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C40C: 386A42B0  addi r3, r10, 0x42b0
	ctx.r[3].s64 = ctx.r[10].s64 + 17072;
	// 8269C410: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C414: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8269C418: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269C41C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C420: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269C424: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C428: 4BDCA9F9  bl 0x82466e20
	ctx.lr = 0x8269C42C;
	sub_82466E20(ctx, base);
	// 8269C42C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C430: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C434: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C438: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269C440 size=24
    let mut pc: u32 = 0x8269C440;
    'dispatch: loop {
        match pc {
            0x8269C440 => {
    //   block [0x8269C440..0x8269C458)
	// 8269C440: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C444: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269C448: 394A7170  addi r10, r10, 0x7170
	ctx.r[10].s64 = ctx.r[10].s64 + 29040;
	// 8269C44C: 816B0EBC  lwz r11, 0xebc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3772 as u32) ) } as u64;
	// 8269C450: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269C454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C458 size=112
    let mut pc: u32 = 0x8269C458;
    'dispatch: loop {
        match pc {
            0x8269C458 => {
    //   block [0x8269C458..0x8269C4C8)
	// 8269C458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C460: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C464: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269C468: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C46C: 392A9E30  addi r9, r10, -0x61d0
	ctx.r[9].s64 = ctx.r[10].s64 + -25040;
	// 8269C470: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C474: 390B7170  addi r8, r11, 0x7170
	ctx.r[8].s64 = ctx.r[11].s64 + 29040;
	// 8269C478: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269C47C: 388ABA94  addi r4, r10, -0x456c
	ctx.r[4].s64 = ctx.r[10].s64 + -17772;
	// 8269C480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C484: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C488: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C48C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C490: 386A42E0  addi r3, r10, 0x42e0
	ctx.r[3].s64 = ctx.r[10].s64 + 17120;
	// 8269C494: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269C498: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8269C49C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C4A0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C4A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C4A8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C4AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C4B0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C4B4: 4BDCA96D  bl 0x82466e20
	ctx.lr = 0x8269C4B8;
	sub_82466E20(ctx, base);
	// 8269C4B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C4C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C4C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C4C8 size=112
    let mut pc: u32 = 0x8269C4C8;
    'dispatch: loop {
        match pc {
            0x8269C4C8 => {
    //   block [0x8269C4C8..0x8269C538)
	// 8269C4C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C4CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C4D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C4D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C4D8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C4DC: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C4E0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C4E4: 390B0EC4  addi r8, r11, 0xec4
	ctx.r[8].s64 = ctx.r[11].s64 + 3780;
	// 8269C4E8: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C4EC: 388ABAA8  addi r4, r10, -0x4558
	ctx.r[4].s64 = ctx.r[10].s64 + -17752;
	// 8269C4F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C4F4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C4F8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C500: 386A4310  addi r3, r10, 0x4310
	ctx.r[3].s64 = ctx.r[10].s64 + 17168;
	// 8269C504: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C508: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C50C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C510: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C514: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C518: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C51C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C520: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C524: 4BDCA8FD  bl 0x82466e20
	ctx.lr = 0x8269C528;
	sub_82466E20(ctx, base);
	// 8269C528: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C52C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C538 size=108
    let mut pc: u32 = 0x8269C538;
    'dispatch: loop {
        match pc {
            0x8269C538 => {
    //   block [0x8269C538..0x8269C5A4)
	// 8269C538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C53C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C544: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C548: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C54C: 38EB0EF4  addi r7, r11, 0xef4
	ctx.r[7].s64 = ctx.r[11].s64 + 3828;
	// 8269C550: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269C554: 388ABAC0  addi r4, r10, -0x4540
	ctx.r[4].s64 = ctx.r[10].s64 + -17728;
	// 8269C558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C55C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C560: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269C564: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C568: 386A4340  addi r3, r10, 0x4340
	ctx.r[3].s64 = ctx.r[10].s64 + 17216;
	// 8269C56C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269C570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C574: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C578: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C57C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C580: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C588: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C58C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269C590: 4BDCA891  bl 0x82466e20
	ctx.lr = 0x8269C594;
	sub_82466E20(ctx, base);
	// 8269C594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C59C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C5A8 size=112
    let mut pc: u32 = 0x8269C5A8;
    'dispatch: loop {
        match pc {
            0x8269C5A8 => {
    //   block [0x8269C5A8..0x8269C618)
	// 8269C5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C5B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C5B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C5B8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C5BC: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C5C0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C5C4: 390B0F10  addi r8, r11, 0xf10
	ctx.r[8].s64 = ctx.r[11].s64 + 3856;
	// 8269C5C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8269C5CC: 388ABAD0  addi r4, r10, -0x4530
	ctx.r[4].s64 = ctx.r[10].s64 + -17712;
	// 8269C5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C5D4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C5D8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C5DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C5E0: 386A4370  addi r3, r10, 0x4370
	ctx.r[3].s64 = ctx.r[10].s64 + 17264;
	// 8269C5E4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C5E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C5EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C5F0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C5F8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C5FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C600: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C604: 4BDCA81D  bl 0x82466e20
	ctx.lr = 0x8269C608;
	sub_82466E20(ctx, base);
	// 8269C608: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C60C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C610: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C618 size=100
    let mut pc: u32 = 0x8269C618;
    'dispatch: loop {
        match pc {
            0x8269C618 => {
    //   block [0x8269C618..0x8269C67C)
	// 8269C618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C624: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C628: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C62C: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C630: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C634: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C638: 388ABAF0  addi r4, r10, -0x4510
	ctx.r[4].s64 = ctx.r[10].s64 + -17680;
	// 8269C63C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C640: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C644: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C648: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C64C: 386A43A0  addi r3, r10, 0x43a0
	ctx.r[3].s64 = ctx.r[10].s64 + 17312;
	// 8269C650: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C654: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C658: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269C65C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C660: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269C664: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C668: 4BDCA7B9  bl 0x82466e20
	ctx.lr = 0x8269C66C;
	sub_82466E20(ctx, base);
	// 8269C66C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C670: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C674: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C678: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C680 size=112
    let mut pc: u32 = 0x8269C680;
    'dispatch: loop {
        match pc {
            0x8269C680 => {
    //   block [0x8269C680..0x8269C6F0)
	// 8269C680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C688: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C68C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C690: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C694: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C698: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C69C: 390B0FD0  addi r8, r11, 0xfd0
	ctx.r[8].s64 = ctx.r[11].s64 + 4048;
	// 8269C6A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269C6A4: 388ABB0C  addi r4, r10, -0x44f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17652;
	// 8269C6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C6AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C6B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C6B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C6B8: 386A43D0  addi r3, r10, 0x43d0
	ctx.r[3].s64 = ctx.r[10].s64 + 17360;
	// 8269C6BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C6C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C6C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C6CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C6D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C6D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C6D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C6DC: 4BDCA745  bl 0x82466e20
	ctx.lr = 0x8269C6E0;
	sub_82466E20(ctx, base);
	// 8269C6E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C6E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C6E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C6EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C6F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C6F0 size=112
    let mut pc: u32 = 0x8269C6F0;
    'dispatch: loop {
        match pc {
            0x8269C6F0 => {
    //   block [0x8269C6F0..0x8269C760)
	// 8269C6F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C6F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C6F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C6FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C700: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C704: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C708: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C70C: 390B0FE8  addi r8, r11, 0xfe8
	ctx.r[8].s64 = ctx.r[11].s64 + 4072;
	// 8269C710: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C714: 388ABB2C  addi r4, r10, -0x44d4
	ctx.r[4].s64 = ctx.r[10].s64 + -17620;
	// 8269C718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C71C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C720: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C724: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C728: 386A4400  addi r3, r10, 0x4400
	ctx.r[3].s64 = ctx.r[10].s64 + 17408;
	// 8269C72C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C730: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C734: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C738: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C73C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C740: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C748: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C74C: 4BDCA6D5  bl 0x82466e20
	ctx.lr = 0x8269C750;
	sub_82466E20(ctx, base);
	// 8269C750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C754: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C758: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C75C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C760 size=112
    let mut pc: u32 = 0x8269C760;
    'dispatch: loop {
        match pc {
            0x8269C760 => {
    //   block [0x8269C760..0x8269C7D0)
	// 8269C760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C768: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C76C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C770: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C774: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C778: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C77C: 390B1018  addi r8, r11, 0x1018
	ctx.r[8].s64 = ctx.r[11].s64 + 4120;
	// 8269C780: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C784: 388ABB50  addi r4, r10, -0x44b0
	ctx.r[4].s64 = ctx.r[10].s64 + -17584;
	// 8269C788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C78C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C790: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C794: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C798: 386A4430  addi r3, r10, 0x4430
	ctx.r[3].s64 = ctx.r[10].s64 + 17456;
	// 8269C79C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C7A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C7A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C7A8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C7AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C7B0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C7B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C7B8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C7BC: 4BDCA665  bl 0x82466e20
	ctx.lr = 0x8269C7C0;
	sub_82466E20(ctx, base);
	// 8269C7C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C7C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C7C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C7CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C7D0 size=112
    let mut pc: u32 = 0x8269C7D0;
    'dispatch: loop {
        match pc {
            0x8269C7D0 => {
    //   block [0x8269C7D0..0x8269C840)
	// 8269C7D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C7D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C7D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C7DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C7E0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C7E4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C7E8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C7EC: 390B1048  addi r8, r11, 0x1048
	ctx.r[8].s64 = ctx.r[11].s64 + 4168;
	// 8269C7F0: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269C7F4: 388ABB78  addi r4, r10, -0x4488
	ctx.r[4].s64 = ctx.r[10].s64 + -17544;
	// 8269C7F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C7FC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C800: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C804: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C808: 386A4460  addi r3, r10, 0x4460
	ctx.r[3].s64 = ctx.r[10].s64 + 17504;
	// 8269C80C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C814: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C818: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C81C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C820: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C828: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C82C: 4BDCA5F5  bl 0x82466e20
	ctx.lr = 0x8269C830;
	sub_82466E20(ctx, base);
	// 8269C830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C834: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C838: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C83C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C840 size=112
    let mut pc: u32 = 0x8269C840;
    'dispatch: loop {
        match pc {
            0x8269C840 => {
    //   block [0x8269C840..0x8269C8B0)
	// 8269C840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C844: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C848: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C84C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C850: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C854: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C858: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C85C: 390B1078  addi r8, r11, 0x1078
	ctx.r[8].s64 = ctx.r[11].s64 + 4216;
	// 8269C860: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269C864: 388ABB9C  addi r4, r10, -0x4464
	ctx.r[4].s64 = ctx.r[10].s64 + -17508;
	// 8269C868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C86C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C870: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C874: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C878: 386A4490  addi r3, r10, 0x4490
	ctx.r[3].s64 = ctx.r[10].s64 + 17552;
	// 8269C87C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C880: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C888: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C88C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C890: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C898: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C89C: 4BDCA585  bl 0x82466e20
	ctx.lr = 0x8269C8A0;
	sub_82466E20(ctx, base);
	// 8269C8A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C8A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C8A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C8B0 size=112
    let mut pc: u32 = 0x8269C8B0;
    'dispatch: loop {
        match pc {
            0x8269C8B0 => {
    //   block [0x8269C8B0..0x8269C920)
	// 8269C8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C8B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C8BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C8C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C8C4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C8C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C8CC: 390B1090  addi r8, r11, 0x1090
	ctx.r[8].s64 = ctx.r[11].s64 + 4240;
	// 8269C8D0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269C8D4: 388ABBBC  addi r4, r10, -0x4444
	ctx.r[4].s64 = ctx.r[10].s64 + -17476;
	// 8269C8D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C8DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C8E0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C8E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C8E8: 386A44C0  addi r3, r10, 0x44c0
	ctx.r[3].s64 = ctx.r[10].s64 + 17600;
	// 8269C8EC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C8F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C8F4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C8F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C8FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C900: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C908: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C90C: 4BDCA515  bl 0x82466e20
	ctx.lr = 0x8269C910;
	sub_82466E20(ctx, base);
	// 8269C910: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C91C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C920 size=112
    let mut pc: u32 = 0x8269C920;
    'dispatch: loop {
        match pc {
            0x8269C920 => {
    //   block [0x8269C920..0x8269C990)
	// 8269C920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C92C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C930: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C934: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C938: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C93C: 390B10A8  addi r8, r11, 0x10a8
	ctx.r[8].s64 = ctx.r[11].s64 + 4264;
	// 8269C940: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269C944: 388ABBD4  addi r4, r10, -0x442c
	ctx.r[4].s64 = ctx.r[10].s64 + -17452;
	// 8269C948: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C94C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C950: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C954: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C958: 386A44F0  addi r3, r10, 0x44f0
	ctx.r[3].s64 = ctx.r[10].s64 + 17648;
	// 8269C95C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C960: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C964: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C968: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C96C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C970: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C978: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C97C: 4BDCA4A5  bl 0x82466e20
	ctx.lr = 0x8269C980;
	sub_82466E20(ctx, base);
	// 8269C980: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C984: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C988: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269C990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269C990 size=112
    let mut pc: u32 = 0x8269C990;
    'dispatch: loop {
        match pc {
            0x8269C990 => {
    //   block [0x8269C990..0x8269CA00)
	// 8269C990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269C994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269C998: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269C99C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C9A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269C9A4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269C9A8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269C9AC: 390B10F0  addi r8, r11, 0x10f0
	ctx.r[8].s64 = ctx.r[11].s64 + 4336;
	// 8269C9B0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269C9B4: 388ABBF0  addi r4, r10, -0x4410
	ctx.r[4].s64 = ctx.r[10].s64 + -17424;
	// 8269C9B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269C9BC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269C9C0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269C9C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269C9C8: 386A4520  addi r3, r10, 0x4520
	ctx.r[3].s64 = ctx.r[10].s64 + 17696;
	// 8269C9CC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269C9D0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269C9D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269C9D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269C9DC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269C9E0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269C9E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269C9E8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269C9EC: 4BDCA435  bl 0x82466e20
	ctx.lr = 0x8269C9F0;
	sub_82466E20(ctx, base);
	// 8269C9F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269C9F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269C9F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269C9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CA00 size=112
    let mut pc: u32 = 0x8269CA00;
    'dispatch: loop {
        match pc {
            0x8269CA00 => {
    //   block [0x8269CA00..0x8269CA70)
	// 8269CA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CA0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CA10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CA14: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CA18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CA1C: 390B1138  addi r8, r11, 0x1138
	ctx.r[8].s64 = ctx.r[11].s64 + 4408;
	// 8269CA20: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269CA24: 388ABC0C  addi r4, r10, -0x43f4
	ctx.r[4].s64 = ctx.r[10].s64 + -17396;
	// 8269CA28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CA2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CA30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CA34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CA38: 386A4550  addi r3, r10, 0x4550
	ctx.r[3].s64 = ctx.r[10].s64 + 17744;
	// 8269CA3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CA40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CA44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CA48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CA4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CA50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CA54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CA58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CA5C: 4BDCA3C5  bl 0x82466e20
	ctx.lr = 0x8269CA60;
	sub_82466E20(ctx, base);
	// 8269CA60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CA64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CA68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CA6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CA70 size=112
    let mut pc: u32 = 0x8269CA70;
    'dispatch: loop {
        match pc {
            0x8269CA70 => {
    //   block [0x8269CA70..0x8269CAE0)
	// 8269CA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CA78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CA7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CA80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CA84: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CA88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CA8C: 390B1150  addi r8, r11, 0x1150
	ctx.r[8].s64 = ctx.r[11].s64 + 4432;
	// 8269CA90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269CA94: 388ABC24  addi r4, r10, -0x43dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17372;
	// 8269CA98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CA9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CAA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CAA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CAA8: 386A4580  addi r3, r10, 0x4580
	ctx.r[3].s64 = ctx.r[10].s64 + 17792;
	// 8269CAAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CAB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CAB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CABC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CAC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CAC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CAC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CACC: 4BDCA355  bl 0x82466e20
	ctx.lr = 0x8269CAD0;
	sub_82466E20(ctx, base);
	// 8269CAD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CAD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CAD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CAE0 size=116
    let mut pc: u32 = 0x8269CAE0;
    'dispatch: loop {
        match pc {
            0x8269CAE0 => {
    //   block [0x8269CAE0..0x8269CB54)
	// 8269CAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CAE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CAEC: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269CAF0: 38E00005  li r7, 5
	ctx.r[7].s64 = 5;
	// 8269CAF4: 390A1180  addi r8, r10, 0x1180
	ctx.r[8].s64 = ctx.r[10].s64 + 4480;
	// 8269CAF8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CAFC: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269CB00: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CB04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CB08: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269CB0C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CB10: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CB14: 388ABC3C  addi r4, r10, -0x43c4
	ctx.r[4].s64 = ctx.r[10].s64 + -17348;
	// 8269CB18: 396B9E58  addi r11, r11, -0x61a8
	ctx.r[11].s64 = ctx.r[11].s64 + -25000;
	// 8269CB1C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CB20: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CB24: 386A45B0  addi r3, r10, 0x45b0
	ctx.r[3].s64 = ctx.r[10].s64 + 17840;
	// 8269CB28: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269CB2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CB30: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269CB34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CB38: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CB3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CB40: 4BDCA2E1  bl 0x82466e20
	ctx.lr = 0x8269CB44;
	sub_82466E20(ctx, base);
	// 8269CB44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CB48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CB4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CB50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CB58 size=116
    let mut pc: u32 = 0x8269CB58;
    'dispatch: loop {
        match pc {
            0x8269CB58 => {
    //   block [0x8269CB58..0x8269CBCC)
	// 8269CB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CB5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CB60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CB64: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269CB68: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8269CB6C: 390A11F8  addi r8, r10, 0x11f8
	ctx.r[8].s64 = ctx.r[10].s64 + 4600;
	// 8269CB70: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CB74: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269CB78: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CB7C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CB80: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269CB84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CB88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CB8C: 388ABC58  addi r4, r10, -0x43a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17320;
	// 8269CB90: 396B9E70  addi r11, r11, -0x6190
	ctx.r[11].s64 = ctx.r[11].s64 + -24976;
	// 8269CB94: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CB98: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CB9C: 386A45E0  addi r3, r10, 0x45e0
	ctx.r[3].s64 = ctx.r[10].s64 + 17888;
	// 8269CBA0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8269CBA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CBA8: 9121006C  stw r9, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[9].u32 ) };
	// 8269CBAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CBB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CBB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CBB8: 4BDCA269  bl 0x82466e20
	ctx.lr = 0x8269CBBC;
	sub_82466E20(ctx, base);
	// 8269CBBC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CBC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CBC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CBC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269CBD0 size=24
    let mut pc: u32 = 0x8269CBD0;
    'dispatch: loop {
        match pc {
            0x8269CBD0 => {
    //   block [0x8269CBD0..0x8269CBE8)
	// 8269CBD0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CBD4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269CBD8: 394A7188  addi r10, r10, 0x7188
	ctx.r[10].s64 = ctx.r[10].s64 + 29064;
	// 8269CBDC: 816B0F0C  lwz r11, 0xf0c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3852 as u32) ) } as u64;
	// 8269CBE0: 916A0050  stw r11, 0x50(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8269CBE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CBE8 size=116
    let mut pc: u32 = 0x8269CBE8;
    'dispatch: loop {
        match pc {
            0x8269CBE8 => {
    //   block [0x8269CBE8..0x8269CC5C)
	// 8269CBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CBEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CBF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CBF4: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 8269CBF8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CBFC: 392B9E9C  addi r9, r11, -0x6164
	ctx.r[9].s64 = ctx.r[11].s64 + -24932;
	// 8269CC00: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CC04: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CC08: 38E90014  addi r7, r9, 0x14
	ctx.r[7].s64 = ctx.r[9].s64 + 20;
	// 8269CC0C: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 8269CC10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CC14: 388ABC8C  addi r4, r10, -0x4374
	ctx.r[4].s64 = ctx.r[10].s64 + -17268;
	// 8269CC18: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CC1C: 396B7188  addi r11, r11, 0x7188
	ctx.r[11].s64 = ctx.r[11].s64 + 29064;
	// 8269CC20: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 8269CC24: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CC28: 90C1005C  stw r6, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[6].u32 ) };
	// 8269CC2C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CC30: 386A4610  addi r3, r10, 0x4610
	ctx.r[3].s64 = ctx.r[10].s64 + 17936;
	// 8269CC34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269CC38: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 8269CC3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CC40: 9101006C  stw r8, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[8].u32 ) };
	// 8269CC44: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269CC48: 4BDCA1D9  bl 0x82466e20
	ctx.lr = 0x8269CC4C;
	sub_82466E20(ctx, base);
	// 8269CC4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CC50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CC54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CC58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CC60 size=112
    let mut pc: u32 = 0x8269CC60;
    'dispatch: loop {
        match pc {
            0x8269CC60 => {
    //   block [0x8269CC60..0x8269CCD0)
	// 8269CC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CC68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CC6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CC70: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CC74: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CC78: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CC7C: 390B1288  addi r8, r11, 0x1288
	ctx.r[8].s64 = ctx.r[11].s64 + 4744;
	// 8269CC80: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269CC84: 388ABCA8  addi r4, r10, -0x4358
	ctx.r[4].s64 = ctx.r[10].s64 + -17240;
	// 8269CC88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CC8C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CC90: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CC94: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CC98: 386A4640  addi r3, r10, 0x4640
	ctx.r[3].s64 = ctx.r[10].s64 + 17984;
	// 8269CC9C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CCA0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CCA4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CCA8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CCAC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CCB0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CCB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CCB8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CCBC: 4BDCA165  bl 0x82466e20
	ctx.lr = 0x8269CCC0;
	sub_82466E20(ctx, base);
	// 8269CCC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CCC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CCC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CCD0 size=112
    let mut pc: u32 = 0x8269CCD0;
    'dispatch: loop {
        match pc {
            0x8269CCD0 => {
    //   block [0x8269CCD0..0x8269CD40)
	// 8269CCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CCD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CCDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CCE0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CCE4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CCE8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CCEC: 390B12E8  addi r8, r11, 0x12e8
	ctx.r[8].s64 = ctx.r[11].s64 + 4840;
	// 8269CCF0: 39200007  li r9, 7
	ctx.r[9].s64 = 7;
	// 8269CCF4: 388ABCC8  addi r4, r10, -0x4338
	ctx.r[4].s64 = ctx.r[10].s64 + -17208;
	// 8269CCF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CCFC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CD00: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CD04: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CD08: 386A4670  addi r3, r10, 0x4670
	ctx.r[3].s64 = ctx.r[10].s64 + 18032;
	// 8269CD0C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CD10: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CD14: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CD18: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CD1C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CD20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CD24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CD28: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CD2C: 4BDCA0F5  bl 0x82466e20
	ctx.lr = 0x8269CD30;
	sub_82466E20(ctx, base);
	// 8269CD30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CD34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CD38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CD40 size=112
    let mut pc: u32 = 0x8269CD40;
    'dispatch: loop {
        match pc {
            0x8269CD40 => {
    //   block [0x8269CD40..0x8269CDB0)
	// 8269CD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CD48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CD4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CD50: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CD54: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CD58: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CD5C: 390B1390  addi r8, r11, 0x1390
	ctx.r[8].s64 = ctx.r[11].s64 + 5008;
	// 8269CD60: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8269CD64: 388ABCE4  addi r4, r10, -0x431c
	ctx.r[4].s64 = ctx.r[10].s64 + -17180;
	// 8269CD68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CD6C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CD70: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CD74: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CD78: 386A46A0  addi r3, r10, 0x46a0
	ctx.r[3].s64 = ctx.r[10].s64 + 18080;
	// 8269CD7C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CD80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CD84: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CD88: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CD8C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CD90: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CD98: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CD9C: 4BDCA085  bl 0x82466e20
	ctx.lr = 0x8269CDA0;
	sub_82466E20(ctx, base);
	// 8269CDA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CDA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CDA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CDB0 size=112
    let mut pc: u32 = 0x8269CDB0;
    'dispatch: loop {
        match pc {
            0x8269CDB0 => {
    //   block [0x8269CDB0..0x8269CE20)
	// 8269CDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CDB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CDBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CDC0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CDC4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CDC8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CDCC: 390B1408  addi r8, r11, 0x1408
	ctx.r[8].s64 = ctx.r[11].s64 + 5128;
	// 8269CDD0: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269CDD4: 388ABD04  addi r4, r10, -0x42fc
	ctx.r[4].s64 = ctx.r[10].s64 + -17148;
	// 8269CDD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CDDC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CDE0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CDE4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CDE8: 386A46D0  addi r3, r10, 0x46d0
	ctx.r[3].s64 = ctx.r[10].s64 + 18128;
	// 8269CDEC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CDF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CDF4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CDF8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CDFC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CE00: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CE04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CE08: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CE0C: 4BDCA015  bl 0x82466e20
	ctx.lr = 0x8269CE10;
	sub_82466E20(ctx, base);
	// 8269CE10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CE14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CE18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CE20 size=112
    let mut pc: u32 = 0x8269CE20;
    'dispatch: loop {
        match pc {
            0x8269CE20 => {
    //   block [0x8269CE20..0x8269CE90)
	// 8269CE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CE28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CE2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CE30: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CE34: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CE38: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CE3C: 390B1450  addi r8, r11, 0x1450
	ctx.r[8].s64 = ctx.r[11].s64 + 5200;
	// 8269CE40: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8269CE44: 388ABD24  addi r4, r10, -0x42dc
	ctx.r[4].s64 = ctx.r[10].s64 + -17116;
	// 8269CE48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CE4C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CE50: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CE54: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CE58: 386A4700  addi r3, r10, 0x4700
	ctx.r[3].s64 = ctx.r[10].s64 + 18176;
	// 8269CE5C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CE60: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CE64: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CE68: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CE6C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CE70: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CE74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CE78: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CE7C: 4BDC9FA5  bl 0x82466e20
	ctx.lr = 0x8269CE80;
	sub_82466E20(ctx, base);
	// 8269CE80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CE84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CE88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CE90 size=112
    let mut pc: u32 = 0x8269CE90;
    'dispatch: loop {
        match pc {
            0x8269CE90 => {
    //   block [0x8269CE90..0x8269CF00)
	// 8269CE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CE98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CE9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CEA0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CEA4: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CEA8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CEAC: 390B14E0  addi r8, r11, 0x14e0
	ctx.r[8].s64 = ctx.r[11].s64 + 5344;
	// 8269CEB0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269CEB4: 388ABD40  addi r4, r10, -0x42c0
	ctx.r[4].s64 = ctx.r[10].s64 + -17088;
	// 8269CEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CEBC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CEC0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CEC4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CEC8: 386A4730  addi r3, r10, 0x4730
	ctx.r[3].s64 = ctx.r[10].s64 + 18224;
	// 8269CECC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CED0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CED4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CED8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CEDC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CEE0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CEE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CEE8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CEEC: 4BDC9F35  bl 0x82466e20
	ctx.lr = 0x8269CEF0;
	sub_82466E20(ctx, base);
	// 8269CEF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CEF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CEF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CF00 size=112
    let mut pc: u32 = 0x8269CF00;
    'dispatch: loop {
        match pc {
            0x8269CF00 => {
    //   block [0x8269CF00..0x8269CF70)
	// 8269CF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CF08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CF0C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CF10: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CF14: 38AA42E0  addi r5, r10, 0x42e0
	ctx.r[5].s64 = ctx.r[10].s64 + 17120;
	// 8269CF18: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CF1C: 390B1540  addi r8, r11, 0x1540
	ctx.r[8].s64 = ctx.r[11].s64 + 5440;
	// 8269CF20: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269CF24: 388ABD58  addi r4, r10, -0x42a8
	ctx.r[4].s64 = ctx.r[10].s64 + -17064;
	// 8269CF28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CF2C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CF30: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CF34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CF38: 386A4760  addi r3, r10, 0x4760
	ctx.r[3].s64 = ctx.r[10].s64 + 18272;
	// 8269CF3C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CF40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CF44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CF48: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CF4C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CF50: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CF54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CF58: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CF5C: 4BDC9EC5  bl 0x82466e20
	ctx.lr = 0x8269CF60;
	sub_82466E20(ctx, base);
	// 8269CF60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CF64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CF68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CF70 size=112
    let mut pc: u32 = 0x8269CF70;
    'dispatch: loop {
        match pc {
            0x8269CF70 => {
    //   block [0x8269CF70..0x8269CFE0)
	// 8269CF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CF78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CF7C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CF80: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CF84: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269CF88: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CF8C: 390B15A0  addi r8, r11, 0x15a0
	ctx.r[8].s64 = ctx.r[11].s64 + 5536;
	// 8269CF90: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269CF94: 388ABD74  addi r4, r10, -0x428c
	ctx.r[4].s64 = ctx.r[10].s64 + -17036;
	// 8269CF98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269CF9C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CFA0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269CFA4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269CFA8: 386A4790  addi r3, r10, 0x4790
	ctx.r[3].s64 = ctx.r[10].s64 + 18320;
	// 8269CFAC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269CFB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269CFB4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269CFB8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269CFBC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269CFC0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269CFC4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269CFC8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269CFCC: 4BDC9E55  bl 0x82466e20
	ctx.lr = 0x8269CFD0;
	sub_82466E20(ctx, base);
	// 8269CFD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269CFD4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269CFD8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269CFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269CFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269CFE0 size=112
    let mut pc: u32 = 0x8269CFE0;
    'dispatch: loop {
        match pc {
            0x8269CFE0 => {
    //   block [0x8269CFE0..0x8269D050)
	// 8269CFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269CFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269CFE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269CFEC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269CFF0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269CFF4: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269CFF8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269CFFC: 390B15D0  addi r8, r11, 0x15d0
	ctx.r[8].s64 = ctx.r[11].s64 + 5584;
	// 8269D000: 39200003  li r9, 3
	ctx.r[9].s64 = 3;
	// 8269D004: 388ABD9C  addi r4, r10, -0x4264
	ctx.r[4].s64 = ctx.r[10].s64 + -16996;
	// 8269D008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D00C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D010: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D014: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D018: 386A47C0  addi r3, r10, 0x47c0
	ctx.r[3].s64 = ctx.r[10].s64 + 18368;
	// 8269D01C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D020: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D024: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D028: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D02C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D030: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D034: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D038: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D03C: 4BDC9DE5  bl 0x82466e20
	ctx.lr = 0x8269D040;
	sub_82466E20(ctx, base);
	// 8269D040: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D050 size=100
    let mut pc: u32 = 0x8269D050;
    'dispatch: loop {
        match pc {
            0x8269D050 => {
    //   block [0x8269D050..0x8269D0B4)
	// 8269D050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D05C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D064: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269D068: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D06C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D070: 388ABDC4  addi r4, r10, -0x423c
	ctx.r[4].s64 = ctx.r[10].s64 + -16956;
	// 8269D074: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D078: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D07C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D080: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D084: 386A47F0  addi r3, r10, 0x47f0
	ctx.r[3].s64 = ctx.r[10].s64 + 18416;
	// 8269D088: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D08C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D090: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269D094: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D098: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269D09C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D0A0: 4BDC9D81  bl 0x82466e20
	ctx.lr = 0x8269D0A4;
	sub_82466E20(ctx, base);
	// 8269D0A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D0A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D0AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D0B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D0B8 size=112
    let mut pc: u32 = 0x8269D0B8;
    'dispatch: loop {
        match pc {
            0x8269D0B8 => {
    //   block [0x8269D0B8..0x8269D128)
	// 8269D0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D0C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D0C4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D0C8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D0CC: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269D0D0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D0D4: 390B1618  addi r8, r11, 0x1618
	ctx.r[8].s64 = ctx.r[11].s64 + 5656;
	// 8269D0D8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D0DC: 388ABDEC  addi r4, r10, -0x4214
	ctx.r[4].s64 = ctx.r[10].s64 + -16916;
	// 8269D0E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D0E4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D0E8: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D0EC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D0F0: 386A4820  addi r3, r10, 0x4820
	ctx.r[3].s64 = ctx.r[10].s64 + 18464;
	// 8269D0F4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D0F8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D0FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D100: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D104: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D108: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D10C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D110: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D114: 4BDC9D0D  bl 0x82466e20
	ctx.lr = 0x8269D118;
	sub_82466E20(ctx, base);
	// 8269D118: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D128 size=100
    let mut pc: u32 = 0x8269D128;
    'dispatch: loop {
        match pc {
            0x8269D128 => {
    //   block [0x8269D128..0x8269D18C)
	// 8269D128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D12C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D130: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D134: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D13C: 38AA4760  addi r5, r10, 0x4760
	ctx.r[5].s64 = ctx.r[10].s64 + 18272;
	// 8269D140: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D144: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D148: 388AB3BC  addi r4, r10, -0x4c44
	ctx.r[4].s64 = ctx.r[10].s64 + -19524;
	// 8269D14C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D150: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D154: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D158: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D15C: 386A4850  addi r3, r10, 0x4850
	ctx.r[3].s64 = ctx.r[10].s64 + 18512;
	// 8269D160: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D164: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D168: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8269D16C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D170: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8269D174: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D178: 4BDC9CA9  bl 0x82466e20
	ctx.lr = 0x8269D17C;
	sub_82466E20(ctx, base);
	// 8269D17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D190 size=108
    let mut pc: u32 = 0x8269D190;
    'dispatch: loop {
        match pc {
            0x8269D190 => {
    //   block [0x8269D190..0x8269D1FC)
	// 8269D190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D198: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D19C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D1A0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D1A4: 38EB1630  addi r7, r11, 0x1630
	ctx.r[7].s64 = ctx.r[11].s64 + 5680;
	// 8269D1A8: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269D1AC: 388ABE14  addi r4, r10, -0x41ec
	ctx.r[4].s64 = ctx.r[10].s64 + -16876;
	// 8269D1B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D1B4: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D1B8: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269D1BC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D1C0: 386A4880  addi r3, r10, 0x4880
	ctx.r[3].s64 = ctx.r[10].s64 + 18560;
	// 8269D1C4: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269D1C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D1CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D1D0: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D1D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D1D8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D1DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D1E0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D1E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D1E8: 4BDC9C39  bl 0x82466e20
	ctx.lr = 0x8269D1EC;
	sub_82466E20(ctx, base);
	// 8269D1EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D200 size=112
    let mut pc: u32 = 0x8269D200;
    'dispatch: loop {
        match pc {
            0x8269D200 => {
    //   block [0x8269D200..0x8269D270)
	// 8269D200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D208: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D20C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D210: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D214: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269D218: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D21C: 390B1678  addi r8, r11, 0x1678
	ctx.r[8].s64 = ctx.r[11].s64 + 5752;
	// 8269D220: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8269D224: 388ABE38  addi r4, r10, -0x41c8
	ctx.r[4].s64 = ctx.r[10].s64 + -16840;
	// 8269D228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D22C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D230: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D234: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D238: 386A48B0  addi r3, r10, 0x48b0
	ctx.r[3].s64 = ctx.r[10].s64 + 18608;
	// 8269D23C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D240: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D244: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D248: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D24C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D250: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D254: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D258: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D25C: 4BDC9BC5  bl 0x82466e20
	ctx.lr = 0x8269D260;
	sub_82466E20(ctx, base);
	// 8269D260: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D270 size=112
    let mut pc: u32 = 0x8269D270;
    'dispatch: loop {
        match pc {
            0x8269D270 => {
    //   block [0x8269D270..0x8269D2E0)
	// 8269D270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D278: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D27C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D280: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D284: 38AA48B0  addi r5, r10, 0x48b0
	ctx.r[5].s64 = ctx.r[10].s64 + 18608;
	// 8269D288: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D28C: 390B16D8  addi r8, r11, 0x16d8
	ctx.r[8].s64 = ctx.r[11].s64 + 5848;
	// 8269D290: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D294: 388ABE44  addi r4, r10, -0x41bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16828;
	// 8269D298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D29C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D2A0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D2A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D2A8: 386A48E0  addi r3, r10, 0x48e0
	ctx.r[3].s64 = ctx.r[10].s64 + 18656;
	// 8269D2AC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D2B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D2B4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D2B8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D2BC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D2C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D2C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D2C8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D2CC: 4BDC9B55  bl 0x82466e20
	ctx.lr = 0x8269D2D0;
	sub_82466E20(ctx, base);
	// 8269D2D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D2D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D2D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D2E0 size=112
    let mut pc: u32 = 0x8269D2E0;
    'dispatch: loop {
        match pc {
            0x8269D2E0 => {
    //   block [0x8269D2E0..0x8269D350)
	// 8269D2E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D2E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D2E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D2EC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D2F0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D2F4: 38AA48B0  addi r5, r10, 0x48b0
	ctx.r[5].s64 = ctx.r[10].s64 + 18608;
	// 8269D2F8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D2FC: 390B16F0  addi r8, r11, 0x16f0
	ctx.r[8].s64 = ctx.r[11].s64 + 5872;
	// 8269D300: 39200002  li r9, 2
	ctx.r[9].s64 = 2;
	// 8269D304: 388ABE54  addi r4, r10, -0x41ac
	ctx.r[4].s64 = ctx.r[10].s64 + -16812;
	// 8269D308: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D30C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D310: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D314: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D318: 386A4910  addi r3, r10, 0x4910
	ctx.r[3].s64 = ctx.r[10].s64 + 18704;
	// 8269D31C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D320: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D328: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D32C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D330: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D334: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D338: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D33C: 4BDC9AE5  bl 0x82466e20
	ctx.lr = 0x8269D340;
	sub_82466E20(ctx, base);
	// 8269D340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D34C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D350 size=112
    let mut pc: u32 = 0x8269D350;
    'dispatch: loop {
        match pc {
            0x8269D350 => {
    //   block [0x8269D350..0x8269D3C0)
	// 8269D350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D35C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D360: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D364: 38AA48B0  addi r5, r10, 0x48b0
	ctx.r[5].s64 = ctx.r[10].s64 + 18608;
	// 8269D368: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D36C: 390B1720  addi r8, r11, 0x1720
	ctx.r[8].s64 = ctx.r[11].s64 + 5920;
	// 8269D370: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D374: 388ABE64  addi r4, r10, -0x419c
	ctx.r[4].s64 = ctx.r[10].s64 + -16796;
	// 8269D378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D37C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D380: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D384: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D388: 386A4940  addi r3, r10, 0x4940
	ctx.r[3].s64 = ctx.r[10].s64 + 18752;
	// 8269D38C: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D390: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D394: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D398: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D39C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D3A0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D3A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D3A8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D3AC: 4BDC9A75  bl 0x82466e20
	ctx.lr = 0x8269D3B0;
	sub_82466E20(ctx, base);
	// 8269D3B0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D3B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D3B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269D3C0 size=24
    let mut pc: u32 = 0x8269D3C0;
    'dispatch: loop {
        match pc {
            0x8269D3C0 => {
    //   block [0x8269D3C0..0x8269D3D8)
	// 8269D3C0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D3C4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269D3C8: 394A7248  addi r10, r10, 0x7248
	ctx.r[10].s64 = ctx.r[10].s64 + 29256;
	// 8269D3CC: 816B1738  lwz r11, 0x1738(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5944 as u32) ) } as u64;
	// 8269D3D0: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8269D3D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D3D8 size=112
    let mut pc: u32 = 0x8269D3D8;
    'dispatch: loop {
        match pc {
            0x8269D3D8 => {
    //   block [0x8269D3D8..0x8269D448)
	// 8269D3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D3E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D3E4: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D3E8: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D3EC: 392A9F00  addi r9, r10, -0x6100
	ctx.r[9].s64 = ctx.r[10].s64 + -24832;
	// 8269D3F0: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D3F4: 390B7248  addi r8, r11, 0x7248
	ctx.r[8].s64 = ctx.r[11].s64 + 29256;
	// 8269D3F8: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269D3FC: 388ABE74  addi r4, r10, -0x418c
	ctx.r[4].s64 = ctx.r[10].s64 + -16780;
	// 8269D400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D404: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D408: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D40C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D410: 386A4970  addi r3, r10, 0x4970
	ctx.r[3].s64 = ctx.r[10].s64 + 18800;
	// 8269D414: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D418: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269D41C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D420: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D424: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D428: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D42C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D430: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D434: 4BDC99ED  bl 0x82466e20
	ctx.lr = 0x8269D438;
	sub_82466E20(ctx, base);
	// 8269D438: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D43C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D448 size=108
    let mut pc: u32 = 0x8269D448;
    'dispatch: loop {
        match pc {
            0x8269D448 => {
    //   block [0x8269D448..0x8269D4B4)
	// 8269D448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D450: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D454: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D458: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D45C: 38EB173C  addi r7, r11, 0x173c
	ctx.r[7].s64 = ctx.r[11].s64 + 5948;
	// 8269D460: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8269D464: 388ABE80  addi r4, r10, -0x4180
	ctx.r[4].s64 = ctx.r[10].s64 + -16768;
	// 8269D468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D46C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D470: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269D474: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D478: 386A49A0  addi r3, r10, 0x49a0
	ctx.r[3].s64 = ctx.r[10].s64 + 18848;
	// 8269D47C: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269D480: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D484: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D488: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D48C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D490: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D494: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D498: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D49C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D4A0: 4BDC9981  bl 0x82466e20
	ctx.lr = 0x8269D4A4;
	sub_82466E20(ctx, base);
	// 8269D4A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D4A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D4AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D4B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D4B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D4B8 size=108
    let mut pc: u32 = 0x8269D4B8;
    'dispatch: loop {
        match pc {
            0x8269D4B8 => {
    //   block [0x8269D4B8..0x8269D524)
	// 8269D4B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D4BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D4C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D4C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D4C8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D4CC: 38EB1758  addi r7, r11, 0x1758
	ctx.r[7].s64 = ctx.r[11].s64 + 5976;
	// 8269D4D0: 39000003  li r8, 3
	ctx.r[8].s64 = 3;
	// 8269D4D4: 388ABE94  addi r4, r10, -0x416c
	ctx.r[4].s64 = ctx.r[10].s64 + -16748;
	// 8269D4D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D4DC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D4E0: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269D4E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D4E8: 386A49D0  addi r3, r10, 0x49d0
	ctx.r[3].s64 = ctx.r[10].s64 + 18896;
	// 8269D4EC: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269D4F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D4F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D4F8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D4FC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D500: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D504: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D508: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D50C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D510: 4BDC9911  bl 0x82466e20
	ctx.lr = 0x8269D514;
	sub_82466E20(ctx, base);
	// 8269D514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D51C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D528 size=116
    let mut pc: u32 = 0x8269D528;
    'dispatch: loop {
        match pc {
            0x8269D528 => {
    //   block [0x8269D528..0x8269D59C)
	// 8269D528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D534: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D538: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D53C: 390B17A0  addi r8, r11, 0x17a0
	ctx.r[8].s64 = ctx.r[11].s64 + 6048;
	// 8269D540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D544: 392A9FC8  addi r9, r10, -0x6038
	ctx.r[9].s64 = ctx.r[10].s64 + -24632;
	// 8269D548: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D54C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8269D550: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269D554: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D558: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D55C: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D560: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D564: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D568: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D56C: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269D570: 388ABEA0  addi r4, r10, -0x4160
	ctx.r[4].s64 = ctx.r[10].s64 + -16736;
	// 8269D574: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D578: 386B4A00  addi r3, r11, 0x4a00
	ctx.r[3].s64 = ctx.r[11].s64 + 18944;
	// 8269D57C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269D580: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D584: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D588: 4BDC9899  bl 0x82466e20
	ctx.lr = 0x8269D58C;
	sub_82466E20(ctx, base);
	// 8269D58C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8269D5A0 size=24
    let mut pc: u32 = 0x8269D5A0;
    'dispatch: loop {
        match pc {
            0x8269D5A0 => {
    //   block [0x8269D5A0..0x8269D5B8)
	// 8269D5A0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D5A4: 3D40827C  lis r10, -0x7d84
	ctx.r[10].s64 = -2105802752;
	// 8269D5A8: 394A7290  addi r10, r10, 0x7290
	ctx.r[10].s64 = ctx.r[10].s64 + 29328;
	// 8269D5AC: 816B17B8  lwz r11, 0x17b8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6072 as u32) ) } as u64;
	// 8269D5B0: 916A0068  stw r11, 0x68(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8269D5B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D5B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D5B8 size=116
    let mut pc: u32 = 0x8269D5B8;
    'dispatch: loop {
        match pc {
            0x8269D5B8 => {
    //   block [0x8269D5B8..0x8269D62C)
	// 8269D5B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D5BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D5C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D5C4: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D5C8: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D5CC: 390B7290  addi r8, r11, 0x7290
	ctx.r[8].s64 = ctx.r[11].s64 + 29328;
	// 8269D5D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D5D4: 392AA038  addi r9, r10, -0x5fc8
	ctx.r[9].s64 = ctx.r[10].s64 + -24520;
	// 8269D5D8: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D5DC: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 8269D5E0: 38AA2B10  addi r5, r10, 0x2b10
	ctx.r[5].s64 = ctx.r[10].s64 + 11024;
	// 8269D5E4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D5E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D5EC: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D5F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D5F4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D5F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D5FC: 3D608297  lis r11, -0x7d69
	ctx.r[11].s64 = -2104033280;
	// 8269D600: 388ABEB4  addi r4, r10, -0x414c
	ctx.r[4].s64 = ctx.r[10].s64 + -16716;
	// 8269D604: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D608: 386B4A30  addi r3, r11, 0x4a30
	ctx.r[3].s64 = ctx.r[11].s64 + 18992;
	// 8269D60C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8269D610: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D614: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D618: 4BDC9809  bl 0x82466e20
	ctx.lr = 0x8269D61C;
	sub_82466E20(ctx, base);
	// 8269D61C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D620: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D624: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D628: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D630 size=108
    let mut pc: u32 = 0x8269D630;
    'dispatch: loop {
        match pc {
            0x8269D630 => {
    //   block [0x8269D630..0x8269D69C)
	// 8269D630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D63C: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D640: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D644: 38EB17C8  addi r7, r11, 0x17c8
	ctx.r[7].s64 = ctx.r[11].s64 + 6088;
	// 8269D648: 39000002  li r8, 2
	ctx.r[8].s64 = 2;
	// 8269D64C: 388ABECC  addi r4, r10, -0x4134
	ctx.r[4].s64 = ctx.r[10].s64 + -16692;
	// 8269D650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D654: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D658: 90E10054  stw r7, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[7].u32 ) };
	// 8269D65C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D660: 386A4A60  addi r3, r10, 0x4a60
	ctx.r[3].s64 = ctx.r[10].s64 + 19040;
	// 8269D664: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 8269D668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D66C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D670: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D674: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D678: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D67C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D680: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D684: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D688: 4BDC9799  bl 0x82466e20
	ctx.lr = 0x8269D68C;
	sub_82466E20(ctx, base);
	// 8269D68C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D698: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D6A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D6A0 size=112
    let mut pc: u32 = 0x8269D6A0;
    'dispatch: loop {
        match pc {
            0x8269D6A0 => {
    //   block [0x8269D6A0..0x8269D710)
	// 8269D6A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D6A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D6A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D6AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D6B0: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D6B4: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269D6B8: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D6BC: 390B17F8  addi r8, r11, 0x17f8
	ctx.r[8].s64 = ctx.r[11].s64 + 6136;
	// 8269D6C0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D6C4: 388ABEF0  addi r4, r10, -0x4110
	ctx.r[4].s64 = ctx.r[10].s64 + -16656;
	// 8269D6C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D6CC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D6D0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D6D4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D6D8: 386A4A90  addi r3, r10, 0x4a90
	ctx.r[3].s64 = ctx.r[10].s64 + 19088;
	// 8269D6DC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D6E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D6E4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D6E8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D6EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D6F0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D6F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D6F8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D6FC: 4BDC9725  bl 0x82466e20
	ctx.lr = 0x8269D700;
	sub_82466E20(ctx, base);
	// 8269D700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D710 size=112
    let mut pc: u32 = 0x8269D710;
    'dispatch: loop {
        match pc {
            0x8269D710 => {
    //   block [0x8269D710..0x8269D780)
	// 8269D710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D71C: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D720: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D724: 392AA090  addi r9, r10, -0x5f70
	ctx.r[9].s64 = ctx.r[10].s64 + -24432;
	// 8269D728: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D72C: 390B1818  addi r8, r11, 0x1818
	ctx.r[8].s64 = ctx.r[11].s64 + 6168;
	// 8269D730: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8269D734: 388ABF10  addi r4, r10, -0x40f0
	ctx.r[4].s64 = ctx.r[10].s64 + -16624;
	// 8269D738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D73C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D740: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D744: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D748: 386A4AC0  addi r3, r10, 0x4ac0
	ctx.r[3].s64 = ctx.r[10].s64 + 19136;
	// 8269D74C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D750: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269D754: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D758: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D75C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D760: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D764: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D768: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D76C: 4BDC96B5  bl 0x82466e20
	ctx.lr = 0x8269D770;
	sub_82466E20(ctx, base);
	// 8269D770: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D774: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D778: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D780 size=112
    let mut pc: u32 = 0x8269D780;
    'dispatch: loop {
        match pc {
            0x8269D780 => {
    //   block [0x8269D780..0x8269D7F0)
	// 8269D780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D78C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D790: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D794: 38AA4A00  addi r5, r10, 0x4a00
	ctx.r[5].s64 = ctx.r[10].s64 + 18944;
	// 8269D798: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D79C: 390B1860  addi r8, r11, 0x1860
	ctx.r[8].s64 = ctx.r[11].s64 + 6240;
	// 8269D7A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8269D7A4: 388ABF2C  addi r4, r10, -0x40d4
	ctx.r[4].s64 = ctx.r[10].s64 + -16596;
	// 8269D7A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D7AC: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D7B0: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D7B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D7B8: 386A4AF0  addi r3, r10, 0x4af0
	ctx.r[3].s64 = ctx.r[10].s64 + 19184;
	// 8269D7BC: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 8269D7C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8269D7C4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8269D7C8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D7CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D7D0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D7D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D7D8: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D7DC: 4BDC9645  bl 0x82466e20
	ctx.lr = 0x8269D7E0;
	sub_82466E20(ctx, base);
	// 8269D7E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D7E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D7E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8269D7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8269D7F0 size=112
    let mut pc: u32 = 0x8269D7F0;
    'dispatch: loop {
        match pc {
            0x8269D7F0 => {
    //   block [0x8269D7F0..0x8269D860)
	// 8269D7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8269D7F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8269D7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8269D7FC: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 8269D800: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 8269D804: 392AA0BC  addi r9, r10, -0x5f44
	ctx.r[9].s64 = ctx.r[10].s64 + -24388;
	// 8269D808: 3D408203  lis r10, -0x7dfd
	ctx.r[10].s64 = -2113732608;
	// 8269D80C: 390B1878  addi r8, r11, 0x1878
	ctx.r[8].s64 = ctx.r[11].s64 + 6264;
	// 8269D810: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 8269D814: 388ABF44  addi r4, r10, -0x40bc
	ctx.r[4].s64 = ctx.r[10].s64 + -16572;
	// 8269D818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8269D81C: 3D408297  lis r10, -0x7d69
	ctx.r[10].s64 = -2104033280;
	// 8269D820: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 8269D824: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8269D828: 386A4B20  addi r3, r10, 0x4b20
	ctx.r[3].s64 = ctx.r[10].s64 + 19232;
	// 8269D82C: 90E1005C  stw r7, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[7].u32 ) };
	// 8269D830: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8269D834: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8269D838: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8269D83C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8269D840: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8269D844: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8269D848: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8269D84C: 4BDC95D5  bl 0x82466e20
	ctx.lr = 0x8269D850;
	sub_82466E20(ctx, base);
	// 8269D850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8269D854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8269D858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8269D85C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


